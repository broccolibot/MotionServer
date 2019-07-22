use log::info;
use crate::generic_message::{GenericDispatch, GenericCommand};
use std::error::Error;

pub struct TraceDevice {
    name: String,
}

impl TraceDevice {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl GenericDispatch for TraceDevice {
    fn dispatch(&mut self, command: &GenericCommand) -> Result<(), Box<Error>> {
        info!("Trace \"{}\": {:?}", self.name, command);
        Ok(())
    }
}
