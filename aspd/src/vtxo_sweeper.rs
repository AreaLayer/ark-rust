//!
//! This module implements the logic to sweep up all expired round UTXOs.
//! Generally this means
//! - sweeping the outputs of the root tx if none of the offchain txs were broadcast
//! - sweeping all the unspent outputs of intermediate vtxo tree txs if some were spent
//! - sweeping all unused connector outputs
//!
//! Some things to keep in mind:
//!
//! # Sweep VTXOs before connectors
//!
//! We need the connectors in case a user attempts a malicious exit.
//! Also, the connectors have way less money in them, so we don't need them as urgently.
//! As such, our strategy is to keep connectors untouched until we are sure all VTXO tree
//! outputs are confirmed.
//!
//! # Races with competing txs
//!
//! All of the VTXO outputs we're going to be spending have alternative spend paths. Either
//! fee anchors (anyone can spend) or VTXO outputs (covenant spend stays valid while we can sweep).
//! This means that it is possible that there may exist transactions competing with ours.
//! None of these txs can steal our money, so "winning the race" is not critical;
//! but they often do incur an extra cost to us, so we prefer to get as little as them confirmed
//! as possible.
//!
//! As such, there are two possible strategies we can use to deal with competing transactions:
//! 1. *double spend": ignore them and simply double spend them,
//! then waiting to see which one confirms and adjust accordingly.
//! 2. *descend*: create descendent txs from the existing ones and make both confirm.
//!
//! In theory both strategies can be the optimal strategy in specific situations, but in practice,
//! the *descend* strategy is going to be way more complicated when it comes to avoiding
//! stuck transactions.
//!
//! Therefore, the strategy implemented in this module is the simpler *double spend* strategy.
//!

use std::cmp;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Context;
use bdk_wallet::ChangeSet;
use bitcoin::absolute::LockTime;
use bitcoin::secp256k1::XOnlyPublicKey;
use bitcoin::{
	psbt, sighash, taproot, Amount, FeeRate, OutPoint, Sequence,
	Transaction, TxOut, Txid, Weight, Witness,
};

use ark::BlockHeight;
use ark::connectors::ConnectorChain;
use ark::util::KeypairExt;

use crate::bitcoind::RpcApi;
use crate::database::StoredRound;
use crate::psbtext::{PsbtInputExt, RoundMeta};
use crate::{txindex, App, DEEPLY_CONFIRMED, SECP};


struct SweepInput<'a> {
	point: OutPoint,
	utxo: TxOut,
	internal_key: XOnlyPublicKey,
	round: &'a ExpiredRound,
	round_meta: RoundMeta,
	weight: Weight,
}

impl<'a> SweepInput<'a> {
	fn amount(&self) -> Amount {
		self.utxo.value
	}

	/// Calculate the surplus that can be gained from sweeping this input.
	///
	/// This is calculated as the inputs value subtracted with the cost
	/// of spending it with the given fee rate.
	///
	/// If negative, returns [None].
	fn surplus(&self, feerate: FeeRate) -> Option<Amount> {
		self.amount().checked_sub(feerate * self.weight)
	}

	fn psbt(&self) -> psbt::Input {
		let round_cosign_pk = self.round.round.signed_tree.spec.round_tx_cosign_pk();
		let (
			spend_cb, spend_script, spend_lv, spend_merkle,
		) = self.round.round.signed_tree.spec.expiry_scriptspend(round_cosign_pk);
		let mut ret = psbt::Input{
			witness_utxo: Some(self.utxo.clone()),
			sighash_type: Some(sighash::TapSighashType::Default.into()),
			tap_internal_key: Some(self.internal_key),
			tap_scripts: [(spend_cb, (spend_script, spend_lv))].into_iter().collect(),
			tap_merkle_root: Some(spend_merkle),
			non_witness_utxo: None,
			..Default::default()
		};
		ret.set_round_meta(self.round_meta.clone());
		ret
	}
}

struct ExpiredRound {
	txid: Txid,
	round: StoredRound,
	connectors: ConnectorChain,

	/// All transactions in this rounds vtxo tree, from the leaves to the root.
	vtxo_txs: Vec<Transaction>,
}

impl ExpiredRound {
	fn new(txid: Txid, round: StoredRound) -> Self {
		Self {
			vtxo_txs: round.signed_tree.all_signed_txs(),
			connectors: ConnectorChain::new(
				round.nb_input_vtxos as usize,
				OutPoint::new(txid, 1),
				round.signed_tree.spec.asp_pk,
			),
			txid, round,
		}
	}
}

/// Build a sweep.
struct SweepBuilder<'a> {
	sweeper: &'a mut VtxoSweeper,
	sweeps: Vec<SweepInput<'a>>,
	feerate: FeeRate,
}

impl<'a> SweepBuilder<'a> {
	fn new(sweeper: &'a mut VtxoSweeper, feerate: FeeRate) -> Self {
		Self {
			sweeps: Vec::new(),
			sweeper, feerate,
		}
	}

	fn total_surplus(&self) -> Amount {
		self.sweeps.iter().map(|s| s.surplus(self.feerate).unwrap_or(Amount::ZERO)).sum()
	}

	/// Add sweep for the given vtxo tree output.
	fn add_vtxo_output(
		&mut self,
		round: &'a ExpiredRound,
		point: OutPoint,
		utxo: TxOut,
		agg_pk: XOnlyPublicKey,
	) {
		trace!("Adding vtxo sweep input {}", point);
		self.sweeps.push(SweepInput {
			point, utxo, round,
			internal_key: agg_pk,
			round_meta: RoundMeta::Vtxo,
			weight: ark::tree::signed::NODE_SPEND_WEIGHT,
		});
	}

	/// Add a sweep for the given connector output.
	fn add_connector_output(
		&mut self,
		round: &'a ExpiredRound,
		point: OutPoint,
		utxo: TxOut,
	) {
		trace!("Adding connector sweep input {}", point);
		self.sweeps.push(SweepInput {
			point, utxo, round,
			internal_key: round.round.signed_tree.spec.asp_pk.x_only_public_key().0,
			round_meta: RoundMeta::Connector,
			weight: ark::connectors::INPUT_WEIGHT,
		});
	}

	/// Purge all sweeps sweeping the given utxo.
	fn purge_sweeps(&mut self, point: &OutPoint) {
		self.sweeps.retain(|s| {
			if s.point != *point {
				trace!("purging sweep for {} because successor tx confirmed", point);
				false
			} else {
				true
			}
		});
	}

	/// Purge all sweeps that are not economical at our configured feerate.
	fn purge_uneconomical(&mut self) {
		self.sweeps.retain(|s| {
			if s.surplus(self.feerate).is_none() {
				slog!(UneconomicalSweepInput, outpoint: s.point, value: s.amount());
				false
			} else {
				true
			}
		})
	}

	/// Sweep the leftovers of the vtxo tree of the given round.
	///
	/// Returns the most recent of the confirmation heights for all sweep txs,
	/// [None] if there are unconfirmed transactions.
	async fn process_vtxos(&mut self, round: &'a ExpiredRound) -> Option<BlockHeight> {
		// First check if the round tx is still available for sweeping, that'd be ideal.
		let tree_root = round.vtxo_txs.last().unwrap();
		let tree_root = self.sweeper.app.txindex.get(&tree_root.compute_txid()).await
			.expect("txindex should contain all round txs");

		if !tree_root.confirmed().await {
			trace!("Tree root tx {} not yet confirmed, sweeping round tx...", tree_root.txid);
			let point = OutPoint::new(round.txid, 0);
			if let Some(h) = self.sweeper.is_swept(point).await {
				trace!("Round tx vtxo tree output {point} it already swept by us at height {h}");
				return Some(h);
			} else {
				trace!("Sweeping round tx vtxo output {}", point);
				let utxo = round.round.tx.output[0].clone();
				let agg_pk = round.round.signed_tree.spec.round_tx_cosign_pk();
				self.add_vtxo_output(round, point, utxo, agg_pk);
				return None;
			}
		}

		// If the root is not available, we have to roll down the tree.
		//
		// The strategy we use is the following:
		// - we traverse the tree from root to leaves
		// - whenever a tx is confirmed
		//   - we remove from our to-sweep set all txs spending inputs from this tx
		//   - for each of its outputs
		//     - check whether a previous sweep tx confirmed
		//     - if not, we add the spend info to the set

		let mut ret = Some(0);
		let signed_txs = round.round.signed_tree.all_signed_txs();
		let agg_pkgs = round.round.signed_tree.spec.cosign_agg_pks();
		for (signed_tx, agg_pk) in signed_txs.into_iter().zip(agg_pkgs).rev() {
			let tx = self.sweeper.app.txindex.get(&signed_tx.compute_txid()).await
				.expect("txindex should contain all round txs");
			if !tx.confirmed().await {
				trace!("tx {} did not confirm yet, not sweeping", tx.txid);
				continue;
			}
			trace!("vtxo tree tx {} confirmed, prepping sweeps", tx.txid);

			// Purge sweeps of our tx's input.
			assert_eq!(1, signed_tx.input.len());
			self.purge_sweeps(&signed_tx.input[0].previous_output);

			for (idx, out) in signed_tx.output.into_iter().enumerate() {
				let point = OutPoint::new(tx.txid, idx as u32);
				if let Some(h) = self.sweeper.is_swept(point).await {
					ret = ret.and_then(|old| Some(cmp::max(old, h)));
				} else {
					let utxo = out;
					self.add_vtxo_output(round, point, utxo, agg_pk);
					ret = None;
				}
			}
		}
		assert_ne!(ret, Some(0), "ret should have changed to something at least");
		ret
	}

	/// Sweep the leftover connectors of the given round.
	///
	/// Returns the most recent of the confirmation heights for all sweep txs,
	/// [None] if there are unconfirmed transactions.
	async fn process_connectors(&mut self, round: &'a ExpiredRound) -> Option<BlockHeight> {
		// When it comes to connectors, we should have a single leftover output to sweep,
		// plus maybe some unused connector(s).

		// NB we don't really know the number of connectors, because we don't know the number
		// of inputs to the round. it doesn't matter, though, they are pre-signed, so
		// we can generate any chain of connector txs and check if the txs are on the chain or not
		let round_point = OutPoint::new(round.txid, 1);
		let mut conn_txs = round.connectors.iter_unsigned_txs();

		let mut last = (round_point, round.round.tx.output[1].clone());
		let mut ret = Some(0);
		loop {
			let tx = match conn_txs.next() {
				None => return Some(0), // all connector txs confirmed and spent
				Some(c) => c,
			};

			let tx = self.sweeper.app.txindex.get(&tx.compute_txid()).await
				.expect("txindex should contain all connector txs");

			if tx.confirmed().await {
				// Check if the connector output is still unspent.
				let conn = OutPoint::new(tx.txid, 1);
				match self.sweeper.app.bitcoind.get_tx_out(&conn.txid, conn.vout, Some(true)) {
					Ok(Some(out)) => {
						if let Some(h) = self.sweeper.is_swept(conn).await {
							ret = ret.and_then(|old| Some(cmp::max(old, h)));
						} else {
							let txout = TxOut {
								value: out.value,
								script_pubkey: out.script_pub_key.script().expect("invalid script"),
							};
							self.add_connector_output(round, conn, txout);
							ret = None;
						}
					},
					Ok(None) => {}, // ignore it
					Err(e) => {
						// we just try later
						error!("Error calling gettxout for connector utxo {}: {}", conn, e);
						return None;
					},
				}

				// Then continue the chain.
				last = (OutPoint::new(tx.txid, 0), tx.tx.output[0].clone());
			} else {
				// add the last point
				let (point, output) = last;
				if let Some(h) = self.sweeper.is_swept(point).await {
					ret = ret.and_then(|old| Some(cmp::max(old, h)));
				} else {
					self.add_connector_output(round, point, output);
					ret = None;
				}
				break;
			}
		}
		assert_ne!(ret, Some(0), "ret should have changed to something at least");
		ret
	}

	async fn process_round(&mut self, round: &'a ExpiredRound, done_height: BlockHeight) {
		trace!("Processing vtxo tree for round {}", round.txid);
		let vtxos_done = self.process_vtxos(round).await;
		if vtxos_done.is_none() || vtxos_done.unwrap() > done_height {
			trace!("Pending vtxo sweeps for this round (height {:?}), waiting for {}",
				vtxos_done, done_height,
			);
			return;
		}

		trace!("Processing connectors for round {}", round.txid);
		let connectors_done = self.process_connectors(round).await;
		if connectors_done.is_none() || connectors_done.unwrap() > done_height {
			trace!("Pending connector sweeps for this round (height {:?}), waiting for {}",
				connectors_done, done_height,
			);
			return;
		}

		//TODO(stevenroose) do this elsewhere
		self.sweeper.round_finished(round).await;
	}

	async fn create_tx(
		&self,
		tip: BlockHeight,
	) -> anyhow::Result<(Transaction, ChangeSet)> {
		let mut wallet = self.sweeper.app.wallet.lock().await;
		let drain_addr = wallet.reveal_next_address(bdk_wallet::KeychainKind::Internal).address;

		let mut txb = wallet.build_tx();
		txb.ordering(bdk_wallet::TxOrdering::Untouched);
		txb.nlocktime(LockTime::from_height(tip as u32).expect("actual height"));

		for sweep in &self.sweeps {
			txb.add_foreign_utxo_with_sequence(
				sweep.point,
				sweep.psbt(),
				sweep.weight,
				Sequence::ZERO,
			).expect("bdk rejected foreign utxo");
		}

		txb.drain_to(drain_addr.script_pubkey());
		txb.fee_rate(self.feerate);
		let mut psbt = txb.finish().expect("bdk failed to create round sweep tx");


		// SIGNING

		let mut shc = sighash::SighashCache::new(&psbt.unsigned_tx);
		let prevouts = psbt.inputs.iter()
			.map(|i| i.witness_utxo.clone().unwrap())
			.collect::<Vec<_>>();

		let connector_keypair = self.sweeper.app.asp_key.for_keyspend();
		for (idx, input) in psbt.inputs.iter_mut().enumerate() {
			if let Some(meta) = input.get_round_meta().context("corrupt psbt")? {
				match meta {
					RoundMeta::Vtxo => {
						let (control, (script, lv)) = input.tap_scripts.iter().next()
							.context("corrupt psbt: missing tap_scripts")?;
						let leaf_hash = taproot::TapLeafHash::from_script(script, *lv);
						let sighash = shc.taproot_script_spend_signature_hash(
							idx,
							&sighash::Prevouts::All(&prevouts),
							leaf_hash,
							sighash::TapSighashType::Default,
						).expect("all prevouts provided");
						trace!("Signing expired VTXO input for sighash {}", sighash);
						let sig = SECP.sign_schnorr(&sighash.into(), &self.sweeper.app.asp_key);
						let wit = Witness::from_slice(
							&[&sig[..], script.as_bytes(), &control.serialize()],
						);
						debug_assert_eq!(wit.size(), ark::tree::signed::NODE_SPEND_WEIGHT.to_wu() as usize);
						input.final_script_witness = Some(wit);
					},
					RoundMeta::Connector => {
						let sighash = shc.taproot_key_spend_signature_hash(
							idx,
							&sighash::Prevouts::All(&prevouts),
							sighash::TapSighashType::Default,
						).expect("all prevouts provided");
						trace!("Signing expired connector input for sighash {}", sighash);
						let sig = SECP.sign_schnorr(&sighash.into(), &connector_keypair);
						input.final_script_witness = Some(Witness::from_slice(&[sig[..].to_vec()]));
					},
				}
			}
		}

		let opts = bdk_wallet::SignOptions {
			trust_witness_utxo: true,
			..Default::default()
		};
		let finalized = wallet.sign(&mut psbt, opts)?;
		assert!(finalized);
		let signed = psbt.extract_tx()?;
		let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Unix epoch is in the past").as_secs();
		wallet.apply_unconfirmed_txs([(Arc::new(signed.clone()), now)]);
		let changeset = wallet.take_staged().expect("inserted new tx");
		Ok((signed, changeset))
	}
}


struct VtxoSweeper {
	app: Arc<App>,
	/// Pending txs indexed by the inputs they spend.
	pending_tx_by_utxo: HashMap<OutPoint, Vec<txindex::Tx>>,
}

impl VtxoSweeper {
	/// Store the tx in our local caches.
	fn store_pending(&mut self, tx: txindex::Tx) {
		for inp in &tx.tx.input {
			self.pending_tx_by_utxo.entry(inp.previous_output).or_insert(Vec::new()).push(tx.clone());

		}
	}

	/// Load the [VtxoSweeper] by loading all pending txs from the database.
	async fn load(app: Arc<App>) -> anyhow::Result<VtxoSweeper> {
		let raw_pending = app.db.fetch_pending_sweeps().context("error fetching pending sweeps")?;
		// Register all pending in the txindex.
		let mut ret = VtxoSweeper {
			app,
			pending_tx_by_utxo: HashMap::with_capacity(raw_pending.values().map(|t| t.input.len()).sum()),
		};

		for (_txid, raw_tx) in raw_pending {
			let tx = ret.app.txindex.broadcast_tx(raw_tx).await;
			ret.store_pending(tx);
		}

		Ok(ret)
	}

	/// Store the pending tx both in the db and mem cache.
	async fn add_new_pending(&mut self, txid: Txid, tx: Transaction) -> anyhow::Result<()> {
		self.app.db.store_pending_sweep(&txid, &tx).context("db error storing pending sweep")?;

		let tx = self.app.txindex.broadcast_tx(tx).await;
		self.store_pending(tx);
		Ok(())
	}

	async fn is_swept(&self, point: OutPoint) -> Option<BlockHeight> {
		if let Some(txs) = self.pending_tx_by_utxo.get(&point) {
			for tx in txs {
				if let Some(h) = tx.status().await.confirmed_in() {
					return Some(h);
				}
			}
		}
		None
	}

	/// Clean jup all aftefacts after a round has been swept.
	async fn round_finished(&mut self, round: &ExpiredRound) {
		slog!(RoundFullySwept, round_id: round.txid);
		if let Err(e) = self.app.db.remove_round(round.txid) {
			error!("Failed to remove round from db after successful sweeping: {}", e);
		}

		// vtxo tree txs
		let vtxo_txs = round.round.signed_tree.all_signed_txs();
		trace!("Removing vtxo txs from internal pending...");
		for tx in &vtxo_txs {
			for i in 0..tx.output.len() {
				self.pending_tx_by_utxo.remove(&OutPoint::new(tx.compute_txid(), i as u32));
			}
		}

		trace!("Removing vtxo txs from txindex...");
		self.app.txindex.unregister_batch(vtxo_txs.iter()).await;

		// connector txs
		trace!("Connector txs from internal pending...");
		for tx in round.connectors.iter_unsigned_txs() {
			for i in 0..tx.output.len() {
				self.pending_tx_by_utxo.remove(&OutPoint::new(tx.compute_txid(), i as u32));
			}
		}
		trace!("Removing connector txs from txindex...");
		self.app.txindex.unregister_batch(round.connectors.iter_unsigned_txs()).await;

		debug!("round_finished returns");

		//TODO(stevenroose) when do we clear pending txs from the db?
	}

	async fn process_rounds(
		&mut self,
	) -> anyhow::Result<()> {
		let sweep_threshold = self.app.config.sweep_threshold;
		let tip = self.app.bitcoind.get_block_count()? as BlockHeight;
		let expired_rounds = self.app.db.get_expired_rounds(tip)?.into_iter().map(|txid| {
			let round = self.app.db.get_round(txid)?.expect("db has round");
			Ok(ExpiredRound::new(txid, round))
		}).collect::<anyhow::Result<Vec<_>>>()?;
		trace!("{} expired rounds fetched", expired_rounds.len());

		let feerate = self.app.config.sweep_tx_fallback_feerate;
		let mut builder = SweepBuilder::new(self, feerate);

		let done_height = tip - DEEPLY_CONFIRMED + 1;
		for round in &expired_rounds {
			trace!("Processing round {}", round.txid);
			builder.process_round(round, done_height).await;
			builder.purge_uneconomical();
			//TODO(stevenroose) check if we exceeded some builder limits
		}

		// We processed all rounds, check if it's worth to sweep at all.
		let surplus = builder.total_surplus();
		trace!("Sweep surpus calculated: {}", surplus);
		if surplus < sweep_threshold {
			slog!(NotSweeping, available_surplus: surplus, nb_inputs: builder.sweeps.len());
			return Ok(());
		}

		let sweep_points = builder.sweeps.iter().map(|s| s.point).collect();
		slog!(SweepingRounds, total_surplus: surplus, inputs: sweep_points);
		for s in &builder.sweeps {
			slog!(SweepingOutput, outpoint: s.point, amount: s.amount(),
				surplus: s.surplus(feerate).unwrap(),
			);
		}

		let (signed, changeset) = builder.create_tx(tip).await.context("creating sweep tx")?;
		self.app.db.store_changeset(&changeset).await?;

		let txid = signed.compute_txid();
		self.add_new_pending(txid, signed.clone()).await?;
		slog!(SweepBroadcast, txid, surplus: surplus);

		Ok(())
	}
}

/// Run a process that will periodically check for expired rounds and
/// sweep them into our internal wallet.
pub async fn run_vtxo_sweeper(
	app: &Arc<App>,
	mut sweep_trigger_rx: tokio::sync::mpsc::Receiver<()>,
) -> anyhow::Result<()> {
	let mut shutdown = app.shutdown_channel.subscribe();

	let mut state = VtxoSweeper::load(app.clone()).await.context("failed to load VtxoSweeper state")?;

	info!("Starting expired vtxo sweep loop");
	loop {
		tokio::select! {
			// Periodic interval for sweeping
			() = tokio::time::sleep(state.app.config.round_sweep_interval) => {},
			// Trigger received via channel
			Some(()) = sweep_trigger_rx.recv() => {
				slog!(ReceivedSweepTrigger);
			},
			_ = shutdown.recv() => {
				info!("Shutdown signal received. Exiting sweep loop...");
				break;
			}
		}

		//TODO(stevenroose) do this better
		// state.prune_confirmed().await;
		if let Err(e) = state.process_rounds().await {
			error!("Error during round processing: {}", e);
		}
	}

	info!("Expired vtxo sweep loop terminated gracefully.");

	Ok(())
}
