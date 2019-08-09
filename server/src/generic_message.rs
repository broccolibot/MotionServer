use serde::{Deserialize, Serialize};

use std::error::Error;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum GenericCommand {
    SetTarget(f32),
    Enable(bool),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum GenericMessage {
    Controller(String, GenericCommand),
    MessageAll(GenericCommand),
}

pub trait GenericDispatch {
    fn dispatch(
        &mut self,
        command: &GenericCommand,
        settings: &GenericDeviceSettings,
    ) -> Result<(), Box<Error>>;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GenericDeviceSettings {
    pub target_mapping: crate::linear_mapping::LinearMapping,
}

impl Default for GenericDeviceSettings {
    fn default() -> Self {
        Self {
            target_mapping: Default::default(),
        }
    }
}
