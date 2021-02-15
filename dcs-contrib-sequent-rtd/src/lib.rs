#![cfg(target_os = "linux")]

use anyhow::Result;
use async_trait::async_trait;
use dcs::sensors::PullSensor;
use embedded_hal::blocking::i2c::WriteRead;
use linux_embedded_hal::I2cdev;

#[derive(Debug)]
pub struct RTD {
    i2c: Box<dyn WriteRead>,
    write_buf: i32,
}

impl RTD {
    pub fn new(channel: i32) -> Result<Self> {
        Ok(RTD {
            i2c: Box::new(I2cdev::new("/dev/i2c-1")?),
            write_buf: [0xff & ((channel - 1) * 4)],
        })
    }
}

#[async_trait]
impl PullSensor<f32> for RTD {
    async fn read(&mut self) -> Result<f32> {
        let mut read_buf = [0u8; 4];
        let _ = self.i2c.write_read(0x40, self.write_buf, &mut read_buf)?;
        let value = f32::from_le_bytes(read_buf);

        Ok(value)
    }
}
