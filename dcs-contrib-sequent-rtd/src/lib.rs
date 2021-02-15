use anyhow::Result;
use async_trait::async_trait;
use dcs::sensors::PullSensor;

#[derive(Debug)]
pub struct RTD;

#[async_trait]
impl PullSensor<i32> for RTD {
    async fn read(&mut self) -> Result<i32> {
        todo!()
    }
}
