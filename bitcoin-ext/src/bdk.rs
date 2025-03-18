
use std::borrow::BorrowMut;
use std::fmt;

use bdk_wallet::chain::BlockId;
use bdk_wallet::{SignOptions, TxBuilder, TxOrdering, Wallet};
use bitcoin::{psbt, FeeRate, OutPoint, Transaction, Txid, Weight};
use cbitcoin::Psbt;

use crate::{fee, P2TR_DUST};
use crate::bitcoin::TransactionExt;


/// An extension trait for [TxBuilder].
pub trait TxBuilderExt<'a, A>: BorrowMut<TxBuilder<'a, A>> {
	fn add_dust_fee_anchor_spend(&mut self, anchor: OutPoint)
	where
		A: bdk_wallet::coin_selection::CoinSelectionAlgorithm,
	{
		let psbt_in = psbt::Input {
			witness_utxo: Some(fee::dust_anchor()),
			final_script_witness: Some(fee::dust_anchor_witness()),
			..Default::default()
		};
		self.borrow_mut().add_foreign_utxo(anchor, psbt_in, fee::DUST_FEE_ANCHOR_SPEND_WEIGHT)
			.expect("adding foreign utxo");
	}
}
impl<'a, A> TxBuilderExt<'a, A> for TxBuilder<'a, A> {}


/// Error resulting from the [WalletExt::make_cpfp] function.
#[derive(Debug)]
pub enum CpfpError {
	/// The given tx doesn't have a fee anchor.
	NoFeeAnchor(Txid),
}

impl fmt::Display for CpfpError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::NoFeeAnchor(t) => write!(f, "tx has no fee anchor: {}", t),
		}
	}
}

impl std::error::Error for CpfpError {}


/// An extension trait for [Wallet].
pub trait WalletExt: BorrowMut<Wallet> {
	/// Collect all utxos that are either being spent or created by pending txs.
	fn pending_utxos(&self) -> Vec<OutPoint> {
		let mut ret = Vec::new();
		for tx in self.borrow().transactions() {
			if tx.chain_position.is_confirmed() {
				continue;
			}

			// add its inputs
			ret.extend(tx.tx_node.tx.input.iter().map(|i| i.previous_output));
			// add its outputs
			ret.extend((0..tx.tx_node.tx.output.len())
				.map(|i| OutPoint::new(tx.tx_node.txid, i as u32)),
			);
		}
		ret
	}

	/// Create a cpfp spend that spends the fee anchors in the given txs.
	///
	/// This method doesn't broadcast any txs.
	fn make_cpfp(
		&mut self,
		txs: &[&Transaction],
		fee_rate: FeeRate,
	) -> Result<Psbt, CpfpError> {
		let wallet = self.borrow_mut();

		assert!(!txs.is_empty());
		let anchors = txs.iter().map(|tx| {
			tx.fee_anchor().ok_or_else(|| CpfpError::NoFeeAnchor(tx.compute_txid()))
		}).collect::<Result<Vec<_>, _>>()?;

		// Since BDK doesn't support adding extra weight for fees, we have to
		// first build the tx regularly, and then build it again.
		// Since we have to guarantee that we have enough money in the inputs,
		// we will "fake" create an output on the first attempt. This might
		// overshoot the fee, but we prefer that over undershooting it.

		let package_weight = txs.iter().map(|t| t.weight()).sum::<Weight>();
		let extra_fee_needed = fee_rate * package_weight;

		// Since BDK doesn't allow tx without recipients, we add a drain output.
		let change_addr = wallet.reveal_next_address(bdk_wallet::KeychainKind::Internal);

		let template_weight = {
			let mut b = wallet.build_tx();
			b.ordering(TxOrdering::Untouched);
			b.only_witness_utxo();
			for anchor in &anchors {
				b.add_dust_fee_anchor_spend(*anchor);
			}
			b.add_recipient(change_addr.address.script_pubkey(), extra_fee_needed + P2TR_DUST);
			b.fee_rate(fee_rate);
			let mut psbt = b.finish().expect("failed to craft anchor spend template");
			let opts = SignOptions {
				trust_witness_utxo: true,
				..Default::default()
			};
			let finalized = wallet.sign(&mut psbt, opts)
				.expect("failed to sign anchor spend template");
			assert!(finalized);
			let tx = psbt.extract_tx()
				.expect("anchor spend template not fully signed");
			assert_eq!(
				tx.input[0].witness.size() as u64,
				fee::DUST_FEE_ANCHOR_SPEND_WEIGHT.to_wu(),
			);
			tx.weight()
		};

		let total_weight = template_weight + package_weight;
		let total_fee = fee_rate * total_weight;
		let extra_fee_needed = total_fee;

		// Then build actual tx.
		let mut b = wallet.build_tx();
		b.only_witness_utxo();
		b.version(3); // for 1p1c package relay
		for anchor in &anchors {
			b.add_dust_fee_anchor_spend(*anchor);
		}
		b.drain_to(change_addr.address.script_pubkey());
		b.fee_absolute(extra_fee_needed);
		Ok(b.finish().expect("failed to craft anchor spend tx"))
	}

	/// Insert a checkpoint into the wallet.
	///
	/// It's advised to use this only when recovering a wallet with a birthday.
	fn set_checkpoint(&mut self, checkpoint: BlockId) {
		let wallet = self.borrow_mut();
		wallet.apply_update(bdk_wallet::Update {
			chain: Some(wallet.latest_checkpoint().insert(checkpoint)),
			..Default::default()
		}).expect("should work, might fail if tip is genesis");
	}
}
impl WalletExt for Wallet {}
