/// A BSON serializer that writes serialized data to a writer.

use std::io::{self, Write};
use byteorder::{LittleEndian, WriteBytesExt};
use super::error::SerializeError;
use super::traits::Serializer;

/// TODO: Implement the Serializer trait for BsonSerializer. Mostly done, but needs error handling.
pub struct BsonSerializer<W: Write + io::Seek> {
    writer: W,
    document_positions: Vec<u64>, // STack of document positions where length needs to be written
}

/// Implementation of the Serializer trait for BsonSerializer.
/// TODO: Error handling
/// TODO: Implement the remaining methods
impl<W: Write + io::Seek> BsonSerializer<W> {
    /// Creates a new BSON serializer that writes serialized data to the specified writer.
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            document_positions: Vec::new(),
        }
    }

    /// Returns the current position of the writer.
    pub fn current_position(&mut self) -> Result<u64, SerializeError> {
        Ok(self.writer.seek(io::SeekFrom::Current(0)).unwrap())
    }

    /// Writes the document length to the current position.
    pub fn write_document_length(&mut self) -> Result<(), SerializeError> {
        let current_position = self.current_position()?;
        let document_position = self.document_positions.pop().unwrap();
        let length = current_position - document_position;
        self.writer
            .seek(io::SeekFrom::Start(document_position))
            .unwrap();
        self.writer.write_u32::<LittleEndian>(length as u32)?;
        self.writer
            .seek(io::SeekFrom::Start(current_position))
            .unwrap();
        Ok(())
    }
}

impl<W: Write + io::Seek> Serializer for BsonSerializer<W> {
    fn serialize_f64(&mut self, value: f64) -> Result<(), SerializeError> {
        self.writer.write_u8(0x01)?;
        self.writer.write_f64::<LittleEndian>(value)?;
        Ok(())
    }

    fn serialize_string(&mut self, value: &str) -> Result<(), SerializeError> {
        self.writer.write_u8(0x02)?;

        // Write the string length
        self.writer
            .write_i32::<LittleEndian>(value.len() as i32 + 1)?;

        // Write the string
        self.writer.write_all(value.as_bytes())?;

        // Write the null terminator
        self.writer.write_u8(0)?;

        Ok(())
    }

    fn serialize_document(&mut self, value: &Document) -> Result<(), SerializeError> {
        self.writer.write_u8(0x03)?;

        // Push the current position to the stack
        let current_pos = self.current_position()?;
        self.document_positions.push(current_pos);

        // Write a placeholder for the document length
        self.writer.write_u32::<LittleEndian>(0)?;

        // Serialize the document
        for (key, value) in value.iter() {
            self.serialize_field_name(key)?;
            value.serialize(self)?;
        }

        // End the document
        self.end_document()?;

        Ok(())
    }

    fn serialize_array(&mut self, value: &Array) -> Result<(), SerializeError> {
        self.writer.write_u8(0x04)?;

        // Push the current position to the stack
        let current_position = self.current_position()?;
        self.document_positions.push(current_position);

        // Write a placeholder for the array length
        self.writer.write_u32::<LittleEndian>(0)?;

        // Serialize the array
        for (index, value) in value.iter().enumerate() {
            self.serialize_field_name(&index.to_string())?;
            value.serialize(self)?;
        }

        // End the document
        self.end_document()?;

        Ok(())
    }

    fn serialize_binary(&mut self, value: &[u8]) -> Result<(), SerializeError> {
        self.writer.write_u8(0x05)?;

        // Write the binary length
        self.writer.write_i32::<LittleEndian>(value.len() as i32)?;

        // Write the binary subtype
        // TODO: Implement BinarySubtype enum
        self.writer.write_u8(0)?;

        // Write the binary data
        self.writer.write_all(value)?;

        Ok(())
    }

    fn serialize_undefined(&mut self) -> Result<(), SerializeError> {
        self.writer.write_u8(0x06)?;
        Ok(())
    }

    fn serialize_object_id(&mut self, value: ObjectId) -> Result<(), SerializeError> {
        self.writer.write_u8(0x07)?;

        // Write the object id
        self.writer.write_all(&value.0)?;

        Ok(())
    }

    fn serialize_boolean(&mut self, value: bool) -> Result<(), SerializeError> {
        self.writer.write_u8(0x08)?;

        // Write the boolean value
        self.writer.write_u8(if value { 0x01 } else { 0x00 })?;

        Ok(())
    }

    fn serialize_utc_datetime(&mut self, value: i64) -> Result<(), SerializeError> {
        self.writer.write_u8(0x09)?;

        // Write the UTC datetime
        self.writer.write_i64::<LittleEndian>(value)?;

        Ok(())
    }

    fn serialize_null(&mut self) -> Result<(), SerializeError> {
        self.writer.write_u8(0x0A)?;
        Ok(())
    }

    fn serialize_regex(&mut self, pattern: &str, options: &str) -> Result<(), SerializeError> {
        self.writer.write_u8(0x0B)?;

        // Write the pattern
        self.writer.write_all(pattern.as_bytes())?;
        self.writer.write_u8(0)?;

        // Write the options
        self.writer.write_all(options.as_bytes())?;
        self.writer.write_u8(0)?;

        Ok(())
    }

    fn serialize_db_pointer(
        &mut self,
        collection: &str,
        id: ObjectId,
    ) -> Result<(), SerializeError> {
        // DEPRECATED
        Err(SerializeError::Deprecated(format!(
            "DBPointer is deprecated. Collection: {}, ID: {}",
            collection, id
        )))
    }

    fn serialize_javascript_code(&mut self, code: &str) -> Result<(), SerializeError> {
        self.writer.write_u8(0x0D)?;

        // Write the code
        self.writer.write_all(code.as_bytes())?;
        self.writer.write_u8(0)?;

        Ok(())
    }

    fn serialize_symbol(&mut self, symbol: &str) -> Result<(), SerializeError> {
        // DEPRECATED
        Err(SerializeError::Deprecated(format!(
            "Symbol is deprecated. Symbol: {}",
            symbol
        )))
    }

    fn serialize_javascript_code_with_scope(
        &mut self,
        code: &str,
        scope: &Document,
    ) -> Result<(), SerializeError> {
        // DEPRECATED

        // Take first 10 chars of code, with "..." appended if truncated
        let truncated_code = code.chars().take(10).collect::<String>()
            + if code.chars().count() > 100 {
                "..."
            } else {
                ""
            };

        Err(SerializeError::Deprecated(format!(
            "JavaScript code with scope is deprecated. Code: {}, Scope: {}",
            truncated_code, scope
        )))
    }

    fn serialize_i32(&mut self, value: i32) -> Result<(), SerializeError> {
        self.writer.write_u8(0x10)?;

        // Write the 32-bit integer
        self.writer.write_i32::<LittleEndian>(value)?;

        Ok(())
    }

    fn serialize_timestamp(&mut self, value: i64) -> Result<(), SerializeError> {
        self.writer.write_u8(0x11)?;

        // Write the timestamp
        self.writer.write_i64::<LittleEndian>(value)?;

        Ok(())
    }

    fn serialize_i64(&mut self, value: i64) -> Result<(), SerializeError> {
        self.writer.write_u8(0x12)?;

        // Write the 64-bit integer
        self.writer.write_i64::<LittleEndian>(value)?;

        Ok(())
    }

    fn serialize_u64(&mut self, value: u64) -> Result<(), SerializeError> {
        self.writer.write_u8(0x13)?;

        // Write the 64-bit integer
        self.writer.write_u64::<LittleEndian>(value)?;

        Ok(())
    }

    // fn serialize_f128(&mut self, value: f128) -> Result<(), SerializeError> {
    //     Err(SerializeError::NotImplemented(format!(
    //         "Serialization of f128 is not supported. STD f128 is unstable. Value: {}",
    //         value
    //     )))
    // }

    fn serialize_min_key(&mut self) -> Result<(), SerializeError> {
        self.writer.write_u8(0xFF)?;
        Ok(())
    }

    fn serialize_max_key(&mut self) -> Result<(), SerializeError> {
        self.writer.write_u8(0x7F)?;
        Ok(())
    }

    fn start_document(&mut self) -> Result<(), SerializeError> {
        self.writer.write_u8(0x03)?;

        // Push the current position to the stack
        let current_position = self.current_position()?;
        self.document_positions.push(current_position);

        self.writer.write_u32::<LittleEndian>(0)?;
        Ok(())
    }

    fn end_document(&mut self) -> Result<(), SerializeError> {
        self.write_document_length()?;
        Ok(())
    }

    fn serialize_field_name(&mut self, name: &str) -> Result<(), SerializeError> {
        self.writer.write_all(name.as_bytes())?;
        self.writer.write_u8(0)?;
        Ok(())
    }
}
