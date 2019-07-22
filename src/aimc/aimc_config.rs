use super::AIMCMessage;
use serde::{Deserialize, Serialize};

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
