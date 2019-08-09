use libaimc::AIMCMessage;
use serde::{Deserialize, Serialize};

/// In-memory representation of AIMC config file
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AIMCConfig {
    pub address: u16,
    pub i2c_bus: String,
    pub startup_commands: Vec<AIMCMessage>,
    #[serde(flatten)]
    pub settings: crate::generic_message::GenericDeviceSettings,
}

impl Default for AIMCConfig {
    /// Default, just here for example purposes.
    fn default() -> Self {
        Self {
            address: 0x00,
            i2c_bus: "/dev/i2c-0".to_string(),
            settings: Default::default(),
            startup_commands: vec![
                AIMCMessage::SetTarget(0.0),
                AIMCMessage::Reset,
                AIMCMessage::Enable(false),
                AIMCMessage::EncoderPolarity(false),
                AIMCMessage::LimitPwm(32),
                AIMCMessage::ModePWM,
                AIMCMessage::ModePneumatic,
                AIMCMessage::ModePID,
                AIMCMessage::SetKp(0.1),
                AIMCMessage::SetKi(0.0),
                AIMCMessage::SetKd(0.0),
                AIMCMessage::Home(0),
                AIMCMessage::LimitTargetMax(0.0),
                AIMCMessage::LimitTargetMin(0.0),
            ],
        }
    }
}
