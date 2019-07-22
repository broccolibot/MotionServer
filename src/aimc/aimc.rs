use super::*;
use crate::generic_message::*;
use i2cdev::{
    core::I2CDevice,
    linux::{LinuxI2CDevice, LinuxI2CError},
};
use std::error::Error;
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

    /// Construct a new AIMC object from the specified I2C device file, and attempt to set the
    /// parameters specified in the config struct.
    pub fn from_config<P: AsRef<Path>>(
        i2c_device_file: P,
        config: AIMCConfig,
    ) -> Result<Self, LinuxI2CError> {
        let mut instance = Self::new(i2c_device_file, config.address)?;
        for message in config.mode.into_messages() {
            instance.write_message(message)?;
        }
        Ok(instance)
    }

    /// Write a message to the device
    pub fn write_message(&mut self, message: AIMCMessage) -> Result<(), LinuxI2CError> {
        self.i2c_device.write(&message.into_bytes())
    }
}

impl GenericDispatch for AIMC {
    fn dispatch(&mut self, command: &GenericCommand) -> Result<(), Box<Error>> {
        self.write_message(match *command {
            GenericCommand::Enable(enable) => AIMCMessage::Enable(enable),
            GenericCommand::SetTarget(target) => AIMCMessage::SetTarget(target),
        })
        .map_err(|e| Box::new(e) as _) //TODO: Remove the as _ when the compiler updates >_>
    }
}
