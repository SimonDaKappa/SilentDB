use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{self, write};
use std::ops::{Deref, DerefMut};

use crate::ser::{SerializeError, Serializer};
use crate::types::{Array, Document, ObjectId, UTCDateTime};

use super::Timestamp;

/// Represents a BSON value.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Double(f64),
    String(String),
    Document(Document),
    Array(Array),
    Binary(Vec<u8>),
    ObjectId(ObjectId),
    Boolean(bool),
    UTCDateTime(i64),
    Null,
    RegularExpression { pattern: String, options: String },
    JavaScriptCode(String),
    JavaScriptCodeWithScope { code: String, scope: Document },
    Int32(i32),
    Timestamp(i64),
    Int64(i64),
    UInt64(u64),
    MinKey,
    MaxKey,
}

impl Value {
    /// Serialize given value using given serializer.
    pub fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), SerializeError> {
        match self {
            Value::Double(value) => serializer.serialize_f64(*value),
            Value::String(value) => serializer.serialize_string(value),
            Value::Document(value) => serializer.serialize_document(value),
            Value::Array(value) => serializer.serialize_array(value),
            Value::Binary(value) => serializer.serialize_binary(value),
            Value::ObjectId(value) => serializer.serialize_object_id(value),
            Value::Boolean(value) => serializer.serialize_boolean(*value),
            Value::UTCDateTime(value) => serializer.serialize_timestamp(*value),
            Value::Null => serializer.serialize_null(),
            Value::RegularExpression { pattern, options } => {
                serializer.serialize_regex(pattern, options)
            }
            Value::JavaScriptCode(value) => serializer.serialize_javascript_code(value),
            Value::JavaScriptCodeWithScope { code, scope } => {
                serializer.serialize_javascript_code_with_scope(code, scope)
            }
            Value::Int32(value) => serializer.serialize_i32(*value),
            Value::Timestamp(value) => serializer.serialize_timestamp(*value),
            Value::Int64(value) => serializer.serialize_i64(*value),
            Value::UInt64(value) => serializer.serialize_u64(*value),
            Value::MinKey => serializer.serialize_min_key(),
            Value::MaxKey => serializer.serialize_max_key(),
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Double(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_i32(&self) -> Option<i32> {
        match self {
            Value::Int32(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::Int64(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_document(&self) -> Option<&Document> {
        match self {
            Value::Document(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Array> {
        match self {
            Value::Array(value) => Some(value),
            _ => None,
        }
    }
}

/* Conversion Traits for Values */

impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Value::Int32(v)
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Value::Int64(v)
    }
}

impl From<u64> for Value {
    fn from(v: u64) -> Self {
        Value::UInt64(v)
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::Double(v)
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Value::Boolean(v)
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::String(v)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_string())
    }
}

impl From<ObjectId> for Value {
    fn from(value: ObjectId) -> Self {
        Value::ObjectId(value)
    }
}

impl From<Document> for Value {
    fn from(v: Document) -> Self {
        Value::Document(v)
    }
}

impl From<Array> for Value {
    fn from(v: Array) -> Self {
        Value::Array(v)
    }
}

impl From<Vec<u8>> for Value {
    fn from(v: Vec<u8>) -> Self {
        Value::Binary(v)
    }
}

impl From<UTCDateTime> for Value {
    fn from(value: UTCDateTime) -> Self {
        Value::UTCDateTime(value.into())
    }
}

impl From<Timestamp> for Value {
    fn from(value: Timestamp) -> Self {
        Value::Timestamp(value.into())
    }
}

/* Pretty Printing Implementation */
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Double(v) => write!(f, "{}", v),
            Value::String(v) => write!(f, "{}", v),
            Value::Document(v) => write!(f, "{}", v),
            Value::Array(v) => {
                write!(f, "[")?;
                for (i, value) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", value)?;
                }
                write!(f, "]")
            }
            Value::Binary(v) => write!(f, "Binary(len: {})", v.len()),
            Value::ObjectId(v) => write!(f, "ObjectID(\"{}\")", v),
            Value::Boolean(v) => write!(f, "{}", v),
            Value::UTCDateTime(v) => write!(f, "DateTime({})", v),
            Value::Null => write!(f, "null"),
            Value::RegularExpression { pattern, options } => {
                 write!(f, "/{}/{}", pattern, options)
            }
            Value::Int32(v) => write!(f, "{}", v),
            Value::Timestamp(v) => write!(f, "Timestamp({})", v),
            Value::Int64(v) => write!(f, "{}", v),
            Value::UInt64(v) => write!(f, "{}", v),
            Value::MinKey => write!(f, "MinKey"),
            Value::MaxKey => write!(f, "MaxKey"),
            Value::JavaScriptCode(v) => {
                // Write first 10 characters of the code
                let truncated_code = v.chars().take(10).collect::<String>();
                write!(f, "JavaScriptCode({})", truncated_code)
            }
            Value::JavaScriptCodeWithScope { code, scope } => {
                // Write first 10 characters of the code
                let truncated_code = code.chars().take(10).collect::<String>();
                write!(f, "JavaScriptCodeWithScope(Code({}), Scope({}))", truncated_code, scope)
            }
        }
    }
}
