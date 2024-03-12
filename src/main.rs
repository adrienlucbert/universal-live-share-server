use std::net::SocketAddr;

use clap::Parser;
use jsonrpsee::core::async_trait;
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::server::middleware::rpc::{RpcServiceBuilder, RpcServiceT};
use jsonrpsee::server::{Server, ServerHandle};
use jsonrpsee::tokio;
use jsonrpsee::types::ErrorObjectOwned;
use jsonrpsee::types::Request;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Port to listen to. 0 assigns an available port.
    #[arg(short = 'p', long, default_value_t = 0)]
    port: u32,
}

#[rpc(server)]
pub trait Rpc {
    /// Ping.
    #[method(name = "ping")]
    async fn ping(&self) -> Result<String, ErrorObjectOwned>;
}

pub struct RpcServerImpl;

#[async_trait]
impl RpcServer for RpcServerImpl {
    async fn ping(&self) -> Result<String, ErrorObjectOwned> {
        Ok("Pong".to_string())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let (server_addr, server_handle) = run_server(args.port).await?;
    let url = format!("ws://{}", server_addr);

    println!("{}", url);

    server_handle.stopped().await;
    Ok(())
}

#[derive(Clone)]
pub struct Logger<S>(S);

impl<'a, S> RpcServiceT<'a> for Logger<S>
where
    S: RpcServiceT<'a> + Send + Sync,
{
    type Future = S::Future;

    fn call(&self, req: Request<'a>) -> Self::Future {
        println!("{}", req.method);
        self.0.call(req)
    }
}

async fn run_server(port: u32) -> anyhow::Result<(SocketAddr, ServerHandle)> {
    let server_url = format!("{}:{}", "0.0.0.0", port);
    let rpc_middleware = RpcServiceBuilder::new().layer_fn(Logger);
    let server = Server::builder()
        .set_rpc_middleware(rpc_middleware)
        .build(server_url)
        .await?;
    let addr = server.local_addr()?;
    let handle = server.start(RpcServerImpl.into_rpc());

    Ok((addr, handle))
}
