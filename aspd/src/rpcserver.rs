
use std::str::FromStr;
use std::sync::Arc;

use ark::lightning::SignedBolt11Payment;
use bitcoin::{Amount, ScriptBuf, Txid};
use bitcoin::hashes::Hash;
use bitcoin::secp256k1::PublicKey;
use lightning_invoice::Bolt11Invoice;
use tokio_stream::{Stream, StreamExt};
use tokio_stream::wrappers::BroadcastStream;

use stream_until::{StreamUntilItem, StreamExt as StreamExtUntil};

use ark::{musig, OffboardRequest, VtxoRequest, Vtxo, VtxoId};

use crate::App;
use crate::rpc;
use crate::round::RoundInput;
use crate::lightning::pay_bolt11;

macro_rules! badarg {
	($($arg:tt)*) => {{
		tonic::Status::invalid_argument(format!($($arg)*))
	}};
}

macro_rules! internal {
	($($arg:tt)*) => {{
		tonic::Status::internal(format!($($arg)*))
	}};
}

macro_rules! not_found {
	($($arg:tt)*) => {{
		tonic::Status::not_found(format!($($arg)*))
	}};
}

/// Just a trait to easily convert some kind of errors to tonic things.
trait ToStatus<T> {
	fn to_status(self) -> Result<T, tonic::Status>;
}

impl<T> ToStatus<T> for anyhow::Result<T> {
	fn to_status(self) -> Result<T, tonic::Status> {
		self.map_err(|e| tonic::Status::internal(format!("internal error: {}", e)))
	}
}

#[tonic::async_trait]
impl rpc::ArkService for Arc<App> {
	async fn get_ark_info(
		&self,
		_req: tonic::Request<rpc::Empty>,
	) -> Result<tonic::Response<rpc::ArkInfo>, tonic::Status> {
		let ret = rpc::ArkInfo {
			network: self.config.network.to_string(),
			pubkey: self.asp_key.public_key().serialize().to_vec(),
			xonly_pubkey: self.asp_key.x_only_public_key().0.serialize().to_vec(),
			nb_round_nonces: self.config.nb_round_nonces as u32,
			vtxo_exit_delta: self.config.vtxo_exit_delta as u32,
			vtxo_expiry_delta: self.config.vtxo_expiry_delta as u32,
		};
		Ok(tonic::Response::new(ret))
	}

	async fn get_fresh_rounds(
		&self,
		req: tonic::Request<rpc::FreshRoundsRequest>,
	) -> Result<tonic::Response<rpc::FreshRounds>, tonic::Status> {
		let ids = self.db.get_fresh_round_ids(req.into_inner().start_height)
			.map_err(|e| internal!("db error: {}", e))?;
		Ok(tonic::Response::new(rpc::FreshRounds {
			txids: ids.into_iter().map(|t| t.to_byte_array().to_vec()).collect(),
		}))
	}

	async fn get_round(
		&self,
		req: tonic::Request<rpc::RoundId>,
	) -> Result<tonic::Response<rpc::RoundInfo>, tonic::Status> {
		let txid = Txid::from_slice(&req.into_inner().txid)
			.map_err(|e| badarg!("invalid txid: {}", e))?;
		let ret = self.db.get_round(txid)
			.map_err(|e| internal!("db error: {}", e))?
			.ok_or_else(|| not_found!("round with txid {} not found", txid))?;
		Ok(tonic::Response::new(rpc::RoundInfo {
			round_tx: bitcoin::consensus::serialize(&ret.tx),
			signed_vtxos: ret.signed_tree.encode(),
		}))
	}

	// onboard

	async fn request_onboard_cosign(
		&self,
		req: tonic::Request<rpc::OnboardCosignRequest>,
	) -> Result<tonic::Response<rpc::OnboardCosignResponse>, tonic::Status> {
		let req = req.into_inner();
		let user_part = ciborium::from_reader::<ark::onboard::UserPart, _>(&req.user_part[..])
			.map_err(|e| badarg!("invalid user part: {}", e))?;
		if user_part.spec.asp_pubkey != self.asp_key.public_key() {
			return Err(badarg!("ASP public key is incorrect!"));
		}

		if let Some(max) = self.config.max_onboard_value {
			if user_part.spec.amount > max {
				return Err(badarg!("onboard amount exceeds limit of {}", max));
			}
		}

		let asp_part = self.cosign_onboard(user_part);
		Ok(tonic::Response::new(rpc::OnboardCosignResponse {
			asp_part: {
				let mut buf = Vec::new();
				ciborium::into_writer(&asp_part, &mut buf).unwrap();
				buf
			},
		}))
	}

	// oor

	async fn request_oor_cosign(
		&self,
		req: tonic::Request<rpc::OorCosignRequest>,
	) -> Result<tonic::Response<rpc::OorCosignResponse>, tonic::Status> {
		let req = req.into_inner();
		let payment = ark::oor::OorPayment::decode(&req.payment)
			.map_err(|e| badarg!("invalid oor payment request: {}", e))?;
		let user_nonces = req.pub_nonces.into_iter().map(|b| {
			musig::MusigPubNonce::from_slice(&b)
				.map_err(|e| badarg!("invalid public nonce: {}", e))
		}).collect::<Result<Vec<_>, tonic::Status>>()?;

		if payment.inputs.len() != user_nonces.len() {
			return Err(badarg!("wrong number of user nonces"));
		}

		let (nonces, sigs) = self.cosign_oor(&payment, &user_nonces).to_status()?;
		Ok(tonic::Response::new(rpc::OorCosignResponse {
			pub_nonces: nonces.into_iter().map(|n| n.serialize().to_vec()).collect(),
			partial_sigs: sigs.into_iter().map(|s| s.serialize().to_vec()).collect(),
		}))
	}

	async fn post_oor_mailbox(
		&self,
		req: tonic::Request<rpc::OorVtxo>,
	) -> Result<tonic::Response<rpc::Empty>, tonic::Status> {
		let req = req.into_inner();
		let pubkey = PublicKey::from_slice(&req.pubkey)
			.map_err(|e| badarg!("invalid pubkey: {}", e))?;
		let vtxo = Vtxo::decode(&req.vtxo)
			.map_err(|e| badarg!("invalid vtxo: {}", e))?;
		self.db.store_oor(pubkey, vtxo).to_status()?;
		Ok(tonic::Response::new(rpc::Empty {}))
	}

	async fn empty_oor_mailbox(
		&self,
		req: tonic::Request<rpc::OorVtxosRequest>,
	) -> Result<tonic::Response<rpc::OorVtxosResponse>, tonic::Status> {
		let req = req.into_inner();
		let pubkey = PublicKey::from_slice(&req.pubkey)
			.map_err(|e| badarg!("invalid pubkey: {}", e))?;
		let vtxos = self.db.pull_oors(pubkey).to_status()?;
		Ok(tonic::Response::new(rpc::OorVtxosResponse {
			vtxos: vtxos.into_iter().map(|v| v.encode()).collect(),
		}))
	}

	// lightning

	async fn start_bolt11_payment(
		&self,
		req: tonic::Request<rpc::Bolt11PaymentRequest>,
	) -> Result<tonic::Response<rpc::Bolt11PaymentDetails>, tonic::Status> {
		let req = req.into_inner();
		let invoice = Bolt11Invoice::from_str(&req.invoice)
			.map_err(|e| badarg!("invalid invoice: {}", e))?;
		invoice.check_signature().map_err(|_| badarg!("invalid invoice signature"))?;

		let inv_amount = invoice.amount_milli_satoshis()
			.map(|v| Amount::from_sat(v.div_ceil(1000)));
		if let (Some(_), Some(inv)) = (req.amount_sats, inv_amount) {
			return Err(badarg!("Invoice has amount of {} encoded. Please omit amount field", inv));
		}
		let amount = req.amount_sats.map(|v| Amount::from_sat(v)).or(inv_amount)
			.ok_or(badarg!("amount field required for invoice without amount"))?;

		let input_vtxos = req.input_vtxos.into_iter().map(|v| Vtxo::decode(&v))
			.collect::<Result<Vec<_>, _>>()
			.map_err(|e| badarg!("invalid vtxo: {}", e))?;
		let user_pubkey = PublicKey::from_slice(&req.user_pubkey)
			.map_err(|e| badarg!("invalid user pubkey: {}", e))?;
		let user_nonces = req.user_nonces.into_iter().map(|b| {
			musig::MusigPubNonce::from_slice(&b)
				.map_err(|e| badarg!("invalid public nonce: {}", e))
		}).collect::<Result<Vec<_>, tonic::Status>>()?;

		let (details, asp_nonces, part_sigs) = self.start_bolt11(
			invoice, amount, input_vtxos, user_pubkey, &user_nonces,
		).map_err(|e| internal!("error making payment: {}", e))?;

		Ok(tonic::Response::new(rpc::Bolt11PaymentDetails {
			details: details.encode(),
			pub_nonces: asp_nonces.into_iter().map(|n| n.serialize().to_vec()).collect(),
			partial_sigs: part_sigs.into_iter().map(|s| s.serialize().to_vec()).collect(),
		}))
	}

	type FinishBolt11PaymentStream = Box<
		dyn Stream<Item = Result<rpc::Bolt11PaymentUpdate, tonic::Status>> + Unpin + Send + 'static
	>;

	async fn finish_bolt11_payment(
		&self,
		req: tonic::Request<rpc::SignedBolt11PaymentDetails>,
	) -> Result<tonic::Response<Self::FinishBolt11PaymentStream>, tonic::Status> {
		let req = req.into_inner();
		let signed = SignedBolt11Payment::decode(&req.signed_payment)
			.map_err(|e| badarg!("invalid payment encoding: {}", e))?;
		if let Err(e) = signed.validate_signatures(&crate::SECP) {
			return Err(badarg!("bad signatures on payment: {}", e));
		}

		let invoice = signed.payment.invoice.clone();

		// Connecting to the grpc-client
		let cln_config = self.config.cln_config.as_ref().ok_or(not_found!("This asp does not support lightning"))?;
		let cln_client = cln_config.grpc_client().await.map_err(|_| internal!("Failed to connect to lightning"))?;
		let sendpay_rx = self.sendpay_updates.as_ref().unwrap().sendpay_rx.resubscribe();

		// Spawn a task that performs the payment
		let rx2 = sendpay_rx.resubscribe();
		let pay_jh = tokio::task::spawn(pay_bolt11(cln_client, signed, rx2));

		let payment_hash = invoice.payment_hash().clone();
		let update_stream = self.get_payment_update_stream(payment_hash.clone());

		let result = update_stream.until(pay_jh).map(move |item| {
			let item = match item {
				StreamUntilItem::Stream(v) => v,
				StreamUntilItem::Future(payment) => {
					match payment {
						Ok(Ok(preimage)) => {
							rpc::Bolt11PaymentUpdate {
								progress_message: "Payment completed".to_string(),
								status: rpc::PaymentStatus::Complete.into(),
								payment_hash: payment_hash.as_byte_array().to_vec(),
								payment_preimage: Some(preimage)
							}
						},
						Ok(Err(err)) => {
							rpc::Bolt11PaymentUpdate {
								progress_message: format!("Payment failed: {}", err),
								status: rpc::PaymentStatus::Failed as i32,
								payment_hash: payment_hash.as_byte_array().to_vec(),
								payment_preimage: None
							}
						},
						Err(err) => {
							rpc::Bolt11PaymentUpdate {
								progress_message: format!("Error during payment. Payment state unknown {:?}", err),
								status: rpc::PaymentStatus::Failed as i32,
								payment_hash: payment_hash.as_byte_array().to_vec(),
								payment_preimage: None
							}
						}
					}
				}
			};
			Ok(item)
		});

		Ok(tonic::Response::new(Box::new(result)))
	}

	// round

	type SubscribeRoundsStream = Box<
		dyn Stream<Item = Result<rpc::RoundEvent, tonic::Status>> + Unpin + Send + 'static
	>;

	async fn subscribe_rounds(
		&self,
		_req: tonic::Request<rpc::Empty>,
	) -> Result<tonic::Response<Self::SubscribeRoundsStream>, tonic::Status> {
		let chan = self.try_rounds().to_status()?.round_event_tx.subscribe();
		let stream = BroadcastStream::new(chan);

		Ok(tonic::Response::new(Box::new(stream.map(|e| {
			let e = e.map_err(|e| internal!("broken stream: {}", e))?;
			Ok(e.into())
		}))))
	}

	async fn submit_payment(
		&self,
		req: tonic::Request<rpc::SubmitPaymentRequest>,
	) -> Result<tonic::Response<rpc::Empty>, tonic::Status> {
		let req = req.into_inner();

		let inputs =  req.input_vtxos.into_iter().map(|vtxo| {
			Ok(Vtxo::decode(&vtxo).map_err(|e| badarg!("invalid vtxo: {}", e))?)
		}).collect::<Result<_, tonic::Status>>()?;

		let mut vtxo_requests = Vec::with_capacity(req.vtxo_requests.len());
		let mut cosign_pub_nonces = Vec::with_capacity(req.vtxo_requests.len());
		for r in req.vtxo_requests {
			let amount = Amount::from_sat(r.amount);
			let pubkey= PublicKey::from_slice(&r.vtxo_public_key)
				.map_err(|e| badarg!("malformed pubkey: {}", e))?;
			let cosign_pk = PublicKey::from_slice(&r.cosign_pubkey)
				.map_err(|e| badarg!("malformed cosign pubkey: {}", e))?;
			vtxo_requests.push(VtxoRequest { amount, pubkey, cosign_pk });

			// Make sure users provided right number of nonces.
			if r.public_nonces.len() != self.config.nb_round_nonces {
				return Err(badarg!(
					"need exactly {} public nonces", self.config.nb_round_nonces,
				));
			}
			let public_nonces = r.public_nonces.into_iter()
				.take(self.config.nb_round_nonces)
				.map(|n| {
					musig::MusigPubNonce::from_slice(&n)
						.map_err(|e| badarg!("invalid public nonce: {}", e))
				}).collect::<Result<_, tonic::Status>>()?;
			cosign_pub_nonces.push(public_nonces);
		}

		let offboards = req.offboard_requests.into_iter().map(|r| {
			let amount = Amount::from_sat(r.amount);
			let script_pubkey = ScriptBuf::from_bytes(r.offboard_spk);
			let ret = OffboardRequest { script_pubkey, amount };
			ret.validate().map_err(|e| badarg!("invalid offboard request: {}", e))?;
			Ok(ret)
		}).collect::<Result<_, tonic::Status>>()?;

		let inp = RoundInput::RegisterPayment {
			inputs, vtxo_requests, cosign_pub_nonces, offboards,
		};
		self.try_rounds().to_status()?.round_input_tx.send(inp).expect("input channel closed");
		Ok(tonic::Response::new(rpc::Empty {}))
	}

	async fn provide_vtxo_signatures(
		&self,
		req: tonic::Request<rpc::VtxoSignaturesRequest>,
	) -> Result<tonic::Response<rpc::Empty>, tonic::Status> {
		let req = req.into_inner();

		let inp = RoundInput::VtxoSignatures {
			pubkey: PublicKey::from_slice(&req.pubkey)
				.map_err(|e| badarg!("invalid pubkey: {}", e))?,
			signatures: req.signatures.into_iter().map(|s| {
				musig::MusigPartialSignature::from_slice(&s)
					.map_err(|e| badarg!("invalid signature: {}", e))
			}).collect::<Result<_, tonic::Status>>()?,
		};
		self.try_rounds().to_status()?.round_input_tx.send(inp).expect("input channel closed");
		Ok(tonic::Response::new(rpc::Empty {}))
	}

	async fn provide_forfeit_signatures(
		&self,
		req: tonic::Request<rpc::ForfeitSignaturesRequest>,
	) -> Result<tonic::Response<rpc::Empty>, tonic::Status> {
		let inp = RoundInput::ForfeitSignatures {
			signatures: req.into_inner().signatures.into_iter().map(|ff| {
				let id = VtxoId::from_slice(&ff.input_vtxo_id)
					.map_err(|e| badarg!("invalid vtxo id: {}", e))?;
				let nonces = ff.pub_nonces.into_iter().map(|n| {
					musig::MusigPubNonce::from_slice(&n)
						.map_err(|e| badarg!("invalid forfeit nonce: {}", e))
				}).collect::<Result<_, tonic::Status>>()?;
				let signatures = ff.signatures.into_iter().map(|s| {
					musig::MusigPartialSignature::from_slice(&s)
						.map_err(|e| badarg!("invalid forfeit sig: {}", e))
				}).collect::<Result<_, tonic::Status>>()?;
				Ok((id, nonces, signatures))
			}).collect::<Result<_, tonic::Status>>()?
		};
		self.try_rounds().to_status()?.round_input_tx.send(inp).expect("input channel closed");
		Ok(tonic::Response::new(rpc::Empty {}))
	}
}

#[tonic::async_trait]
impl rpc::AdminService for Arc<App> {
	async fn wallet_status(
		&self,
		_req: tonic::Request<rpc::Empty>,
	) -> Result<tonic::Response<rpc::WalletStatusResponse>, tonic::Status> {
		Ok(tonic::Response::new(rpc::WalletStatusResponse {
			// NB the order matters here, we want to sync first
			balance: self.sync_onchain_wallet().await.to_status()?.to_sat(),
			address: self.new_onchain_address().await.to_status()?.to_string(),
		}))
	}

	async fn trigger_round(
		&self,
		_req: tonic::Request<rpc::Empty>,
	) -> Result<tonic::Response<rpc::Empty>, tonic::Status> {
		match self.try_rounds().to_status()?.round_trigger_tx.try_send(()) {
			Err(tokio::sync::mpsc::error::TrySendError::Closed(())) => {
				Err(internal!("round scheduler closed"))
			},
			_ => Ok(tonic::Response::new(rpc::Empty {})),
		}
	}

	async fn stop(
		&self,
		_req: tonic::Request<rpc::Empty>,
	) -> Result<tonic::Response<rpc::Empty>, tonic::Status> {
		info!("Shutting down because of RPC stop command...");
		//TODO(stevenroose) implement graceful shutdown
		std::process::exit(0);
	}
}

/// Run the public gRPC endpoint.
pub async fn run_public_rpc_server(app: Arc<App>) -> anyhow::Result<()> {
	let addr = app.config.public_rpc_address;
	info!("Starting public gRPC service on address {}", addr);
	let ark_server = rpc::ArkServiceServer::new(app.clone());
	tonic::transport::Server::builder()
		.add_service(ark_server)
		.serve(addr)
		.await?;
	info!("Started public gRPC service on address {}", addr);
	Ok(())
}

/// Run the public gRPC endpoint.
pub async fn run_admin_rpc_server(app: Arc<App>) -> anyhow::Result<()> {
	let addr = app.config.admin_rpc_address.expect("shouldn't call this method otherwise");
	info!("Starting admin gRPC service on address {}", addr);
	let admin_server = rpc::AdminServiceServer::new(app.clone());
	tonic::transport::Server::builder()
		.add_service(admin_server)
		.serve(addr)
		.await?;
	info!("Started admin gRPC service on address {}", addr);
	Ok(())
}
