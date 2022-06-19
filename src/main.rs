#[macro_use] extern crate rocket;

use std::error::Error;
use std::net::SocketAddr;

use env_logger::Env;
use structopt::StructOpt;

mod server;
mod device_data_provider;
mod entertainment_controller;
mod system_controller;

// TODO: Read from config file?
pub const GOVEE_COLLECTOR_ADDRESS: &str = "http://127.0.0.1:50051";
pub const ENTERTAINMENT_MONITOR_ADDRESS: &str = "http://127.0.0.1:12345";

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
    env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();

    let opt = Opt::from_args();
    server::serve(opt.address).await?;
    Ok(())
}