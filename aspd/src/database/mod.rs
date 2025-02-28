use std::collections::HashMap;

use anyhow::Context;
use ark::{tree::signed::CachedSignedVtxoTree, ArkoorVtxo, BlockHeight, OnboardVtxo, Vtxo, VtxoId};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use bdk_wallet::{chain::Merge, ChangeSet};
use bitcoin::{consensus::serialize, secp256k1::{schnorr, PublicKey}, Transaction, Txid};
use futures::{Stream, StreamExt, TryStreamExt};
use model::{MailboxArkoor, PendingSweep, StoredRound, VtxoState};
use tokio_postgres::{types::Type, Client, GenericClient, NoTls};

use crate::Config;

pub mod model;

mod embedded {
	use refinery::embed_migrations;
	embed_migrations!("src/database/migrations");
}

const DEFAULT_DATABASE: &str = "postgres";

pub struct Db {
	client: Pool<PostgresConnectionManager<NoTls>>
}

impl Db {
	async fn run_migrations<'a>(manager: &Pool<PostgresConnectionManager<NoTls>>) -> anyhow::Result<()> {
		let mut conn = manager.get().await?;
		embedded::migrations::runner().run_async::<Client>(&mut conn).await?;
		info!("All migrations got successfully run");
		Ok(())
	}

	fn config(database: &str, app_config: &Config) -> tokio_postgres::Config {
		let mut config = tokio_postgres::Config::new();
		config.host(&app_config.postgres.host);
		config.port(app_config.postgres.port);
		config.dbname(database);
		if let Some(user) = &app_config.postgres.user {
			config.user(user);
		}
		if let Some(password) = &app_config.postgres.password {
			config.password(password);
		}

		config
	}

	pub async fn create(app_config: &Config) -> anyhow::Result<Self> {
		let config = Self::config(DEFAULT_DATABASE, app_config);

		let manager = PostgresConnectionManager::new(config, NoTls);
		let pool = Pool::builder().build(manager).await.expect("Failed to create pool");

		let conn = pool.get().await?;
		let statement = conn.prepare(
			&format!("CREATE DATABASE \"{}\"", app_config.postgres.name)
		).await?;
		conn.execute(&statement, &[]).await?;

		Self::connect(app_config).await
	}

	pub async fn connect(app_config: &Config) -> anyhow::Result<Self> {
		let config = Self::config(&app_config.postgres.name, app_config);

		let manager = PostgresConnectionManager::new(config, NoTls);
		let pool = Pool::builder().build(manager).await.expect("Failed to create pool");

		Self::run_migrations(&pool).await?;
		Ok(Self { client: pool })
	}

	/**
	 * VTXOs
	*/

	async fn inner_insert_vtxos<T>(client: &T, vtxos: &[Vtxo]) -> anyhow::Result<()>
		where T: GenericClient
	{
		// Store all vtxos created in this round.
		let statement = client.prepare_typed("
			INSERT INTO vtxo (id, vtxo, expiry) VALUES ($1, $2, $3);
		", &[Type::TEXT, Type::BYTEA, Type::INT4]).await?;

		for vtxo in vtxos {
			let vtxo_id = vtxo.id();

			client.execute(
				&statement,
				&[
					&vtxo_id.to_string(),
					&Vtxo::encode(&vtxo),
					&(vtxo.spec().expiry_height as i32)
				]
			).await?;
		}

		Ok(())
	}

	/// Atomically insert the given vtxos.
	pub async fn insert_vtxos(&self, vtxos: &[Vtxo]) -> anyhow::Result<()> {
		let mut conn = self.client.get().await?;
		let tx = conn.transaction().await?;

		Self::inner_insert_vtxos(&tx, vtxos).await?;

		tx.commit().await?;
		Ok(())
	}

	/// Get all onboard vtxos that expired before or on `height`.
	pub async fn get_expired_onboards(
		&self,
		height: BlockHeight,
	) -> anyhow::Result<impl Stream<Item = anyhow::Result<OnboardVtxo>> + '_> {
		let conn = self.client.get().await?;

		// TODO: maybe store kind in a column to filter onboard at the db level
		let statement = conn.prepare_typed("
			SELECT id, vtxo, expiry, oor_spent, forfeit_sigs FROM vtxo WHERE expiry <= $1
		", &[Type::INT4]).await?;

		let rows = conn.query_raw(&statement, &[&(height as i32)]).await?;

		Ok(rows.filter_map(|row| async move {
			row
				.map(|row | VtxoState::try_from(row).expect("corrupt db").vtxo.into_onboard())
				.map_err(Into::into)
				.transpose()
		}).fuse())
	}

	pub async fn remove_onboard(&self, vtxo: &OnboardVtxo) -> anyhow::Result<()> {
		let conn = self.client.get().await?;

		// TODO: implement soft deletion
		let statement = conn.prepare("
			DELETE FROM vtxo WHERE id = $1;
		").await?;

		conn.execute(&statement, &[&vtxo.id().to_string()]).await?;

		Ok(())
	}

	async fn get_vtxo<T>(client: &T, id: &VtxoId) -> anyhow::Result<VtxoState>
		where T: GenericClient {
		let statement = client.prepare("
			SELECT id, vtxo, expiry, oor_spent, forfeit_sigs FROM vtxo WHERE id = $1
		").await?;

		let row = client.query_opt(&statement, &[&id.to_string()]).await?
			.context(*id)
			.with_context(|| format!("vtxo {} not found", id))?;

		Ok(VtxoState::try_from(row).expect("corrupt db"))
	}

	/// Check whether the vtxos were already spent, and fetch them if not.
	///
	/// There is no guarantee that the vtxos are still all unspent by
	/// the time this call returns. The caller should ensure no changes
	/// are made to them meanwhile.
	pub async fn check_fetch_unspent_vtxos(&self, ids: &[VtxoId]) -> anyhow::Result<Vec<Vtxo>> {
		let conn = self.client.get().await?;
		let mut ret = Vec::with_capacity(ids.len());

		for id in ids {
			let vtxo_state = Self::get_vtxo(&*conn, id).await?;

			if !vtxo_state.is_spendable() {
				return Err(anyhow!("vtxo {} is not spendable: {:?}", id, vtxo_state)
					.context(*id));
			}

			ret.push(vtxo_state.vtxo.clone());
		}

		Ok(ret)
	}

	/// Set the vtxo as being forfeited.
	pub async fn set_vtxo_forfeited(&self, id: VtxoId, sigs: Vec<schnorr::Signature>) -> anyhow::Result<()> {
		let conn = self.client.get().await?;
		let statement = conn.prepare_typed("
			UPDATE vtxo SET forfeit_sigs = $2 WHERE id = $1;
		", &[Type::TEXT, Type::BYTEA_ARRAY]).await?;

		let vtxo_state = Self::get_vtxo(&*conn, &id).await?;
		if !vtxo_state.is_spendable() {
			error!("Marking unspendable vtxo as forfeited: {:?}", vtxo_state);
		}

		conn.execute(
			&statement,
			&[
				&id.to_string(),
				&sigs.into_iter().map(|s| s.serialize().to_vec()).collect::<Vec<_>>()
			]
		).await?;

		Ok(())
	}

	/// Returns [None] if all the ids were not previously marked as signed
	/// and are now correctly marked as such.
	/// Returns [Some] for the first vtxo that was already signed.
	///
	/// Also stores the new OOR vtxos atomically.
	pub async fn check_set_vtxo_oor_spent(
		&self,
		spent_ids: &[VtxoId],
		spending_tx: Txid,
		new_vtxos: &[ArkoorVtxo],
	) -> anyhow::Result<Option<VtxoId>> {
		let mut conn = self.client.get().await?;
		let tx = conn.transaction().await?;

		let statement = tx.prepare_typed("
			UPDATE vtxo SET oor_spent = $2 WHERE id = $1;
		", &[Type::TEXT, Type::BYTEA]).await?;

		for id in spent_ids {
			let vtxo_state = Self::get_vtxo(&tx, id).await?;
			if !vtxo_state.is_spendable() {
				return Ok(Some(*id));
			}

			tx.execute(&statement, &[&id.to_string(), &serialize(&spending_tx)]).await?;
		}

		let new_vtxos = new_vtxos.into_iter().map(|a| a.clone().into()).collect::<Vec<_>>();
		Self::inner_insert_vtxos(&tx, &new_vtxos).await?;

		tx.commit().await?;
		Ok(None)
	}

	/**
	 * Arkoors
	*/

	pub async fn store_oor(&self, pubkey: PublicKey, vtxo: Vtxo) -> anyhow::Result<()> {
		let conn = self.client.get().await?;
		let statement = conn.prepare("
			INSERT INTO arkoor_mailbox (id, pubkey, vtxo) VALUES ($1, $2, $3);
		").await?;
		conn.execute(
			&statement,
			&[&vtxo.id().to_string(), &pubkey.serialize().to_vec(), &vtxo.encode()]
		).await?;

		Ok(())
	}

	pub async fn pull_oors(&self, pubkey: PublicKey) -> anyhow::Result<Vec<Vtxo>> {
		let conn = self.client.get().await?;
		let statement = conn.prepare("
			SELECT id, pubkey, vtxo FROM arkoor_mailbox WHERE pubkey = $1
		").await?;

		let rows = conn.query(&statement, &[&pubkey.serialize().to_vec()]).await?;
		let oors = rows
			.into_iter()
			.map(|row| -> anyhow::Result<Vtxo> { Ok(MailboxArkoor::try_from(row).expect("corrupt db").vtxo) })
			.collect::<Result<Vec<_>, _>>()?;

		// TODO: implement soft deletion
		let statement = conn.prepare("
			DELETE FROM arkoor_mailbox WHERE pubkey = $1;
		").await?;
		let result = conn.execute(&statement, &[&pubkey.serialize().to_vec()]).await?;
		assert_eq!(result, oors.len() as u64);

		Ok(oors)
	}

	/**
	 * Rounds
	*/

	pub async fn store_round(
		&self,
		round_tx: Transaction,
		vtxos: CachedSignedVtxoTree,
		nb_input_vtxos: usize,
	) -> anyhow::Result<()> {
		let round_id = round_tx.compute_txid();

		let mut conn = self.client.get().await?;
		let tx = conn.transaction().await?;

		let statement = tx.prepare_typed("
			INSERT INTO round (id, tx, signed_tree, nb_input_vtxos, expiry)
			VALUES ($1, $2, $3, $4, $5);
		", &[Type::TEXT, Type::BYTEA, Type::BYTEA, Type::INT4, Type::INT4]).await?;
		tx.execute(
			&statement,
			&[
				&round_id.to_string(),
				&serialize(&round_tx),
				&vtxos.spec.encode(),
				&(nb_input_vtxos as i32),
				&(vtxos.spec.spec.expiry_height as i32)
			]
		).await?;

		Self::inner_insert_vtxos(&tx, &vtxos.all_vtxos().collect::<Vec<_>>()).await?;

		tx.commit().await?;
		Ok(())
	}

	/// Get an iterator that yields each round in the database.
	///
	/// No particular order is guaranteed.
	pub async fn fetch_all_rounds(&self) -> anyhow::Result<impl Stream<Item = anyhow::Result<StoredRound>> + '_> {
		let conn = self.client.get().await?;
		let statement = conn.prepare("
			SELECT id, tx, signed_tree, nb_input_vtxos FROM round
		").await?;

		let params: Vec<String> = vec![];
		let rows = conn.query_raw(&statement, params).await?;

		Ok(
			rows
				.map_ok(|row| StoredRound::try_from(row).expect("corrupt db"))
				.map_err(Into::into)
		)
	}

	pub async fn get_round(&self, id: Txid) -> anyhow::Result<Option<StoredRound>> {
		let conn = self.client.get().await?;
		let statement = conn.prepare("
			SELECT id, tx, signed_tree, nb_input_vtxos FROM round WHERE id = $1;
		").await?;

		let rows = conn.query(&statement, &[&id.to_string()]).await?;
		let round = match rows.get(0) {
			Some(row) => Some(StoredRound::try_from(row.clone()).expect("corrupt db")),
			_ => None
		};

		Ok(round)
	}

	pub async fn remove_round(&self, id: Txid) -> anyhow::Result<()> {
		let conn = self.client.get().await?;

		// TODO: implement soft deletion
		let statement = conn.prepare("
			DELETE FROM round WHERE id = $1;
		").await?;

		conn.execute(&statement, &[&id.to_string()]).await?;

		Ok(())
	}

	/// Get all round IDs of rounds that expired before or on `height`.
	pub async fn get_expired_rounds(&self, height: BlockHeight) -> anyhow::Result<Vec<Txid>> {
		let conn = self.client.get().await?;
		let statement = conn.prepare("
			SELECT id, tx, signed_tree, nb_input_vtxos FROM round WHERE expiry <= $1
		").await?;

		let rows = conn.query_raw(&statement, &[&(height as i32)]).await?;
		Ok(rows.map_ok(|row| StoredRound::try_from(row).expect("corrupt db").id).try_collect::<Vec<_>>().await?)
	}

	pub async fn get_fresh_round_ids(&self, height: u32) -> anyhow::Result<Vec<Txid>> {
		let conn = self.client.get().await?;
		let statement = conn.prepare("
			SELECT id, tx, signed_tree, nb_input_vtxos FROM round WHERE expiry > $1
		").await?;

		let rows = conn.query_raw(&statement, &[&(height as i32)]).await?;
		Ok(rows.map_ok(|row| StoredRound::try_from(row).expect("corrupt db").id).try_collect::<Vec<_>>().await?)
	}

	/**
	 * Sweeps
	*/

	/// Add the pending sweep tx.
	pub async fn store_pending_sweep(&self, txid: &Txid, tx: &Transaction) -> anyhow::Result<()> {
		let conn = self.client.get().await?;
		let statement = conn.prepare_typed("
			INSERT INTO pending_sweep (txid, tx) VALUES ($1, $2);
		", &[Type::TEXT, Type::BYTEA]).await?;
		conn.execute(
			&statement,
			&[&txid.to_string(), &serialize(tx)]
		).await?;

		Ok(())
	}

	/// Drop the pending sweep tx by txid.
	pub async fn drop_pending_sweep(&self, txid: &Txid) -> anyhow::Result<()> {
		let conn = self.client.get().await?;

		// TODO: implement soft deletion
		let statement = conn.prepare("
			DELETE FROM pending_sweep WHERE txid = $1;
		").await?;
		conn.execute(&statement, &[&txid.to_string()]).await?;

		Ok(())
	}

	/// Fetch all pending sweep txs.
	pub async fn fetch_pending_sweeps(&self) -> anyhow::Result<HashMap<Txid, Transaction>> {
		let conn = self.client.get().await?;
		let statement = conn.prepare("
			SELECT txid, tx FROM pending_sweep
		").await?;

		let rows = conn.query(&statement, &[]).await?;

		let pending_sweeps = rows
			.into_iter()
			.map(|row| -> anyhow::Result<(Txid, Transaction)> {
				let sweep = PendingSweep::try_from(row).expect("corrupt db");
				Ok((sweep.txid, sweep.tx))
			})
			.collect::<Result<HashMap<Txid, Transaction>, _>>()?;

		Ok(pending_sweeps)
	}

	/**
	 * Wallet
	*/

	pub async fn get_master_seed(&self) -> anyhow::Result<Option<Vec<u8>>> {
		let conn = self.client.get().await?;
		let statement = conn.prepare("
			SELECT seed FROM wallet
		").await?;
		let rows = conn.query(&statement, &[]).await?;

		Ok(rows.get(0).map(|row| row.get::<_, Vec<u8>>(0)))
	}

	pub async fn get_master_mnemonic(&self) -> anyhow::Result<Option<String>> {
		let conn = self.client.get().await?;
		let statement = conn.prepare("
			SELECT mnemonic FROM wallet
		").await?;
		let rows = conn.query(&statement, &[]).await?;

		Ok(rows.get(0).map(|row| row.get::<_, String>(0)))
	}

	pub async fn store_master_mnemonic_and_seed(&self, mnemonic: &bip39::Mnemonic) -> anyhow::Result<()> {
		if self.get_master_seed().await?.is_some() {
			bail!("a wallet already exists in this DB")
		}

		let conn = self.client.get().await?;
		let statement = conn.prepare_typed("
			INSERT INTO wallet (mnemonic, seed) VALUES ($1, $2);
		", &[Type::TEXT, Type::BYTEA]).await?;
		conn.execute(
			&statement,
			&[&mnemonic.to_string(), &mnemonic.to_seed("").to_vec()]
		).await?;

		Ok(())
	}

	pub async fn store_changeset(&self, c: &ChangeSet) -> anyhow::Result<()> {
		let mut buf = Vec::new();
		ciborium::into_writer(c, &mut buf).unwrap();

		let conn = self.client.get().await?;
		let statement = conn.prepare_typed("
			INSERT INTO wallet_changeset (content) VALUES ($1);
		", &[Type::BYTEA]).await?;
		conn.execute(&statement, &[&buf]).await?;

		Ok(())
	}

	pub async fn read_aggregate_changeset(&self) -> anyhow::Result<Option<ChangeSet>> {
		let mut ret = Option::<ChangeSet>::None;

		let conn = self.client.get().await?;
		let statement = conn.prepare("
			SELECT content FROM wallet_changeset
		").await?;
		let rows = conn.query(&statement, &[]).await?;

		for row in rows {
			let value = row.get::<_, Vec<u8>>(0);
			let cs = ciborium::from_reader::<ChangeSet, _>(&*value).context("corrupt db: changeset value")?;

			if let Some(ref mut r) = ret {
				r.merge(cs);
			} else {
				ret = Some(cs);
			}
		}

		Ok(ret)
	}
}