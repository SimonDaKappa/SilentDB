/// Serialization Errors.

use std::io;

/// Represents errors that can occur during serialization.
#[derive(Debug, thiserror::Error)]
pub enum SerializeError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("Buffer Overflow")]
    BufferOverflow,
    #[error("Invalid value: {0}")]
    InvalidValue(String),
    #[error("Invalid UTF-8 string: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("Invalid BSON document: {0}")]
    InvalidDocument(String),
    #[error("Deprecated: {0}")]
    Deprecated(String),
    #[error("Not Implemented")]
    NotImplemented,
    #[error("Not Supported")]
    NotSupported(String),
}

pub type Result<T> = std::result::Result<T, SerializeError>;