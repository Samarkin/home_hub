use tokio::sync::Mutex;

use govee_collector::GetDeviceDataRequest;
use govee_collector::device_data_provider_client::DeviceDataProviderClient;

pub use govee_collector::{DeviceData, GetDeviceDataResponse};

mod govee_collector {
    tonic::include_proto!("govee_collector"); // The string specified here must match the proto package name
}

type RawClient = DeviceDataProviderClient<tonic::transport::Channel>;

pub struct DeviceDataProvider {
    address: &'static str,
    client: Mutex<Option<RawClient>>,
}

impl DeviceDataProvider {
    async fn try_connect(address: &'static str) -> Option<RawClient> {
        info!("Connecting to {}...", address);
        DeviceDataProviderClient::connect(address).await.map_err(|err| {
            error!("Failed to connect to the device data provider at {}: {}", address, err);
            err
        }).ok()
    }

    pub async fn new(address: &'static str) -> DeviceDataProvider {
        let client = Mutex::new(DeviceDataProvider::try_connect(address).await);
        DeviceDataProvider{address, client}
    }

    pub async fn get_device_data(&self) -> Result<GetDeviceDataResponse, tonic::Status> {
        let mut client = self.client.lock().await;
        let request = GetDeviceDataRequest {
            unique_ids: vec![],
        };
        let raw_client = match &mut *client {
            Some(raw_client) => raw_client,
            None => match DeviceDataProvider::try_connect(self.address).await {
                Some(raw_client) => client.insert(raw_client),
                None => return Err(tonic::Status::unavailable("Failed to connect to the device data provider")),
            },
        };
        Ok(raw_client.get_device_data(request).await?.into_inner())
    }
}
