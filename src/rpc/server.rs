use std::net::SocketAddr;

use jsonrpsee::server::middleware::rpc::RpcServiceBuilder;
use jsonrpsee::server::{Server, ServerHandle};

use super::logger;
use super::rpc_impl::RpcServerImpl;
use super::traits::RpcServer;

pub async fn start(port: u32) -> anyhow::Result<(SocketAddr, ServerHandle)> {
    let server_url = format!("{}:{}", "0.0.0.0", port);
    let rpc_middleware = RpcServiceBuilder::new().layer_fn(logger::Logger);
    let server = Server::builder()
        .set_rpc_middleware(rpc_middleware)
        .build(server_url)
        .await?;
    let addr = server.local_addr()?;
    let handle = server.start(RpcServerImpl.into_rpc());

    Ok((addr, handle))
}
