
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::Context;
use bitcoin::{Amount, FeeRate};
use config::{Environment, File};
use serde::Deserialize;

use crate::{serde_util, sweeps};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Bitcoind {
	/// the URL of the bitcoind RPC (mandatory)
	pub url: String,
	/// the path of the cookie file for the bitcoind RPC
	/// It is mandatory to configure exactly one authentication method
	/// This could either be [bitcoind.cookie] or [bitcoind.rpc_user] and [bitcoind.rpc_pass]
	pub cookie: Option<PathBuf>,
	/// the user for the bitcoind RPC
	/// It is mandatory to configure exactly one authentication method
	/// If a [bitcoind.rpc_pass] is provided [bitcoind.rpc_user] must be provided
	pub rpc_user: Option<String>,
	/// the password for the bitcoind RPC
	/// It is mandatory to configure exactly one authentication method
	/// If a [bitcoind.rpc_user] is provided [bitcoind.rpc_pass] must be provided
	pub rpc_pass: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Rpc {
	/// The socket to bind to for the public Ark gRPC.
	pub public_address: SocketAddr,
	/// The socket to bind to for the private admin gRPC.
	pub admin_address: Option<SocketAddr>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Lightningd {
	#[serde(with = "serde_util::uri")]
	pub uri: tonic::transport::Uri,
	pub server_cert_path: PathBuf,
	pub client_cert_path: PathBuf,
	pub client_key_path: PathBuf,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Postgres {
	pub host: String,
	pub port: u16,
	pub name: String,
	pub user: Option<String>,
	pub password: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
	pub data_dir: PathBuf,
	/// Directory to place structured log files.
	pub log_dir: Option<PathBuf>,
	pub network: bitcoin::Network,
	pub vtxo_expiry_delta: u16,
	pub vtxo_exit_delta: u16,
	pub htlc_delta: u16,
	pub htlc_expiry_delta: u16,

	/// Maximum value any vtxo can have.
	#[serde(with = "bitcoin::amount::serde::as_sat::opt")]
	pub max_vtxo_amount: Option<Amount>,
	/// Number of confirmations needed for board vtxos to be spend in rounds.
	pub round_board_confirmations: usize,
	/// Number of confirmations untrusted inputs of the round tx need to have.
	pub round_tx_untrusted_input_confirmations: usize,

	#[serde(with = "serde_util::duration")]
	pub round_interval: Duration,
	#[serde(with = "serde_util::duration")]
	pub round_submit_time: Duration,
	#[serde(with = "serde_util::duration")]
	pub round_sign_time: Duration,
	pub nb_round_nonces: usize,
	#[serde(with = "serde_util::fee_rate")]
	pub round_tx_feerate: FeeRate,

	/// Whether or not to add full error information to RPC internal errors.
	pub rpc_rich_errors: bool,

	/// The interval at which the txindex checks tx statuses.
	#[serde(with = "serde_util::duration")]
	pub txindex_check_interval: Duration,

	/// A message that can be used by the operator to make
	/// announcements to all cliens.
	pub handshake_psa: Option<String>,

	pub otel_collector_endpoint: Option<String>,

	/// Config for the VtxoSweeper process.
	pub vtxo_sweeper: sweeps::Config,
	pub rpc: Rpc,
	pub postgres: Postgres,

	pub bitcoind: Bitcoind,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub lightningd: Option<Lightningd>,

	// compatibility flags

	pub legacy_wallet: bool,
}

impl Default for Config {
	fn default() -> Self {
		Config {
			data_dir: "./aspd".into(),
			log_dir: None,
			network: bitcoin::Network::Regtest,
			vtxo_expiry_delta: 24 * 6,
			vtxo_exit_delta: 2 * 6,
			htlc_delta: 6,
			htlc_expiry_delta: 6,

			max_vtxo_amount: None,
			round_board_confirmations: 12,
			round_tx_untrusted_input_confirmations: 2,

			round_interval: Duration::from_secs(10),
			round_submit_time: Duration::from_millis(2000),
			round_sign_time: Duration::from_millis(2000),
			nb_round_nonces: 64,
			round_tx_feerate: FeeRate::from_sat_per_vb_unchecked(10),

			rpc_rich_errors: true,

			handshake_psa: None,

			txindex_check_interval: Duration::from_secs(30),

			otel_collector_endpoint: None,
			vtxo_sweeper: Default::default(),
			rpc: Rpc {
				public_address: "127.0.0.1:3535".parse().unwrap(),
				admin_address: Some("127.0.0.1:3536".parse().unwrap()),
			},
			bitcoind: Bitcoind {
				url: "http://127.0.0.1:18443".into(),
				cookie: None,
				rpc_user: None,
				rpc_pass: None,
			},
			postgres: Postgres {
				host: String::from("localhost"),
				port: 5432,
				name: String::from("aspdb"),
				user: None,
				password: None
			},
			lightningd : None,

			legacy_wallet: false,
		}
	}
}

impl Config {
	fn load_with_custom_env(
		config_file: Option<&Path>,
		#[cfg(test)]
		custom_env: Option<std::collections::HashMap<String, String>>,
	) -> anyhow::Result<Self> {
		let default = config::Config::try_from(&Self::default())
			.expect("default config failed to deconstruct");

		// We'll add three layers of config:
		// - the defaults defined in Config's Default impl
		// - the config file passed in this function, if any
		// - environment variables (prefixed with `ASPD_`)

		let mut builder = config::Config::builder()
			.add_source(default);
		if let Some(file) = config_file {
			builder = builder.add_source(File::from(file));
		}
		let env = Environment::with_prefix("ASPD")
			.separator("__");
		#[cfg(test)]
		let env = env.source(custom_env);
		builder = builder.add_source(env);

		let cfg = builder.build().context("error building config")?;
		Ok(cfg.try_deserialize().context("error parsing config")?)
	}

	pub fn load(config_file: Option<&Path>) -> anyhow::Result<Self> {
		Self::load_with_custom_env(config_file, #[cfg(test)] None)
	}

	/// Verifies if the specified configuration is valid
	///
	/// It also checks if all required configurations are available
	pub fn validate(&self) -> anyhow::Result<()> {
		let with_user_pass = match (&self.bitcoind.rpc_user, &self.bitcoind.rpc_pass) {
			(Some(_), None) => bail!("Missing configuration bitcoind.rpc_pass. \
				This is required if bitcoind.rpc_user is provided"),
			(None, Some(_)) => bail!("Missing configuration bitcoind.rpc_user. \
				This is required if bitcoind.rpc_pass is provided"),
			(None, None) => false,
			(Some(_),Some(_)) => true,
		};

		if !with_user_pass && self.bitcoind.cookie.is_none() {
			bail!("Configuring authentication to bitcoind is mandatory. \
				Specify either bitcoind.cookie or (bitcoind.rpc_user and bitcoind.rpc_pass).")
		} else if with_user_pass && self.bitcoind.cookie.is_some() {
			bail!("Invalid configuration for authentication to bitcoind. Use either \
				bitcoind.cookie or (bitcoind.rpc_user and bitcoind.rpc_pass) but not both.")
		}

		Ok(())
	}

	pub fn bitcoind_auth(&self) -> bdk_bitcoind_rpc::bitcoincore_rpc::Auth {
		match (&self.bitcoind.rpc_user, &self.bitcoind.rpc_pass) {
			(Some(user), Some(pass)) => bdk_bitcoind_rpc::bitcoincore_rpc::Auth::UserPass(
				user.into(), pass.into(),
			),
			(Some(_), None) => panic!("Missing configuration for bitcoind.rpc_pass."),
			(None, Some(_)) => panic!("Missing configuration for bitcoind.rpc_user."),
			(None, None) => {
				let bitcoind_cookie_file = self.bitcoind.cookie.as_ref()
					.expect("The bitcoind.cookie must be set if username and password aren't provided");

				bdk_bitcoind_rpc::bitcoincore_rpc::Auth::CookieFile(bitcoind_cookie_file.into())
			}
		}
	}

	pub fn write_to_file(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
		let path = path.as_ref();
		let s = toml::to_string_pretty(self).expect("config serialization error");
		std::fs::write(path, &s)
			.with_context(|| format!("error writing config to {}", path.display()))?;
		Ok(())
	}
}

#[cfg(test)]
mod test {
	use std::collections::HashMap;
	use std::str::FromStr;
	use tonic::transport::Uri;
	use super::*;

	#[test]
	fn validate_bitcoind_config() {
		let bitcoind_url = String::from("http://belson.labs:13444");
		let bitcoind_cookie = Some(PathBuf::from("/not/hot/dog/but/cookie"));
		let bitcoind_rpc_user = Some(String::from("erlich"));
		let bitcoind_rpc_pass = Some(String::from("belson"));

		let mut cfg = Config::load(None).unwrap();
		cfg.bitcoind.url = bitcoind_url.clone();
		cfg.bitcoind.cookie = bitcoind_cookie.clone();
		cfg.validate().expect("This config should be valid");

		let mut cfg = Config::load(None).unwrap();
		cfg.bitcoind.url = bitcoind_url.clone();
		cfg.bitcoind.rpc_user = bitcoind_rpc_user.clone();
		cfg.bitcoind.rpc_pass = bitcoind_rpc_pass.clone();
		cfg.validate().expect("This config should be valid");

		let mut cfg = Config::load(None).unwrap();
		cfg.bitcoind.url = bitcoind_url.clone();
		cfg.validate().expect_err("Invalid because auth info is missing");

		let mut cfg = Config::load(None).unwrap();
		cfg.bitcoind.url = bitcoind_url.clone();
		cfg.bitcoind.rpc_user = bitcoind_rpc_user.clone();
		cfg.validate().expect_err("Invalid because pass is missing");

		let mut cfg = Config::load(None).unwrap();
		cfg.bitcoind.url = bitcoind_url.clone();
		cfg.bitcoind.cookie = bitcoind_cookie.clone();
		cfg.bitcoind.rpc_user = bitcoind_rpc_user.clone();
		cfg.bitcoind.rpc_pass = bitcoind_rpc_pass.clone();
		cfg.validate().expect_err("Invalid. Either cookie or pass but not both");
	}

	#[test]
	fn init_accepts_full_cln_config() {
		let bitcoind_cookie = Some(PathBuf::from("/not/hot/dog/but/cookie"));
		let uri = "http://belson.labs:13444".to_string();
		let server_cert_path = "/hooli/http_public/certs/server.crt".to_string();
		let client_cert_path = "/hooli/http_public/certs/client.crt".to_string();
		let client_key_path = "/hooli/http_public/certs/client.key".to_string();

		let mut cfg = Config::load(None).unwrap();

		let cln = Lightningd {
			uri: Uri::from_str(uri.clone().as_str()).unwrap(),
			server_cert_path: PathBuf::from(server_cert_path.clone()),
			client_cert_path: PathBuf::from(client_cert_path.clone()),
			client_key_path: PathBuf::from(client_key_path.clone()),
		};

		cfg.bitcoind.cookie = bitcoind_cookie.clone();
		cfg.lightningd = Some(cln);

		cfg.validate().expect("invalid configuration");

		let lncfg = cfg.lightningd.as_ref().unwrap();
		assert_eq!(lncfg.uri, Uri::from_str(uri.clone().as_str()).unwrap());
		assert_eq!(lncfg.server_cert_path, PathBuf::from(server_cert_path));
		assert_eq!(lncfg.client_cert_path, PathBuf::from(client_cert_path));
		assert_eq!(lncfg.client_key_path, PathBuf::from(client_key_path));
	}

	#[test]
	// ignoring this test because concurrency with environment variables is causing problems.
	fn cln_config_from_env_vars() {
		let uri = "http://belson.labs:12345";
		let server_cert_path = "/hooli/http_public/certs/server.crt";
		let client_cert_path = "/hooli/http_public/certs/client.crt";
		let client_key_path = "/hooli/http_public/certs/client.key";

		let env = [
			("ASPD__VTXO_EXPIRY_DELTA", "42"),
			("ASPD__BITCOIND__COOKIE", "/not/hot/dog/but/cookie"),
			("ASPD__LIGHTNINGD__URI", uri),
			("ASPD__LIGHTNINGD__SERVER_CERT_PATH", server_cert_path),
			("ASPD__LIGHTNINGD__CLIENT_CERT_PATH", client_cert_path),
			("ASPD__LIGHTNINGD__CLIENT_KEY_PATH", client_key_path),
		].into_iter().map(|(k, v)| (k.into(), v.into())).collect::<HashMap<String, String>>();

		let cfg = Config::load_with_custom_env(None, Some(env)).unwrap();
		cfg.validate().expect("invalid configuration");

		assert_eq!(cfg.vtxo_expiry_delta, 42);
		assert_eq!(cfg.bitcoind.cookie, Some("/not/hot/dog/but/cookie".into()));
		let lncfg = cfg.lightningd.as_ref().unwrap();
		assert_eq!(lncfg.uri, Uri::from_str(uri).unwrap());
		assert_eq!(lncfg.server_cert_path, PathBuf::from(server_cert_path));
		assert_eq!(lncfg.client_cert_path, PathBuf::from(client_cert_path));
		assert_eq!(lncfg.client_key_path, PathBuf::from(client_key_path));
	}
}
