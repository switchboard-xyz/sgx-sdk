// src/error.rs

use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    CustomMessage(String),
    CustomError {
        message: String,
        source: Box<dyn StdError + 'static>,
    },
    ConfigError(String), // Add other error variants as needed
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::CustomError { source, .. } => Some(source.as_ref()), // Handle other error variants as needed
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CustomMessage(message) => write!(f, "error: {}", message.as_str()),
            Error::CustomError {
                message, source, ..
            } => write!(f, "error: {} - {:?}", message.as_str(), source),
            // Handle other error variants as needed
            Error::ConfigError(var) => write!(f, "config error: {}", var.as_str()),
        }
    }
}
