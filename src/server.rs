use std::error::Error;
use std::net::SocketAddr;

use rocket::{Config, State};
use rocket::http::Status;

use itertools::join;

use crate::device_data_provider::DeviceDataProvider;

#[get("/climate")]
async fn get_climate(device_data_provider: &State<DeviceDataProvider>) -> Result<String, Status> {
    let response = device_data_provider.get_device_data().await.map_err(|err| {
        error!("Error when calling Govee collector ({}): {}", err.code(), err.message());
        Status::ServiceUnavailable
    })?;
    let ss = response.devices.into_iter()
        .map(|data| format!("{}: {}ºC", data.friendly_name, data.temperature_in_c.unwrap_or_default()));
    let s = join(ss, "\n");
    Ok(s)
}

pub async fn serve(address: SocketAddr) -> Result<(), Box<dyn Error>> {
    let config = Config {
        address: address.ip(),
        port: address.port(),
        ..Config::default()
    };
    rocket::custom(config)
        .manage(DeviceDataProvider::new().await)
        .mount("/", routes![get_climate])
        .launch()
        .await?;
    Ok(())
}