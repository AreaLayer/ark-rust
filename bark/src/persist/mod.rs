pub mod sqlite;

use ark::{Movement, Vtxo, VtxoId};
use bdk_wallet::WalletPersister;
use bitcoin::{secp256k1::PublicKey, Amount};

use crate::{exit::ExitIndex, Config, Pagination, WalletProperties};

pub trait BarkPersister: Clone + WalletPersister {
	/// Initialise wallet in the database
	///
	/// Will fail after first call
	fn init_wallet(&self, config: &Config, properties: &WalletProperties) -> anyhow::Result<()>;

	fn write_config(&self, config: &Config) -> anyhow::Result<()>;
	fn read_properties(&self) -> anyhow::Result<Option<WalletProperties>>;
	fn read_config(&self) -> anyhow::Result<Option<Config>>;

	/// Returns a paginated list of movements
	fn list_movements(&self, pagination: Pagination) -> anyhow::Result<Vec<Movement>>;
	/// Register incoming payment
	fn register_receive(&self, vtxo: &Vtxo) -> anyhow::Result<()>;
	/// Register outgoint payment
	fn register_send<'a>(
		&self,
		vtxos: impl IntoIterator<Item = &'a Vtxo>,
		destination: String,
		change: Option<&Vtxo>,
		fees: Option<Amount>
	) -> anyhow::Result<()>;
	/// Register in-round refresh
	fn register_refresh(&self, input_vtxos: &[Vtxo], output_vtxos: &[Vtxo]) -> anyhow::Result<()>;

	/// Fetch a VTXO by id in the database
	fn get_vtxo(&self, id: VtxoId) -> anyhow::Result<Option<Vtxo>>;
	/// Fetch all currently spendable VTXOs in the database
	fn get_all_spendable_vtxos(&self) -> anyhow::Result<Vec<Vtxo>>;
	/// Get the soonest-expiring vtxos with total value at least `min_value`.
	fn get_expiring_vtxos(&self, min_value: Amount) -> anyhow::Result<Vec<Vtxo>>;
	/// Remove a VTXO from the database
	fn remove_vtxo(&self, id: VtxoId) -> anyhow::Result<Option<Vtxo>>;
	/// Check whether a VTXO has been spent already or not
	fn has_spent_vtxo(&self, id: VtxoId) -> anyhow::Result<bool>;

	/// Store a newly revealed index
	fn store_vtxo_key_index(&self, index: u32, public_key: PublicKey) -> anyhow::Result<()>;
	/// Get last revealed index
	fn get_last_vtxo_key_index(&self) -> anyhow::Result<Option<u32>>;
	/// Get index of vtxo key
	fn get_vtxo_key_index(&self, vtxo: &Vtxo) -> anyhow::Result<u32>;
	/// Checks if provided public key exists in the database,
	/// meaning that it is owned by the wallet
	fn check_vtxo_key_exists(&self, public_key: &PublicKey) -> anyhow::Result<bool>;

	/// Store the ongoing exit process.
	fn store_exit(&self, exit: &ExitIndex) -> anyhow::Result<()>;
	/// Fetch an ongoing exit process.
	fn fetch_exit(&self) -> anyhow::Result<Option<ExitIndex>>;

	fn get_last_ark_sync_height(&self) -> anyhow::Result<u32>;
	fn store_last_ark_sync_height(&self, height: u32) -> anyhow::Result<()>;
}