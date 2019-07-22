use libaimc::{AIMCMessage, LinuxI2CError, AIMC};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Mode setting for the config file
#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum AIMCMode {
    PID(f32, f32, f32),
    Pneumatic,
    PWM,
}

impl AIMCMode {
    /// Convert this AIMCMode into a series of messages that initialize the device
    pub fn into_messages(&self) -> Vec<AIMCMessage> {
        match self {
            AIMCMode::PID(kp, ki, kd) => vec![
                AIMCMessage::ModePID,
                AIMCMessage::SetKp(*kp),
                AIMCMessage::SetKi(*ki),
                AIMCMessage::SetKd(*kd),
            ],
            AIMCMode::PWM => vec![AIMCMessage::ModePWM],
            AIMCMode::Pneumatic => vec![AIMCMessage::ModePneumatic],
        }
    }
}

/// In-memory representation of AIMC config file
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct AIMCConfig {
    pub address: u16,
    pub mode: AIMCMode,
}

impl Default for AIMCConfig {
    /// Default, just here for example purposes.
    fn default() -> Self {
        Self {
            address: 0x00,
            mode: AIMCMode::PID(0.0, 0.0, 0.0),
        }
    }
}

impl AIMCConfig {
    /// Construct a new AIMC object from the specified I2C device file, and attempt to set the
    /// parameters specified in the config struct.
    pub fn into_aimc<P: AsRef<Path>>(&self, i2c_device_file: P) -> Result<AIMC, LinuxI2CError> {
        let mut instance = AIMC::new(i2c_device_file, self.address)?;
        for message in self.mode.into_messages() {
            instance.write_message(message)?;
        }
        Ok(instance)
    }
}
