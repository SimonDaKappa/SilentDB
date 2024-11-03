use hex;

/// BSON object ID implementation.
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectId {
    inner: [u8; 12],
}

impl ObjectId {
    /// Creates a new `ObjectId` with a random value.
    pub fn new() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut inner = [0; 12];
        rng.fill(&mut inner);
        ObjectId { inner }
    }

    /// Creates a new `ObjectId` from the given bytes.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The bytes to create the `ObjectId` from. Must be exactly 12 bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// let bytes = [0; 12];
    /// let object_id = ObjectId::from_bytes(bytes);
    /// assert_eq!(object_id, ObjectId::new());
    /// ```
    pub fn from_bytes(bytes: [u8; 12]) -> Self {
        ObjectId { inner: bytes }
    }

    /// Returns the bytes of the `ObjectId`.
    pub fn as_bytes(&self) -> &[u8; 12] {
        &self.inner
    }
}

impl From<&str> for ObjectId {
    fn from(s: &str) -> Self {
        let bytes = hex::decode(s).unwrap();
        let mut inner = [0; 12];
        inner.copy_from_slice(&bytes);
        ObjectId { inner }
    }
}

impl Into<String> for ObjectId {
    fn into(self) -> String {
        hex::encode(&self.inner)
    }
}

impl Into<Vec<u8>> for ObjectId {
    fn into(self) -> Vec<u8> {
        self.inner.to_vec()
    }
}

impl Into<[u8; 12]> for ObjectId {
    fn into(self) -> [u8; 12] {
        self.inner
    }
}

impl std::fmt::Display for ObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(&self.inner))
    }
}