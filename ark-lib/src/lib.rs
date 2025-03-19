
pub extern crate bitcoin;

#[macro_use] extern crate serde;
#[macro_use] extern crate lazy_static;

pub mod connectors;
pub mod forfeit;
pub mod lightning;
pub mod musig;
pub mod board;
pub mod oor;
pub mod rounds;
pub mod tree;
pub mod util;
pub mod vtxo;

pub use crate::vtxo::{ArkoorVtxo, Bolt11ChangeVtxo, BoardVtxo, RoundVtxo, VtxoId, VtxoSpec, Vtxo};

#[cfg(test)]
mod napkin;


use std::fmt;
use std::time::Duration;

use bitcoin::secp256k1::schnorr::Signature;
use bitcoin::{Amount, BlockHash, FeeRate, Network, Script, ScriptBuf, TxOut, Weight};
use bitcoin::secp256k1::PublicKey;
use bitcoin_ext::{P2PKH_DUST_VB, P2SH_DUST_VB, P2TR_DUST_VB, P2WPKH_DUST_VB, P2WSH_DUST_VB};


/// Type representing a block height in the bitcoin blockchain.
pub type BlockHeight = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockRef {
	pub height: BlockHeight,
	pub hash: BlockHash,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArkInfo {
	pub network: Network,
	pub asp_pubkey: PublicKey,
	pub round_interval: Duration,
	pub nb_round_nonces: usize,
	pub vtxo_exit_delta: u16,
	pub vtxo_expiry_delta: u16,
}

/// Input of a round
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct VtxoIdInput {
	pub vtxo_id: VtxoId,
	/// A schnorr signature over a message containing a static prefix,
	/// a random challenge generated by the ASP and the VTXO's id.
	/// See [`rounds::VtxoOwnershipChallenge`].
	///
	/// Should be produced using VTXO's private key
	pub ownership_proof: Signature,
}

/// Request for the creation of a VTXO.
///
/// NB This differs from the [VtxoRequest] type in ark-lib in the fact that
/// it doesn't have a cosign pubkey attached yet.
/// With covenants we can remove this type distinction.
/// Or we might be able to use it for OOR payments.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct PaymentRequest {
	pub pubkey: PublicKey,
	#[serde(rename = "amount_sat", with = "bitcoin::amount::serde::as_sat")]
	pub amount: Amount,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct VtxoRequest {
	pub pubkey: PublicKey,
	#[serde(rename = "amount_sat", with = "bitcoin::amount::serde::as_sat")]
	pub amount: Amount,
	/// The public key used by the client to cosign the transaction tree
	/// The client SHOULD forget this key after signing it
	pub cosign_pk: PublicKey,
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InvalidOffboardRequestError(&'static str);

impl fmt::Display for InvalidOffboardRequestError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "invalid offboard request: {}", self.0)
	}
}

impl std::error::Error for InvalidOffboardRequestError {}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct OffboardRequest {
	pub script_pubkey: ScriptBuf,
	#[serde(rename = "amount_sat", with = "bitcoin::amount::serde::as_sat")]
	pub amount: Amount,
}

impl OffboardRequest {
	/// Calculate the fee we have to charge for adding an output
	/// with the given scriptPubkey to a transaction.
	///
	/// Returns an error if the output type is non-standard.
	pub fn calculate_fee(
		script_pubkey: &Script,
		fee_rate: FeeRate,
	) -> Result<Amount, InvalidOffboardRequestError> {
		// NB We calculate the required extra fee as the "dust" fee for the given feerate.
		// We take Bitcoin's dust amounts, which are calculated at 3 sat/vb, but then
		// calculated for the given feerate. For more on dust, see:
		// https://bitcoin.stackexchange.com/questions/10986/what-is-meant-by-bitcoin-dust

		let vb = if script_pubkey.is_p2pkh() {
			P2PKH_DUST_VB
		} else if script_pubkey.is_p2sh() {
			P2SH_DUST_VB
		} else if script_pubkey.is_p2wpkh() {
			P2WPKH_DUST_VB
		} else if script_pubkey.is_p2wsh() {
			P2WSH_DUST_VB
		} else if script_pubkey.is_p2tr() {
			P2TR_DUST_VB
		} else if script_pubkey.is_op_return() {
			if script_pubkey.len() > 83 {
				return Err(InvalidOffboardRequestError("OP_RETURN over 83 bytes"));
			} else {
				bitcoin::consensus::encode::VarInt(script_pubkey.len() as u64).size() as u64
					+ script_pubkey.len() as u64
					+ 8  // output amount
					// the input data (scriptSig and witness length fields included)
					+ 36 // input prevout
					+ 4  // sequence
					+ 1  // 0 length scriptsig
					+ 1  // 0 length witness
			}
		} else {
			return Err(InvalidOffboardRequestError("non-standard scriptPubkey"));
		};
		Ok(fee_rate * Weight::from_vb(vb).expect("no overflow"))
	}

	/// Validate that the offboard has a valid script.
	pub fn validate(&self) -> Result<(), InvalidOffboardRequestError> {
		Self::calculate_fee(&self.script_pubkey, FeeRate::ZERO)?;
		Ok(())
	}

	/// Convert into a tx output.
	pub fn to_txout(&self) -> TxOut {
		TxOut {
			script_pubkey: self.script_pubkey.clone(),
			value: self.amount,
		}
	}

	/// Returns the fee charged for the user to make this offboard given the fee rate.
	pub fn fee(&self, fee_rate: FeeRate) -> Result<Amount, InvalidOffboardRequestError> {
		Ok(Self::calculate_fee(&self.script_pubkey, fee_rate)?)
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VtxoSubset {
	pub id: VtxoId,
	#[serde(rename = "amount_sat", with = "bitcoin::amount::serde::as_sat")]
	pub amount: Amount
}

/// A [`Movement`] represents any balance change, it can be of three kinds.
///
/// ### Incoming payment
/// The wallet receives a new VTXO: the balance increases.
/// The resulting movement will only have `receives` field filled
///
/// ### Outgoing payment
/// The wallet sends a set of VTXOs: the balance decreases.
/// The resulting movement will reference spent VTXOs in `spends` field,
/// change VTXO in `receives` one and a non-null destination (either pubkey or BOLT11)
///
/// ### Refreshes
/// Wallet's VTXOs are replaced by new ones, and a small fee is paid: the balance decreases.
/// The resulting movement will reference refreshed VTXOs in `spends` field,
/// new ones in `receives`, and no destination.
#[derive(Debug, Deserialize, Serialize)]
pub struct Movement {
	pub id: u32,
	/// Can either be a publickey or a bolt11 invoice
	///
	/// Paid amount can be computed as: `paid = sum(spends) - sum(receives) - fees`
	pub destination: Option<String>,
	/// Fees paid for the movement
	pub fees: Amount,
	/// wallet's VTXOs spent in this movement
	pub spends: Vec<VtxoSubset>,
	/// Received VTXOs from this movement
	pub receives: Vec<VtxoSubset>,
	/// Movement date
	pub created_at: String,
}
