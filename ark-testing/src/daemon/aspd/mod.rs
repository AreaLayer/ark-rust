
pub mod proxy;

use std::env;
use std::time::{Duration, Instant};
use std::path::PathBuf;

use aspd_rpc as rpc;
use bitcoin::{Amount, Network};
use bitcoin::address::{Address, NetworkUnchecked};
use tokio::sync::{self, mpsc};
use tokio::process::Command;

use aspd_log::{LogMsg, ParsedRecord, TipUpdated, TxIndexUpdateFinished};

use crate::{Bitcoind, Daemon, DaemonHelper, Lightningd};
use crate::constants::bitcoind::{BITCOINRPC_TEST_PASSWORD, BITCOINRPC_TEST_USER};
use crate::constants::env::ASPD_EXEC;
use crate::util::resolve_path;

pub type Aspd = Daemon<AspdHelper>;

pub type AdminClient = rpc::AdminServiceClient<tonic::transport::Channel>;
pub type ArkClient = rpc::ArkServiceClient<tonic::transport::Channel>;


pub struct AspdHelper {
	name: String,
	state: AspdState,
	config: AspdConfig,
	bitcoind: Bitcoind
}

#[derive(Debug)]
pub struct AspdConfig {
	pub datadir: PathBuf,
	pub round_interval: Duration,
	pub round_submit_time: Duration,
	pub round_sign_time: Duration,
	pub vtxo_expiry_delta: u16,
	pub vtxo_exit_delta: u16,
	pub nb_round_nonces: usize,
	pub sweep_threshold: Amount,
	pub round_onboard_confirmations: u64,

	/// Whether the apsd will use user+password for connection to bitcoind (true)
	/// or cookies (false).
	pub use_bitcoind_auth_pass: bool,
	pub cln_grpc_uri: Option<String>,
	pub cln_grpc_server_cert_path: Option<PathBuf>,
	pub cln_grpc_client_cert_path: Option<PathBuf>,
	pub cln_grpc_client_key_path: Option<PathBuf>,
}

#[derive(Default)]
struct AspdState {
	public_grpc_address: Option<String>,
	admin_grpc_address: Option<String>,
}

impl Aspd {
	pub fn bitcoind(&self) -> &Bitcoind {
		&self.inner.bitcoind
	}

	/// Gracefully shutdown bitcoind associated with this ASP.
	pub async fn shutdown_bitcoind(&self) {
		use bitcoincore_rpc::RpcApi;
		self.inner.bitcoind.sync_client().stop().unwrap();

		// Need to wait a bit until all files are written, otherwise the
		// daemon may be sigkilled, leaving incomplete files. Subsequent
		// restart would fail.
		tokio::time::sleep(Duration::from_secs(1)).await;
	}

	pub fn base_cmd() -> Command {
		let e = env::var(ASPD_EXEC).expect("ASPD_EXEC env not set");
		let exec = resolve_path(e).expect("failed to resolve ASPD_EXEC");
		Command::new(exec)
	}

	/// Creates ASP with a dedicated bitcoind daemon.
	pub fn new(name: impl AsRef<str>, bitcoind: Bitcoind, config: AspdConfig) -> Self {
		let helper = AspdHelper {
			name: name.as_ref().to_string(),
			config,
			state: AspdState::default(),
			bitcoind
		};

		Daemon::wrap(helper)
	}

	pub fn asp_url(&self) -> String {
		self.inner.asp_url()
	}

	pub async fn get_admin_client(&self) -> AdminClient {
		self.inner.connect_admin_client().await.unwrap()
	}

	pub async fn get_public_client(&self) -> ArkClient {
		self.inner.connect_public_client().await.unwrap()
	}

	pub async fn get_funding_address(&self) -> Address {
		let mut admin_client = self.get_admin_client().await;
		let response = admin_client.wallet_status(rpc::Empty {}).await.unwrap().into_inner();
		response.address.parse::<Address<NetworkUnchecked>>().unwrap()
			.require_network(Network::Regtest).unwrap()
	}

	pub async fn trigger_round(&self) {
		let start = Instant::now();
		let minimum_wait = tokio::time::sleep(Duration::from_millis(500));
		let mut l1 = self.subscribe_log::<TipUpdated>().await;
		let mut l2 = self.subscribe_log::<TxIndexUpdateFinished>().await;
		self.bitcoind().generate(1).await;
		let _ = tokio::join!(l1.recv(), l2.recv(), minimum_wait);
		trace!("Waited {} ms before starting round", start.elapsed().as_millis());
		self.get_admin_client().await.trigger_round(rpc::Empty {}).await.unwrap();
	}

	/// Subscribe to all structured logs of the given type.
	pub async fn subscribe_log<L: LogMsg>(&self) -> mpsc::UnboundedReceiver<L> {
		let (tx, rx) = sync::mpsc::unbounded_channel();
		self.add_stdout_handler(move |l: &str| {
			if l.starts_with("{") {
				let rec = serde_json::from_str::<ParsedRecord>(l)
					.expect("invalid slog struct");
				if rec.is::<L>() {
					return tx.send(rec.try_as().expect("invalid slog data")).is_err();
				}
			}
			false
		}).await;
		rx
	}

	/// Wait for the first occurrence of the given log message type and return it.
	pub async fn wait_for_log<L: LogMsg>(&self) -> L {
		let (tx, mut rx) = sync::mpsc::channel(1);
		self.add_stdout_handler(move |l: &str| {
			if l.starts_with("{") {
				let rec = serde_json::from_str::<ParsedRecord>(l)
					.expect("invalid slog struct");
				if rec.is::<L>() {
					let msg = rec.try_as().expect("invalid slog data");
					// if channel already closed, user is no longer interested
					let _ = tx.try_send(msg);
					return true;
				}
			}
			false
		}).await;
		rx.recv().await.expect("log wait channel closed")
	}
}

impl DaemonHelper for AspdHelper {
	fn name(&self) -> &str {
		&self.name
	}

	fn datadir(&self) -> PathBuf {
		self.config.datadir.clone()
	}

	async fn make_reservations(&mut self) -> anyhow::Result<()> {
		let public_grpc_port = portpicker::pick_unused_port().expect("No ports free");
		let admin_grpc_port = portpicker::pick_unused_port().expect("No ports free");

		let public_grpc_address = format!("0.0.0.0:{}", public_grpc_port);
		let admin_grpc_address = format!("127.0.0.1:{}", admin_grpc_port);

		let mut base_cmd = Aspd::base_cmd();

		let datadir = self.config.datadir.clone();
		let pgrpc = public_grpc_address.clone();
		let agrpc = admin_grpc_address.clone();

		let output = base_cmd.args([
			"--datadir",
			&datadir.display().to_string(),
			"set-config",
			"--public-rpc-address",
			&pgrpc,
			"--admin-rpc-address",
			&agrpc,
		]).output().await?;

		if !output.status.success() {
			let stderr = String::from_utf8(output.stderr)?;
			error!("{}", stderr);
			bail!("Failed to configure ports for aspd '{}'", self.name());
		};

		self.state.public_grpc_address = Some(public_grpc_address);
		self.state.admin_grpc_address = Some(admin_grpc_address);

		Ok(())
	}

	async fn prepare(&self) -> anyhow::Result<()> {
		if !self.datadir().exists() {
			self.create().await?;
		} else {
			// If datadir exists, it means ASP is being restarted but
			// some settings might have changed in the meantime.
			let output = Aspd::base_cmd().args([
				"--datadir",&self.datadir().display().to_string(),
				"set-config",
				"--bitcoind-url", &self.bitcoind.rpc_url()
			]).output().await?;

			if !output.status.success() {
				let stderr = String::from_utf8(output.stderr)?;
				error!("{}", stderr);
				bail!("Failed to configure ports for aspd '{}'", self.name());
			};
		}
		Ok(())
	}

	async fn get_command(&self) -> anyhow::Result<Command> {
		let mut base_cmd = Aspd::base_cmd();
		base_cmd
			.arg("--datadir")
			.arg(&self.config.datadir)
			.arg("start");

		Ok(base_cmd)
	}

	async fn wait_for_init(&self) -> anyhow::Result<()> {
		while !self.is_ready().await {
			tokio::time::sleep(Duration::from_millis(100)).await;
		}
		Ok(())
	}
}

impl AspdHelper {
	async fn is_ready(&self) -> bool {
		return self.admin_grpc_is_ready().await && self.public_grpc_is_ready().await
	}

	async fn public_grpc_is_ready(&self) -> bool {
		if let Ok(mut c) = self.connect_public_client().await {
			c.get_ark_info(rpc::Empty {}).await.is_ok()
		} else {
			false
		}
	}

	async fn admin_grpc_is_ready(&self) -> bool {
		if let Ok(mut c) = self.connect_admin_client().await {
			c.wallet_status(rpc::Empty {}).await.is_ok()
		} else {
			false
		}
	}

	pub fn asp_url(&self) -> String {
		format!("http://{}", self.state.public_grpc_address.clone().expect("asp not running"))
	}

	pub fn admin_url(&self) -> String {
		format!("http://{}", self.state.admin_grpc_address.clone().expect("asp not running"))
	}

	pub async fn connect_public_client(&self) -> Result<ArkClient, tonic::transport::Error> {
		ArkClient::connect(self.asp_url()).await
	}

	pub async fn connect_admin_client(&self) -> Result<AdminClient, tonic::transport::Error> {
		AdminClient::connect(self.admin_url()).await
	}

	async fn create(&self) -> anyhow::Result<()> {
		let cfg = &self.config;
		let output = {
			let mut cmd = Aspd::base_cmd();
			let datadir = cfg.datadir.display().to_string();
			let bitcoind_url = self.bitcoind.rpc_url().to_string();
			let round_interval = cfg.round_interval.as_millis().to_string();
			let round_submit_time = cfg.round_submit_time.as_millis().to_string();
			let round_sign_time = cfg.round_sign_time.as_millis().to_string();
			let nb_round_nonces = cfg.nb_round_nonces.to_string();
			let vtxo_expiry_delta = cfg.vtxo_expiry_delta.to_string();
			let vtxo_exit_delta = cfg.vtxo_exit_delta.to_string();
			let sweep_threshold = cfg.sweep_threshold.to_string();
			let onboard_confs = cfg.round_onboard_confirmations.to_string();

			let mut args = vec![
				"create",
				"--datadir", &datadir,
				"--bitcoind-url", &bitcoind_url,
				"--network", "regtest",
				"--round-interval", &round_interval,
				"--round-submit-time", &round_submit_time,
				"--round-sign-time",  &round_sign_time,
				"--nb-round-nonces", &nb_round_nonces,
				"--vtxo-expiry-delta", &vtxo_expiry_delta,
				"--vtxo-exit-delta", &vtxo_exit_delta,
				"--sweep-threshold", &sweep_threshold,
				"--round-onboard-confirmations", &onboard_confs,
			];

			let bitcoind_auth = if cfg.use_bitcoind_auth_pass {
				bitcoincore_rpc::Auth::UserPass(BITCOINRPC_TEST_USER.into(), BITCOINRPC_TEST_PASSWORD.into())
			} else {
				self.bitcoind.auth()
			};

			match bitcoind_auth {
				bitcoincore_rpc::Auth::CookieFile(ref cookie) => {
					args.extend([
						"--bitcoind-cookie", cookie.to_str().unwrap(),
					]);
				},
				bitcoincore_rpc::Auth::UserPass(ref user, ref pass) => {
					args.extend([
						"--bitcoind-rpc-user", user,
						"--bitcoind-rpc-pass", pass,
					]);
				},
				bitcoincore_rpc::Auth::None => {
					panic!("aspd has no bitcoin rpc authentication method configured");
				},
			}

			if cfg.cln_grpc_uri.is_some() {
				args.extend(["--cln-grpc-uri", cfg.cln_grpc_uri.as_ref().unwrap()]);
			}
			if cfg.cln_grpc_server_cert_path.is_some() {
				args.extend(["--cln-grpc-server-cert-path", cfg.cln_grpc_server_cert_path.as_ref().unwrap().to_str().unwrap()]);
			}
			if cfg.cln_grpc_client_cert_path.is_some() {
				args.extend(["--cln-grpc-client-cert-path", cfg.cln_grpc_client_cert_path.as_ref().unwrap().to_str().unwrap()]);
			}
			if cfg.cln_grpc_client_key_path.is_some() {
				args.extend(["--cln-grpc-client-key-path", cfg.cln_grpc_client_key_path.as_ref().unwrap().to_str().unwrap()]);
			}

			trace!("base_cmd={:?}; args={:?}", cmd, args);
			cmd.args(args).output()
		}.await?;

		if output.status.success() {
			Ok(())
		} else {
			let stderr = String::from_utf8(output.stderr)?;
			error!("stderr: {}", stderr);
			bail!("Failed to create aspd '{}'", self.name());
		}
	}
}

impl AspdConfig {
	pub async fn configure_lighting(&mut self, lightningd: &Lightningd) {
		let grpc_details = lightningd.grpc_details().await;

		self.cln_grpc_uri = Some(grpc_details.uri);
		self.cln_grpc_server_cert_path = Some(grpc_details.server_cert_path);
		self.cln_grpc_client_cert_path = Some(grpc_details.client_cert_path);
		self.cln_grpc_client_key_path = Some(grpc_details.client_key_path);
	}
}
