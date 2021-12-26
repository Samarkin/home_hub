use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;

use rocket::{Config, State};
use rocket::http::Status;
use rocket::serde::{json::Json, Serialize};

use crate::device_data_provider::DeviceDataProvider;

#[derive(Serialize)]
struct ClimateDeviceData {
    friendly_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature_in_c: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    humidity: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    battery: Option<f32>,
}

#[derive(Serialize)]
struct StatusData {
    climate: HashMap<String, ClimateDeviceData>,
}

#[get("/status")]
async fn get_status(device_data_provider: &State<DeviceDataProvider>) -> Result<Json<StatusData>, Status> {
    let response = device_data_provider.get_device_data().await.map_err(|err| {
        error!("Error when calling Govee collector ({}): {}", err.code(), err.message());
        Status::ServiceUnavailable
    })?;
    let climate = response.devices.into_iter()
        .map(|data| (
            data.unique_id,
            ClimateDeviceData {
                friendly_name: data.friendly_name,
                temperature_in_c: data.temperature_in_c,
                humidity: data.humidity,
                battery: data.battery,
            }))
        .collect();
    Ok(Json(StatusData { climate }))
}

pub async fn serve(address: SocketAddr) -> Result<(), Box<dyn Error>> {
    let config = Config {
        address: address.ip(),
        port: address.port(),
        ..Config::default()
    };
    rocket::custom(config)
        .manage(DeviceDataProvider::new().await)
        .mount("/", routes![get_status])
        .launch()
        .await?;
    Ok(())
}