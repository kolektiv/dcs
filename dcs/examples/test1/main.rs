use anyhow::Result;
use async_trait::async_trait;
use dcs::sensors::{
    self,
    PullSensor,
    Sensor,
};
use std::time::Duration;

#[derive(Debug)]
pub struct TestSensor;

#[async_trait]
impl PullSensor<i32> for TestSensor {
    async fn read(&mut self) -> Result<i32> {
        todo!()
    }
}

fn _test() -> () {
    let rtd_1 = sensors::from_pull_sensor(TestSensor, Duration::from_secs(1));
    let rtd_1_channels = rtd_1.get_channels();
    let (_rtd_1_future, _rtd_1_data, _rtd_1_kill) = rtd_1_channels.decompose();
}

fn main() {
    println!("Hello, world!");
}
