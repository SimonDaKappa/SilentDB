// src/lib.rs

// Declare modules
mod deser;
mod raw;
mod ser;
mod types;
mod utils;

// Re-export commonly used items
pub use deser::{Decoder, from_bytes, from_reader};
pub use ser::{Encoder, to_bytes, to_writer};
pub use types::{
    Document,
    Value,
    ObjectId,
    Timestamp,
    Binary,
    Regex,
    // ... other types TODO: add other types
};

// Optional: create a prelude module for convenient imports
pub mod prelude {
    pub use crate::types::{Document, Value};
    pub use crate::deser::{from_bytes, from_reader};
    pub use crate::ser::{to_bytes, to_writer};
}