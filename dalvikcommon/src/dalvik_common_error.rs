use std::{fmt, error};

#[derive(Debug)]
pub enum DalvikCommonError {
    InvalidString,
    StringSizeDoesNotMatch,
    NotEnoughData,
}

impl error::Error for DalvikCommonError {}

impl fmt::Display for DalvikCommonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DalvikCommonError::InvalidString => write!(f, "invalid string"),
            DalvikCommonError::StringSizeDoesNotMatch => write!(f, "string size does not match"),
            DalvikCommonError::NotEnoughData => write!(f, "not enough data"),
        }
    }
}
