use crate::{
    aimc_config::AIMCConfig, aimc_conversion, generic_message::*, i2c_trace_mock::I2CTraceMock,
};
use i2cdev::core::I2CDevice;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};
use log::info;
use std::collections::HashMap;

pub enum DispatcherDevice {
    AIMC(LinuxI2CDevice),
    DebugTrace,
}

impl DispatcherDevice {
    pub fn dispatch(&mut self, command: &GenericCommand) -> Result<(), DispatchError> {
        match self {
            DispatcherDevice::AIMC(i2chandle) => i2chandle
                .write(
                    &aimc_conversion::convert(command).map_err(|e| DispatchError::Conversion(e))?,
                )
                .map_err(|e| DispatchError::I2C(e)),
            DispatcherDevice::DebugTrace => {
                info!("Dispatcher trace: {:?}", command);
                Ok(())
            }
        }
    }
}

pub struct Dispatcher(HashMap<String, DispatcherDevice>);

impl Dispatcher {
    pub fn from_config(config: DispatcherConfig) -> Result<Self, DispatcherCreationError> {
        let mut devices = HashMap::new();

        for (name, parameters) in config.aimcs {
            let device = LinuxI2CDevice::new(config.i2c_device_file, parameters.address)
                .map_err(|e| DispatcherCreationError::I2C(e))?;
            devices.insert(String::from(name), DispatcherDevice::AIMC(device));
        }

        for name in config.debug_devices {
            devices.insert(name, DispatcherDevice::DebugTrace);
        }

        Ok(Self(devices))
    }

    pub fn dispatch(&mut self, message: GenericMessage) -> Result<(), DispatchError> {
        match message {
            GenericMessage::MessageAll(command) => {
                for value in self.0.values_mut() {
                    value.dispatch(&command)?;
                }
                Ok(())
            }
            GenericMessage::Controller(motor, command) => match self.0.get_mut(&motor) {
                Some(device) => device.dispatch(&command),
                None => Err(DispatchError::MissingKey(motor)),
            },
        }
    }
}

pub struct DispatcherConfig<'a> {
    pub i2c_device_file: &'a str,
    pub debug_devices: Vec<String>,
    pub aimcs: HashMap<&'a str, AIMCConfig>,
}

#[derive(Debug)]
pub enum DispatcherCreationError {
    //ConflictingNames(Vec<String>),
    I2C(LinuxI2CError),
}

#[derive(Debug)]
pub enum DispatchError {
    MissingKey(String),
    I2C(LinuxI2CError),
    Conversion(GenericMessageConversionError),
}
