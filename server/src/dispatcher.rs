use crate::{aimc_config::AIMCConfig, generic_message::*, trace_device::TraceDevice};
use libaimc::{AIMCMessage, AIMC};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

/// Command dispatcher. A translation layer between GenericCommands and real devices.
pub struct Dispatcher(HashMap<String, (Box<dyn GenericDispatch>, GenericDeviceSettings)>);

impl Dispatcher {
    /// Initialize the dispatcher from the specified config struct.
    pub fn from_config(config: DispatcherConfig) -> Result<Self, Box<Error>> {
        let mut devices: HashMap<String, (Box<dyn GenericDispatch>, GenericDeviceSettings)> =
            HashMap::new();

        for (name, config) in config.aimcs {
            let mut device = AIMC::new(config.i2c_bus, config.address)?;
            for command in config.startup_commands {
                device.write_message(command)?;
            }
            devices.insert(name, (Box::new(device), config.settings));
        }

        for name in config.debug_devices {
            devices.insert(
                name.clone(),
                (Box::new(TraceDevice::new(name)), Default::default()),
            );
        }

        Ok(Self(devices))
    }

    /// Dispatch a generic command to the devices
    pub fn dispatch(&mut self, message: GenericMessage) -> Result<(), DispatchError> {
        match message {
            GenericMessage::MessageAll(command) => {
                for (device, settings) in self.0.values_mut() {
                    device
                        .dispatch(&command, &settings)
                        .map_err(|e| DispatchError::ControllerFailure(e))?;
                }
                Ok(())
            }
            GenericMessage::Controller(name, command) => match self.0.get_mut(&name) {
                Some((device, settings)) => device
                    .dispatch(&command, &settings)
                    .map_err(|e| DispatchError::ControllerFailure(e)),
                None => Err(DispatchError::MissingKey(name)),
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DispatcherConfig {
    pub debug_devices: Vec<String>,
    pub aimcs: HashMap<String, AIMCConfig>,
}

impl Default for DispatcherConfig {
    fn default() -> Self {
        Self {
            aimcs: [("example".to_string(), AIMCConfig::default())]
                .iter()
                .cloned()
                .collect(),
            debug_devices: vec!["debug".to_string()],
        }
    }
}

#[derive(Debug)]
pub enum DispatchError {
    MissingKey(String),
    ControllerFailure(Box<Error>),
}

impl GenericDispatch for AIMC {
    fn dispatch(
        &mut self,
        command: &GenericCommand,
        settings: &GenericDeviceSettings,
    ) -> Result<(), Box<Error>> {
        self.write_message(match *command {
            GenericCommand::Enable(enable) => AIMCMessage::Enable(enable),
            GenericCommand::SetTarget(target) => {
                AIMCMessage::SetTarget(settings.target_mapping.map(target))
            }
        })
        .map_err(|e| Box::new(e) as _) //TODO: Remove the as _ when the compiler updates >_>
    }
}
