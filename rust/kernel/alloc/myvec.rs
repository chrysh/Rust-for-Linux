use kernel::prelude::*;
use core::ops::Deref;
use core::ops::DerefMut;

/// A wrapper around `Vec<T>` that provides additional functionality.
#[derive(Debug)]
pub struct MyVec<T> (Vec<T>);

impl<T> DerefMut for MyVec<T> {
    /// Returns a mutable reference to the inner `Vec<T>`.
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Deref for MyVec<T> {
    type Target = Vec<T>;
    
    /// Returns an immutable reference to the inner `Vec<T>`.
    fn deref(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T> MyVec<T> {
    /// Creates a new empty `MyVec<T>`.
    pub fn new() -> Self {
        MyVec(Vec::new())
    }
    
    /// Creates a new `MyVec<T>` with the specified capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The desired capacity of the `MyVec<T>`.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - If the `MyVec<T>` is successfully created.
    /// * `Err(super::AllocError)` - If there is an allocation error.
    pub fn with_capacity(capacity: usize) -> Result<Self, super::AllocError> {
        let v = Vec::with_capacity(capacity, GFP_KERNEL);
        match v {
            Ok(v) => Ok(MyVec(v)),
            Err(e) => Err(e),
        }
    }
}

impl<T: Clone> Clone for MyVec<T> {
    /// Creates a new `MyVec<T>` with the same elements as the original.
    fn clone(&self) -> Self {
        let mut vec = Vec::new();
        vec.try_reserve(self.len()).unwrap();
        for elem in self.iter() {
            let _ = vec.push(elem.clone(), GFP_KERNEL);
        }
        MyVec(vec)
    }
}

impl<T> FromIterator<T> for MyVec<T> {
    #[inline]
    /// Creates a new `MyVec<T>` from an iterator.
    ///
    /// # Arguments
    ///
    /// * `iter` - An iterator that yields elements of type `T`.
    ///
    /// # Returns
    ///
    /// * `MyVec<T>` - The `MyVec<T>` containing the elements from the iterator.
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> MyVec<T> {
        let mut vec = Vec::new();
        for i in iter {
            let _ = vec.push(i, GFP_KERNEL);
        }
        MyVec(vec)
    }
}

impl<T: AsRef<[u8]>> MyVec<T> {
    /// Returns a byte slice representing the contents of the `MyVec<T>`.
    ///
    /// # Returns
    ///
    /// * `&[u8]` - A byte slice representing the contents of the `MyVec<T>`.
    pub fn as_bytes(&self) -> &[u8] {
        if let Some(first_item) = self.0.first() {
            let start_ptr = first_item.as_ref().as_ptr();
            let len = self.0.len() * core::mem::size_of::<T>();
            unsafe { core::slice::from_raw_parts(start_ptr, len) }
        } else {
            &[]
        }
    }
}