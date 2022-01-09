use std::error::Error;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct TvStatus {
    pub state: String,
    pub input: Option<String>,
}

pub struct EntertainmentController {
    address: &'static str,
}

impl EntertainmentController {
    pub fn new(address: &'static str) -> EntertainmentController {
        EntertainmentController{address}
    }

    pub async fn get_tv_status(&self) -> Result<TvStatus, Box<dyn Error>> {
        let resp = reqwest::get(format!("{}/tv", self.address)).await?.json().await?;
        Ok(resp)
    }

    pub async fn turn_on_tv(&self, input: &str) -> Result<(), Box<dyn Error>> {
        let _ = reqwest::Client::new()
            .post(format!("{}/tv/on?input={}", self.address, input))
            .send()
            .await?;
        Ok(())
    }

    pub async fn turn_on_pc(&self) -> Result<(), Box<dyn Error>> {
        let _ = reqwest::Client::new()
            .post(format!("{}/pc/on", self.address))
            .send()
            .await?;
        Ok(())
    }
}