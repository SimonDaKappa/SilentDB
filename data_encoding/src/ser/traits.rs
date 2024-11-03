use super::SerializeError;

/// The main Serializer trait. Defines methods for serializing BSON values.
///
/// This trait defines methods for serializing different types of data, including integers,
/// floating-point numbers, strings, binary data, and special BSON types. Implementors of this
/// trait are responsible for providing the logic to serialize these types into a specific format.
pub trait Serializer {
    /* Serialization Functions */

    /// Serializes a 64-bit floating-point number. Type byte: 0x01
    ///
    /// # Arguments
    ///
    /// * `value` - The 64-bit floating-point number to serialize.
    ///
    /// # Errors
    ///
    /// Returns an error if the serialization fails.
    fn serialize_f64(&mut self, value: f64) -> Result<(), SerializeError>;

    /// Serializes a string. Type byte: 0x02
    ///
    /// # Arguments
    ///
    /// * `value` - The string to serialize.
    ///
    /// # Errors
    ///
    /// Returns an error if the serialization fails.
    fn serialize_string(&mut self, value: &str) -> Result<(), SerializeError>;

    /// Serializes a document. Type byte: 0x03
    ///
    /// # Arguments
    ///
    /// * `value` - The document to serialize.
    ///
    /// # Errors
    ///
    /// Returns an error if the serialization fails.
    fn serialize_document(&mut self, value: &Document) -> Result<(), SerializeError>;

    /// Serializes an array. Type byte: 0x04
    ///
    /// # Arguments
    ///
    /// * `value` - The array to serialize.
    ///
    /// # Errors
    ///
    /// Returns an error if the serialization fails.
    fn serialize_array(&mut self, value: &Array) -> Result<(), SerializeError>;

    /// Serializes binary data. Type byte: 0x05
    ///
    /// # Arguments
    ///
    /// * `value` - The binary data to serialize.
    ///
    /// # Errors
    ///
    /// Returns an error if the serialization fails.
    fn serialize_binary(&mut self, value: &[u8]) -> Result<(), SerializeError>;

    /// Serializes an undefined value. Type byte: 0x06
    /// # `Deprecated`
    ///
    /// # Errors
    ///
    /// ALWAYS Returns Err(SerializationError::Deprecated)
    fn serialize_undefined(&mut self) -> Result<(), SerializeError>;

    /// Serializes an object id. Type byte: 0x07
    ///
    /// # Arguments
    ///
    /// * `value` - The object id to serialize.
    ///
    /// # Errors
    ///
    /// Returns an error if the serialization fails.
    fn serialize_object_id(&mut self, value: ObjectId) -> Result<(), SerializeError>;

    /// Serializes a boolean value. Type byte: 0x08
    ///
    /// # Arguments
    ///
    /// * `value` - The boolean value to serialize.
    ///
    /// # Errors
    ///
    /// Returns an error if the serialization fails.
    fn serialize_boolean(&mut self, value: bool) -> Result<(), SerializeError>;

    /// Serializes a UTC datetime. Type byte: 0x09
    ///
    /// # Arguments
    ///
    /// * `value` - The UTC datetime to serialize.
    ///
    /// # Errors
    ///
    /// Returns an error if the serialization fails.
    fn serialize_utc_datetime(&mut self, value: i64) -> Result<(), SerializeError>;

    /// Serializes null. Type byte: 0x0A
    ///
    /// # Errors
    ///
    /// Returns an error if the serialization fails.
    fn serialize_null(&mut self) -> Result<(), SerializeError>;

    /// Serializes a regular expression. Type byte: 0x0B
    ///
    /// # Arguments
    ///
    /// * `pattern` - The regular expression pattern.
    ///
    /// * `options` - The regular expression options.
    ///
    /// # Errors
    ///
    /// Returns an error if the serialization fails.
    fn serialize_regex(&mut self, pattern: &str, options: &str) -> Result<(), SerializeError>;

    /// Serializes a database reference. Type byte: 0x0C
    /// # `Deprecated`
    ///
    /// # Errors
    ///
    /// ALWAYS Returns Err(SerializationError::Deprecated)
    fn serialize_db_pointer(
        &mut self,
        collection: &str,
        id: ObjectId,
    ) -> Result<(), SerializeError>;

    /// Serializes JavaScript code. Type byte: 0x0D
    ///
    /// # Arguments
    ///
    /// * `code` - The JavaScript code to serialize.
    ///
    /// # Errors
    ///
    /// Returns an error if the serialization fails.
    fn serialize_javascript_code(&mut self, code: &str) -> Result<(), SerializeError>;

    /// Serializes a symbol. Type byte: 0x0E
    /// # `Deprecated`
    ///
    /// # Errors
    ///
    /// ALWAYS Returns Err(SerializationError::Deprecated)
    fn serialize_symbol(&mut self, symbol: &str) -> Result<(), SerializeError>;

    /// Serializes JavaScript code with scope. Type byte: 0x0F  
    /// # `Deprecated`
    ///
    /// # Errors
    ///
    /// ALWAYS Returns Err(SerializationError::Deprecated)
    ///
    fn serialize_javascript_code_with_scope(
        &mut self,
        code: &str,
        scope: &Document,
    ) -> Result<(), SerializeError>;

    /// Serializes a 32-bit integer. Type byte: 0x10
    ///
    /// # Arguments
    ///
    /// * `value` - The 32-bit integer to serialize.
    ///
    /// # Errors
    ///
    /// Returns an error if the serialization fails.
    fn serialize_i32(&mut self, value: i32) -> Result<(), SerializeError>;

    /// Serializes a timestamp. Type byte: 0x11
    ///
    /// # Arguments
    ///
    /// * `value` - The timestamp to serialize.
    ///
    /// # Errors
    ///
    /// Returns an error if the serialization fails.
    fn serialize_timestamp(&mut self, value: i64) -> Result<(), SerializeError>;

    /// Serializes a 64-bit integer. Type byte: 0x12
    ///
    /// # Arguments
    ///
    /// * `value` - The 64-bit integer to serialize.
    ///
    /// # Errors
    ///
    /// Returns an error if the serialization fails.
    fn serialize_i64(&mut self, value: i64) -> Result<(), SerializeError>;

    /// Serializes a 64-bit integer. Type byte: 0x13
    ///
    /// # Arguments
    ///
    /// * `value` - The 64-bit integer to serialize.
    ///
    /// # Errors
    ///
    /// Returns an error if the serialization fails.
    fn serialize_u64(&mut self, value: u64) -> Result<(), SerializeError>;

    /// Serializes a f128. Type byte: 0x13
    /// # `Not Supported` - Rust does not have a stable native f128 type
    /// 
    /// # Arguments
    ///
    /// * `value` - The f128 to serialize.
    ///
    /// # Errors
    ///
    /// Returns an error if the serialization fails.
    // fn serialize_f128(&mut self, value: f128) -> Result<(), SerializeError>;

    /// Serializes a min key. Type byte: 0xFF (-1)
    ///
    /// # Errors
    ///
    /// Returns an error if the serialization fails.
    fn serialize_min_key(&mut self) -> Result<(), SerializeError>;

    /// Serializes a max key. Type byte: 0x7F
    ///
    /// # Errors
    ///
    /// Returns an error if the serialization fails.
    fn serialize_max_key(&mut self) -> Result<(), SerializeError>;

    /* Document Helpers */

    /// Starts a new document.
    ///
    /// # Errors
    /// Returns an error if the serialization fails.
    fn start_document(&mut self) -> Result<(), SerializeError>;

    /// Ends the current document.
    ///
    /// # Errors
    /// Returns an error if the serialization fails.
    fn end_document(&mut self) -> Result<(), SerializeError>;

    /// Serializes a field name.
    ///
    /// # Arguments
    /// * `name` - The name of the field to serialize.
    /// # Errors
    /// Returns an error if the serialization fails.
    fn serialize_field_name(&mut self, name: &str) -> Result<(), SerializeError>;
}
