use async_trait::async_trait;
use dcs::sensors::PullSensor;

pub struct HueLight;

#[async_trait]
impl PullSensor<i32> for HueLight {
    async fn read(&mut self) -> anyhow::Result<i32> {
        todo!()
    }
}