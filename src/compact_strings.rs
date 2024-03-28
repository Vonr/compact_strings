use core::{
    fmt::Debug,
    ops::{Deref, Index},
};

use crate::CompactBytestrings;

/// A more compact but limited representation of a list of strings.
///
/// Strings are stored contiguously in a vector of bytes, with their lengths and starting indices
/// being stored separately.
///
/// Limitations include being unable to mutate strings stored in the vector.
///
/// # Examples
/// ```
/// # use compact_strings::CompactStrings;
/// let mut cmpstrs = CompactStrings::with_capacity(20, 3);
///
/// cmpstrs.push("One");
/// cmpstrs.push("Two");
/// cmpstrs.push("Three");
///
/// cmpstrs.remove(1);
///
/// assert_eq!(cmpstrs.get(0), Some("One"));
/// assert_eq!(cmpstrs.get(1), Some("Three"));
/// assert_eq!(cmpstrs.get(2), None);
/// ```
#[repr(transparent)]
#[derive(Clone)]
pub struct CompactStrings(pub(crate) CompactBytestrings);

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
        Self(CompactBytestrings::new())
    }

    /// Constructs a new, empty [`CompactStrings`] with at least the specified capacities in each
    /// vector.
    ///
    /// - `data_capacity`: The capacity of the data vector where the bytes of the strings are stored.
    /// - `capacity_meta`: The capacity of the meta vector where the starting indices and lengths
    /// of the strings are stored.
    ///
    /// The [`CompactStrings`] will be able to hold at least *data_capacity* bytes worth of strings
    /// without reallocating the data vector, and at least *capacity_meta* of starting indices and
    /// lengths without reallocating the meta vector. This method is allowed to allocate for more bytes
    /// than the capacities. If a capacity is 0, the vector will not allocate.
    ///
    /// It is important to note that although the data and meta vectors have the
    /// minimum capacities specified, they will have a zero *length*.
    ///
    /// If it is important to know the exact allocated capacity of the data vector, always use the
    /// [`capacity`] method after construction.
    ///
    /// [`capacity`]: CompactStrings::capacity
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactStrings;
    /// let mut cmpstrs = CompactStrings::with_capacity(20, 3);
    ///
    /// assert_eq!(cmpstrs.len(), 0);
    /// assert!(cmpstrs.capacity() >= 20);
    /// assert!(cmpstrs.capacity_meta() >= 3);
    /// ```
    pub fn with_capacity(data_capacity: usize, capacity_meta: usize) -> Self {
        Self(CompactBytestrings::with_capacity(
            data_capacity,
            capacity_meta,
        ))
    }

    /// Appends a string to the back of the [`CompactStrings`].
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactStrings;
    /// let mut cmpstrs = CompactStrings::new();
    /// cmpstrs.push("One");
    /// cmpstrs.push("Two");
    /// cmpstrs.push("Three");
    ///
    /// assert_eq!(cmpstrs.get(0), Some("One"));
    /// assert_eq!(cmpstrs.get(1), Some("Two"));
    /// assert_eq!(cmpstrs.get(2), Some("Three"));
    /// assert_eq!(cmpstrs.get(3), None);
    /// ```
    pub fn push<S>(&mut self, string: S)
    where
        S: Deref<Target = str>,
    {
        self.0.push(string.as_bytes())
    }

    /// Returns a reference to the string stored in the [`CompactStrings`] at that position.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactStrings;
    /// let mut cmpstrs = CompactStrings::new();
    /// cmpstrs.push("One");
    /// cmpstrs.push("Two");
    /// cmpstrs.push("Three");
    ///
    /// assert_eq!(cmpstrs.get(0), Some("One"));
    /// assert_eq!(cmpstrs.get(1), Some("Two"));
    /// assert_eq!(cmpstrs.get(2), Some("Three"));
    /// assert_eq!(cmpstrs.get(3), None);
    /// ```
    pub fn get(&self, index: usize) -> Option<&str> {
        let bytes = self.0.get(index)?;
        if cfg!(feature = "no_unsafe") {
            core::str::from_utf8(bytes).ok()
        } else {
            unsafe { Some(core::str::from_utf8_unchecked(bytes)) }
        }
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
    /// cmpstrs.push("One");
    /// cmpstrs.push("Two");
    /// cmpstrs.push("Three");
    ///
    /// unsafe {
    ///     assert_eq!(cmpstrs.get_unchecked(0), "One");
    ///     assert_eq!(cmpstrs.get_unchecked(1), "Two");
    ///     assert_eq!(cmpstrs.get_unchecked(2), "Three");
    /// }
    /// ```
    #[cfg(not(feature = "no_unsafe"))]
    pub unsafe fn get_unchecked(&self, index: usize) -> &str {
        let bytes = self.0.get_unchecked(index);
        core::str::from_utf8_unchecked(bytes)
    }

    /// Returns the number of strings in the [`CompactStrings`], also referred to as its 'length'.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactStrings;
    /// let mut cmpstrs = CompactStrings::new();
    ///
    /// cmpstrs.push("One");
    /// cmpstrs.push("Two");
    /// cmpstrs.push("Three");
    ///
    /// assert_eq!(cmpstrs.len(), 3);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the [`CompactStrings`] contains no strings.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactStrings;
    /// let mut cmpstrs = CompactStrings::new();
    /// assert!(cmpstrs.is_empty());
    ///
    /// cmpstrs.push("One");
    ///
    /// assert!(!cmpstrs.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of bytes the data vector can store without reallocating.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactStrings;
    /// let mut cmpstrs = CompactStrings::with_capacity(20, 3);
    ///
    /// cmpstrs.push("One");
    ///
    /// assert!(cmpstrs.capacity() >= 20);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// Returns the number of starting indices and lengths can store without reallocating.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactStrings;
    /// let mut cmpstrs = CompactStrings::with_capacity(20, 3);
    ///
    /// cmpstrs.push("One");
    /// cmpstrs.push("Two");
    /// cmpstrs.push("Three");
    /// assert!(cmpstrs.capacity_meta() >= 3);
    ///
    /// cmpstrs.push("Three");
    /// assert!(cmpstrs.capacity_meta() > 3);
    /// ```
    #[inline]
    pub fn capacity_meta(&self) -> usize {
        self.0.capacity_meta()
    }

    /// Clears the [`CompactStrings`], removing all strings.
    ///
    /// Note that this method has no effect on the allocated capacity of the vectors.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactStrings;
    /// let mut cmpstrs = CompactStrings::new();
    ///
    /// cmpstrs.push("One");
    /// cmpstrs.push("Two");
    /// cmpstrs.push("Three");
    /// cmpstrs.clear();
    ///
    /// assert!(cmpstrs.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Shrinks the capacity of the data vector, which stores the bytes of the held strings, as much as possible.
    ///
    /// It will drop down as close as possible to the length but the allocator
    /// may still inform the vector that there is space for a few more elements.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactStrings;
    /// let mut cmpstrs = CompactStrings::with_capacity(20, 3);
    ///
    /// cmpstrs.push("One");
    /// cmpstrs.push("Two");
    /// cmpstrs.push("Three");
    ///
    /// assert!(cmpstrs.capacity() >= 20);
    /// cmpstrs.shrink_to_fit();
    /// assert!(cmpstrs.capacity() >= 3);
    /// ```
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.0.shrink_to_fit();
    }

    /// Shrinks the capacity of the info vector, which stores the starting indices and lengths of
    /// the held strings, as much as possible.
    ///
    /// It will drop down as close as possible to the length but the allocator
    /// may still inform the vector that there is space for a few more elements.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactStrings;
    /// let mut cmpstrs = CompactStrings::with_capacity(20, 10);
    ///
    /// cmpstrs.push("One");
    /// cmpstrs.push("Two");
    /// cmpstrs.push("Three");
    ///
    /// assert!(cmpstrs.capacity_meta() >= 10);
    /// cmpstrs.shrink_to_fit();
    /// assert!(cmpstrs.capacity_meta() >= 3);
    /// ```
    #[inline]
    pub fn shrink_meta_to_fit(&mut self) {
        self.0.shrink_meta_to_fit();
    }

    /// Shrinks the capacity of the data vector, which stores the bytes of the held strings, with a lower bound.
    ///
    /// The capacity will remain at least as large as both the length and the supplied value.
    ///
    /// If the current capacity is less than the lower limit, this is a no-op.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactStrings;
    /// let mut cmpstrs = CompactStrings::with_capacity(20, 4);
    ///
    /// cmpstrs.push("One");
    /// cmpstrs.push("Two");
    /// cmpstrs.push("Three");
    ///
    /// assert!(cmpstrs.capacity() >= 20);
    /// cmpstrs.shrink_to(4);
    /// assert!(cmpstrs.capacity() >= 4);
    /// ```
    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.0.shrink_to(min_capacity);
    }

    /// Shrinks the capacity of the meta vector, which starting indices and lengths of the held strings,
    /// with a lower bound.
    ///
    /// The capacity will remain at least as large as both the length and the supplied value.
    ///
    /// If the current capacity is less than the lower limit, this is a no-op.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactStrings;
    /// let mut cmpstrs = CompactStrings::with_capacity(20, 10);
    ///
    /// cmpstrs.push("One");
    /// cmpstrs.push("Two");
    /// cmpstrs.push("Three");
    ///
    /// assert!(cmpstrs.capacity_meta() >= 10);
    /// cmpstrs.shrink_meta_to(4);
    /// assert!(cmpstrs.capacity_meta() >= 4);
    /// ```
    #[inline]
    pub fn shrink_meta_to(&mut self, min_capacity: usize) {
        self.0.shrink_meta_to(min_capacity);
    }

    /// Removes the data pointing to where the string at the specified index is stored.
    ///
    /// Note: This does not remove the bytes of the string from memory, you may want to use
    /// [`remove`] if you desire that behavior.
    ///
    /// Note: Because this shifts over the remaining elements in the meta vector, it has a
    /// worst-case performance of *O*(*n*).
    ///
    /// [`remove`]: CompactStrings::remove
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactStrings;
    /// let mut cmpstrs = CompactStrings::with_capacity(20, 3);
    ///
    /// cmpstrs.push("One");
    /// cmpstrs.push("Two");
    /// cmpstrs.push("Three");
    ///
    /// cmpstrs.ignore(1);
    ///
    /// assert_eq!(cmpstrs.get(0), Some("One"));
    /// assert_eq!(cmpstrs.get(1), Some("Three"));
    /// assert_eq!(cmpstrs.get(2), None);
    /// ```
    pub fn ignore(&mut self, index: usize) {
        self.0.ignore(index);
    }

    /// Removes the bytes of the string and data pointing to the string is stored.
    ///
    /// Note: This does not shrink the vectors where the bytes of the string and data to the string
    /// are stored. You may shrink the data vector with [`shrink_to`] and [`shrink_to_fit`] and the
    /// meta vector with [`shrink_meta_to`] and [`shrink_meta_to_fit`].
    ///
    /// Note: Because this shifts over the remaining elements in both data and meta vectors, it
    /// has a worst-case performance of *O*(*n*). If you don't need the bytes of the string to
    /// be removed, use [`ignore`] instead.
    ///
    /// [`shrink_to`]: CompactStrings::shrink_to
    /// [`shrink_to_fit`]: CompactStrings::shrink_to_fit
    /// [`shrink_meta_to`]: CompactStrings::shrink_meta_to
    /// [`shrink_meta_to_fit`]: CompactStrings::shrink_meta_to_fit
    /// [`ignore`]: CompactStrings::ignore
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactStrings;
    /// let mut cmpstrs = CompactStrings::with_capacity(20, 3);
    ///
    /// cmpstrs.push("One");
    /// cmpstrs.push("Two");
    /// cmpstrs.push("Three");
    ///
    /// cmpstrs.remove(1);
    ///
    /// assert_eq!(cmpstrs.get(0), Some("One"));
    /// assert_eq!(cmpstrs.get(1), Some("Three"));
    /// assert_eq!(cmpstrs.get(2), None);
    /// ```
    pub fn remove(&mut self, index: usize) {
        self.0.remove(index)
    }

    /// Returns an iterator over the slice.
    ///
    /// The iterator yields all items from start to end.
    ///
    /// # Examples
    ///
    /// ```
    /// # use compact_strings::CompactStrings;
    /// let mut cmpstrs = CompactStrings::with_capacity(20, 3);
    /// cmpstrs.push("One");
    /// cmpstrs.push("Two");
    /// cmpstrs.push("Three");
    /// let mut iterator = cmpstrs.iter();
    ///
    /// assert_eq!(iterator.next(), Some("One"));
    /// assert_eq!(iterator.next(), Some("Two"));
    /// assert_eq!(iterator.next(), Some("Three"));
    /// assert_eq!(iterator.next(), None);
    /// ```
    #[inline]
    pub fn iter(&self) -> Iter<'_> {
        Iter(self.0.iter())
    }
}

impl PartialEq for CompactStrings {
    fn eq(&self, other: &Self) -> bool {
        let len = self.len();
        if len != other.len() {
            return false;
        }

        for idx in 0..len {
            if self[idx] != other[idx] {
                return false;
            }
        }

        true
    }
}

impl Debug for CompactStrings {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<S> Extend<S> for CompactStrings
where
    S: Deref<Target = str>,
{
    #[inline]
    fn extend<I: IntoIterator<Item = S>>(&mut self, iter: I) {
        for s in iter {
            self.push(s);
        }
    }
}

impl Index<usize> for CompactStrings {
    type Output = str;

    #[inline]
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
/// cmpstrs.push("One");
/// cmpstrs.push("Two");
/// cmpstrs.push("Three");
///
/// let mut iter = cmpstrs.into_iter();
/// assert_eq!(iter.next(), Some("One"));
/// assert_eq!(iter.next(), Some("Two"));
/// assert_eq!(iter.next(), Some("Three"));
/// assert_eq!(iter.next(), None);
/// ```
pub struct Iter<'a>(crate::compact_bytestrings::Iter<'a>);

impl<'a> Iter<'a> {
    pub fn new(inner: &'a CompactStrings) -> Self {
        Self(inner.0.iter())
    }

    fn from_utf8_maybe_checked(bytes: &[u8]) -> Option<&str> {
        if cfg!(feature = "no_unsafe") {
            core::str::from_utf8(bytes).ok()
        } else {
            Some(unsafe { core::str::from_utf8_unchecked(bytes) })
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().and_then(Self::from_utf8_maybe_checked)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a> DoubleEndedIterator for Iter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().and_then(Self::from_utf8_maybe_checked)
    }
}

impl ExactSizeIterator for Iter<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a> IntoIterator for &'a CompactStrings {
    type Item = &'a str;

    type IntoIter = Iter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<S> FromIterator<S> for CompactStrings
where
    S: Deref<Target = str>,
{
    fn from_iter<I: IntoIterator<Item = S>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let meta_capacity = match iter.size_hint() {
            (a, Some(b)) if a == b => a,
            _ => 0,
        };

        let mut out = CompactStrings::with_capacity(0, meta_capacity);
        for s in iter {
            out.push(s);
        }

        out
    }
}

impl<S, I> From<I> for CompactStrings
where
    S: Deref<Target = str>,
    I: IntoIterator<Item = S>,
{
    #[inline]
    fn from(value: I) -> Self {
        FromIterator::from_iter(value)
    }
}

impl TryFrom<CompactBytestrings> for CompactStrings {
    type Error = core::str::Utf8Error;

    fn try_from(value: CompactBytestrings) -> Result<Self, Self::Error> {
        for bstr in &value {
            let _ = core::str::from_utf8(bstr)?;
        }

        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use crate::CompactStrings;

    #[test]
    fn exact_size_iterator() {
        let mut cmpstrs = CompactStrings::new();

        cmpstrs.push("One");
        cmpstrs.push("Two");
        cmpstrs.push("Three");

        let mut iter = cmpstrs.iter();
        assert_eq!(iter.len(), 3);
        let _ = iter.next();
        assert_eq!(iter.len(), 2);
        let _ = iter.next();
        assert_eq!(iter.len(), 1);
        let _ = iter.next();
        assert_eq!(iter.len(), 0);
        let _ = iter.next();
        assert_eq!(iter.len(), 0);
    }

    #[test]
    fn double_ended_iterator() {
        let mut cmpbytes = CompactStrings::new();

        cmpbytes.push("One");
        cmpbytes.push("Two");
        cmpbytes.push("Three");
        cmpbytes.push("Four");

        let mut iter = cmpbytes.iter();
        assert_eq!(iter.next(), Some("One"));
        assert_eq!(iter.next_back(), Some("Four"));
        assert_eq!(iter.next(), Some("Two"));
        assert_eq!(iter.next_back(), Some("Three"));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }
}

#[cfg(feature = "serde")]
mod serde {
    use serde::{
        de::{SeqAccess, Visitor},
        Deserialize, Deserializer, Serialize,
    };

    use crate::CompactStrings;

    impl Serialize for CompactStrings {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serializer.collect_seq(self)
        }
    }

    impl<'de> Deserialize<'de> for CompactStrings {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_seq(CompactStringsVisitor)
        }
    }

    struct CompactStringsVisitor;

    impl<'de> Visitor<'de> for CompactStringsVisitor {
        type Value = CompactStrings;

        fn expecting(&self, formatter: &mut alloc::fmt::Formatter) -> alloc::fmt::Result {
            formatter.write_str("an array of strings")
        }

        #[inline]
        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut out = CompactStrings::with_capacity(0, seq.size_hint().unwrap_or_default());
            while let Some(str) = seq.next_element::<&str>()? {
                out.push(str);
            }

            Ok(out)
        }
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", allow(unused_imports))]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub use self::serde::*;
