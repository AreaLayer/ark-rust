

use std::str::FromStr;

use bitcoin::Address;
use bitcoincore_rpc::bitcoin::amount::Amount;
use bitcoincore_rpc::RpcApi;

use ark::vtxo::exit_spk;
use bark_json::primitives::VtxoType;

use ark_testing::{TestContext, Bark, btc, sat};
use ark_testing::constants::ONBOARD_CONFIRMATIONS;

async fn progress_exit(bark: &Bark) {
	let mut flip = false;
	for _ in 0..20 {
		let res = bark.exit().await;
		if res.done {
			return;
		}
		if let Some(height) = res.height {
			let current = bark.bitcoind().sync_client().get_block_count().unwrap();
			bark.bitcoind().generate(height as u64 - current).await;
		} else {
			flip = if flip {
				bark.bitcoind().generate(1).await;
				false
			} else {
				true
			};
		}
	}
	panic!("failed to finish unilateral exit of bark {}", bark.name());
}

#[tokio::test]
async fn exit_round() {
	let random_addr = Address::from_str(
		"bcrt1phrqwzmu8yvudewqefjatk20lh23vqqqnn3l57l0u2m98kd3zd70sjn2kqx"
	).unwrap().assume_checked();

	// Initialize the test
	let ctx = TestContext::new("exit/exit_round").await;
	let aspd = ctx.new_aspd("aspd", None).await;

	// Fund the asp
	ctx.fund_asp(&aspd, btc(10)).await;

	// Create a few clients
	let bark1 = ctx.new_bark("bark1".to_string(), &aspd).await;
	let bark2 = ctx.new_bark("bark2".to_string(), &aspd).await;
	let bark3 = ctx.new_bark("bark3".to_string(), &aspd).await;
	let bark4 = ctx.new_bark("bark4".to_string(), &aspd).await;
	let bark5 = ctx.new_bark("bark5".to_string(), &aspd).await;
	let bark6 = ctx.new_bark("bark6".to_string(), &aspd).await;
	let bark7 = ctx.new_bark("bark7".to_string(), &aspd).await;
	let bark8 = ctx.new_bark("bark8".to_string(), &aspd).await;

	tokio::join!(
		ctx.fund_bark(&bark1, sat(1_000_000)),
		ctx.fund_bark(&bark2, sat(1_000_000)),
		ctx.fund_bark(&bark3, sat(1_000_000)),
		ctx.fund_bark(&bark4, sat(1_000_000)),
		ctx.fund_bark(&bark5, sat(1_000_000)),
		ctx.fund_bark(&bark6, sat(1_000_000)),
		ctx.fund_bark(&bark7, sat(1_000_000)),
		ctx.fund_bark(&bark8, sat(1_000_000)),
	);
	ctx.bitcoind.generate(1).await;

	tokio::join!(
		bark1.onboard(sat(500_000)),
		bark2.onboard(sat(500_000)),
		bark3.onboard(sat(500_000)),
		bark4.onboard(sat(500_000)),
		bark5.onboard(sat(500_000)),
		bark6.onboard(sat(500_000)),
		bark7.onboard(sat(500_000)),
		bark8.onboard(sat(500_000)),
	);
	ctx.bitcoind.generate(ONBOARD_CONFIRMATIONS).await;

	tokio::join!(
		bark1.refresh_all(),
		bark2.refresh_all(),
		bark3.refresh_all(),
		bark4.refresh_all(),
		bark5.refresh_all(),
		bark6.refresh_all(),
		bark7.refresh_all(),
		bark8.refresh_all(),
	);

	let bark1_round_vtxo = &bark1.vtxos().await[0];
	let bark2_round_vtxo = &bark2.vtxos().await[0];
	let bark3_round_vtxo = &bark3.vtxos().await[0];
	let bark4_round_vtxo = &bark4.vtxos().await[0];
	let bark5_round_vtxo = &bark5.vtxos().await[0];
	let bark6_round_vtxo = &bark6.vtxos().await[0];
	let bark7_round_vtxo = &bark7.vtxos().await[0];
	let bark8_round_vtxo = &bark8.vtxos().await[0];

	// We don't need ASP for exits.
	aspd.stop().await.unwrap();
	progress_exit(&bark1).await;
	progress_exit(&bark2).await;
	progress_exit(&bark3).await;
	progress_exit(&bark4).await;
	progress_exit(&bark5).await;
	progress_exit(&bark6).await;
	progress_exit(&bark7).await;
	progress_exit(&bark8).await;

	// All wallets have 1_000_000 sats of funds minus fees
	//
	// However, what fees are paid by which client is not fully predictable
	// This depends on the shape of the tree and the order of the exit
	//
	// We can't control the shape of the tree in the test.
	// The order of the exit is also somewhat random
	assert!(bark1.onchain_balance().await >= bark1_round_vtxo.amount + Amount::ONE_SAT);
	assert!(bark2.onchain_balance().await >= bark2_round_vtxo.amount + Amount::ONE_SAT);
	assert!(bark3.onchain_balance().await >= bark3_round_vtxo.amount + Amount::ONE_SAT);
	assert!(bark4.onchain_balance().await >= bark4_round_vtxo.amount + Amount::ONE_SAT);
	assert!(bark5.onchain_balance().await >= bark5_round_vtxo.amount + Amount::ONE_SAT);
	assert!(bark6.onchain_balance().await >= bark6_round_vtxo.amount + Amount::ONE_SAT);
	assert!(bark7.onchain_balance().await >= bark7_round_vtxo.amount + Amount::ONE_SAT);
	assert!(bark8.onchain_balance().await >= bark8_round_vtxo.amount + Amount::ONE_SAT);

	// Verify exit outputs are considered as part of the wallets
	assert!(bark1.utxos().await.iter().any(|u| u.outpoint == bark1_round_vtxo.utxo && u.amount == bark1_round_vtxo.amount));
	assert!(bark2.utxos().await.iter().any(|u| u.outpoint == bark2_round_vtxo.utxo && u.amount == bark2_round_vtxo.amount));
	assert!(bark3.utxos().await.iter().any(|u| u.outpoint == bark3_round_vtxo.utxo && u.amount == bark3_round_vtxo.amount));
	assert!(bark4.utxos().await.iter().any(|u| u.outpoint == bark4_round_vtxo.utxo && u.amount == bark4_round_vtxo.amount));
	assert!(bark5.utxos().await.iter().any(|u| u.outpoint == bark5_round_vtxo.utxo && u.amount == bark5_round_vtxo.amount));
	assert!(bark6.utxos().await.iter().any(|u| u.outpoint == bark6_round_vtxo.utxo && u.amount == bark6_round_vtxo.amount));
	assert!(bark7.utxos().await.iter().any(|u| u.outpoint == bark7_round_vtxo.utxo && u.amount == bark7_round_vtxo.amount));
	assert!(bark8.utxos().await.iter().any(|u| u.outpoint == bark8_round_vtxo.utxo && u.amount == bark8_round_vtxo.amount));

	// Verify we can send exited utxos
	// Sending sats more than vtxo amount ensure we spend all available utxos (vtxo output and cpfp change)
	bark1.onchain_send(&random_addr, bark1_round_vtxo.amount + Amount::ONE_SAT).await;
	bark2.onchain_send(&random_addr, bark2_round_vtxo.amount + Amount::ONE_SAT).await;
	bark3.onchain_send(&random_addr, bark3_round_vtxo.amount + Amount::ONE_SAT).await;
	bark4.onchain_send(&random_addr, bark4_round_vtxo.amount + Amount::ONE_SAT).await;
	bark5.onchain_send(&random_addr, bark5_round_vtxo.amount + Amount::ONE_SAT).await;
	bark6.onchain_send(&random_addr, bark6_round_vtxo.amount + Amount::ONE_SAT).await;
	bark7.onchain_send(&random_addr, bark7_round_vtxo.amount + Amount::ONE_SAT).await;
	bark8.onchain_send(&random_addr, bark8_round_vtxo.amount + Amount::ONE_SAT).await;
}

#[tokio::test]
async fn exit_after_onboard() {
	let random_addr = Address::from_str(
		"bcrt1phrqwzmu8yvudewqefjatk20lh23vqqqnn3l57l0u2m98kd3zd70sjn2kqx"
	).unwrap().assume_checked();

	let ctx = TestContext::new("exit/exit_after_onboard").await;
	let aspd = ctx.new_aspd("aspd", None).await;

	// Fund the bark instance
	let bark = ctx.new_bark_with_funds("bark", &aspd, sat(1_000_000)).await;

	// Onboard funds
	bark.onboard(sat(900_000)).await;

	let onboard_vtxo = &bark.vtxos().await[0];

	// Exit unilaterally
	aspd.stop().await.unwrap();
	progress_exit(&bark).await;

	let balance = bark.onchain_balance().await;
	assert!(balance > sat(900_000), "balance: {balance}");

	// Verify exit output is considered as part of the wallet
	let utxos = bark.utxos().await;
	assert_eq!(utxos.len(), 2, "We have cpfp change (spent in exit process) + exited utxo");
	assert!(utxos.iter().any(|u| u.outpoint == onboard_vtxo.utxo && u.amount == onboard_vtxo.amount));

	// Verify we can send both utxos
	bark.onchain_send(random_addr, onboard_vtxo.amount + Amount::ONE_SAT).await;
}

#[tokio::test]
async fn exit_oor() {
	let ctx = TestContext::new("exit/exit_oor").await;
	let aspd = ctx.new_aspd("aspd", None).await;

	// Bark1 will pay bark2 oor.
	// Bark2 will attempt an exit
	let bark1 = ctx.new_bark_with_funds("bark1", &aspd, sat(1_000_000)).await;
	let bark2 = ctx.new_bark_with_funds("bark2", &aspd, sat(1_000_000)).await;

	ctx.bitcoind.generate(1).await;

	// Bark1 onboard funds and sends some part to bark2
	bark1.onboard(sat(900_000)).await;
	let bark2_pubkey = bark2.vtxo_pubkey().await;
	bark1.send_oor(bark2_pubkey, sat(100_000)).await;

	// By calling bark2 vtxos we ensure the wallet is synced
	// This ensures bark2 knows the vtxo exists
	let vtxos = bark2.vtxos().await;
	assert_eq!(vtxos.len(), 1, "We have received one vtxo");
	let oor_vtxo = &vtxos[0];
	assert_eq!(oor_vtxo.vtxo_type, VtxoType::Oor);

	// We stop the asp
	aspd.stop().await.unwrap();

	// Make bark2 exit and check the balance
	// It should be FUND_AMOUNT + VTXO_AMOUNT - fees
	progress_exit(&bark2).await;
	assert_eq!(bark2.onchain_balance().await, sat(1_089_521));

	// Verify exit output is considered as part of the wallet
	let utxos = bark2.utxos().await;
	assert_eq!(utxos.len(), 2, "We have cpfp change (spent in exit process) + exited utxo");
	assert!(utxos.iter().any(|u| u.outpoint == oor_vtxo.utxo && u.amount == oor_vtxo.amount));

	// Verify we can send both utxos
	bark2.onchain_send(bark1.get_onchain_address().await, oor_vtxo.amount + Amount::ONE_SAT).await;
}

#[tokio::test]
async fn double_exit_call() {
	let ctx = TestContext::new("bark/double_exit_call").await;
	let aspd = ctx.new_aspd_with_funds("aspd", None, btc(10)).await;
	let bark1 = ctx.new_bark_with_funds("bark1", &aspd, sat(1_000_000)).await;
	let bark2 = ctx.new_bark_with_funds("bark2", &aspd, sat(1_000_000)).await;

	bark2.onboard(sat(800_000)).await;

	// refresh vtxo
	bark1.onboard(sat(200_000)).await;
	ctx.bitcoind.generate(12).await;
	bark1.refresh_all().await;

	// onboard vtxo
	bark1.onboard(sat(300_000)).await;
	ctx.bitcoind.generate(12).await;

	// oor vtxo
	bark2.send_oor(&bark1.vtxo_pubkey().await, sat(330_000)).await;



	let vtxos = bark1.vtxos().await;

	progress_exit(&bark1).await;
	assert_eq!(bark1.onchain_balance().await, sat(1_305_941));

	let movements = bark1.list_movements().await;
	assert_eq!(movements.len(), 7);

	let last_moves = &movements[0..=2];
	assert!(
		vtxos.iter().all(|v| last_moves.iter().any(|m|
			m.spends.first().unwrap().id == v.id &&
			m.destination.clone().unwrap() ==
				exit_spk(v.user_pubkey, v.asp_pubkey, v.exit_delta).to_string()
		)), "each exited vtxo should be linked to a movement with exit_spk as destination"
	);
	assert_eq!(bark1.vtxos().await.len(), 0, "all vtxos should be marked as spent");

	// create a new vtxo to exit
	bark2.send_oor(bark1.vtxo_pubkey().await, sat(145_000)).await;
	let vtxos = bark1.vtxos().await;
	assert_eq!(vtxos.len(), 1);
	let vtxo = vtxos.first().unwrap();

	progress_exit(&bark1).await;

	let movements = bark1.list_movements().await;
	assert_eq!(movements.len(), 9);

	// check we only exited last vtxo
	let last_move = movements.first().unwrap();
	assert_eq!(last_move.spends.len(), 1, "we should only exit last spendable vtxo");
	assert_eq!(last_move.spends.first().unwrap().id, vtxo.id);
	let exit_spk = exit_spk(vtxo.user_pubkey, vtxo.asp_pubkey, vtxo.exit_delta).to_string();
	assert_eq!(last_move.destination.clone().unwrap(), exit_spk, "movement destination should be exit_spk");
	assert_eq!(bark1.vtxos().await.len(), 0, "vtxo should be marked as spent");

	progress_exit(&bark1).await;
	assert_eq!(bark1.list_movements().await.len(), 9, "should not create new movement when no new vtxo to exit");
}
