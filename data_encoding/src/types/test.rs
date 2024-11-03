#[cfg(test)]
mod tests {
    use crate::types::array::Array;
    use crate::types::document::Document;
    use crate::types::object_id::ObjectId;
    use crate::types::time::Timestamp;
    use crate::types::time::UTCDateTime;
    use crate::types::value::Value;

    // -------------------------------------
    //          Document Tests
    // -------------------------------------

    #[test]
    fn test_document_new() {
        let document = Document::new();
        assert!(document.is_empty());
    }

    #[test]
    fn test_document_new_with_capacity() {
        let document = Document::new_with_capacity(10);
        assert!(document.is_empty());
        assert!(document.capacity() == 10);
    }

    #[test]
    fn test_document_basic_operations() {
        let mut doc = Document::new();

        doc.insert("key1", 1);
        doc.insert("key2", "value2");
        doc.insert("key3", 3.0);

        assert_eq!(doc.len(), 3);
        assert_eq!(doc.get("key1"), Some(&1.into()));
        assert_eq!(doc.get("key2"), Some(&"value2".into()));
        assert!(!doc.contains_key("key4"));

        doc.remove("key1");
        assert_eq!(doc.len(), 2);
        assert_eq!(doc.get("key1"), None);

        doc.clear();
        assert!(doc.is_empty());
    }

    #[test]
    fn test_document_add_get_all_values() {
        let mut doc = Document::new();

        // Float64
        doc.insert("Double", 3.0 as f64);
        assert_eq!(doc.get("Double"), Some(&3.0.into()));

        // String
        doc.insert("String", "value2");
        assert_eq!(doc.get("String"), Some(&"value2".into()));

        // Document
        let mut dummy_doc = Document::new();
        dummy_doc.insert("key", "value");
        doc.insert("Document", dummy_doc.clone());
        assert_eq!(doc.get("Document"), Some(&dummy_doc.clone().into()));

        // Array
        let array = Array::from_vec(vec![1.into(), "string".into(), dummy_doc.clone().into()]);
        doc.insert("Array", array.clone());
        assert_eq!(doc.get("Array"), Some(&array.into()));

        // Binary
        doc.insert("Binary", vec![0, 1, 2, 3]);
        assert_eq!(doc.get("Binary"), Some(&vec![0, 1, 2, 3].into()));

        // ObjectId
        let object_id = ObjectId::from("5e4f2f2d7f3d2d2d2d2d2d2d".to_string().as_ref());
        doc.insert("ObjectId", object_id.clone());
        assert_eq!(doc.get("ObjectId"), Some(&object_id.into()));

        // Boolean
        doc.insert("Boolean", true);
        assert_eq!(doc.get("Boolean"), Some(&true.into()));

        // UTCDateTime
        let utc_date_time = UTCDateTime::from(1234567890);
        doc.insert("UTCDateTime", utc_date_time.clone());
        assert_eq!(doc.get("UTCDateTime"), Some(&utc_date_time.into()));

        // Null
        doc.insert("Null", Value::Null);
        assert_eq!(doc.get("Null"), Some(&Value::Null));

        // RegularExpression
        doc.insert(
            "RegularExpression",
            Value::RegularExpression { pattern: "pattern".into(), options: "value".into() },
        );
        assert_eq!(
            doc.get("RegularExpression"),
            Some(&Value::RegularExpression { pattern: "pattern".into(), options: "options".into() })
        );

        // JavaScriptCode
        doc.insert("JavaScriptCode", "code");
        assert_eq!(doc.get("JavaScriptCode"), Some(&"code".into()));

        // Int32
        doc.insert("Int32", 32);
        assert_eq!(doc.get("Int32"), Some(&32.into()));

        // Timestamp
        let time_stamp = Timestamp::from(1234567890);
        doc.insert("Timestamp", time_stamp.clone());
        assert_eq!(doc.get("Timestamp"), Some(&time_stamp.into()));

        // Int64
        doc.insert("Int64", 64);
        assert_eq!(doc.get("Int64"), Some(&64.into()));

        // UInt64
        doc.insert("UInt64", 64 as u64);
        assert_eq!(doc.get("UInt64"), Some(&(64 as u64).into()));

        // MinKey
        doc.insert("MinKey", Value::MinKey);
        assert_eq!(doc.get("MinKey"), Some(&Value::MinKey));

        // MaxKey
        doc.insert("MaxKey", Value::MaxKey);
        assert_eq!(doc.get("MaxKey"), Some(&Value::MaxKey));

        // JavaScriptCodeWithScope
        let current_scope = doc.clone();
        doc.insert(
            "JavaScriptCodeWithScope",
            Value::JavaScriptCodeWithScope { code: "code".to_string(), scope: current_scope.clone() },
        );
        assert_eq!(
            doc.get("JavaScriptCodeWithScope"),
            Some(&Value::JavaScriptCodeWithScope { code: "code".to_string(), scope: current_scope.into() })
        );
    }

    // -------------------------------------
    //            Array Tests
    // -------------------------------------

    #[test]
    fn test_array_new() {
        let array = Array::new();
        assert!(array.is_empty());
    }

    #[test]
    fn test_array_basic_operations() {
        let mut array = Array::new();

        array.push(1);
        array.push("value2");
        array.push(3.0);

        assert_eq!(array.len(), 3);
        assert_eq!(array.get(0), Some(&1.into()));
        assert_eq!(array.get(1), Some(&"value2".into()));
        assert_eq!(array.get(2), Some(&3.0.into()));

        array.pop();
        assert_eq!(array.len(), 2);
        assert_eq!(array.get(2), None);

        array.clear();
        assert!(array.is_empty());
    }

    #[test]
    fn test_array_add_get_all_values() {
        let mut array = Array::new();

        // Float64
        array.push(3.0 as f64);
        assert_eq!(array.get(0), Some(&3.0.into()));

        // String
        array.push("value2");
        assert_eq!(array.get(1), Some(&"value2".into()));

        // Document
        let dummy_doc = Document::new().insert("key", "value");
        array.push(dummy_doc.unwrap());
        assert_eq!(array.get(2), Some(&dummy_doc.unwrap().into()));

        // Array
        let inner_array = Array::from_vec(vec![1.into(), "string".into(), dummy_doc.unwrap().into()]);
        array.push(inner_array.clone());
        assert_eq!(array.get(3), Some(&inner_array.into()));

        // Binary
        array.push(vec![0, 1, 2, 3]);
        assert_eq!(array.get(4), Some(&vec![0, 1, 2, 3].into()));

        // ObjectId
        let object_id = ObjectId::from("5e4f2f2d7f3d2d2d2d2d2d2d".to_string().as_ref());
        array.push(object_id);
        assert_eq!(array.get(5), Some(&object_id.into()));

        // Boolean
        array.push(true);
        assert_eq!(array.get(6), Some(&true.into()));

        // UTCDateTime
        let utc_date_time = UTCDateTime::from(1234567890 as i64);
        array.push(utc_date_time);
        assert_eq!(array.get(7), Some(&utc_date_time.into()));

        // Null
        array.push(Value::Null);
        assert_eq!(array.get(8), Some(&Value::Null));

        // RegularExpression
        array.push(Value::RegularExpression{pattern: "pattern".to_string(), options: "options".to_string()});
        assert_eq!(
            array.get(9),
            Some(&Value::RegularExpression{pattern: "pattern".to_string(), options: "options".to_string()})
        );

        // JavaScriptCode
        array.push("code");
        assert_eq!(array.get(10), Some(&"code".into()));

        // Int32
        array.push(32 as i32);
        assert_eq!(array.get(11), Some(&32.into()));

        // Timestamp
        let time_stamp = Timestamp::from(1234567890 as i64);
        array.push(time_stamp);
        assert_eq!(array.get(12), Some(&time_stamp.into()));

        // Int64
        array.push(64 as i64);
        assert_eq!(array.get(13), Some(&64.into()));

        // UInt64
        array.push(64 as u64);
        assert_eq!(array.get(14), Some(&(64 as u64).into()));

        // MinKey
        array.push(());
        assert_eq!(array.get(15), Some(&().into()));

        // MaxKey
        array.push(());
        assert_eq!(array.get(16), Some(&().into()));

        // JavaScriptCodeWithScope
        let current_scope = Document::new();
        array.push(("code".to_string(), current_scope.clone()));
        assert_eq!(
            array.get(17),
            Some(&("code".to_string(), current_scope.into()))
        );
    }

    // -------------------------------------
    //          ObjectId Tests
    // -------------------------------------

    #[test]
    fn test_object_id_new() {
        let object_id = ObjectId::new();
        assert_eq!(object_id.as_bytes().len(), 12);
    }

    #[test]
    fn test_object_id_from_bytes() {
        let bytes = [0; 12];
        let object_id = ObjectId::from_bytes(bytes);
        assert_eq!(*object_id.as_bytes(), bytes);
    }

    #[test]
    fn test_object_id_from_str() {
        let object_id_str: &str = "5e4f2f2d7f3d2d2d2d2d2d2d";
        let object_id = ObjectId::from(object_id_str);
        assert_eq!(object_id.into(), object_id_str);
    }

    #[test]
    fn test_object_id_into_string() {
        let object_id_str: &str = "5e4f2f2d7f3d2d2d2d2d2d2d";
        let object_id = ObjectId::from(object_id_str);
        assert_eq!(object_id.into(), object_id_str);
    }

    #[test]
    fn test_object_id_into_vec() {
        let object_id_str: &str = "5e4f2f2d7f3d2d2d2d2d2d2d";
        let object_id = ObjectId::from(object_id_str);
        assert_eq!(object_id.into(), hex::decode(object_id_str).unwrap());
    }

    // -------------------------------------
    //          UTCDateTime Tests
    // -------------------------------------

    #[test]
    fn test_utc_date_time_now() {
        let utc_date_time = UTCDateTime::now();
        assert!(utc_date_time.as_secs() > 0);
    }

    #[test]
    fn test_utc_date_time_from_secs() {
        let utc_date_time = UTCDateTime::from_secs(1234567890);
        assert_eq!(utc_date_time.as_secs(), 1234567890);
    }

    #[test]
    fn test_utc_date_time_from_i64() {
        let utc_date_time: i64 = 1234567890;
        let utc_date_time = UTCDateTime::from(utc_date_time);
        assert_eq!(utc_date_time.as_secs(), 1234567890);
    }

    #[test]
    fn test_utc_date_time_from_system_time() {
        let system_time = std::time::SystemTime::now();
        let utc_date_time = UTCDateTime::from(system_time);
        assert!(utc_date_time.as_secs() > 0);
    }

    #[test]
    fn test_utc_date_time_from_str() {
        let utc_date_time_str: &str = "1234567890";
        let utc_date_time = UTCDateTime::from(utc_date_time_str);
        assert_eq!(utc_date_time.as_secs(), 1234567890);
    }

    #[test]
    fn test_utc_date_time_into_i64() {
        let utc_date_time = UTCDateTime::from_secs(1234567890);
        assert_eq!(utc_date_time.into(), 1234567890);
    }

    #[test]
    fn test_utc_date_time_into_system_time() {
        let utc_date_time = UTCDateTime::from_secs(1234567890);
        let system_time: std::time::SystemTime = utc_date_time.into();
        assert_eq!(system_time, std::time::UNIX_EPOCH + std::time::Duration::from_secs(1234567890));
    }

    #[test]
    fn test_utc_date_time_into_string() {
        let utc_date_time = UTCDateTime::from_secs(1234567890);
        assert_eq!(utc_date_time.into(), "1234567890");
    }

    // -------------------------------------
    //          Timestamp Tests
    // -------------------------------------

    #[test]
    fn test_timestamp_now() {
        let time_stamp = Timestamp::now();
        assert!(time_stamp.as_secs() > 0);
    }

    #[test]
    fn test_timestamp_from_secs() {
        let time_stamp = Timestamp::from_secs(1234567890);
        assert_eq!(time_stamp.as_secs(), 1234567890);
    }

    #[test]
    fn test_timestamp_from_system_time() {
        let system_time = std::time::SystemTime::now();
        let time_stamp = Timestamp::from(system_time);
        assert!(time_stamp.as_secs() > 0);
    }

    #[test]
    fn test_timestamp_into_i64() {
        let time_stamp = Timestamp::from_secs(1234567890);
        assert_eq!(time_stamp.into(), 1234567890);
    }

    #[test]
    fn test_timestamp_into_system_time() {
        let time_stamp = Timestamp::from_secs(1234567890);
        let system_time: std::time::SystemTime = time_stamp.into();
        assert_eq!(system_time, std::time::UNIX_EPOCH + std::time::Duration::from_secs(1234567890));
    }

    #[test]
    fn test_timestamp_into_string() {
        let time_stamp = Timestamp::from_secs(1234567890);
        assert_eq!(time_stamp.into(), "1234567890");
    }

    // -------------------------------------
    //          Value Tests
    // -------------------------------------

    #[test]
    fn test_value_as_f64() {
        let value = 3.0;
        assert_eq!(value.into(), 3.0);
    }

    #[test]
    fn test_value_as_str() {
        let value = "string";
        assert_eq!(value.into(), "string");
    }

    #[test]
    fn test_value_as_i32() {
        let value = 32;
        assert_eq!(value.into(), 32);
    }

    #[test]
    fn test_value_as_i64() {
        let value = 64;
        assert_eq!(value.into(), 64);
    }

    #[test]
    fn test_value_as_document() {
        let value = Document::new();
        assert_eq!(value.into(), Document::new());
    }

    #[test]
    fn test_value_as_array() {
        let value = Array::new();
        assert_eq!(value.into(), Array::new());
    }

    // -------------------------------------
    //          Conversion Traits
    // -------------------------------------

    #[test]
    fn test_value_from_i32() {
        let value = 32;
        assert_eq!(value.into(), 32.into());
    }

    #[test]
    fn test_value_from_i64() {
        let value = 64;
        assert_eq!(value.into(), 64.into());
    }

    #[test]
    fn test_value_from_f64() {
        let value = 3.0;
        assert_eq!(value.into(), 3.0.into());
    }

    #[test]
    fn test_value_from_system_time() {
        let system_time = std::time::SystemTime::now();
        let value = UTCDateTime::from(system_time);
        assert_eq!(value.into(), UTCDateTime::from(system_time).into());
    }

    #[test]
    fn test_value_from_str() {
        let value = "string";
        assert_eq!(value.into(), "string".into());
    }

    #[test]
    fn test_value_into_string() {
        let value = "string";
        assert_eq!(value.into(), "string".to_string());
    }

    #[test]
    fn test_value_into_vec() {
        let value = vec![0, 1, 2, 3];
        assert_eq!(value.into(), vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_value_into_array() {
        let value = Array::new();
        assert_eq!(value.into(), Array::new());
    }

    #[test]
    fn test_value_into_document() {
        let value = Document::new();
        assert_eq!(value.into(), Document::new());
    }

    #[test]
    fn test_value_into_object_id() {
        let value = ObjectId::new();
        assert_eq!(value.into(), ObjectId::new());
    }

    #[test]
    fn test_value_into_utc_date_time() {
        let value = UTCDateTime::now();
        assert_eq!(value.into(), UTCDateTime::now());
    }

    #[test]
    fn test_value_into_timestamp() {
        let value = Timestamp::now();
        assert_eq!(value.into(), Timestamp::now());
    }

    #[test]
    fn test_value_into_i32() {
        let value = 32;
        assert_eq!(value.into(), 32);
    }

    #[test]
    fn test_value_into_i64() {
        let value = 64;
        assert_eq!(value.into(), 64);
    }

    #[test]
    fn test_value_into_u64() {
        let value = 64 as u64;
        assert_eq!(value.into(), 64 as u64);
    }

    #[test]
    fn test_value_into_min_key() {
        let value = ();
        assert_eq!(value.into(), ());
    }

    #[test]

    fn test_value_into_max_key() {
        let value = ();
        assert_eq!(value.into(), ());
    }

    #[test]
    fn test_value_into_regex() {
        let value = ("pattern".to_string(), "options".to_string());
        assert_eq!(value.into(), ("pattern".to_string(), "options".to_string()));
    }

    #[test]
    fn test_value_into_javascript_code() {
        let value = "code";
        assert_eq!(value.into(), "code");
    }

    #[test]
    fn test_value_into_javascript_code_with_scope() {
        let value = ("code".to_string(), Document::new());
        assert_eq!(value.into(), ("code".to_string(), Document::new()));
    }
}
