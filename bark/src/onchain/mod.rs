
mod chain;
pub use self::chain::ChainSource;

use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Context;
use ark::fee;
use ark::util::{TransactionExt, SECP};
use bdk_wallet::coin_selection::BranchAndBoundCoinSelection;
use bdk_wallet::{LocalOutput, PersistedWallet, SignOptions, TxBuilder, TxOrdering, WalletPersister};
use bitcoin::{
	bip32, psbt, sighash, Address, Amount, FeeRate, Network, OutPoint, Psbt, Sequence, Transaction, TxOut, Txid, Weight
};
use serde::ser::StdError;

use crate::psbtext::PsbtInputExt;
use crate::VtxoSeed;
use crate::{
	exit::SpendableVtxo, persist::BarkPersister
};
pub use crate::onchain::chain::ChainSourceClient;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Utxo {
	Local(LocalOutput),
	Exit(SpendableVtxo)
}

pub trait TxBuilderExt {
	fn add_exit_outputs(&mut self, exit_outputs: &[SpendableVtxo]);
}

impl TxBuilderExt for TxBuilder<'_, BranchAndBoundCoinSelection> {
	fn add_exit_outputs(&mut self, exit_outputs: &[SpendableVtxo]) {
		self.version(2);

		for input in exit_outputs {
			let mut psbt_in = psbt::Input::default();
			psbt_in.set_exit_claim_input(&input.vtxo);
			psbt_in.witness_utxo = Some(TxOut {
				script_pubkey: input.vtxo.spec().exit_spk(),
				value: input.vtxo.spec().amount,
			});

			self.add_foreign_utxo_with_sequence(
				input.vtxo.point(),
				psbt_in,
				input.vtxo.claim_satisfaction_weight(),
				Sequence::from_height(input.vtxo.spec().exit_delta),
			).expect("error adding foreign utxo for claim input");
		}
	}
}

pub struct Wallet<P: BarkPersister> {
	/// NB: onchain wallet needs to be able to reconstruct
	/// vtxo keypair in order to sign vtxo exit output if any
	seed: [u8; 64],
	network: Network,

	wallet: PersistedWallet<P>,
	db: P,

	pub (crate) exit_outputs: Vec<SpendableVtxo>,
	pub (crate)	chain_source: ChainSourceClient,
}

impl <P>Wallet<P> where
	P: BarkPersister,
	<P as WalletPersister>::Error: 'static + std::fmt::Debug + std::fmt::Display + Send + Sync + StdError
{
	pub fn create(
		network: Network,
		seed: [u8; 64],
		mut db: P,
		chain_source: ChainSource,
	) -> anyhow::Result<Wallet<P>> {
		let xpriv = bip32::Xpriv::new_master(network, &seed).expect("valid seed");
		let desc = format!("tr({}/84'/0'/0'/0/*)", xpriv);

		let wallet_opt = bdk_wallet::Wallet::load()
			.descriptor(bdk_wallet::KeychainKind::External, Some(desc.clone()))
			.extract_keys()
			.check_network(network)
			.load_wallet(&mut db)?;

		let wallet = match wallet_opt {
			Some(wallet) => wallet,
			None => bdk_wallet::Wallet::create_single(desc)
				.network(network)
				.create_wallet(&mut db)?,
		};

		let chain_source = ChainSourceClient::new(chain_source)?;

		Ok(Wallet {
			seed,
			network,

			wallet,
			db,

			chain_source,
			exit_outputs: vec![]
		})
	}

	pub fn require_chainsource_version(&self) -> anyhow::Result<()> {
		self.chain_source.require_version()
	}

	pub async fn tip(&self) -> anyhow::Result<u32> {
		self.chain_source.tip().await
	}

	pub (crate) async fn broadcast_tx(&self, tx: &Transaction) -> anyhow::Result<()> {
		self.chain_source.broadcast_tx(tx).await
	}

	/// Sync the onchain wallet and returns the balance.
	pub async fn sync(&mut self) -> anyhow::Result<Amount> {
		Ok(self.chain_source.sync_wallet(&mut self.wallet, &mut self.db).await?)
	}

	/// Return the balance of the onchain wallet.
	///
	/// Make sure you sync before calling this method.
	pub fn balance(&self) -> Amount {
		let exit_total = self.exit_outputs.iter().fold(Amount::ZERO, |acc, v| acc + v.vtxo.spec().amount);
		self.wallet.balance().total() + exit_total
	}

	pub fn utxos(&self) -> Vec<Utxo> {
		let mut utxos = self.wallet.list_unspent().map(|o| Utxo::Local(o)).collect::<Vec<_>>();
		utxos.extend(self.exit_outputs.clone().into_iter().map(|e| Utxo::Exit(e)));

		utxos
	}

	pub (crate) fn prepare_tx(&mut self, dest: Address, amount: Amount) -> anyhow::Result<Psbt> {
		let fee_rate = self.chain_source.regular_feerate();
		let mut b = self.wallet.build_tx();
		b.add_exit_outputs(&self.exit_outputs.clone());
		b.ordering(TxOrdering::Untouched);
		b.add_recipient(dest.script_pubkey(), amount);
		b.fee_rate(fee_rate);
		Ok(b.finish()?)
	}

	pub (crate) fn prepare_send_all_tx(&mut self, dest: Address) -> anyhow::Result<Psbt> {
		let fee_rate = self.chain_source.regular_feerate();
		let mut b = self.wallet.build_tx();
		b.add_exit_outputs(&self.exit_outputs.clone());
		b.drain_to(dest.script_pubkey());
		b.drain_wallet();
		b.fee_rate(fee_rate);
		b.finish().context("error building tx")
	}

	fn sign_exit_inputs(&self, psbt: &mut Psbt) -> anyhow::Result<()> {
		let vtxo_seed = VtxoSeed::new(self.network, &self.seed);

		let prevouts = psbt.inputs.iter()
			.map(|i| i.witness_utxo.clone().unwrap())
			.collect::<Vec<_>>();

		let prevouts = sighash::Prevouts::All(&prevouts);
		let mut shc = sighash::SighashCache::new(&psbt.unsigned_tx);

		for (i, input) in psbt.inputs.iter_mut().enumerate() {
			let vtxo = input.get_exit_claim_input();

			if let Some(vtxo) = vtxo {
				let keypair_idx = self.db.get_vtxo_key_index(&vtxo)?;
				let keypair = vtxo_seed.derive_keypair(keypair_idx);

				input.try_sign_exit_claim_input(
					&SECP,
					&mut shc,
					&prevouts,
					i,
					&keypair
				);
			}
		}

		Ok(())
	}

	pub (crate) fn finish_tx(&mut self, mut psbt: Psbt) -> anyhow::Result<Transaction> {
		self.sign_exit_inputs(&mut psbt)?;

		let opts = SignOptions {
			trust_witness_utxo: true,
			..Default::default()
		};
		let finalized = self.wallet.sign(&mut psbt, opts).context("signing error")?;
		assert!(finalized);
		let tx = psbt.extract_tx()?;
		let unix = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
		self.wallet.apply_unconfirmed_txs([(tx.clone(), unix)]);
		self.wallet.persist(&mut self.db)?;
		Ok(tx)
	}

	pub async fn send(&mut self, dest: Address, amount: Amount) -> anyhow::Result<Txid> {
		let psbt = self.prepare_tx(dest, amount)?;
		let tx = self.finish_tx(psbt)?;
		self.broadcast_tx(&tx).await?;
		Ok(tx.compute_txid())
	}

	/// Reveals a new onchain address
	///
	/// The revealed address is directly persisted, so calling this method twice in a row will result in 2 different addresses
	pub fn address(&mut self) -> anyhow::Result<Address> {
		let ret = self.wallet.reveal_next_address(bdk_wallet::KeychainKind::External).address;
		self.wallet.persist(&mut self.db)?;
		Ok(ret)
	}

	fn add_anchors<A>(b: &mut bdk_wallet::TxBuilder<A>, anchors: &[OutPoint])
	where
		A: bdk_wallet::coin_selection::CoinSelectionAlgorithm,
	{
		for utxo in anchors {
			let psbt_in = psbt::Input {
				witness_utxo: Some(ark::fee::dust_anchor()),
				final_script_witness: Some(ark::fee::dust_anchor_witness()),
				..Default::default()
			};
			b.add_foreign_utxo(*utxo, psbt_in, fee::DUST_ANCHOR_SPEND_WEIGHT)
				.expect("adding foreign utxo");
		}
	}

	/// Create a cpfp spend that spends the fee anchors in the given txs.
	///
	/// This method doesn't broadcast any txs.
	pub (crate) async fn make_cpfp(
		&mut self,
		txs: &[&Transaction],
		fee_rate: FeeRate,
	) -> anyhow::Result<Transaction> {
		assert!(!txs.is_empty());
		let anchors = txs.iter().map(|tx| {
			tx.fee_anchor().with_context(|| format!("tx {} has no fee anchor", tx.compute_txid()))
		}).collect::<Result<Vec<_>, _>>()?;

		// Since BDK doesn't support adding extra weight for fees, we have to
		// first build the tx regularly, and then build it again.
		// Since we have to guarantee that we have enough money in the inputs,
		// we will "fake" create an output on the first attempt. This might
		// overshoot the fee, but we prefer that over undershooting it.

		let package_weight = txs.iter().map(|t| t.weight()).sum::<Weight>();
		let extra_fee_needed = fee_rate * package_weight;

		// Since BDK doesn't allow tx without recipients, we add a drain output.
		let change_addr = self.wallet.reveal_next_address(bdk_wallet::KeychainKind::Internal);

		let template_weight = {
			let mut b = self.wallet.build_tx();
			b.ordering(TxOrdering::Untouched);
			b.only_witness_utxo();
			Self::add_anchors(&mut b, &anchors);
			b.add_recipient(change_addr.address.script_pubkey(), extra_fee_needed + ark::P2TR_DUST);
			b.fee_rate(fee_rate);
			let mut psbt = b.finish().expect("failed to craft anchor spend template");
			let opts = SignOptions {
				trust_witness_utxo: true,
				..Default::default()
			};
			let finalized = self.wallet.sign(&mut psbt, opts)
				.expect("failed to sign anchor spend template");
			assert!(finalized);
			let tx = psbt.extract_tx()?;
			debug_assert_eq!(tx.input[0].witness.size() as u64, fee::DUST_ANCHOR_SPEND_WEIGHT.to_wu());
			tx.weight()
		};

		let total_weight = template_weight + package_weight;
		let total_fee = fee_rate * total_weight;
		let extra_fee_needed = total_fee;

		// Then build actual tx.
		let mut b = self.wallet.build_tx();
		b.only_witness_utxo();
		b.version(3); // for 1p1c package relay
		Self::add_anchors(&mut b, &anchors);
		b.drain_to(change_addr.address.script_pubkey());
		b.fee_absolute(extra_fee_needed);
		let psbt = b.finish().expect("failed to craft anchor spend tx");
		let tx = self.finish_tx(psbt).context("error finalizing anchor spend tx")?;

		Ok(tx)
	}
}
