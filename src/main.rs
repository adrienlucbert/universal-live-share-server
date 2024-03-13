use std::net::SocketAddr;

use clap::Parser;

mod rpc;

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
    let args = Args::parse();

    let addr = format!("{}:{}", args.host, args.port).parse::<SocketAddr>()?;
    println!("Listening on port {}", addr.port());

    rpc::server::build().serve(addr).await?;
    Ok(())
}
