use serde::{Deserialize, Serialize};

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

#[derive(Debug)]
pub enum GenericMessageConversionError {
    Unsupported(GenericCommand),
}
