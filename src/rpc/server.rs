use super::live_share::live_share_server::LiveShareServer;
use super::live_share::LiveShareImpl;

use tonic::transport::server::Router;
use tonic::transport::Server;

pub fn build() -> Router {
    let live_share = LiveShareImpl::default();

    Server::builder().add_service(LiveShareServer::new(live_share))
}
