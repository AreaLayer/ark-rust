
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

use futures::FutureExt;
use tokio_stream::Stream;

use aspd_rpc as rpc;

/// Trait used to easily implement aspd proxy interfaces.
#[tonic::async_trait]
pub trait AspdRpcProxy: Send + Sync + Clone + 'static {
	fn upstream(&self) -> rpc::ArkServiceClient<tonic::transport::Channel>;

	async fn get_ark_info(&mut self, req: rpc::Empty) -> Result<rpc::ArkInfo, tonic::Status> {
		Ok(self.upstream().get_ark_info(req).await?.into_inner())
	}

	async fn get_fresh_rounds(&mut self, req: rpc::FreshRoundsRequest) -> Result<rpc::FreshRounds, tonic::Status> {
		Ok(self.upstream().get_fresh_rounds(req).await?.into_inner())
	}

	async fn get_round(&mut self, req: rpc::RoundId) -> Result<rpc::RoundInfo, tonic::Status> {
		Ok(self.upstream().get_round(req).await?.into_inner())
	}

	async fn request_onboard_cosign(&mut self, req: rpc::OnboardCosignRequest) -> Result<rpc::OnboardCosignResponse, tonic::Status> {
		Ok(self.upstream().request_onboard_cosign(req).await?.into_inner())
	}

	async fn register_onboard_vtxos(&mut self, req: rpc::OnboardVtxosRequest) -> Result<rpc::Empty, tonic::Status> {
		Ok(self.upstream().register_onboard_vtxos(req).await?.into_inner())
	}

	async fn request_oor_cosign(&mut self, req: rpc::OorCosignRequest) -> Result<rpc::OorCosignResponse, tonic::Status> {
		Ok(self.upstream().request_oor_cosign(req).await?.into_inner())
	}

	async fn post_oor_mailbox(&mut self, req: rpc::OorVtxo) -> Result<rpc::Empty, tonic::Status> {
		Ok(self.upstream().post_oor_mailbox(req).await?.into_inner())
	}

	async fn empty_oor_mailbox(&mut self, req: rpc::OorVtxosRequest) -> Result<rpc::OorVtxosResponse, tonic::Status> {
		Ok(self.upstream().empty_oor_mailbox(req).await?.into_inner())
	}

	async fn start_bolt11_payment(&mut self, req: rpc::Bolt11PaymentRequest) -> Result<rpc::Bolt11PaymentDetails, tonic::Status> {
		Ok(self.upstream().start_bolt11_payment(req).await?.into_inner())
	}

	async fn finish_bolt11_payment(&mut self, req: rpc::SignedBolt11PaymentDetails) -> Result<Box<
		dyn Stream<Item = Result<rpc::Bolt11PaymentUpdate, tonic::Status>> + Unpin + Send + 'static
	>, tonic::Status> {
		Ok(Box::new(self.upstream().finish_bolt11_payment(req).await?.into_inner()))
	}

	async fn subscribe_rounds(&mut self, req: rpc::Empty) -> Result<Box<
		dyn Stream<Item = Result<rpc::RoundEvent, tonic::Status>> + Unpin + Send + 'static
	>, tonic::Status> {
		Ok(Box::new(self.upstream().subscribe_rounds(req).await?.into_inner()))
	}

	async fn submit_payment(&mut self, req: rpc::SubmitPaymentRequest) -> Result<rpc::Empty, tonic::Status> {
		Ok(self.upstream().submit_payment(req).await?.into_inner())
	}

	async fn provide_vtxo_signatures(&mut self, req: rpc::VtxoSignaturesRequest) -> Result<rpc::Empty, tonic::Status> {
		Ok(self.upstream().provide_vtxo_signatures(req).await?.into_inner())
	}

	async fn provide_forfeit_signatures(&mut self, req: rpc::ForfeitSignaturesRequest) -> Result<rpc::Empty, tonic::Status> {
		Ok(self.upstream().provide_forfeit_signatures(req).await?.into_inner())
	}
}

pub struct AspdRpcProxyServer {
	pub client: rpc::ArkServiceClient<tonic::transport::Channel>,
	pub stop: tokio::sync::oneshot::Sender<()>,
	pub address: String,
}

impl AspdRpcProxyServer {
	/// Run an aspd proxy server.
	pub async fn start(proxy: impl AspdRpcProxy) -> AspdRpcProxyServer {
		loop {
			let (stop_tx, stop_rx) = tokio::sync::oneshot::channel();
			let stop_rx = FutureExt::map(stop_rx, |_| ());

			let port = rand::random::<u16>();
			let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
			let server = rpc::server::ArkServiceServer::new(AspdRpcProxyWrapper(proxy.clone()));
			tokio::spawn(async move {
				tonic::transport::Server::builder()
					.add_service(server)
					.serve_with_shutdown(addr, stop_rx)
					.await.expect("proxy server shut down with error");
			});

			// try to connect
			let addr = format!("http://{}", addr);
			let client = loop {
				tokio::time::sleep(Duration::from_millis(10)).await;
				if let Ok(c) = rpc::ArkServiceClient::connect(addr.clone()).await {
					break c;
				}
			};

			return AspdRpcProxyServer {
				client: client,
				stop: stop_tx,
				address: addr,
			};
		}
	}
}

/// A wrapper struct around a proxy implementation to run a tonic server.
struct AspdRpcProxyWrapper<T: AspdRpcProxy>(T);

#[tonic::async_trait]
impl<T: AspdRpcProxy> rpc::ark_service_server::ArkService for AspdRpcProxyWrapper<T> {
	async fn get_ark_info(
		&self, req: tonic::Request<rpc::Empty>,
	) -> Result<tonic::Response<rpc::ArkInfo>, tonic::Status> {
		Ok(tonic::Response::new(AspdRpcProxy::get_ark_info(&mut self.0.clone(), req.into_inner()).await?))
	}

	async fn get_fresh_rounds(
		&self, req: tonic::Request<rpc::FreshRoundsRequest>,
	) -> Result<tonic::Response<rpc::FreshRounds>, tonic::Status> {
		Ok(tonic::Response::new(AspdRpcProxy::get_fresh_rounds(&mut self.0.clone(), req.into_inner()).await?))
	}

	async fn get_round(
		&self, req: tonic::Request<rpc::RoundId>,
	) -> Result<tonic::Response<rpc::RoundInfo>, tonic::Status> {
		Ok(tonic::Response::new(AspdRpcProxy::get_round(&mut self.0.clone(), req.into_inner()).await?))
	}

	async fn request_onboard_cosign(
		&self, req: tonic::Request<rpc::OnboardCosignRequest>,
	) -> Result<tonic::Response<rpc::OnboardCosignResponse>, tonic::Status> {
		Ok(tonic::Response::new(AspdRpcProxy::request_onboard_cosign(&mut self.0.clone(), req.into_inner()).await?))
	}

	async fn register_onboard_vtxos(
		&self, req: tonic::Request<rpc::OnboardVtxosRequest>,
	) -> Result<tonic::Response<rpc::Empty>, tonic::Status> {
		Ok(tonic::Response::new(AspdRpcProxy::register_onboard_vtxos(&mut self.0.clone(), req.into_inner()).await?))
	}

	async fn request_oor_cosign(
		&self, req: tonic::Request<rpc::OorCosignRequest>,
	) -> Result<tonic::Response<rpc::OorCosignResponse>, tonic::Status> {
		Ok(tonic::Response::new(AspdRpcProxy::request_oor_cosign(&mut self.0.clone(), req.into_inner()).await?))
	}

	async fn post_oor_mailbox(
		&self, req: tonic::Request<rpc::OorVtxo>,
	) -> Result<tonic::Response<rpc::Empty>, tonic::Status> {
		Ok(tonic::Response::new(AspdRpcProxy::post_oor_mailbox(&mut self.0.clone(), req.into_inner()).await?))
	}

	async fn empty_oor_mailbox(
		&self, req: tonic::Request<rpc::OorVtxosRequest>,
	) -> Result<tonic::Response<rpc::OorVtxosResponse>, tonic::Status> {
		Ok(tonic::Response::new(AspdRpcProxy::empty_oor_mailbox(&mut self.0.clone(), req.into_inner()).await?))
	}

	async fn start_bolt11_payment(
		&self, req: tonic::Request<rpc::Bolt11PaymentRequest>,
	) -> Result<tonic::Response<rpc::Bolt11PaymentDetails>, tonic::Status> {
		Ok(tonic::Response::new(AspdRpcProxy::start_bolt11_payment(&mut self.0.clone(), req.into_inner()).await?))
	}

	type FinishBolt11PaymentStream = Box<
		dyn Stream<Item = Result<rpc::Bolt11PaymentUpdate, tonic::Status>> + Unpin + Send + 'static
	>;

	async fn finish_bolt11_payment(
		&self, req: tonic::Request<rpc::SignedBolt11PaymentDetails>,
	) -> Result<tonic::Response<Self::FinishBolt11PaymentStream>, tonic::Status> {
		Ok(tonic::Response::new(AspdRpcProxy::finish_bolt11_payment(&mut self.0.clone(), req.into_inner()).await?))
	}

	type SubscribeRoundsStream = Box<
		dyn Stream<Item = Result<rpc::RoundEvent, tonic::Status>> + Unpin + Send + 'static
	>;

	async fn subscribe_rounds(
		&self, req: tonic::Request<rpc::Empty>,
	) -> Result<tonic::Response<Self::SubscribeRoundsStream>, tonic::Status> {
		Ok(tonic::Response::new(AspdRpcProxy::subscribe_rounds(&mut self.0.clone(), req.into_inner()).await?))
	}

	async fn submit_payment(
		&self, req: tonic::Request<rpc::SubmitPaymentRequest>,
	) -> Result<tonic::Response<rpc::Empty>, tonic::Status> {
		Ok(tonic::Response::new(AspdRpcProxy::submit_payment(&mut self.0.clone(), req.into_inner()).await?))
	}

	async fn provide_vtxo_signatures(
		&self, req: tonic::Request<rpc::VtxoSignaturesRequest>,
	) -> Result<tonic::Response<rpc::Empty>, tonic::Status> {
		Ok(tonic::Response::new(AspdRpcProxy::provide_vtxo_signatures(&mut self.0.clone(), req.into_inner()).await?))
	}

	async fn provide_forfeit_signatures(
		&self, req: tonic::Request<rpc::ForfeitSignaturesRequest>,
	) -> Result<tonic::Response<rpc::Empty>, tonic::Status> {
		Ok(tonic::Response::new(AspdRpcProxy::provide_forfeit_signatures(&mut self.0.clone(), req.into_inner()).await?))
	}
}
