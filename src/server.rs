mod interceptors;
mod services;

use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::service::interceptor;
use tonic::transport::Server;

use interceptors::logger::RequestLogger;
use services::live_share::live_share_server::LiveShareServer;
use services::live_share::LiveShareImpl;

pub async fn serve(listener: TcpListener) -> Result<(), tonic::transport::Error> {
    let live_share = LiveShareImpl::default();

    Server::builder()
        .layer(interceptor(RequestLogger {}))
        .add_service(LiveShareServer::new(live_share))
        .serve_with_incoming(TcpListenerStream::new(listener))
        .await
}
