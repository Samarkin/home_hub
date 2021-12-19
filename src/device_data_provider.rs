use tokio::sync::Mutex;

use govee_collector::GetDeviceDataRequest;
use govee_collector::device_data_provider_client::DeviceDataProviderClient;

pub use govee_collector::{DeviceData, GetDeviceDataResponse};

mod govee_collector {
    tonic::include_proto!("govee_collector"); // The string specified here must match the proto package name
}

type RawClient = DeviceDataProviderClient<tonic::transport::Channel>;

pub struct DeviceDataProvider {
    client: Mutex<Option<RawClient>>,
}

impl DeviceDataProvider {
    async fn try_connect() -> Option<RawClient> {
        // TODO: Implement service discovery
        let addr = "http://127.0.0.1:50051";
        info!("Connecting to {}...", addr);
        DeviceDataProviderClient::connect(addr).await.map_err(|err| {
            error!("Failed to connect to the device data provider at {}: {}", addr, err);
            err
        }).ok()
    }

    pub async fn new() -> DeviceDataProvider {
        let client = Mutex::new(DeviceDataProvider::try_connect().await);
        DeviceDataProvider{client}
    }

    pub async fn get_device_data(&self) -> Result<GetDeviceDataResponse, tonic::Status> {
        let mut client = self.client.lock().await;
        let request = GetDeviceDataRequest {
            unique_ids: vec![],
        };
        let raw_client = match &mut *client {
            Some(raw_client) => raw_client,
            None => match DeviceDataProvider::try_connect().await {
                Some(raw_client) => client.insert(raw_client),
                None => return Err(tonic::Status::unavailable("Failed to connect to the device data provider")),
            },
        };
        Ok(raw_client.get_device_data(request).await?.into_inner())
    }
}
