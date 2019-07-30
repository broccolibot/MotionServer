// Arduino-based Intelligent Motor Controller protocol
use crate::{read_encoder_target_pair, AIMCMessage};
pub use i2cdev::linux::LinuxI2CError;
use i2cdev::{core::I2CDevice, linux::LinuxI2CDevice};
use std::path::Path;

pub struct AIMC {
    i2c_device: LinuxI2CDevice,
}

impl AIMC {
    /// Create a new AIMC from the specified I2C device file and address.
    pub fn new<P: AsRef<Path>>(i2c_device_file: P, address: u16) -> Result<Self, LinuxI2CError> {
        Ok(Self {
            i2c_device: LinuxI2CDevice::new(i2c_device_file, address)?,
        })
    }

    /// Write a message to the device
    pub fn write_message(&mut self, message: AIMCMessage) -> Result<(), LinuxI2CError> {
        self.i2c_device.write(&message.into_bytes())
    }

    /// Read the encoder
    pub fn read_encoder_and_target(&mut self) -> Result<(f32, f32, f32), LinuxI2CError> {
        let mut buffer = [0u8; 12];
        self.i2c_device.read(&mut buffer)?;
        Ok(read_encoder_target_pair(buffer))
    }
}
