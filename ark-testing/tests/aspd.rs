#[macro_use]
extern crate log;

use std::iter;
use std::sync::Arc;
use std::time::Duration;

use bitcoin::Amount;
use bitcoin::hashes::Hash;
use bitcoin::script::PushBytes;
use bitcoin::secp256k1::{Keypair, PublicKey};
use bitcoin::{ScriptBuf, WPubkeyHash};
use futures::future::join_all;
use futures::{Stream, StreamExt};
use tokio::sync::{mpsc, Mutex};

use ark::{musig, VtxoId};
use ark::rounds::VtxoOwnershipChallenge;
use ark::util::SECP;
use aspd_log::{
	NotSweeping, BoardFullySwept, RoundFinished, RoundFullySwept, RoundUserVtxoAlreadyRegistered,
	RoundUserVtxoUnknown, SweepBroadcast, SweeperStats, SweepingOutput, TxIndexUpdateFinished,
	UnconfirmedBoardSpendAttempt
};
use aspd_rpc as rpc;

use ark_testing::{Aspd, TestContext, btc, sat};
use ark_testing::constants::BOARD_CONFIRMATIONS;
use ark_testing::constants::bitcoind::{BITCOINRPC_TEST_PASSWORD, BITCOINRPC_TEST_USER};
use ark_testing::daemon::aspd;
use ark_testing::daemon::aspd::proxy::AspdRpcProxyServer;
use ark_testing::util::{FutureExt, ReceiverExt};

lazy_static::lazy_static! {
	static ref RANDOM_PK: PublicKey = "02c7ef7d49b365974cd219f7036753e1544a3cdd2120eb7247dd8a94ef91cf1e49".parse().unwrap();
}

#[tokio::test]
async fn check_aspd_version() {
	let output = Aspd::base_cmd()
		.arg("--version")
		.output()
		.await
		.expect("Failed to spawn process and capture output");

	let stdout = String::from_utf8(output.stdout).expect("Output is valid utf-8");
	assert!(stdout.starts_with("bark-aspd"))
}

#[tokio::test]
async fn bitcoind_auth_connection() {
	let ctx = TestContext::new("aspd/bitcoind_auth_connection").await;

	let aspd = ctx.new_aspd_with_cfg("aspd", aspd::Config {
		bitcoind: aspd::config::Bitcoind {
			url: "".into(), //t set later by test framework
			cookie: None,
			rpc_user: Some(BITCOINRPC_TEST_USER.to_string()),
			rpc_pass: Some(BITCOINRPC_TEST_PASSWORD.to_string()),
		},
		..ctx.aspd_default_cfg("aspd", None).await
	}).await;
	ctx.fund_asp(&aspd, sat(1_000_000)).await;

	let mut admin = aspd.get_admin_client().await;
	let response = admin.wallet_status(rpc::Empty {}).await.unwrap().into_inner();
	assert_eq!(response.balance, 1_000_000);
}

#[tokio::test]
async fn bitcoind_cookie_connection() {
	let ctx = TestContext::new("aspd/bitcoind_cookie_connection").await;
	let aspd = ctx.new_aspd_with_funds("aspd", None, btc(0.01)).await;

	let mut admin = aspd.get_admin_client().await;
	let response = admin.wallet_status(rpc::Empty {}).await.unwrap().into_inner();
	assert_eq!(response.balance, 1_000_000);
}

#[tokio::test]
async fn round_started_log_can_be_captured() {
	let ctx = TestContext::new("aspd/capture_log").await;
	let aspd = ctx.new_aspd("aspd", None).await;

	let mut log_stream = aspd.subscribe_log::<aspd_log::RoundStarted>().await;
	while let Some(l) = log_stream.recv().await {
		info!("Captured log: Round started at {}", l.round_seq);
		break;
	}

	let l = aspd.wait_for_log::<aspd_log::RoundStarted>().await;
	info!("Captured log: Round started with round_num {}", l.round_seq);

	// make sure we only capture the log once.
	assert!(aspd.wait_for_log::<aspd_log::RoundStarted>().try_fast().await.is_err());
}

#[tokio::test]
async fn fund_asp() {
	let ctx = TestContext::new("aspd/fund_aspd").await;
	let aspd = ctx.new_aspd("aspd", None).await;
	let mut admin_client = aspd.get_admin_client().await;

	// Query the wallet balance of the asp
	let response = admin_client.wallet_status(rpc::Empty {}).await.expect("Get response").into_inner();
	assert_eq!(response.balance, 0);

	// Fund the aspd
	ctx.fund_asp(&aspd, btc(10)).await;
	ctx.bitcoind.generate(1).await;

	// Confirm that the balance is updated
	let response = admin_client.wallet_status(rpc::Empty {}).await.expect("Get response").into_inner();
	assert!(response.balance > 0);
}

#[tokio::test]
async fn restart_key_stability() {
	//! Test to ensure that the asp key stays stable accross loads
	//! but gives new on-chain addresses.

	let ctx = TestContext::new("aspd/restart_key_stability").await;
	let aspd = ctx.new_aspd("aspd", None).await;

	let asp_key1 = aspd.ark_info().await.asp_pubkey;
	let addr1 = {
		let mut admin_client = aspd.get_admin_client().await;
		let res = admin_client.wallet_status(rpc::Empty {}).await.unwrap().into_inner();
		res.address
	};

	// Fund the aspd's addr
	ctx.bitcoind.fund_addr(&addr1, btc(1)).await;
	ctx.bitcoind.generate(1).await;

	// Restart aspd.
	let _ = aspd.get_admin_client().await.stop(rpc::Empty {}).await;
	// bitcoind must be shut down gracefully otherwise it will not restart properly
	aspd.shutdown_bitcoind().await;
	aspd.stop().await.unwrap();

	let mut cfg = aspd.config().clone();
	cfg.bitcoind.url = String::new();
	let aspd = ctx.new_aspd_with_cfg("aspd", cfg).await;
	let asp_key2 = aspd.ark_info().await.asp_pubkey;
	let addr2 = {
		let mut admin_client = aspd.get_admin_client().await;
		let res = admin_client.wallet_status(rpc::Empty {}).await.expect("Get response").into_inner();
		res.address
	};

	assert_eq!(asp_key1, asp_key2);
	assert_ne!(addr1, addr2);
}

#[tokio::test]
async fn max_vtxo_amount() {
	let ctx = TestContext::new("aspd/max_vtxo_amount").await;
	let aspd = ctx.new_aspd_with_cfg("aspd", aspd::Config {
		max_vtxo_amount: Some(Amount::from_sat(500_000)),
		..ctx.aspd_default_cfg("aspd", None).await
	}).await;
	ctx.fund_asp(&aspd, Amount::from_int_btc(10)).await;
	let mut bark1 = ctx.new_bark_with_funds("bark1", &aspd, Amount::from_sat(1_500_000)).await;
	bark1.timeout = Duration::from_millis(2_500);

	// exceeds limit, should fail
	// TODO(stevenroose) once we have better error reporting, assert error content
	assert!(bark1.try_board(Amount::from_sat(600_000)).await.is_err());
	bark1.board(Amount::from_sat(500_000)).await;
	bark1.board(Amount::from_sat(500_000)).await;
	ctx.bitcoind.generate(BOARD_CONFIRMATIONS).await;

	// try send OOR exceeding limit
	let err = bark1.try_send_oor(*RANDOM_PK, Amount::from_sat(600_000)).await.unwrap_err();
	assert!(err.to_string().contains("output exceeds maximum vtxo amount of"));
	bark1.send_oor(*RANDOM_PK, Amount::from_sat(400_000)).await;

	// then try send in a round
	let err = bark1.try_refresh_all().await.unwrap_err();
	assert!(err.to_string().contains("output exceeds maximum vtxo amount of"));

	// but we can offboard the entire amount!
	let address = ctx.bitcoind.get_new_address();
	bark1.offboard_all(address.clone()).await;
	ctx.bitcoind.generate(1).await;
	let balance = ctx.bitcoind.get_received_by_address(&address);
	assert_eq!(balance, Amount::from_sat(598_117));
}

#[tokio::test]
async fn sweep_vtxos() {
	//! Testing aspd spending expired rounds.
	let ctx = TestContext::new("aspd/sweep_vtxos").await;

	// TODO: in this test, blocks are generated by aspd's bitcoin node.
	// Ideally they would be generated by ctx.bitcoind but it will
	// require some synchronization.

	let aspd = ctx.new_aspd_with_cfg("aspd", aspd::Config {
		round_interval: Duration::from_millis(500000000),
		vtxo_expiry_delta: 64,
		sweep_threshold: sat(100_000),
		..ctx.aspd_default_cfg("aspd", None).await
	}).await;
	ctx.fund_asp(&aspd, sat(1_000_000)).await;
	let bark = Arc::new(ctx.new_bark_with_funds("bark", &aspd, sat(500_000)).await);
	let mut admin = aspd.get_admin_client().await;

	// subscribe to a few log messages
	let mut log_not_sweeping = aspd.subscribe_log::<NotSweeping>().await;
	let mut log_sweeping = aspd.subscribe_log::<SweepBroadcast>().await;
	let mut log_board_done = aspd.subscribe_log::<BoardFullySwept>().await;
	let mut log_round_done = aspd.subscribe_log::<RoundFullySwept>().await;
	let mut log_sweeps = aspd.subscribe_log::<SweepingOutput>().await;

	// we board one vtxo and then a few blocks later another
	bark.board(sat(75_000)).await;
	ctx.bitcoind.generate(5).await;
	bark.board(sat(75_000)).await;
	ctx.bitcoind.generate(BOARD_CONFIRMATIONS).await;

	// before either expires not sweeping yet because nothing available
	aspd.wait_for_log::<TxIndexUpdateFinished>().await;
	admin.trigger_sweep(rpc::Empty{}).await.unwrap();
	assert_eq!(sat(0), log_not_sweeping.recv().fast().await.unwrap().available_surplus);

	// we can't make vtxos expire, so we have to refresh them
	ctx.bitcoind.generate(18).await;
	let b = bark.clone();
	tokio::spawn(async move {
		b.refresh_all().await;
	});
	aspd.trigger_round().await;
	let _ = aspd.wait_for_log::<RoundFinished>().try_wait(1000).await;
	ctx.bitcoind.generate(30).await;

	// now we expire the first one, still not sweeping because not enough surplus
	aspd.wait_for_log::<TxIndexUpdateFinished>().await;
	admin.trigger_sweep(rpc::Empty{}).await.unwrap();
	assert_eq!(sat(73790), log_not_sweeping.recv().wait(1500).await.unwrap().available_surplus);

	// now we expire the second, but the amount is not enough to sweep
	ctx.bitcoind.generate(5).await;
	aspd.wait_for_log::<TxIndexUpdateFinished>().wait(6000).await;
	admin.trigger_sweep(rpc::Empty{}).await.unwrap();
	assert_eq!(sat(147580), log_sweeping.recv().wait(1500).await.unwrap().surplus);
	let sweeps = log_sweeps.collect();
	assert_eq!(2, sweeps.len());
	assert_eq!(sweeps[0].sweep_type, "board");
	assert_eq!(sweeps[1].sweep_type, "board");

	// now we swept both board vtxos, let's sweep the round we created above
	ctx.bitcoind.generate(30).await;
	aspd.wait_for_log::<TxIndexUpdateFinished>().await;
	admin.trigger_sweep(rpc::Empty{}).await.unwrap();
	assert_eq!(sat(149980), log_sweeping.recv().wait(2500).await.unwrap().surplus);
	let sweeps = log_sweeps.collect();
	assert_eq!(1, sweeps.len());
	assert_eq!(sweeps[0].sweep_type, "vtxo");

	// then after a while, we should sweep the connectors,
	// but they don't make the surplus threshold, so we add another board
	bark.board(sat(101_000)).await;
	ctx.bitcoind.generate(70).await;
	aspd.wait_for_log::<TxIndexUpdateFinished>().await;
	admin.trigger_sweep(rpc::Empty{}).await.unwrap();
	assert_eq!(sat(100615), log_sweeping.recv().wait(1500).await.unwrap().surplus);
	let sweeps = log_sweeps.collect();
	assert_eq!(2, sweeps.len());
	assert_eq!(sweeps[0].sweep_type, "connector");
	assert_eq!(sweeps[1].sweep_type, "board");

	ctx.bitcoind.generate(65).await;
	aspd.wait_for_log::<TxIndexUpdateFinished>().await;
	let mut log_stats = aspd.subscribe_log::<SweeperStats>().await;
	admin.trigger_sweep(rpc::Empty{}).await.unwrap();

	// and eventually the round should be finished
	log_board_done.recv().wait(1000).await.unwrap();
	info!("board done signal received");
	log_round_done.recv().wait(1000).await.unwrap();
	info!("Round done signal received");
	let stats = log_stats.recv().fast().await.unwrap();
	assert_eq!(0, stats.nb_pending_utxos);
	assert_eq!(1241212, admin.wallet_status(rpc::Empty {}).await.unwrap().into_inner().balance);
}

#[tokio::test]
async fn restart_fresh_aspd() {
	let ctx = TestContext::new("aspd/restart_fresh_aspd").await;
	let mut aspd = ctx.new_aspd("aspd", None).await;
	aspd.stop().await.unwrap();
	aspd.start().await.unwrap();
}

#[tokio::test]
async fn restart_funded_aspd() {
	let ctx = TestContext::new("aspd/restart_funded_aspd").await;
	let mut aspd = ctx.new_aspd_with_funds("aspd", None, btc(10)).await;
	aspd.stop().await.unwrap();
	aspd.start().await.unwrap();
}

#[tokio::test]
async fn restart_aspd_with_payments() {
	let ctx = TestContext::new("aspd/restart_aspd_with_payments").await;
	let mut aspd = ctx.new_aspd_with_funds("aspd", None, btc(10)).await;
	let bark1 = ctx.new_bark("bark1", &aspd).await;
	let bark2 = ctx.new_bark("bark2", &aspd).await;
	ctx.fund_bark(&bark1, sat(1_000_000)).await;
	ctx.fund_bark(&bark2, sat(1_000_000)).await;

	bark2.board(sat(800_000)).await;
	bark1.board(sat(200_000)).await;
	ctx.bitcoind.generate(BOARD_CONFIRMATIONS).await;
	bark1.refresh_all().await;

	bark2.send_oor(&bark1.vtxo_pubkey().await, sat(330_000)).await;
	bark1.send_oor(&bark2.vtxo_pubkey().await, sat(350_000)).await;
	aspd.stop().await.unwrap();
	aspd.start().await.unwrap();
}

#[tokio::test]
async fn full_round() {
	let ctx = TestContext::new("aspd/full_round").await;
	let aspd = ctx.new_aspd_with_cfg("aspd", aspd::Config {
		round_interval: Duration::from_millis(2_000),
		round_submit_time: Duration::from_millis(10_000),
		round_sign_time: Duration::from_millis(10_000),
		nb_round_nonces: 2,
		..ctx.aspd_default_cfg("aspd", None).await
	}).await;
	ctx.fund_asp(&aspd, btc(10)).await;

	// based on nb_round_nonces
	const MAX_OUTPUTS: usize = 16;
	const NB_BARKS: usize = 5;
	const VTXOS_PER_BARK: usize = 4;
	assert!(NB_BARKS * VTXOS_PER_BARK > MAX_OUTPUTS);

	// Since we can have 16 inputs, we will create 5 barks with 4 vtxos each.

	let barks = join_all((1..=NB_BARKS).map(|i| {
		let name = format!("bark{}", i);
		ctx.new_bark_with_funds(name, &aspd, sat(40_000))
	})).await;
	ctx.bitcoind.generate(1).await;

	// have each board 4 times
	for _ in 0..VTXOS_PER_BARK {
		futures::future::join_all(barks.iter().map(|bark| async {
			bark.board(sat(1_000)).await;
		})).await;
		ctx.bitcoind.generate(BOARD_CONFIRMATIONS).await;
	}

	let (tx, mut rx) = mpsc::unbounded_channel();

	/// This proxy will keep track of how many times `submit payment` has been called.
	/// Once it reaches MAX_OUTPUTS, it asserts the next one fails.
	/// Once that happened succesfully, it fullfils the result channel.
	#[derive(Clone)]
	struct Proxy(aspd::ArkClient, Arc<Mutex<usize>>, Arc<mpsc::UnboundedSender<tonic::Status>>);
	#[tonic::async_trait]
	impl aspd::proxy::AspdRpcProxy for Proxy {
		fn upstream(&self) -> aspd::ArkClient { self.0.clone() }

		async fn submit_payment(&mut self, req: rpc::SubmitPaymentRequest) -> Result<rpc::Empty, tonic::Status> {
			let mut lock = self.1.lock().await;
			let res = self.upstream().submit_payment(req).await;
			// the last bark should fail being registered
			let ret = if *lock == NB_BARKS-1 {
				let err = res.expect_err("must error at max");
				trace!("proxy: NOK: {}", err);
				self.2.send(err.clone()).unwrap();
				Err(err)
			} else {
				trace!("proxy: OK (nb={})", *lock);
				res.map(|r| r.into_inner())
			};
			*lock += 1;
			ret
		}
	}

	let proxy = Proxy(aspd.get_public_client().await, Arc::new(Mutex::new(0)), Arc::new(tx));
	let proxy = aspd::proxy::AspdRpcProxyServer::start(proxy).await;
	futures::future::join_all(barks.iter().map(|bark| {
		bark.set_asp(&proxy.address)
	})).await;

	//TODO(stevenroose) need to find a way to ensure that all these happen in the same round
	tokio::spawn(async move {
		futures::future::join_all(barks.iter().map(|bark| async {
			// ignoring error as last one will fail
			let _ = bark.refresh_all().await;
		})).await;
	});

	// then we wait for the error to happen
	let err = rx.recv().wait(30_000).await.unwrap();
	assert!(err.to_string().contains("Message arrived late or round was full"));
}

#[tokio::test]
async fn double_spend_oor() {
	let ctx = TestContext::new("aspd/double_spend_oor").await;

	/// This proxy will always duplicate OOR requests and store the latest request in the mutex.
	#[derive(Clone)]
	struct Proxy(aspd::ArkClient, Arc<Mutex<Option<rpc::OorCosignRequest>>>);
	#[tonic::async_trait]
	impl aspd::proxy::AspdRpcProxy for Proxy {
		fn upstream(&self) -> aspd::ArkClient { self.0.clone() }

		async fn request_oor_cosign(&mut self, req: rpc::OorCosignRequest) -> Result<rpc::OorCosignResponse, tonic::Status> {
			let (mut c1, mut c2) = (self.0.clone(), self.0.clone());
			let (res1, res2) = tokio::join!(
				c1.request_oor_cosign(req.clone()),
				c2.request_oor_cosign(req.clone()),
			);
			self.1.lock().await.replace(req);
			match (res1, res2) {
				(Ok(_), Ok(_)) => panic!("one of them should fail"),
				(Err(_), Err(_)) => panic!("one of them should work"),
				(Ok(r), Err(e)) | (Err(e), Ok(r)) => {
					assert!(e.to_string().contains("attempted to sign OOR for vtxo already in flux"));
					Ok(r.into_inner())
				},
			}
		}
	}

	let aspd = ctx.new_aspd_with_funds("aspd", None, btc(10)).await;
	let last_req = Arc::new(Mutex::new(None));
	let proxy = Proxy(aspd.get_public_client().await, last_req.clone());
	let proxy = AspdRpcProxyServer::start(proxy).await;

	let bark = ctx.new_bark_with_funds("bark".to_string(), &proxy.address, sat(1_000_000)).await;
	bark.board(sat(800_000)).await;
	ctx.bitcoind.generate(BOARD_CONFIRMATIONS).await;

	bark.send_oor(&*RANDOM_PK, sat(100_000)).await;

	// then after it's done, fire the request again, which should fail.
	let req = last_req.lock().await.take().unwrap();
	let err = aspd.get_public_client().await.request_oor_cosign(req).await.unwrap_err();
	assert!(err.to_string().contains("attempted to sign OOR for already spent vtxo"));
}

#[tokio::test]
async fn double_spend_round() {
	let ctx = TestContext::new("aspd/double_spend_round").await;

	/// This proxy will duplicate all round payment submission requests.
	#[derive(Clone)]
	struct Proxy(aspd::ArkClient);
	#[tonic::async_trait]
	impl aspd::proxy::AspdRpcProxy for Proxy {
		fn upstream(&self) -> aspd::ArkClient { self.0.clone() }

		async fn submit_payment(&mut self, req: rpc::SubmitPaymentRequest) -> Result<rpc::Empty, tonic::Status> {
			let vtxoid = VtxoId::from_slice(&req.input_vtxos[0].vtxo_id).unwrap();

			let (mut c1, mut c2) = (self.0.clone(), self.0.clone());
			let res1 = c1.submit_payment(req.clone()).await;
			let res2 = c2.submit_payment(req).await;

			assert!(res1.is_ok());
			assert!(res2.unwrap_err().message().contains(&format!("vtxo {} already registered", vtxoid)));
			Ok(rpc::Empty{})
		}
	}

	let aspd = ctx.new_aspd_with_funds("aspd", None, btc(10)).await;
	let proxy = AspdRpcProxyServer::start(Proxy(aspd.get_public_client().await)).await;

	let bark = ctx.new_bark_with_funds("bark".to_string(), &proxy.address, sat(1_000_000)).await;
	bark.board(sat(800_000)).await;
	ctx.bitcoind.generate(BOARD_CONFIRMATIONS).await;

	let mut l = aspd.subscribe_log::<RoundUserVtxoAlreadyRegistered>().await;
	bark.refresh_all().await;
	l.recv().wait(2500).await;
}

#[tokio::test]
async fn test_participate_round_wrong_step() {
	let ctx = TestContext::new("aspd/test_participate_round_wrong_step").await;

	/// This proxy will send a `provide_vtxo_signatures` req instead of `submit_payment` one
	#[derive(Clone)]
	struct ProxyA(aspd::ArkClient);
	#[tonic::async_trait]
	impl aspd::proxy::AspdRpcProxy for ProxyA {
		fn upstream(&self) -> aspd::ArkClient { self.0.clone() }
		async fn submit_payment(&mut self, _req: rpc::SubmitPaymentRequest) -> Result<rpc::Empty, tonic::Status> {
			self.0.provide_vtxo_signatures(rpc::VtxoSignaturesRequest {
				pubkey: RANDOM_PK.serialize().to_vec(), signatures: vec![]
			}).await?;
			Ok(rpc::Empty{})
		}
	}

	let aspd = ctx.new_aspd_with_funds("aspd", None, Amount::from_int_btc(10)).await;

	let proxy = AspdRpcProxyServer::start(ProxyA(aspd.get_public_client().await)).await;
	let bark = ctx.new_bark_with_funds("bark".to_string(), &proxy.address, Amount::from_sat(1_000_000)).await;
	bark.board(Amount::from_sat(800_000)).await;
	ctx.bitcoind.generate(BOARD_CONFIRMATIONS).await;

	let res = bark.try_refresh_all().await;
	assert!(res.unwrap_err().to_string().contains("unexpected message. current step is payment registration"));

	/// This proxy will send a `provide_forfeit_signatures` req instead of `provide_vtxo_signatures` one
	#[derive(Clone)]
	struct ProxyB(aspd::ArkClient);
	#[tonic::async_trait]
	impl aspd::proxy::AspdRpcProxy for ProxyB {
		fn upstream(&self) -> aspd::ArkClient { self.0.clone() }
		async fn provide_vtxo_signatures(&mut self, _req: rpc::VtxoSignaturesRequest) -> Result<rpc::Empty, tonic::Status> {
			self.0.provide_forfeit_signatures(rpc::ForfeitSignaturesRequest { signatures: vec![] }).await?;
			Ok(rpc::Empty{})
		}
	}

	let proxy = AspdRpcProxyServer::start(ProxyB(aspd.get_public_client().await)).await;
	let bark2 = ctx.new_bark_with_funds("bark2".to_string(), &proxy.address, Amount::from_sat(1_000_000)).await;
	bark2.board(Amount::from_sat(800_000)).await;
	ctx.bitcoind.generate(BOARD_CONFIRMATIONS).await;

	let res = bark2.try_refresh_all().await;
	let error_string = res.unwrap_err().to_string();
	assert!(error_string.contains("current step is vtxo signatures submission"),
			"Error: {}",
			error_string
	);

	/// This proxy will send a `submit_payment` req instead of `provide_forfeit_signatures` one
	#[derive(Clone)]
	struct ProxyC(aspd::ArkClient);
	#[tonic::async_trait]
	impl aspd::proxy::AspdRpcProxy for ProxyC {
		fn upstream(&self) -> aspd::ArkClient { self.0.clone() }
		async fn provide_forfeit_signatures(&mut self, _req: rpc::ForfeitSignaturesRequest) -> Result<rpc::Empty, tonic::Status> {
			self.0.submit_payment(rpc::SubmitPaymentRequest {
				input_vtxos: vec![], vtxo_requests: vec![], offboard_requests: vec![]
			}).await?;
			Ok(rpc::Empty{})
		}
	}

	let proxy = AspdRpcProxyServer::start(ProxyC(aspd.get_public_client().await)).await;
	let bark3 = ctx.new_bark_with_funds("bark3".to_string(), &proxy.address, Amount::from_sat(1_000_000)).await;
	bark3.board(Amount::from_sat(800_000)).await;
	ctx.bitcoind.generate(BOARD_CONFIRMATIONS).await;

	let res = bark3.try_refresh_all().await;
	let err_string = res.unwrap_err().to_string();
	assert!(err_string.contains("Message arrived late or round was full."),
			"Error: {}",
			err_string
	);
}

#[tokio::test]
async fn spend_unregistered_board() {
	let ctx = TestContext::new("aspd/spend_unregistered_board").await;

	#[derive(Clone)]
	struct Proxy(aspd::ArkClient);
	#[tonic::async_trait]
	impl aspd::proxy::AspdRpcProxy for Proxy {
		fn upstream(&self) -> aspd::ArkClient { self.0.clone() }

		async fn register_board_vtxo(&mut self, _req: rpc::BoardVtxoRequest) -> Result<rpc::Empty, tonic::Status> {
			// drop the request
			Ok(rpc::Empty{})
		}
	}

	let aspd = ctx.new_aspd_with_funds("aspd", None, btc(10)).await;
	let proxy = AspdRpcProxyServer::start(Proxy(aspd.get_public_client().await)).await;

	let bark = ctx.new_bark_with_funds("bark".to_string(), &proxy.address, sat(1_000_000)).await;
	bark.board(sat(800_000)).await;
	ctx.bitcoind.generate(BOARD_CONFIRMATIONS).await;

	let mut l = aspd.subscribe_log::<RoundUserVtxoUnknown>().await;
	tokio::spawn(async move {
		let _ = bark.refresh_all().await;
		// we don't care that that call fails
	});
	l.recv().wait(2500).await;
}

#[tokio::test]
async fn spend_unconfirmed_board_round() {
	let ctx = TestContext::new("aspd/spend_unconfirmed_board_round").await;

	let aspd = ctx.new_aspd_with_funds("aspd", None, btc(10)).await;

	let bark = ctx.new_bark_with_funds("bark".to_string(), &aspd, sat(1_000_000)).await;
	bark.board(sat(800_000)).await;

	let mut l = aspd.subscribe_log::<UnconfirmedBoardSpendAttempt>().await;
	tokio::spawn(async move {
		let _ = bark.refresh_all().await;
		// we don't care that that call fails
	});
	l.recv().wait(2500).await;
}

#[tokio::test]
async fn spend_unconfirmed_board_oor() {
	let ctx = TestContext::new("aspd/spend_unconfirmed_board_oor").await;

	let aspd = ctx.new_aspd_with_funds("aspd", None, btc(10)).await;

	let bark1 = ctx.new_bark_with_funds("bark1".to_string(), &aspd, sat(1_000_000)).await;
	let bark2 = ctx.new_bark("bark2".to_string(), &aspd).await;
	bark1.board(sat(800_000)).await;

	let mut l = aspd.subscribe_log::<UnconfirmedBoardSpendAttempt>().await;
	tokio::spawn(async move {
		let _ = bark1.send_oor(bark2.vtxo_pubkey().await, sat(400_000)).await;
		// we don't care that that call fails
	});
	l.recv().wait(2500).await;
}

async fn reject_revocation_on_successful_ln_payment() {
	let ctx = TestContext::new("aspd/reject_revocation_on_successful_ln_payment").await;

	#[derive(Clone)]
	struct Proxy(aspd::ArkClient);
	#[tonic::async_trait]
	impl aspd::proxy::AspdRpcProxy for Proxy {
		fn upstream(&self) -> aspd::ArkClient { self.0.clone() }

		async fn finish_bolt11_payment(&mut self, req: rpc::SignedBolt11PaymentDetails) -> Result<Box<
			dyn Stream<Item = Result<rpc::Bolt11PaymentUpdate, tonic::Status>> + Unpin + Send + 'static
		>, tonic::Status> {
			// Wait until payment is successful then we drop update so client asks for revocation
			let mut stream = self.upstream().finish_bolt11_payment(req).await?.into_inner();
			while let Some(msg) = stream.next().await {
				if msg.unwrap().payment_preimage().len() > 0 {
					break;
				}
			}
			Ok(Box::new(futures::stream::empty()))
		}
	}

	// Start a three lightning nodes
	// And connect them in a line.
	trace!("Start lightningd-1, lightningd-2, ...");
	let lightningd_1 = ctx.new_lightningd("lightningd-1").await;
	let lightningd_2 = ctx.new_lightningd("lightningd-2").await;

	trace!("Funding all lightning-nodes");
	ctx.fund_lightning(&lightningd_1, btc(10)).await;
	ctx.bitcoind.generate(6).await;
	lightningd_1.wait_for_block_sync().await;

	trace!("Creating channel between lightning nodes");
	lightningd_1.connect(&lightningd_2).await;
	lightningd_1.fund_channel(&lightningd_2, btc(8)).await;

	// TODO: find a way how to remove this sleep
	// maybe: let ctx.bitcoind wait for channel funding transaction
	// without the sleep we get infinite 'Waiting for gossip...'
	tokio::time::sleep(std::time::Duration::from_millis(8_000)).await;
	ctx.bitcoind.generate(6).await;

	lightningd_1.wait_for_gossip(1).await;

	// Start an aspd and link it to our cln installation
	let aspd_1 = ctx.new_aspd("aspd-1", Some(&lightningd_1)).await;

	// Start a bark and create a VTXO
	let onchain_amount = btc(7);
	let board_amount = btc(5);

	let proxy = AspdRpcProxyServer::start(Proxy(aspd_1.get_public_client().await)).await;
	let bark_1 = ctx.new_bark_with_funds("bark-1", &proxy.address, onchain_amount).await;

	bark_1.board(board_amount).await;
	ctx.bitcoind.generate(6).await;

	// Create a payable invoice
	let invoice_amount = btc(2);
	let invoice = lightningd_2.invoice(Some(invoice_amount), "test_payment", "A test payment").await;

	assert_eq!(bark_1.offchain_balance().await, board_amount);
	let res = bark_1.try_send_bolt11(invoice, None).await;
	assert!(res.unwrap_err().to_string().contains("This lightning payment has completed. preimage: "));
}

#[tokio::test]
async fn bad_round_input() {
	let ctx = TestContext::new("aspd/bad_round_input").await;
	let aspd = ctx.new_aspd_with_cfg("aspd", aspd::Config {
		round_interval: Duration::from_secs(10000000),
		round_submit_time: Duration::from_secs(30),
		..ctx.aspd_default_cfg("aspd", None).await
	}).await;
	let bark = ctx.new_bark_with_funds("bark", &aspd, btc(1)).await;
	bark.board(btc(0.5)).await;
	let [vtxo] = bark.vtxos().await.try_into().unwrap();

	let ark_info = aspd.ark_info().await;
	let mut rpc = aspd.get_public_client().await;
	let mut stream = rpc.subscribe_rounds(rpc::Empty {}).await.unwrap().into_inner();
	aspd.get_admin_client().await.trigger_round(rpc::Empty {}).await.unwrap();
	let challenge = loop {
		match stream.next().await.unwrap().unwrap() {
			rpc::RoundEvent { event: Some(event) } => match event {
				rpc::round_event::Event::Attempt(a) => {
					break VtxoOwnershipChallenge::new(a.vtxo_ownership_challenge.try_into().unwrap());
				},
				_ => {},
			},
			_ => panic!("unexpected msg"),
		}
	};

	// build some legit params
	let key = Keypair::new(&SECP, &mut bitcoin::secp256k1::rand::thread_rng());
	let key2 = Keypair::new(&SECP, &mut bitcoin::secp256k1::rand::thread_rng());
	let input = rpc::InputVtxo {
		vtxo_id: vtxo.id.to_bytes().to_vec(),
		ownership_proof: challenge.sign_with(vtxo.id, key).serialize().to_vec(),
	};
	let vtxo_req = rpc::VtxoRequest {
		amount: 1000,
		vtxo_public_key: key.public_key().serialize().to_vec(),
		cosign_pubkey: key2.public_key().serialize().to_vec(),
		public_nonces: iter::repeat({
			let (_sec, pb) = musig::nonce_pair(&key);
			pb.serialize().to_vec()
		}).take(ark_info.nb_round_nonces as usize).collect(),
	};
	let offb_req = rpc::OffboardRequest {
		amount: 1000,
		offboard_spk: ScriptBuf::new_p2wpkh(&WPubkeyHash::from_byte_array(rand::random())).to_bytes(),
	};

	// let's fire some bad attempts

	info!("no inputs");
	let err = rpc.submit_payment(rpc::SubmitPaymentRequest {
		input_vtxos: vec![],
		vtxo_requests: vec![vtxo_req.clone()],
		offboard_requests: vec![],
	}).fast().await.unwrap_err();
	assert_eq!(err.code(), tonic::Code::InvalidArgument, "[{}]: {}", err.code(), err.message());
	let err = rpc.submit_payment(rpc::SubmitPaymentRequest {
		input_vtxos: vec![],
		vtxo_requests: vec![],
		offboard_requests: vec![offb_req.clone()],
	}).fast().await.unwrap_err();
	assert_eq!(err.code(), tonic::Code::InvalidArgument, "[{}]: {}", err.code(), err.message());

	info!("no outputs");
	let err = rpc.submit_payment(rpc::SubmitPaymentRequest {
		input_vtxos: vec![input.clone()],
		vtxo_requests: vec![],
		offboard_requests: vec![],
	}).fast().await.unwrap_err();
	assert_eq!(err.code(), tonic::Code::InvalidArgument, "[{}]: {}", err.code(), err.message());

	info!("unknown input");
	let fake_vtxo = VtxoId::from_slice(&rand::random::<[u8; 36]>()[..]).unwrap();
	let fake_input = rpc::InputVtxo {
		vtxo_id: fake_vtxo.to_bytes().to_vec(),
		ownership_proof: challenge.sign_with(fake_vtxo, key).serialize().to_vec(),
	};
	let err = rpc.submit_payment(rpc::SubmitPaymentRequest {
		input_vtxos: vec![fake_input],
		vtxo_requests: vec![vtxo_req.clone()],
		offboard_requests: vec![],
	}).fast().await.unwrap_err();
	assert_eq!(err.code(), tonic::Code::NotFound, "[{}]: {}", err.code(), err.message());
	assert_eq!(err.metadata().get("identifiers").unwrap().to_str().unwrap(), fake_vtxo.to_string());

	info!("non-standard script");
	let err = rpc.submit_payment(rpc::SubmitPaymentRequest {
		input_vtxos: vec![input.clone()],
		vtxo_requests: vec![],
		offboard_requests: vec![rpc::OffboardRequest {
			amount: 1000,
			offboard_spk: vec![0x00],
		}],
	}).fast().await.unwrap_err();
	assert_eq!(err.code(), tonic::Code::InvalidArgument, "[{}]: {}", err.code(), err.message());
	assert!(err.message().contains("non-standard"), "{}", err.message());

	info!("op_return too large");
	let err = rpc.submit_payment(rpc::SubmitPaymentRequest {
		input_vtxos: vec![input.clone()],
		vtxo_requests: vec![],
		offboard_requests: vec![rpc::OffboardRequest {
			amount: 1000,
			offboard_spk: ScriptBuf::new_op_return(<&PushBytes>::try_from(&[1u8; 84][..]).unwrap()).to_bytes(),
		}],
	}).fast().await.unwrap_err();
	assert_eq!(err.code(), tonic::Code::InvalidArgument, "[{}]: {}", err.code(), err.message());
	assert!(err.message().contains("OP_RETURN"), "{}", err.message());
}

#[tokio::test]
async fn register_onboard_is_idempotent() {
	let ctx = TestContext::new("aspd/register_onboard_is_idempotent").await;
	let aspd = ctx.new_aspd("aspd", None).await;
	let bark_wallet = ctx.new_bark("bark", &aspd).await;

	ctx.fund_bark(&bark_wallet, bitcoin::Amount::from_sat(50_000)).await;
	let onboard = bark_wallet.board_all().await;

	let bark_client = bark_wallet.client().await;
	let vtxo = bark_client.get_vtxo_by_id(onboard.vtxos[0].id).unwrap();
	let funding_tx = bark_client.onchain.get_tx(onboard.funding_txid).unwrap();


	// We will now call the register_onboard a few times
	let mut rpc = aspd.get_public_client().await;
	let request = rpc::BoardVtxoRequest {
		board_vtxo: vtxo.encode(),
		board_tx: bitcoin::consensus::encode::serialize(&funding_tx),
	};

	for _ in 0..5 {
		rpc.register_board_vtxo(request.clone()).await.unwrap();
	}
}
