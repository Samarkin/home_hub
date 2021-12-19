#[macro_use] extern crate rocket;

use std::error::Error;
use std::net::SocketAddr;

use env_logger::Env;
use structopt::StructOpt;

mod server;
mod device_data_provider;

#[derive(StructOpt)]
#[structopt(
name = "home_hub",
about = "Home hub HTTP server",
version = env!("VERGEN_SEMVER"),
)]
struct Opt {
    #[structopt(short, long, help = "Socket address to listen on", default_value="0.0.0.0:8080")]
    address: SocketAddr,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let opt = Opt::from_args();
    server::serve(opt.address).await?;
    Ok(())
}