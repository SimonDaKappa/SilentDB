/// src/types/array.rs
use crate::types::Value;


/// Represents a BSON array.
#[derive(Debug, Clone, PartialEq)]
pub struct Array {
    inner: Vec<Value>,
}


impl Array {
    /// Creates a new, empty `Array`.
    ///
    /// # Examples
    ///
    /// ```
    /// let array = Array::new();
    /// assert!(array.is_empty());
    /// ```
    pub fn new() -> Self {
        Array { inner: Vec::new() }
    }

    /// Creates a new `Array` with the specified capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The number of elements the array can hold without reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// let array = Array::with_capacity(10);
    /// assert!(array.capacity() >= 10);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Array {
            inner: Vec::with_capacity(capacity),
        }
    }

    /// Creates a new `Array` from the given vector of values.
    /// 
    /// # Arguments
    /// 
    /// * `vec` - The vector of values to create the array from.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let vec = vec![Value::from(1), Value::from(2)];
    /// let array = Array::from_vec(vec);
    /// assert_eq!(array.len(), 2);
    /// ```
    pub fn from_vec(vec: Vec<Value>) -> Self {
        Array { inner: vec }
    }

    /// Adds a value to the end of the array.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to add to the array. It must implement the `Into<Value>` trait.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut array = Array::new();
    /// array.push(1);
    /// assert_eq!(array.len(), 1);
    /// ```
    pub fn push<V>(&mut self, value: V)
    where
        V: Into<Value>,
    {
        self.inner.push(value.into());
    }

    /// Removes and returns the last element of the array, or `None` if it is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut array = Array::new();
    /// array.push(1);
    /// let value = array.pop();
    /// assert_eq!(value, Some(Value::from(1)));
    /// assert!(array.is_empty());
    /// ```
    pub fn pop(&mut self) -> Option<Value> {
        self.inner.pop()
    }

    /// Returns the element at the specified index, or `None` if the index is out of bounds.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut array = Array::new();
    /// array.push(1);
    /// assert_eq!(array.get(0), Some(&Value::from(1)));
    /// assert_eq!(array.get(1), None);
    /// ```
    pub fn get(&self, index: usize) -> Option<&Value> {
        self.inner.get(index)
    }

    /// Returns a mutable reference to the element at the specified index, or `None` if the index is out of bounds.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut array = Array::new();
    /// array.push(1);
    /// if let Some(value) = array.get_mut(0) {
    ///     *value = Value::from(2);
    /// }
    /// ```
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Value> {
        self.inner.get_mut(index)
    }

    /// Returns the number of elements in the array.
    ///
    /// # Examples
    ///
    /// ```
    /// let array = Array::new();
    /// assert_eq!(array.len(), 0);
    /// ```
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns `true` if the array contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// let array = Array::new();
    /// assert!(array.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Removes all elements from the array.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut array = Array::new();
    /// array.push(1);
    /// array.clear();
    /// assert!(array.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// Returns an iterator over the elements of the array.
    ///
    /// # Examples
    ///
    /// ```
    /// let array = Array::new();
    /// for value in array.iter() {
    ///     println!("{:?}", value);
    /// }
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = &Value> {
        self.inner.iter()
    }

    /// Returns a mutable iterator over the elements of the array.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut array = Array::new();
    /// for value in array.iter_mut() {
    ///     *value = Value::from(2);
    /// }
    /// ```
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Value> {
        self.inner.iter_mut()
    }
}

impl Default for Array {
    fn default() -> Self {
        Array::new()
    }
}

impl From<Vec<Value>> for Array {
    fn from(vec: Vec<Value>) -> Self {
        Array::from_vec(vec)
    }
}

impl Into<Vec<Value>> for Array {
    fn into(self) -> Vec<Value> {
        self.inner
    }
}
