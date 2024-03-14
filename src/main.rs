mod logger;
mod server;

use log::info;
use std::net::SocketAddr;

use clap::Parser;
use tokio::net::TcpListener;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Host to listen to
    #[arg(long, default_value = "0.0.0.0")]
    host: String,

    /// Port to listen to
    #[arg(short = 'p', long, default_value_t = 1337)]
    port: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger::init();

    let args = Args::parse();

    let addr = format!("{}:{}", args.host, args.port).parse::<SocketAddr>()?;
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on port {}", listener.local_addr()?.port());

    server::serve(listener).await?;
    Ok(())
}
