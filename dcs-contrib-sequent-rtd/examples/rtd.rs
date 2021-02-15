use anyhow::Result;

#[cfg(target_os = "linux")]
#[tokio::main]
async fn main() -> Result<()> {
    use dcs::sensors::{
        self,
        Sensor,
    };
    use dcs_contrib_sequent_rtd::RTD;
    use std::time::Duration;

    let rtd_1 = sensors::from_pull_sensor(RTD, Duration::from_secs(1));
    let rtd_1_channels = rtd_1.get_channels();
    let (_rtd_1_future, _rtd_1_data, _rtd_1_kill) = rtd_1_channels.decompose();

    Ok(())
}

#[cfg(not(target_os = "linux"))]
#[tokio::main]
async fn main() -> Result<()> {
    println!("The RTD example runs only under Linux.");

    Ok(())
}