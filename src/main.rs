mod rpc;

use rpc::server;

use clap::Parser;
use jsonrpsee::tokio;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Port to listen to. 0 assigns an available port.
    #[arg(short = 'p', long, default_value_t = 0)]
    port: u32,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let (server_addr, server_handle) = server::start(args.port).await?;
    let url = format!("ws://{}", server_addr);

    println!("{}", url);

    server_handle.stopped().await;
    Ok(())
}
