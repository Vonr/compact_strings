//! Implements a cache-friendly but limited representation of a list of strings.
//!
//! Strings are stored contiguously in a vector of bytes, with their lengths and starting indices
//! being stored separately.
//!
//! Limitations include being unable to mutate strings stored in the vector.
//!
//! # Examples
//! ```
//! # use compact_strings::CompactStrings;
//! let mut cmpstrs = CompactStrings::with_capacity(10);
//!
//! cmpstrs.push("One".to_string());
//! cmpstrs.push("Two".to_string());
//! cmpstrs.push("Three".to_string());
//!
//! cmpstrs.remove(1);
//!
//! assert_eq!(cmpstrs.get(0), Some("One"));
//! assert_eq!(cmpstrs.get(1), Some("Three"));
//! assert_eq!(cmpstrs.get(2), None);
//! ```

use std::ops::Index;

/// A cache-friendly but limited representation of a list of strings.
///
/// Strings are stored contiguously in a vector of bytes, with their lengths and starting indices
/// being stored separately.
///
/// Limitations include being unable to mutate strings stored in the vector.
///
/// # Examples
/// ```
/// # use compact_strings::CompactStrings;
/// let mut cmpstrs = CompactStrings::with_capacity(10);
///
/// cmpstrs.push("One".to_string());
/// cmpstrs.push("Two".to_string());
/// cmpstrs.push("Three".to_string());
///
/// cmpstrs.remove(1);
///
/// assert_eq!(cmpstrs.get(0), Some("One"));
/// assert_eq!(cmpstrs.get(1), Some("Three"));
/// assert_eq!(cmpstrs.get(2), None);
/// ```
pub struct CompactStrings {
    inner: Vec<u8>,
    strings: Vec<(usize, usize)>,
}

impl CompactStrings {
    /// Constructs a new, empty [`CompactStrings`].
    ///
    /// The [`CompactStrings`] will not allocate until strings are pushed into it.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactStrings;
    /// let mut cmpstrs = CompactStrings::new();
    /// ```
    pub const fn new() -> Self {
        Self {
            inner: Vec::new(),
            strings: Vec::new(),
        }
    }

    /// Constructs a new, empty [`CompactStrings`] with at least the specified capacity in the inner
    /// vector where the bytes of the strings are stored.
    ///
    /// Note that this does not affect the indices and lengths vectors which store information
    /// about where each string is stored.
    ///
    /// The [`CompactStrings`] will be able to hold at least *capacity* bytes worth of strings
    /// without reallocating the inner vector. This method is allowed to allocate for more bytes
    /// than *capacity*. If *capacity* is 0, the inner vector will not allocate.
    ///
    /// It is important to note that although the returned vector has the
    /// minimum *capacity* specified, the inner vector will have a zero *length*.
    ///
    /// If it is important to know the exact allocated capacity of the inner vector, always use the
    /// [`capacity`] method after construction.
    ///
    /// [`capacity`]: CompactStrings::capacity
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactStrings;
    /// let mut cmpstrs = CompactStrings::with_capacity(10);
    ///
    /// assert_eq!(cmpstrs.len(), 0);
    /// assert!(cmpstrs.capacity() >= 10);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
            strings: Vec::new(),
        }
    }

    /// Appends a string to the back of the [`CompactStrings`].
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactStrings;
    /// let mut cmpstrs = CompactStrings::new();
    /// cmpstrs.push("One".to_string());
    /// cmpstrs.push("Two".to_string());
    /// cmpstrs.push("Three".to_string());
    ///
    /// assert_eq!(cmpstrs.get(0), Some("One"));
    /// assert_eq!(cmpstrs.get(1), Some("Two"));
    /// assert_eq!(cmpstrs.get(2), Some("Three"));
    /// assert_eq!(cmpstrs.get(3), None);
    /// ```
    pub fn push(&mut self, string: String) {
        let bytes = string.into_bytes();
        self.strings.push((self.inner.len(), bytes.len()));
        self.inner.extend_from_slice(&bytes);
    }

    /// Returns a reference to the string stored in the [`CompactStrings`] at that position.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactStrings;
    /// let mut cmpstrs = CompactStrings::new();
    /// cmpstrs.push("One".to_string());
    /// cmpstrs.push("Two".to_string());
    /// cmpstrs.push("Three".to_string());
    ///
    /// assert_eq!(cmpstrs.get(0), Some("One"));
    /// assert_eq!(cmpstrs.get(1), Some("Two"));
    /// assert_eq!(cmpstrs.get(2), Some("Three"));
    /// assert_eq!(cmpstrs.get(3), None);
    /// ```
    pub fn get(&self, index: usize) -> Option<&str> {
        let (start, len) = *self.strings.get(index)?;
        let bytes = self.inner.get(start..start + len)?;
        unsafe { Some(std::str::from_utf8_unchecked(bytes)) }
    }

    /// Returns a reference to the string stored in the [`CompactStrings`] at that position, without
    /// doing bounds checking.
    ///
    /// # Safety
    /// Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactStrings;
    /// let mut cmpstrs = CompactStrings::new();
    /// cmpstrs.push("One".to_string());
    /// cmpstrs.push("Two".to_string());
    /// cmpstrs.push("Three".to_string());
    ///
    /// unsafe {
    ///     assert_eq!(cmpstrs.get_unchecked(0), "One");
    ///     assert_eq!(cmpstrs.get_unchecked(1), "Two");
    ///     assert_eq!(cmpstrs.get_unchecked(2), "Three");
    /// }
    /// ```
    pub unsafe fn get_unchecked(&self, index: usize) -> &str {
        let (start, len) = *self.strings.get_unchecked(index);
        let bytes = self.inner.get_unchecked(start..start + len);
        std::str::from_utf8_unchecked(bytes)
    }

    /// Returns the number of strings in the [`CompactStrings`], also referred to as its 'length'.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactStrings;
    /// let mut cmpstrs = CompactStrings::new();
    ///
    /// cmpstrs.push("One".to_string());
    /// cmpstrs.push("Two".to_string());
    /// cmpstrs.push("Three".to_string());
    ///
    /// assert_eq!(cmpstrs.len(), 3);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.strings.len()
    }

    /// Returns true if the [`CompactStrings`] contains no strings.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactStrings;
    /// let mut cmpstrs = CompactStrings::new();
    /// assert!(cmpstrs.is_empty());
    ///
    /// cmpstrs.push("One".to_string());
    ///
    /// assert!(!cmpstrs.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of bytes the inner vector can store without reallocating.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactStrings;
    /// let mut cmpstrs = CompactStrings::with_capacity(10);
    ///
    /// cmpstrs.push("One".to_string());
    ///
    /// assert!(cmpstrs.capacity() >= 10);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Clears the [`CompactStrings`], removing all strings.
    ///
    /// Note that this method has no effect on the allocated capacity of the vector.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactStrings;
    /// let mut cmpstrs = CompactStrings::new();
    ///
    /// cmpstrs.push("One".to_string());
    /// cmpstrs.push("Two".to_string());
    /// cmpstrs.push("Three".to_string());
    /// cmpstrs.clear();
    ///
    /// assert!(cmpstrs.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.inner.clear();
        self.strings.clear();
    }

    /// Shrinks the capacity of the inner vector, which stores the bytes of the held strings, as much as possible.
    ///
    /// It will drop down as close as possible to the length but the allocator
    /// may still inform the vector that there is space for a few more elements.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactStrings;
    /// let mut cmpstrs = CompactStrings::with_capacity(10);
    ///
    /// cmpstrs.push("One".to_string());
    /// cmpstrs.push("Two".to_string());
    /// cmpstrs.push("Three".to_string());
    ///
    /// assert!(cmpstrs.capacity() >= 10);
    /// cmpstrs.shrink_to_fit();
    /// assert!(cmpstrs.capacity() >= 3);
    /// ```
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit();
    }

    /// Shrinks the capacity of the inner vector, which stores the bytes of the held strings, with a lower bound.
    ///
    /// The capacity will remain at least as large as both the length and the supplied value.
    ///
    /// If the current capacity is less than the lower limit, this is a no-op.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactStrings;
    /// let mut cmpstrs = CompactStrings::with_capacity(10);
    ///
    /// cmpstrs.push("One".to_string());
    /// cmpstrs.push("Two".to_string());
    /// cmpstrs.push("Three".to_string());
    ///
    /// assert!(cmpstrs.capacity() >= 10);
    /// cmpstrs.shrink_to(4);
    /// assert!(cmpstrs.capacity() >= 4);
    /// ```
    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.inner.shrink_to(min_capacity);
    }

    /// Removes the data pointing to where the string at the specified index is stored.
    ///
    /// Note that this does not remove the bytes of the string from memory, you may want to use
    /// [`remove_full`] if you desire that behavior.
    ///
    /// [`remove_full`]: CompactStrings::remove_full
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactStrings;
    /// let mut cmpstrs = CompactStrings::with_capacity(10);
    ///
    /// cmpstrs.push("One".to_string());
    /// cmpstrs.push("Two".to_string());
    /// cmpstrs.push("Three".to_string());
    ///
    /// cmpstrs.remove(1);
    ///
    /// assert_eq!(cmpstrs.get(0), Some("One"));
    /// assert_eq!(cmpstrs.get(1), Some("Three"));
    /// assert_eq!(cmpstrs.get(2), None);
    /// ```
    pub fn remove(&mut self, index: usize) {
        assert!(self.len() > index);
        self.strings.remove(index);
    }

    /// Removes the data pointing to where the string at the specified index is stored.
    ///
    /// Note that this also removes the bytes of the string from memory, which requires all bytes
    /// after the string to be shifted into the empty space, you may want to use [`remove`] if you
    /// do not desire that behavior.
    ///
    /// [`remove`]: CompactStrings::remove
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactStrings;
    /// let mut cmpstrs = CompactStrings::with_capacity(10);
    ///
    /// cmpstrs.push("One".to_string());
    /// cmpstrs.push("Two".to_string());
    /// cmpstrs.push("Three".to_string());
    ///
    /// cmpstrs.remove_full(1);
    ///
    /// assert_eq!(cmpstrs.get(0), Some("One"));
    /// assert_eq!(cmpstrs.get(1), Some("Three"));
    /// assert_eq!(cmpstrs.get(2), None);
    /// ```
    pub fn remove_full(&mut self, index: usize) {
        assert!(self.len() > index);
        let (start, len) = self.strings.remove(index);
        let inner_len = self.inner.len();
        for (idx, _) in self.strings.iter_mut().skip(index) {
            *idx -= len;
        }
        self.inner.copy_within(start + len.., start);
        self.inner.truncate(inner_len - len);
    }
}

impl Index<usize> for CompactStrings {
    type Output = str;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

/// Iterator over strings in a [`CompactStrings`]
///
/// # Examples
/// ```
/// # use compact_strings::CompactStrings;
/// let mut cmpstrs = CompactStrings::new();
/// cmpstrs.push("One".to_string());
/// cmpstrs.push("Two".to_string());
/// cmpstrs.push("Three".to_string());
///
/// let mut iter = cmpstrs.into_iter();
/// assert_eq!(iter.next(), Some("One"));
/// assert_eq!(iter.next(), Some("Two"));
/// assert_eq!(iter.next(), Some("Three"));
/// assert_eq!(iter.next(), None);
/// ```
pub struct CompactStringIterator<'a> {
    inner: &'a CompactStrings,
    index: usize,
}

impl<'a> Iterator for CompactStringIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.inner.get(self.index);
        self.index += 1;

        out
    }
}

impl<'a> IntoIterator for &'a CompactStrings {
    type Item = &'a str;

    type IntoIter = CompactStringIterator<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            inner: self,
            index: 0,
        }
    }
}

impl ExactSizeIterator for CompactStringIterator<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.strings.len()
    }
}
