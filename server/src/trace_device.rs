use crate::generic_message::{GenericCommand, GenericDeviceSettings, GenericDispatch};
use log::info;
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
    fn dispatch(
        &mut self,
        command: &GenericCommand,
        _: &GenericDeviceSettings,
    ) -> Result<(), Box<Error>> {
        info!("Trace \"{}\": {:?}", self.name, command);
        Ok(())
    }
}
