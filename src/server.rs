use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;

use rocket::{Config, State};
use rocket::http::Status;
use rocket::serde::{json::Json, Serialize};

use crate::device_data_provider::DeviceDataProvider;
use crate::entertainment_controller::{EntertainmentController, TvStatus};

#[derive(Serialize)]
struct ClimateDeviceData {
    friendly_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature_in_c: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    humidity: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    battery: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_update_timestamp: Option<u64>,
}

#[derive(Serialize)]
struct StatusData {
    climate: HashMap<String, ClimateDeviceData>,
    tv: TvStatus,
}

#[derive(Serialize)]
struct EmptyResponse {
}

#[get("/status")]
async fn get_status(
    device_data_provider: &State<DeviceDataProvider>,
    entertainment_controller: &State<EntertainmentController>
) -> Result<Json<StatusData>, Status> {
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
                last_update_timestamp: data.last_update_timestamp,
            }))
        .collect();
    let tv = entertainment_controller.get_tv_status().await.map_err(|err| {
        error!("Error when calling entertainment monitor: {}", err);
        Status::ServiceUnavailable
    })?;
    Ok(Json(StatusData { climate, tv }))
}

#[post("/gaming-mode")]
async fn start_gaming_mode(
    entertainment_controller: &State<EntertainmentController>
) -> Result<Json<EmptyResponse> ,Status> {
    entertainment_controller.turn_on_pc().await.map_err(|err| {
        error!("Error when calling entertainment monitor: {}", err);
        Status::ServiceUnavailable
    })?;
    entertainment_controller.turn_on_tv("pc").await.map_err(|err| {
        error!("Error when calling entertainment monitor: {}", err);
        Status::ServiceUnavailable
    })?;
    Ok(Json(EmptyResponse{}))
}

pub async fn serve(address: SocketAddr) -> Result<(), Box<dyn Error>> {
    let config = Config {
        address: address.ip(),
        port: address.port(),
        ..Config::default()
    };
    rocket::custom(config)
        .manage(DeviceDataProvider::new(crate::GOVEE_COLLECTOR_ADDRESS).await)
        .manage(EntertainmentController::new(crate::ENTERTAINMENT_MONITOR_ADDRESS))
        .mount("/", routes![get_status, start_gaming_mode])
        .launch()
        .await?;
    Ok(())
}