pub struct JsonSerializer{
    output: String,
    pretty: bool,
    indent_level: usize,
}

impl JsonSerializer {
    pub fn new(pretty: bool) -> Self {
        JsonSerializer {
            output: String::new(),
            pretty,
            indent_level: 0,
        }
    }

    fn write_indent(&mut self) {
        if self.pretty {
            self.output.push_str(&"  ".repeat(self.indent_level));
        }
    }

    fn write_newline(&mut self) {
        if self.pretty {
            self.output.push('\n');
        }
    }

    fn write_separator(&mut self) {
        if self.pretty {
            self.output.push_str(",\n");
        } else {
            self.output.push(',');
        }
    }
}

impl Serializer for JsonSerializer{

    fn serialize_f64(&mut self, value: f64) -> Result<(), SerializeError> {
        if value.is_finite() {
            self.output.push_str(&value.to_string());
        } else if value.is_nan() {
            self.output.push_str("null");
        } else if value.is_infinite() {
            if value.is_sign_positive() {
                self.output.push_str("null"); 
            } else {
                self.output.push_str("null");
            }
        }
        Ok(())
    }

    fn serialize_i32(&mut self, value: i32) -> Result<(), SerializeError> {
        self.output.push_str(&value.to_string());
        Ok(())
    }

    fn serialize_i64(&mut self, value: i64) -> Result<(), SerializeError> {
        self.output.push_str(&value.to_string());
        Ok(())
    }

    fn serialize_string(&mut self, value: &str) -> Result<(), SerializeError> {
        self.output.push('"');
        
        // Properly escape the string
        for c in value.chars() {
            match c {
                '"' => self.output.push_str("\\\""),
                '\\' => self.output.push_str("\\\\"),
                '\n' => self.output.push_str("\\n"),
                '\r' => self.output.push_str("\\r"),
                '\t' => self.output.push_str("\\t"),
                c if c.is_control() => {
                    self.output.push_str(&format!("\\u{:04x}", c as u32));
                }
                c => self.output.push(c),
            }
        }
        
        self.output.push('"');
        Ok(())
    }

    fn serialize_document(&mut self, value: &Document) -> Result<(), SerializeError> {
        self.start_document()?;
        
        let mut first = true;
        for (key, value) in value.iter() {
            if !first {
                self.write_separator();
            }
            first = false;
            
            self.serialize_field_name(key)?;
            value.serialize(self)?;
        }
        
        self.end_document()?;
        Ok(())
    }

    fn serialize_array(&mut self, value: &Array) -> Result<(), SerializeError> {
        self.output.push('[');
        self.indent_level += 1;
        self.write_newline();
        
        let mut first = true;
        for value in value.iter() {
            if !first {
                self.write_separator();
            }
            first = false;
            
            self.write_indent();
            value.serialize(self)?;
        }
        
        self.indent_level -= 1;
        self.write_newline();
        self.write_indent();
        self.output.push(']');
        Ok(())
    }

    fn start_document(&mut self) -> Result<(), SerializeError> {
        self.output.push('{');
        self.indent_level += 1;
        self.write_newline();
        Ok(())
    }

    fn end_document(&mut self) -> Result<(), SerializeError> {
        self.indent_level -= 1;
        self.write_newline();
        self.write_indent();
        self.output.push('}');
        Ok(())
    }

    fn serialize_field_name(&mut self, name: &str) -> Result<(), SerializeError> {
        self.write_indent();
        self.serialize_string(name)?;
        self.output.push_str(": ");
        Ok(())
    }
}