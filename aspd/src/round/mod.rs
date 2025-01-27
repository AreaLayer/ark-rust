

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Context;
use bdk_bitcoind_rpc::bitcoincore_rpc::RawTx;
use bitcoin::{Amount, FeeRate, OutPoint, Psbt, Txid};
use bitcoin::hashes::Hash;
use bitcoin::locktime::absolute::LockTime;
use bitcoin::secp256k1::{rand, schnorr, Keypair, PublicKey};
use tokio::sync::OwnedMutexGuard;
use tokio::time::Instant;
use ark::{musig, BlockHeight, OffboardRequest, Vtxo, VtxoId, VtxoRequest};
use ark::connectors::ConnectorChain;
use ark::musig::{MusigPubNonce, MusigSecNonce};
use ark::rounds::RoundEvent;
use ark::tree::signed::{CachedSignedVtxoTree, UnsignedVtxoTree, VtxoTreeSpec};

use crate::{App, SECP};

#[derive(Debug)]
pub enum RoundInput {
	RegisterPayment {
		inputs: Vec<VtxoId>,
		vtxo_requests: Vec<VtxoRequest>,
		/// One set of nonces per vtxo request.
		cosign_pub_nonces: Vec<Vec<musig::MusigPubNonce>>,
		offboards: Vec<OffboardRequest>,
	},
	VtxoSignatures {
		pubkey: PublicKey,
		signatures: Vec<musig::MusigPartialSignature>,
	},
	ForfeitSignatures {
		signatures: Vec<(VtxoId, Vec<musig::MusigPubNonce>, Vec<musig::MusigPartialSignature>)>,
	},
}

fn validate_forfeit_sigs(
	connectors: &ConnectorChain,
	user_nonces: &[musig::MusigPubNonce],
	part_sigs: &[musig::MusigPartialSignature],
) -> anyhow::Result<()> {
	if user_nonces.len() != connectors.len() || part_sigs.len() != connectors.len() {
		bail!("not enough forfeit signatures provided");
	}

	Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VtxoParticipant {
	pub req: VtxoRequest,
	pub nonces: Vec<MusigPubNonce>,
}

pub struct RoundData {
	max_output_vtxos: usize,
	nb_vtxo_nonces: usize,
	offboard_feerate: FeeRate,
}

pub struct CollectingPayments {
	round_id: u64,
	attempt: usize,
	round_data: RoundData,

	/// All inputs that have participated, but might have dropped out.
	all_locked_inputs: HashSet<VtxoId>,
	cosign_key: Keypair,
	allowed_inputs: Option<HashSet<VtxoId>>,
	all_inputs: HashMap<VtxoId, Vtxo>,
	all_outputs: Vec<VtxoParticipant>,
	/// Keep track of which input vtxos belong to which inputs.
	inputs_per_cosigner: HashMap<PublicKey, Vec<VtxoId>>,
	all_offboards: Vec<OffboardRequest>,

	attempt_start: Instant,
	proceed: bool,
}

impl CollectingPayments {
	fn new(
		round_id: u64,
		attempt: usize,
		round_data: RoundData,
		all_locked_inputs: HashSet<VtxoId>,
		allowed_inputs: Option<HashSet<VtxoId>>,
	) -> CollectingPayments {
		CollectingPayments {
			round_id,
			attempt,
			round_data,
			all_locked_inputs,
			allowed_inputs,

			// Generate a one-time use signing key.
			cosign_key: Keypair::new(&SECP, &mut rand::thread_rng()),

			all_inputs: HashMap::new(),
			all_outputs: Vec::new(),
			inputs_per_cosigner: HashMap::new(),
			all_offboards: Vec::new(),

			attempt_start: Instant::now(),
			proceed: false,
		}
	}

	fn first_attempt(&self) -> bool {
		assert_eq!(self.attempt == 0, self.allowed_inputs.is_none());
		self.attempt == 0
	}

	/// Returns whether there are no valid inputs left in the round
	/// and we need to start a new round.
	fn need_new_round(&self) -> bool {
		!self.first_attempt() && self.allowed_inputs.as_ref().unwrap().is_empty()
	}

	fn have_payments(&self) -> bool {
		!self.all_inputs.is_empty() && (!self.all_outputs.is_empty() || !self.all_offboards.is_empty())
	}

	fn validate_payment_amounts(
		&self,
		inputs: &[Vtxo],
		outputs: &[VtxoRequest],
		offboards: &[OffboardRequest],
	) -> anyhow::Result<()> {
		let mut in_set = HashSet::with_capacity(inputs.len());
		let mut in_sum = Amount::ZERO;
		for input in inputs {
			in_sum += input.amount();
			if in_sum > Amount::MAX_MONEY{
				bail!("total input amount overflow");
			}
			if !in_set.insert(input.id()) {
				bail!("duplicate input");
			}
		}

		let mut out_sum = Amount::ZERO;
		for output in outputs {
			out_sum += output.amount;
			if out_sum > in_sum {
				bail!("total output amount exceeds total input amount");
			}
		}
		for offboard in offboards {
			let fee = match offboard.fee(self.round_data.offboard_feerate) {
				Some(v) => v,
				None => bail!("invalid offboard address"),
			};
			out_sum += offboard.amount + fee;
			if out_sum > in_sum {
				bail!("total output amount (with offboards) exceeds total input amount");
			}
		}

		Ok(())
	}

	/// This methods does checks on the user input that can be done fast and without
	/// the need to fetch the input vtxos.
	fn validate_payment_data(
		&self,
		inputs: &[VtxoId],
		outputs: &[VtxoRequest],
		cosign_pub_nonces: &[Vec<musig::MusigPubNonce>],
	) -> anyhow::Result<()> {
		if self.all_outputs.len() + outputs.len() > self.round_data.max_output_vtxos {
			warn!("Got payment we don't have space for, dropping");
			bail!("not enough outputs left in this round, try next round");
		}
		//TODO(stevenroose) verify ownership over inputs

		let mut unique_inputs = HashSet::with_capacity(inputs.len());
		for input in inputs {
			if !unique_inputs.insert(input) {
				slog!(RoundUserVtxoDuplicateInput, round_id: self.round_id, attempt: self.attempt,
					vtxo: *input,
				);
				bail!("user provided duplicate inputs");
			}
			if self.all_inputs.contains_key(input) {
				slog!(RoundUserVtxoAlreadyRegistered, round_id: self.round_id, attempt: self.attempt,
					vtxo: *input,
				);
				bail!("vtxo {input} already registered");
			}
		}

		if let Some(ref allowed) = self.allowed_inputs {
			// This means we're not trying first time and we filter inputs.
			if let Some(bad) = inputs.iter().find(|i| !allowed.contains(&i)) {
				slog!(RoundUserVtxoNotAllowed, round_id: self.round_id, attempt: self.attempt,
					vtxo: *bad,
				);
				bail!("input vtxo {} has been banned for this round", bad);
			}
		}

		if outputs.len() != cosign_pub_nonces.len() {
			bail!("incorrect number of sets of cosign nonces");
		}
		if cosign_pub_nonces.iter().any(|n| n.len() != self.round_data.nb_vtxo_nonces) {
			bail!("incorrect number of cosign nonces per set");
		}
		for out in outputs {
			if self.inputs_per_cosigner.contains_key(&out.cosign_pk) {
				bail!("duplicate cosign key {}", out.cosign_pk);
			}
		}

		Ok(())
	}

	async fn process_payment(
		&mut self,
		app: &App,
		inputs: Vec<VtxoId>,
		vtxo_requests: Vec<VtxoRequest>,
		cosign_pub_nonces: Vec<Vec<musig::MusigPubNonce>>,
		offboards: Vec<OffboardRequest>,
	) -> anyhow::Result<()> {
		self.validate_payment_data(&inputs, &vtxo_requests, &cosign_pub_nonces)?;

		// If some inputs already have been submitted in a previous attempt,
		// no need to lock/check them again.
		let new_inputs = inputs.iter().copied().filter(|v| {
			!self.all_locked_inputs.contains(v)
		}).collect::<Vec<_>>();
		if let Err(id) = app.atomic_check_put_vtxo_in_flux(&new_inputs).await {
			slog!(RoundUserVtxoInFlux, round_id: self.round_id, attempt: self.attempt, vtxo: id);
			bail!("vtxo {id} already in flux");
		}

		// Check if the input vtxos exist and are unspent.
		let input_vtxos = match app.db.check_fetch_unspent_vtxos(&inputs) {
			Ok(i) => i,
			Err(e) => {
				let id = e.downcast_ref::<VtxoId>().cloned();
				slog!(RoundUserVtxoUnknown, round_id: self.round_id, vtxo: id);
				app.release_vtxos_in_flux(&new_inputs).await;
				bail!("unknown vtxo {:?}", id);
			},
		};

		if let Err(e) = self.validate_payment_amounts(&input_vtxos, &vtxo_requests, &offboards) {
			slog!(RoundPaymentRegistrationFailed, round_id: self.round_id,
				attempt: self.attempt, error: e.to_string(),
			);
			app.release_vtxos_in_flux(&new_inputs).await;
			bail!("registration failed: {e}");
		}

		// Finally we are done
		self.register_payment(input_vtxos, vtxo_requests, cosign_pub_nonces, offboards);

		Ok(())
	}

	fn register_payment(
		&mut self,
		inputs: Vec<Vtxo>,
		vtxo_requests: Vec<VtxoRequest>,
		cosign_pub_nonces: Vec<Vec<musig::MusigPubNonce>>,
		offboards: Vec<OffboardRequest>,
	) {
		slog!(RoundPaymentRegistered, round_id: self.round_id, attempt: self.attempt,
			nb_inputs: inputs.len(), nb_outputs: vtxo_requests.len(), nb_offboards: offboards.len(),
		);

		// If we're adding inputs for the first time, also add them to all_locked_inputs.
		if self.first_attempt() {
			self.all_locked_inputs.extend(inputs.iter().map(|v| v.id()));
		}

		let input_ids = inputs.iter().map(|v| v.id()).collect::<Vec<_>>();
		self.all_inputs.extend(inputs.into_iter().map(|v| (v.id(), v)));

		assert_eq!(vtxo_requests.len(), cosign_pub_nonces.len());
		self.all_outputs.reserve(vtxo_requests.len());
		self.inputs_per_cosigner.reserve(vtxo_requests.len());
		for (output, nonces) in vtxo_requests.into_iter().zip(cosign_pub_nonces) {
			assert!(self.inputs_per_cosigner.insert(output.cosign_pk, input_ids.clone()).is_none());
			self.all_outputs.push(VtxoParticipant { req: output, nonces });
		}

		self.all_offboards.extend(offboards);

		// Check whether our round is full.
		const REGULAR_PAYMENT_NB_OUTPUTS: usize = 2;
		if self.all_outputs.len() + REGULAR_PAYMENT_NB_OUTPUTS >= self.round_data.max_output_vtxos {
			slog!(FullRound, round_id: self.round_id, attempt: self.attempt,
				nb_outputs: self.all_outputs.len(), max_output_vtxos: self.round_data.max_output_vtxos,
			);
			self.proceed = true;
		}
	}

	async fn progress(mut self, app: &App) -> SigningVtxoTree {
		let tip = app.chain_tip().await.height;
		let expiry_height = tip + app.config.vtxo_expiry_delta as BlockHeight;
		slog!(ConstructingRoundVtxoTree, round_id: self.round_id, attempt: self.attempt,
			tip_block_height: tip, vtxo_expiry_block_height: expiry_height,
		);

		// Since it's possible in testing that we only have to do offboards,
		// and since it's pretty annoying to deal with the case of no vtxos,
		// if there are no vtxos, we will just add a fake vtxo for the ASP.
		// In practice, in later versions, it is very likely that the ASP
		// will actually want to create change vtxos, so temporarily, this
		// dummy vtxo will be a placeholder for a potential change vtxo.
		let mut change_vtxo = if self.all_outputs.is_empty() {
			lazy_static::lazy_static! {
				static ref UNSPENDABLE: PublicKey =
					"031575a4c3ad397590ccf7aa97520a60635c3215047976afb9df220bc6b4241b0d".parse().unwrap();
			}
			let cosign_key = Keypair::new(&SECP, &mut rand::thread_rng());
			let (cosign_sec_nonces, cosign_pub_nonces) = {
				let mut secs = Vec::with_capacity(app.config.nb_round_nonces);
				let mut pubs = Vec::with_capacity(app.config.nb_round_nonces);
				for _ in 0..app.config.nb_round_nonces {
					let (s, p) = musig::nonce_pair(&cosign_key);
					secs.push(s);
					pubs.push(p);
				}
				(secs, pubs)
			};
			let req = VtxoRequest {
				pubkey: *UNSPENDABLE,
				amount: ark::fee::DUST,
				cosign_pk: cosign_key.public_key(),
			};
			self.all_outputs.push(VtxoParticipant {
				req: req.clone(),
				nonces: cosign_pub_nonces.clone(),
			});
			Some((req, cosign_key, cosign_sec_nonces, cosign_pub_nonces))
		} else {
			None
		};

		let vtxos_spec = VtxoTreeSpec::new(
			self.all_outputs.iter().map(|p| p.req.clone()).collect(),
			app.asp_key.public_key(),
			self.cosign_key.public_key(),
			expiry_height,
			app.config.vtxo_exit_delta,
		);
		//TODO(stevenroose) this is inefficient, improve this with direct getter
		let nb_nodes = vtxos_spec.nb_nodes();
		assert!(nb_nodes <= app.config.nb_round_nonces);
		let connector_output = ConnectorChain::output(
			self.all_inputs.len(), app.asp_key.public_key(),
		);

		// Build round tx.
		//TODO(stevenroose) think about if we can release lock sooner
		let mut wallet_lock = app.wallet.clone().lock_owned().await;
		let round_tx_psbt = {
			let mut b = wallet_lock.build_tx();
			b.ordering(bdk_wallet::TxOrdering::Untouched);
			b.nlocktime(LockTime::from_height(tip as u32).expect("actual height"));
			b.add_recipient(vtxos_spec.round_tx_spk(), vtxos_spec.total_required_value());
			b.add_recipient(connector_output.script_pubkey, connector_output.value);
			for offb in &self.all_offboards {
				b.add_recipient(offb.script_pubkey.clone(), offb.amount);
			}
			b.fee_rate(app.config.round_tx_feerate);
			b.finish().expect("bdk failed to create round tx")
		};
		let unsigned_round_tx = round_tx_psbt.clone().extract_tx()
			.expect("failed to extract tx from psbt");
		let round_txid = unsigned_round_tx.compute_txid();
		let vtxos_utxo = OutPoint::new(round_txid, 0);

		// Generate vtxo nonces and combine with user's nonces.
		let (cosign_sec_nonces, cosign_pub_nonces) = {
			let mut secs = Vec::with_capacity(nb_nodes);
			let mut pubs = Vec::with_capacity(nb_nodes);
			for _ in 0..nb_nodes {
				let (s, p) = musig::nonce_pair(&self.cosign_key);
				secs.push(s);
				pubs.push(p);
			}
			(secs, pubs)
		};
		let user_cosign_nonces = self.all_outputs.into_iter().map(|req| {
			(req.req.cosign_pk, req.nonces)
		}).collect::<HashMap<_, _>>();
		let cosign_agg_nonces = vtxos_spec.calculate_cosign_agg_nonces(
			&user_cosign_nonces, &cosign_pub_nonces,
		);

		// Send out vtxo proposal to signers.
		app.rounds().round_event_tx.send(RoundEvent::VtxoProposal {
			round_id: self.round_id,
			unsigned_round_tx: unsigned_round_tx.clone(),
			vtxos_spec: vtxos_spec.clone(),
			cosign_agg_nonces: cosign_agg_nonces.clone(),
		}).expect("round event channel broken");

		let unsigned_vtxo_tree = vtxos_spec.into_unsigned_tree(vtxos_utxo);
		let mut cosign_part_sigs = HashMap::with_capacity(unsigned_vtxo_tree.nb_leaves());
		let mut proceed = false;

		// first add our own change (or dummy) vtxo
		if let Some((req, pk, sec, _pub)) = change_vtxo.take() {
			let sigs = unsigned_vtxo_tree.cosign_branch(
				&cosign_agg_nonces,
				&req,
				&pk,
				sec,
			).expect("we're in the tree");
			cosign_part_sigs.insert(pk.public_key(), sigs);
			proceed = true;
		}

		SigningVtxoTree {
			round_id: self.round_id,
			round_data: self.round_data,
			attempt: self.attempt,
			expiry_height,
			cosign_key: self.cosign_key,
			cosign_sec_nonces,
			cosign_pub_nonces,
			cosign_agg_nonces,
			all_inputs: self.all_inputs,
			all_locked_inputs: self.all_locked_inputs,
			cosign_part_sigs,
			unsigned_vtxo_tree,
			wallet_lock,
			round_tx_psbt,
			round_txid,
			user_cosign_nonces,
			inputs_per_cosigner: self.inputs_per_cosigner,
			attempt_start: self.attempt_start,
			proceed,
		}
	}
}

pub struct SigningVtxoTree {
	round_id: u64,
	attempt: usize,
	round_data: RoundData,
	expiry_height: BlockHeight,

	cosign_key: Keypair,
	cosign_sec_nonces: Vec<MusigSecNonce>,
	cosign_pub_nonces: Vec<MusigPubNonce>,
	cosign_part_sigs: HashMap<PublicKey, Vec<musig::MusigPartialSignature>>,
	cosign_agg_nonces: Vec<musig::MusigAggNonce>,
	unsigned_vtxo_tree: UnsignedVtxoTree,
	wallet_lock: OwnedMutexGuard<bdk_wallet::Wallet>,
	round_tx_psbt: Psbt,
	round_txid: Txid,

	// data from earlier
	all_inputs: HashMap<VtxoId, Vtxo>,
	user_cosign_nonces: HashMap<PublicKey, Vec<musig::MusigPubNonce>>,
	inputs_per_cosigner: HashMap<PublicKey, Vec<VtxoId>>,
	/// All inputs that have participated, but might have dropped out.
	all_locked_inputs: HashSet<VtxoId>,

	attempt_start: Instant,

	proceed: bool,
}

impl SigningVtxoTree {
	pub fn register_signature(
		&mut self,
		pubkey: PublicKey,
		signatures: Vec<musig::MusigPartialSignature>,
	) -> anyhow::Result<()> {
		// Check for duplicates.
		if self.cosign_part_sigs.contains_key(&pubkey) {
			trace!("User with pubkey {} submitted partial vtxo sigs again", pubkey);
			bail!("duplicate signatures for pubkey");
		}

		let req = match self.unsigned_vtxo_tree.spec.vtxos.iter().find(|v| v.cosign_pk == pubkey) {
			Some(r) => r,
			None => {
				trace!("Received signatures from non-signer: {}", pubkey);
				bail!("pubkey is not part of cosigner group");
			},
		};
		slog!(RoundVtxoSignaturesRegistered, round_id: self.round_id, attempt: self.attempt,
			nb_vtxo_signatures: signatures.len(), cosigner: pubkey,
		);

		let res = self.unsigned_vtxo_tree.verify_branch_cosign_partial_sigs(
			&self.cosign_agg_nonces,
			req,
			&self.user_cosign_nonces.get(&req.cosign_pk).expect("vtxo part of round"),
			&signatures,
		);
		if let Err(e) = res {
			debug!("Received invalid partial vtxo sigs from signer: {}: {}", pubkey, e);
			bail!("invalid partial signatures: {}", e);
		}

		self.cosign_part_sigs.insert(pubkey, signatures);

		// Stop the loop once we have all.
		if self.cosign_part_sigs.len() == self.unsigned_vtxo_tree.nb_leaves() {
			self.proceed = true;
		}
		Ok(())
	}

	fn restart(self) -> CollectingPayments {
		let mut allowed_inputs = self.all_inputs.keys().copied().collect::<HashSet<_>>();
		for (pk, vtxos) in self.inputs_per_cosigner.iter() {
			if !self.cosign_part_sigs.contains_key(pk) {
				// Disallow all inputs by this cosigner.
				slog!(DroppingLateVtxoSignatureVtxos, round_id: self.round_id,
					attempt: self.attempt, disallowed_vtxos: vtxos.clone(),
				);
				for id in vtxos {
					allowed_inputs.remove(id);
				}
			}
		}
		CollectingPayments::new(
			self.round_id,
			self.attempt + 1,
			self.round_data,
			self.all_locked_inputs,
			Some(allowed_inputs),
		)
	}

	fn progress(self, app: &App) -> SigningForfeits {
		// Combine the vtxo signatures.
		let combine_signatures_start = Instant::now();
		let asp_cosign_sigs = self.unsigned_vtxo_tree.cosign_tree(
			&self.cosign_agg_nonces,
			&self.cosign_key,
			self.cosign_sec_nonces,
		);
		debug_assert_eq!(self.unsigned_vtxo_tree.verify_all_cosign_partial_sigs(
			self.cosign_key.public_key(),
			&self.cosign_agg_nonces,
			&self.cosign_pub_nonces,
			&asp_cosign_sigs,
		), Ok(()));
		let cosign_sigs = self.unsigned_vtxo_tree.combine_partial_signatures(
			&self.cosign_agg_nonces,
			&self.cosign_part_sigs,
			asp_cosign_sigs,
		).expect("failed to combine partial vtxo cosign signatures");
		debug_assert_eq!(self.unsigned_vtxo_tree.verify_cosign_sigs(&cosign_sigs), Ok(()));

		// Then construct the final signed vtxo tree.
		let signed_vtxos = self.unsigned_vtxo_tree
			.into_signed_tree(cosign_sigs)
			.into_cached_tree();
		slog!(CreatedSignedVtxoTree, round_id: self.round_id, attempt: self.attempt,
			nb_vtxo_signatures: signed_vtxos.spec.cosign_sigs.len(),
			duration: Instant::now().duration_since(combine_signatures_start),
		);

		// ****************************************************************
		// * Broadcast signed vtxo tree and gather forfeit signatures
		// ****************************************************************

		// Prepare nonces for forfeit txs.
		// We need to prepare N nonces for each of N inputs.
		let mut forfeit_pub_nonces = HashMap::with_capacity(self.all_inputs.len());
		let mut forfeit_sec_nonces = HashMap::with_capacity(self.all_inputs.len());
		for id in self.all_inputs.keys() {
			let mut secs = Vec::with_capacity(self.all_inputs.len());
			let mut pubs = Vec::with_capacity(self.all_inputs.len());
			for _ in 0..self.all_inputs.len() {
				let (s, p) = musig::nonce_pair(&app.asp_key);
				secs.push(s);
				pubs.push(p);
			}
			forfeit_pub_nonces.insert(*id, pubs);
			forfeit_sec_nonces.insert(*id, secs);
		}

		// Send out round proposal to signers.
		app.rounds().round_event_tx.send(RoundEvent::RoundProposal {
			round_id: self.round_id,
			cosign_sigs: signed_vtxos.spec.cosign_sigs.clone(),
			forfeit_nonces: forfeit_pub_nonces.clone(),
		}).expect("round event channel broken");

		let conns_utxo = OutPoint::new(self.round_txid, 1);
		let connectors = ConnectorChain::new(
			self.all_inputs.len(), conns_utxo, app.asp_key.public_key(),
		);

		SigningForfeits {
			round_id: self.round_id,
			attempt: self.attempt,
			round_data: self.round_data,
			expiry_height: self.expiry_height,
			forfeit_sec_nonces: Some(forfeit_sec_nonces),
			forfeit_pub_nonces,
			forfeit_part_sigs: HashMap::with_capacity(self.all_inputs.len()),
			forfeit_sigs: None,
			signed_vtxos,
			all_inputs: self.all_inputs,
			all_locked_inputs: self.all_locked_inputs,
			connectors,
			wallet_lock: self.wallet_lock,
			round_tx_psbt: self.round_tx_psbt,
			attempt_start: self.attempt_start,
			proceed: false,
		}
	}
}

pub struct SigningForfeits {
	round_id: u64,
	attempt: usize,
	round_data: RoundData,
	expiry_height: BlockHeight,

	forfeit_sec_nonces: Option<HashMap<VtxoId, Vec<MusigSecNonce>>>,
	forfeit_pub_nonces: HashMap<VtxoId, Vec<MusigPubNonce>>,
	forfeit_part_sigs: HashMap<VtxoId, (Vec<musig::MusigPubNonce>, Vec<musig::MusigPartialSignature>)>,
	forfeit_sigs: Option<HashMap<VtxoId, Vec<schnorr::Signature>>>,

	// data from earlier
	signed_vtxos: CachedSignedVtxoTree,
	all_inputs: HashMap<VtxoId, Vtxo>,
	/// All inputs that have participated, but might have dropped out.
	all_locked_inputs: HashSet<VtxoId>,

	// other public data
	connectors: ConnectorChain,

	wallet_lock: OwnedMutexGuard<bdk_wallet::Wallet>,
	round_tx_psbt: Psbt,
	attempt_start: Instant,

	proceed: bool,
}

impl SigningForfeits {
	pub fn register_forfeits(
		&mut self,
		signatures: Vec<(VtxoId, Vec<musig::MusigPubNonce>, Vec<musig::MusigPartialSignature>)>,
	) -> anyhow::Result<()> {
		slog!(ReceivedForfeitSignatures, round_id: self.round_id, attempt: self.attempt,
			nb_forfeits: signatures.len(), vtxo_ids: signatures.iter().map(|v| v.0).collect::<Vec<_>>(),
		);

		for (id, nonces, sigs) in signatures {
			if let Some(_vtxo) = self.all_inputs.get(&id) {
				match validate_forfeit_sigs(
					&self.connectors,
					&nonces,
					&sigs,
				) {
					Ok(()) => { self.forfeit_part_sigs.insert(id, (nonces, sigs)); },
					Err(e) => debug!("Invalid forfeit sigs for {}: {}", id, e),
				}
			} else {
				slog!(UnknownForfeitSignature, round_id: self.round_id,
					attempt: self.attempt, vtxo_id: id,
				);
			}
		}

		// Check whether we have all and can skip the loop.
		if self.forfeit_part_sigs.len() == self.all_inputs.len() {
			self.proceed = true;
		}
		Ok(())
	}

	fn restart_missing_forfeits(self, missing: Option<HashSet<VtxoId>>) -> CollectingPayments {
		let allowed_inputs = if let Some(missing) = missing {
			for input in &missing {
				slog!(MissingForfeits, round_id: self.round_id,
					attempt: self.attempt, input: *input,
				);
			}

			self.all_inputs.keys().copied()
				.filter(|v| !missing.contains(v))
				.collect()
		} else {
			self.all_inputs.keys().copied().filter(|v| {
				if !self.forfeit_part_sigs.contains_key(v) {
					slog!(MissingForfeits, round_id: self.round_id,
						attempt: self.attempt, input: *v,
					);
					false
				} else {
					true
				}
			}).collect()
		};
		slog!(RestartMissingForfeits, round_id: self.round_id, attempt: self.attempt);
		CollectingPayments::new(
			self.round_id,
			self.attempt + 1,
			self.round_data,
			self.all_locked_inputs,
			Some(allowed_inputs),
		)
	}

	fn check_forfeits(mut self, app: &App) -> RoundState {
		// Finish the forfeit signatures.
		let mut forfeit_sec_nonces = self.forfeit_sec_nonces.take().unwrap();
		let mut forfeit_sigs = HashMap::with_capacity(self.all_inputs.len());
		let mut missing = HashSet::new();
		for (id, vtxo) in &self.all_inputs {
			if let Some((user_nonces, partial_sigs)) = self.forfeit_part_sigs.get(id) {
				let sec_nonces = forfeit_sec_nonces.remove(id).unwrap().into_iter();
				let pub_nonces = self.forfeit_pub_nonces.get(id).unwrap();
				let connectors = self.connectors.connectors();
				let mut sigs = Vec::with_capacity(self.all_inputs.len());
				for (i, (conn, sec)) in connectors.zip(sec_nonces.into_iter()).enumerate() {
					let (sighash, _) = ark::forfeit::forfeit_sighash(&vtxo, conn);
					let agg_nonce = musig::nonce_agg(&[&user_nonces[i], &pub_nonces[i]]);
					let (_, sig) = musig::partial_sign(
						[app.asp_key.public_key(), vtxo.spec().user_pubkey],
						agg_nonce,
						&app.asp_key,
						sec,
						sighash.to_byte_array(),
						Some(vtxo.spec().exit_taptweak().to_byte_array()),
						Some(&[&partial_sigs[i]]),
					);
					sigs.push(sig.expect("should be signed"));
				}
				forfeit_sigs.insert(*id, sigs);
			} else {
				missing.insert(*id);
			}
		}

		if !missing.is_empty() {
			RoundState::CollectingPayments(self.restart_missing_forfeits(Some(missing)))
		} else {
			self.forfeit_sigs = Some(forfeit_sigs);
			RoundState::SigningForfeits(self)
		}
	}

	async fn finish(
		mut self,
		app: &App,
	) -> anyhow::Result<()> {
		// Sign the on-chain tx.
		let sign_start = Instant::now();
		let opts = bdk_wallet::SignOptions {
			trust_witness_utxo: true,
			..Default::default()
		};
		let finalized = self.wallet_lock.sign(&mut self.round_tx_psbt, opts)?;
		assert!(finalized);
		let signed_round_tx = self.round_tx_psbt.extract_tx()?;
		let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Unix epoch is in the past").as_secs();
		self.wallet_lock.apply_unconfirmed_txs([(Arc::new(signed_round_tx.clone()), now)]);
		if let Some(change) = self.wallet_lock.take_staged() {
			app.db.store_changeset(&change).await?;
		}
		drop(self.wallet_lock); // we no longer need the lock
		slog!(BroadcastingFinalizedRoundTransaction, round_id: self.round_id, attempt: self.attempt,
			tx_hex: signed_round_tx.raw_hex(), signing_time: Instant::now().duration_since(sign_start),
		);
		let signed_round_tx = app.txindex.broadcast_tx(signed_round_tx).await;

		// Send out the finished round to users.
		trace!("Sending out finish event.");
		app.rounds().round_event_tx.send(RoundEvent::Finished {
			round_id: self.round_id,
			signed_round_tx: signed_round_tx.tx.clone(),
		}).expect("round event channel broken");

		// Store forfeit txs and round info in database.
		let mut forfeit_sigs = self.forfeit_sigs.take().unwrap();
		for (id, vtxo) in self.all_inputs {
			let forfeit_sigs = forfeit_sigs.remove(&id).unwrap();
			slog!(StoringForfeitVtxo, round_id: self.round_id, attempt: self.attempt,
				out_point: vtxo.point(),
			);
			app.db.set_vtxo_forfeited(id, forfeit_sigs)?;
		}
		app.release_vtxos_in_flux(self.all_locked_inputs).await;

		trace!("Storing round result");
		app.txindex.register_batch(self.signed_vtxos.all_signed_txs().iter().cloned()).await;
		app.txindex.register_batch(self.connectors.iter_signed_txs(&app.asp_key)).await;
		app.db.store_round(signed_round_tx.tx.clone(), self.signed_vtxos, self.connectors.len())?;

		slog!(RoundFinished, round_id: self.round_id, attempt: self.attempt,
			txid: signed_round_tx.txid, vtxo_expiry_block_height: self.expiry_height,
			duration: Instant::now().duration_since(self.attempt_start),
		);

		// Sync our wallet so that it sees the broadcasted tx.
		app.sync_onchain_wallet().await.context("error syncing onchain wallet")?;

		Ok(())
	}
}

pub enum RoundState {
	CollectingPayments(CollectingPayments),
	SigningVtxoTree(SigningVtxoTree),
	SigningForfeits(SigningForfeits),
}

impl RoundState {
	fn proceed(&self) -> bool {
		match self {
			Self::CollectingPayments(s) => s.proceed,
			Self::SigningVtxoTree(s) => s.proceed,
			Self::SigningForfeits(s) => s.proceed,
		}
	}

	fn collecting_payments(&mut self) -> &mut CollectingPayments {
		match self {
			RoundState::CollectingPayments(s) => s,
			_ => panic!("wrong state"),
		}
	}
	fn signing_vtxo_tree(&mut self) -> &mut SigningVtxoTree {
		match self {
			RoundState::SigningVtxoTree(s) => s,
			_ => panic!("wrong state"),
		}
	}
	fn into_signing_vtxo_tree(self) -> SigningVtxoTree {
		match self {
			RoundState::SigningVtxoTree(s) => s,
			_ => panic!("wrong state"),
		}
	}
	fn signing_forfeits(&mut self) -> &mut SigningForfeits {
		match self {
			RoundState::SigningForfeits(s) => s,
			_ => panic!("wrong state"),
		}
	}
	fn into_signing_forfeits(self) -> SigningForfeits {
		match self {
			RoundState::SigningForfeits(s) => s,
			_ => panic!("wrong state"),
		}
	}

	async fn progress(self, app: &App) -> Self {
		match self {
			Self::CollectingPayments(s) => s.progress(app).await.into(),
			Self::SigningVtxoTree(s) => s.progress(app).into(),
			Self::SigningForfeits(_) => panic!("can't progress from signingforfeits"),
		}
	}
}

impl From<CollectingPayments> for RoundState {
	fn from(s: CollectingPayments) -> RoundState {
		RoundState::CollectingPayments(s)
	}
}

impl From<SigningVtxoTree> for RoundState {
	fn from(s: SigningVtxoTree) -> RoundState {
		RoundState::SigningVtxoTree(s)
	}
}

impl From<SigningForfeits> for RoundState {
	fn from(s: SigningForfeits) -> RoundState {
		RoundState::SigningForfeits(s)
	}
}

/// This method is called from a tokio thread so it can be long-lasting.
pub async fn run_round_coordinator(
	app: &App,
	mut round_input_rx: tokio::sync::mpsc::UnboundedReceiver<RoundInput>,
	mut round_trigger_rx: tokio::sync::mpsc::Receiver<()>,
) -> anyhow::Result<()> {
	let mut shutdown = app.shutdown_channel.subscribe();
	// Whether we should sync the onchain wallet at the next round attempt.
	let mut sync_next_attempt = true;

	'round: loop {
		// Sleep for the round interval, but discard all incoming messages.
		tokio::pin! { let timeout = tokio::time::sleep(app.config.round_interval); }
		'sleep: loop {
			tokio::select! {
				() = &mut timeout => break 'sleep,
				Some(()) = round_trigger_rx.recv() => {
					info!("Starting round based on admin RPC trigger");
					sync_next_attempt = false; // start round fast
					break 'sleep;
				},
				_ = round_input_rx.recv() => {},
				_ = shutdown.recv() => {
					info!("Shutdown signal received. Exiting round coordinator loop...");
					return Ok(());
				}
			}
		}

		let round_id = (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() /
			app.config.round_interval.as_millis()) as u64;
		slog!(RoundStarted, round_id);

		// Start new round, announce.
		let offboard_feerate = app.config.round_tx_feerate;
		app.rounds().round_event_tx.send(RoundEvent::Start {
			round_id,
			offboard_feerate,
		}).expect("round event channel broken");

		// Allocate this data once per round so that we can keep them
		// Perhaps we could even keep allocations between all rounds, but time
		// in between attempts is way more critial than in between rounds.

		let round_data = RoundData {
			// The maximum number of output vtxos per round based on the max number
			// of vtxo tree nonces we require users to provide.
			max_output_vtxos: (app.config.nb_round_nonces * 3 ) / 4,
			nb_vtxo_nonces: app.config.nb_round_nonces,
			offboard_feerate: offboard_feerate,
		};
		let mut round_state = RoundState::CollectingPayments(
			CollectingPayments::new(round_id, 0, round_data, HashSet::new(), None)
		);

		// In this loop we will try to finish the round and make new attempts.
		'attempt: loop {
			let attempt = round_state.collecting_payments().attempt;
			slog!(AttemptingRound, round_id, attempt);
			if sync_next_attempt {
				app.sync_onchain_wallet().await.context("error syncing onchain wallet")?;
			}
			sync_next_attempt = true;

			app.rounds().round_event_tx.send(RoundEvent::Attempt {
				round_id,
				attempt: attempt as u64,
			}).expect("round event channel broken");

			// Start receiving payments.
			let receive_payments_start = Instant::now();
			tokio::pin! { let timeout = tokio::time::sleep(app.config.round_submit_time); }
			'receive: loop {
				tokio::select! {
					() = &mut timeout => break 'receive,
					input = round_input_rx.recv() => match input.expect("broken channel") {
						RoundInput::RegisterPayment {
							inputs, vtxo_requests, cosign_pub_nonces, offboards,
						} => {
							let state = round_state.collecting_payments();
							if let Err(e) = state.process_payment(
								app, inputs, vtxo_requests, cosign_pub_nonces, offboards,
							).await {
								debug!("error processing payment: {e}");
								continue 'receive;
							}

							if round_state.proceed() {
								break 'receive;
							}
						},
						_ => trace!("unexpected message"),
					}
				}
			}
			if !round_state.collecting_payments().have_payments() {
				slog!(NoRoundPayments, round_id, attempt,
					max_round_submit_time: app.config.round_submit_time,
				);
				continue 'round;
			}
			let receive_payment_duration = Instant::now().duration_since(receive_payments_start);
			slog!(ReceivedRoundPayments, round_id, attempt,
				nb_inputs: round_state.collecting_payments().all_inputs.len(),
				nb_outputs: round_state.collecting_payments().all_outputs.len(),
				duration: receive_payment_duration, max_round_submit_time: app.config.round_submit_time,
			);


			// ****************************************************************
			// * Vtxo tree construction and signing
			// *
			// * - We will always store vtxo tx data from top to bottom,
			// *   meaning from the root tx down to the leaves.
			// ****************************************************************

			let send_vtxo_proposal_start = Instant::now();
			round_state = round_state.progress(app).await;

			// Wait for signatures from users.
			slog!(AwaitingRoundSignatures, round_id, attempt,
				max_round_sign_time: app.config.round_sign_time,
				duration_since_sending: Instant::now().duration_since(send_vtxo_proposal_start),
			);
			let vtxo_signatures_receive_start = Instant::now();
			tokio::pin! { let timeout = tokio::time::sleep(app.config.round_sign_time); }
			'receive: loop {
				if round_state.proceed() {
					break 'receive;
				}
				tokio::select! {
					_ = &mut timeout => {
						warn!("Timed out receiving vtxo partial signatures.");
						let new = round_state.into_signing_vtxo_tree().restart();
						if new.need_new_round() {
							app.release_vtxos_in_flux(new.all_locked_inputs).await;
							continue 'round;
						} else {
							round_state = new.into();
							continue 'attempt;
						}
					},
					input = round_input_rx.recv() => {
						let state = round_state.signing_vtxo_tree();
						match input.expect("broken channel") {
							RoundInput::VtxoSignatures { pubkey, signatures } => {
								if let Err(e) = state.register_signature(pubkey, signatures) {
									slog!(VtxoSignatureRegistrationFailed, round_id, attempt, error: e.to_string());
									continue 'receive;
								}
							},
							_ => trace!("unexpected message"),
						}
					}
				}
			}
			slog!(ReceivedRoundVtxoSignatures, round_id, attempt, duration: Instant::now().duration_since(vtxo_signatures_receive_start),
				max_round_sign_time: app.config.round_sign_time,
			);

			let send_round_proposal_start = Instant::now();
			round_state = round_state.progress(&app).await;

			// Wait for signatures from users.
			slog!(AwaitingRoundForfeits, round_id, attempt,
				max_round_sign_time: app.config.round_sign_time,
				duration_since_sending: Instant::now().duration_since(send_round_proposal_start),
			);
			let receive_forfeit_signatures_start = Instant::now();
			tokio::pin! { let timeout = tokio::time::sleep(app.config.round_sign_time); }
			'receive: loop {
				tokio::select! {
					_ = &mut timeout => {
						warn!("Timed out receiving forfeit signatures.");
						let new = round_state.into_signing_forfeits().restart_missing_forfeits(None);
						if new.need_new_round() {
							app.release_vtxos_in_flux(new.all_locked_inputs).await;
							continue 'round;
						} else {
							round_state = new.into();
							continue 'attempt;
						}
					}
					input = round_input_rx.recv() => {
						match input.expect("broken channel") {
							RoundInput::ForfeitSignatures { signatures } => {
								if let Err(e) = round_state.signing_forfeits().register_forfeits(signatures) {
									slog!(ForfeitRegistrationFailed, round_id, attempt, error: e.to_string());
									continue 'receive;
								}

								if round_state.proceed() {
									break 'receive;
								}
							},
							_ => trace!("unexpected message"),
						}
					}
				}
			}
			slog!(ReceivedRoundForfeits, round_id, attempt,
				max_round_sign_time: app.config.round_sign_time,
				nb_forfeits: round_state.signing_forfeits().forfeit_part_sigs.len(),
				duration: Instant::now().duration_since(receive_forfeit_signatures_start),
			);

			match round_state.into_signing_forfeits().check_forfeits(&app) {
				s @ RoundState::CollectingPayments(_) => {
					round_state = s;
					continue 'attempt;
				},
				s @ RoundState::SigningForfeits(_) => {
					round_state = s;
				},
				_ => unreachable!(),
			}


			// ****************************************************************
			// * Finish the round
			// ****************************************************************

			round_state.into_signing_forfeits().finish(&app).await.context("error finishing round")?;
			break 'attempt;
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use ark::{fee, RoundVtxo, Vtxo, VtxoRequest, VtxoSpec};
	use bitcoin::secp256k1::{PublicKey, Secp256k1};
	use bitcoin::{Amount, OutPoint, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Witness};
	use std::collections::HashSet;
	use std::str::FromStr;

	fn generate_pubkey() -> PublicKey {
		let secp = Secp256k1::new();
		let (_secret_key, public_key) = secp.generate_keypair(&mut rand::thread_rng());
		public_key
	}

	fn get_asp_pubkey() -> PublicKey {
		PublicKey::from_str(
			"02e6642fd69bd211f93f7f1f36ca51a26a5290eb2dd1b0d8279a87bb0d480c8443",
		).unwrap()
	}

	fn create_vtxo_spec(amount: u64) -> VtxoSpec {
		VtxoSpec {
			user_pubkey: generate_pubkey(),
			asp_pubkey: get_asp_pubkey(),
			expiry_height: 100_000,
			exit_delta: 2016,
			amount: Amount::from_sat(amount),
		}
	}

	fn create_round_vtxo(amount: u64) -> Vtxo {
		let spec = create_vtxo_spec(amount);
		let tx = Transaction {
			version: bitcoin::transaction::Version(3),
			lock_time: bitcoin::absolute::LockTime::ZERO,
			input: vec![TxIn {
				previous_output: OutPoint::new(
					Txid::from_slice(&rand::random::<[u8; 32]>()[..]).unwrap(),
					0,
				),
				sequence: Sequence::MAX,
				script_sig: ScriptBuf::new(),
				witness: Witness::new(),
			}],
			output: vec![
				TxOut {
					script_pubkey: spec.exit_spk(),
					value: spec.amount,
				},
				fee::dust_anchor(),
			],
		};

		Vtxo::Round(RoundVtxo {
			spec: spec,
			leaf_idx: 0,
			exit_branch: vec![tx],
		})
	}

	fn create_vtxo_request(amount: u64) -> VtxoRequest {
		VtxoRequest {
			pubkey: generate_pubkey(),
			amount: Amount::from_sat(amount),
			cosign_pk: generate_pubkey(),
		}
	}

	fn create_nonces(nb: usize, data: &RoundData) -> Vec<Vec<MusigPubNonce>> {
		let key = Keypair::new(&SECP, &mut rand::thread_rng());
		let (_sec, pb) = musig::nonce_pair(&key);
		vec![vec![pb; data.nb_vtxo_nonces]; nb]
	}

	fn create_collecting_payments(max_output_vtxos: usize) -> CollectingPayments {
		let round_data = RoundData {
			max_output_vtxos: max_output_vtxos,
			nb_vtxo_nonces: (max_output_vtxos * 4) / 3,
			offboard_feerate: FeeRate::ZERO,
		};
		CollectingPayments::new(0, 0, round_data, HashSet::new(), None)
	}

	#[test]
	fn test_register_payment_valid() {
		const INPUT_AMOUNT: u64 = 400;
		const OUTPUT_AMOUNT: u64 = 400;

		let mut state = create_collecting_payments(2);

		let inputs = vec![create_round_vtxo(INPUT_AMOUNT)];
		let input_ids = inputs.iter().map(|v| v.id()).collect::<Vec<_>>();

		let outputs = vec![create_vtxo_request(OUTPUT_AMOUNT)];
		let nonces = create_nonces(1, &state.round_data);

		state.validate_payment_data(&input_ids, &outputs, &nonces).unwrap();
		state.validate_payment_amounts(&inputs, &outputs, &[]).unwrap();

		state.register_payment(inputs, outputs.clone(), nonces, vec![]);
		assert_eq!(state.all_inputs.len(), 1);
		assert_eq!(state.all_outputs.len(), 1);
		assert_eq!(state.all_offboards.len(), 0);
		assert_eq!(state.inputs_per_cosigner.len(), 1);
		assert_eq!(1, state.inputs_per_cosigner.get(&outputs[0].cosign_pk).unwrap().len());
	}

	#[test]
	fn test_register_payment_output_exceeds_input() {
		const INPUT_AMOUNT: u64 = 400;
		const OUTPUT_AMOUNT: u64 = INPUT_AMOUNT + 100;

		let state = create_collecting_payments(2);

		let inputs = vec![create_round_vtxo(INPUT_AMOUNT)];
		let input_ids = inputs.iter().map(|v| v.id()).collect::<Vec<_>>();

		let outputs = vec![create_vtxo_request(OUTPUT_AMOUNT)];
		let nonces = create_nonces(1, &state.round_data);

		state.validate_payment_data(&input_ids, &outputs, &nonces).unwrap();
		state.validate_payment_amounts(&inputs, &outputs, &[]).unwrap_err();
	}

	#[test]
	fn test_register_payment_duplicate_inputs() {
		const INPUT_AMOUNT: u64 = 400;
		const OUTPUT_AMOUNT: u64 = 300;

		let state = create_collecting_payments(2);

		let input = create_round_vtxo(INPUT_AMOUNT);
		let inputs = vec![input.clone(), input.clone()];
		let input_ids = inputs.iter().map(|v| v.id()).collect::<Vec<_>>();

		let outputs = vec![create_vtxo_request(OUTPUT_AMOUNT)];
		let nonces = create_nonces(1, &state.round_data);

		state.validate_payment_data(&input_ids, &outputs, &nonces).unwrap_err();
	}

	#[test]
	fn test_register_payment_exceeds_max_outputs() {
		const INPUT_AMOUNT: u64 = 400;
		const OUTPUT_AMOUNT_1: u64 = 100;
		const OUTPUT_AMOUNT_2: u64 = 300;

		let state = create_collecting_payments(1);

		let input = create_round_vtxo(INPUT_AMOUNT);
		let inputs = vec![input.clone(), input.clone()];
		let input_ids = inputs.iter().map(|v| v.id()).collect::<Vec<_>>();

		let outputs = vec![
			create_vtxo_request(OUTPUT_AMOUNT_1),
			create_vtxo_request(OUTPUT_AMOUNT_2),
		];
		let nonces = create_nonces(2, &state.round_data);

		state.validate_payment_data(&input_ids, &outputs, &nonces).unwrap_err();
	}

	#[test]
	fn test_register_payment_disallowed_input() {
		const INPUT_AMOUNT: u64 = 400;
		const OUTPUT_AMOUNT: u64 = 300;

		let mut state = create_collecting_payments(2);
		state.allowed_inputs = Some(HashSet::new());

		let inputs = vec![create_round_vtxo(INPUT_AMOUNT)];
		let input_ids = inputs.iter().map(|v| v.id()).collect::<Vec<_>>();

		let outputs = vec![create_vtxo_request(OUTPUT_AMOUNT)];
		let nonces = create_nonces(1, &state.round_data);

		state.validate_payment_data(&input_ids, &outputs, &nonces).unwrap_err();
	}

	#[test]
	fn test_register_payment_duplicate_cosign_pubkey() {
		const INPUT_AMOUNT: u64 = 400;
		const OUTPUT_AMOUNT_1: u64 = 200;
		const OUTPUT_AMOUNT_2: u64 = 200;

		let mut state = create_collecting_payments(2);

		let inputs1 = vec![create_round_vtxo(INPUT_AMOUNT)];
		let input_ids1 = inputs1.iter().map(|v| v.id()).collect::<Vec<_>>();
		let inputs2 = vec![create_round_vtxo(INPUT_AMOUNT)];
		let input_ids2 = inputs2.iter().map(|v| v.id()).collect::<Vec<_>>();

		let outputs1 = vec![create_vtxo_request(OUTPUT_AMOUNT_1)];
		let nonces1 = create_nonces(1, &state.round_data);
		let mut outputs2 = vec![create_vtxo_request(OUTPUT_AMOUNT_2)];
		outputs2[0].cosign_pk = outputs1[0].cosign_pk;
		let nonces2 = create_nonces(1, &state.round_data);

		state.validate_payment_data(&input_ids1, &outputs1, &nonces1).unwrap();
		state.register_payment(inputs1, outputs1, nonces1, vec![]);
		state.validate_payment_data(&input_ids2, &outputs2, &nonces2).unwrap_err();
	}

	#[test]
	fn test_register_wrong_nb_cosign_nonces() {
		const INPUT_AMOUNT: u64 = 400;
		const OUTPUT_AMOUNT: u64 = 300;

		let state = create_collecting_payments(4);

		let inputs1 = vec![create_round_vtxo(INPUT_AMOUNT)];
		let input_ids1 = inputs1.iter().map(|v| v.id()).collect::<Vec<_>>();

		let outputs1 = vec![create_vtxo_request(OUTPUT_AMOUNT), create_vtxo_request(OUTPUT_AMOUNT)];
		let nonces1 = create_nonces(1, &state.round_data);

		state.validate_payment_data(&input_ids1, &outputs1, &nonces1).unwrap_err();
	}

	#[test]
	fn test_register_multiple_payments() {
		const INPUT_AMOUNT: u64 = 400;
		const OUTPUT_AMOUNT: u64 = 300;

		let mut state = create_collecting_payments(4);

		let inputs1 = vec![create_round_vtxo(INPUT_AMOUNT)];
		let input_ids1 = inputs1.iter().map(|v| v.id()).collect::<Vec<_>>();
		let inputs2 = vec![create_round_vtxo(INPUT_AMOUNT)];
		let input_ids2 = inputs2.iter().map(|v| v.id()).collect::<Vec<_>>();

		let outputs1 = vec![create_vtxo_request(OUTPUT_AMOUNT), create_vtxo_request(OUTPUT_AMOUNT)];
		let nonces1 = create_nonces(2, &state.round_data);
		let outputs2 = vec![create_vtxo_request(OUTPUT_AMOUNT), create_vtxo_request(OUTPUT_AMOUNT)];
		let nonces2 = create_nonces(2, &state.round_data);

		state.validate_payment_data(&input_ids1, &outputs1, &nonces1).unwrap();
		state.register_payment(inputs1, outputs1.clone(), nonces1, vec![]);
		state.validate_payment_data(&input_ids2, &outputs2, &nonces2).unwrap();
		state.register_payment(inputs2, outputs2.clone(), nonces2, vec![]);

		assert_eq!(state.all_inputs.len(), 2);
		assert_eq!(state.all_outputs.len(), 4);
		assert_eq!(state.inputs_per_cosigner.len(), 4);
		assert!(state.inputs_per_cosigner.contains_key(&outputs1[0].cosign_pk));
		assert!(state.inputs_per_cosigner.contains_key(&outputs2[0].cosign_pk));
		assert!(state.proceed, "Proceed should be set after second registration");
	}
}
