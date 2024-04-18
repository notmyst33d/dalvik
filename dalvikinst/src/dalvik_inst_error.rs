use std::{error, fmt};
use dalvikcommon::DalvikCommonError;

#[derive(Debug)]
pub enum DalvikInstError {
    NoDecoder(u32),
    Foreign(String),
}

impl error::Error for DalvikInstError {}

impl fmt::Display for DalvikInstError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DalvikInstError::NoDecoder(opcode) => write!(f, "no decoder for opcode 0x{:02x?}", opcode),
            DalvikInstError::Foreign(message) => write!(f, "foreign error: {}", message),
        }
    }
}

impl From<DalvikCommonError> for DalvikInstError {
    fn from(error: DalvikCommonError) -> Self {
        Self::Foreign(error.to_string())
    }
}
