use crate::aimc;
use crate::generic_message::{GenericCommand, GenericMessageConversionError};

pub fn convert(command: &GenericCommand) -> Result<[u8; 5], GenericMessageConversionError> {
    Ok(match command {
        GenericCommand::Enable(enable) => aimc::enable(*enable),
        GenericCommand::SetTarget(target) => aimc::set_target(*target),
    })
}
