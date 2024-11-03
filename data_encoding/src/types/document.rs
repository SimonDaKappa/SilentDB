// src/types/document.rs
use std::{collections::HashMap, fmt};
use crate::types::Value;


#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    inner: HashMap<String, Value>,
}

impl Document {
    pub fn new() -> Self {
        Document {
            inner: HashMap::new()
        }
    }

    // TODO: Document methods. Initial framework done.
    
    /// Createsa document with capacity.
    pub fn new_with_capacity(capacity: usize) -> Self {
        Document {
            inner: HashMap::with_capacity(capacity)
        }
    }

    pub fn insert<K, V>(&mut self, key: K, value: V) -> Option<Value>
    where
        K: Into<String>,
        V: Into<Value>,
    {
        self.inner.insert(key.into(), value.into())
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.inner.get(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.inner.get_mut(key)
    }

    pub fn remove(&mut self, key: &str) -> Option<Value> {
        self.inner.remove(key)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.inner.contains_key(key)
    }

    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn clear(&mut self) {
        self.inner.clear()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Value)> {
        self.inner.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&String, &mut Value)> {
        self.inner.iter_mut()
    }
}

impl Default for Document {
    fn default() -> Self {
        Document::new()
    }
}

// Convert HashMap<String, Value> to Document
impl From<HashMap<String, Value>> for Document {
    fn from(inner: HashMap<String, Value>) -> Self {
        Document { inner }
    }
}

// Convert Document to HashMap<String, Value>
impl Into<HashMap<String, Value>> for Document {
    fn into(self) -> HashMap<String, Value> {
        self.inner
    }
}

impl fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for (i, (key, value)) in self.inner.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "\"{}\": {}", key, value)?;
        }
        write!(f, "}}")
    }
}
