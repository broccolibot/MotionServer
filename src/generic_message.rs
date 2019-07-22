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
    fn dispatch(&mut self, command: &GenericCommand) -> Result<(), Box<Error>>;
}
