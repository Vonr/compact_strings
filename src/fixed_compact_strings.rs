use core::{
    fmt::Debug,
    ops::{Deref, Index},
};

use crate::FixedCompactBytestrings;

/// An even more compact but limited representation of a list of strings.
///
/// Strings are stored contiguously in a vector of bytes, with their starting indices
/// being stored separately.
///
/// Limitations include being unable to mutate strings stored in the vector.
///
/// # Examples
/// ```
/// # use compact_strings::FixedCompactStrings;
/// let mut cmpstrs = FixedCompactStrings::with_capacity(20, 3);
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
pub struct FixedCompactStrings(pub(crate) FixedCompactBytestrings);

impl FixedCompactStrings {
    /// Constructs a new, empty [`FixedCompactStrings`].
    ///
    /// The [`FixedCompactStrings`] will not allocate until strings are pushed into it.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::FixedCompactStrings;
    /// let mut cmpstrs = FixedCompactStrings::new();
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self(FixedCompactBytestrings::new())
    }

    /// Constructs a new, empty [`FixedCompactStrings`] with at least the specified capacities in each
    /// vector.
    ///
    /// - `data_capacity`: The capacity of the data vector where the bytes of the strings are stored.
    /// - `capacity_meta`: The capacity of the meta vector where the starting indices
    /// of the strings are stored.
    ///
    /// The [`FixedCompactStrings`] will be able to hold at least *`data_capacity`* bytes worth of strings
    /// without reallocating the data vector, and at least *`capacity_meta`* of starting indices
    /// without reallocating the meta vector. This method is allowed to allocate for more bytes
    /// than the capacities. If a capacity is 0, the vector will not allocate.
    ///
    /// It is important to note that although the data and meta vectors have the
    /// minimum capacities specified, they will have a zero *length*.
    ///
    /// If it is important to know the exact allocated capacity of the data vector, always use the
    /// [`capacity`] method after construction.
    ///
    /// [`capacity`]: FixedCompactStrings::capacity
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::FixedCompactStrings;
    /// let mut cmpstrs = FixedCompactStrings::with_capacity(20, 3);
    ///
    /// assert_eq!(cmpstrs.len(), 0);
    /// assert!(cmpstrs.capacity() >= 20);
    /// assert!(cmpstrs.capacity_meta() >= 3);
    /// ```
    #[must_use]
    pub fn with_capacity(data_capacity: usize, capacity_meta: usize) -> Self {
        Self(FixedCompactBytestrings::with_capacity(
            data_capacity,
            capacity_meta,
        ))
    }

    /// Appends a string to the back of the [`FixedCompactStrings`].
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::FixedCompactStrings;
    /// let mut cmpstrs = FixedCompactStrings::new();
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
        self.0.push(string.as_bytes());
    }

    /// Returns a reference to the string stored in the [`FixedCompactStrings`] at that position.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::FixedCompactStrings;
    /// let mut cmpstrs = FixedCompactStrings::new();
    /// cmpstrs.push("One");
    /// cmpstrs.push("Two");
    /// cmpstrs.push("Three");
    ///
    /// assert_eq!(cmpstrs.get(0), Some("One"));
    /// assert_eq!(cmpstrs.get(1), Some("Two"));
    /// assert_eq!(cmpstrs.get(2), Some("Three"));
    /// assert_eq!(cmpstrs.get(3), None);
    /// ```
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&str> {
        let bytes = self.0.get(index)?;
        if cfg!(feature = "no_unsafe") {
            core::str::from_utf8(bytes).ok()
        } else {
            unsafe { Some(core::str::from_utf8_unchecked(bytes)) }
        }
    }

    /// Returns a reference to the string stored in the [`FixedCompactStrings`] at that position, without
    /// doing bounds checking.
    ///
    /// # Safety
    /// Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::FixedCompactStrings;
    /// let mut cmpstrs = FixedCompactStrings::new();
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
    #[must_use]
    #[cfg(not(feature = "no_unsafe"))]
    pub unsafe fn get_unchecked(&self, index: usize) -> &str {
        let bytes = self.0.get_unchecked(index);
        core::str::from_utf8_unchecked(bytes)
    }

    /// Returns the number of strings in the [`FixedCompactStrings`], also referred to as its 'length'.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::FixedCompactStrings;
    /// let mut cmpstrs = FixedCompactStrings::new();
    ///
    /// cmpstrs.push("One");
    /// cmpstrs.push("Two");
    /// cmpstrs.push("Three");
    ///
    /// assert_eq!(cmpstrs.len(), 3);
    /// ```
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the [`FixedCompactStrings`] contains no strings.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::FixedCompactStrings;
    /// let mut cmpstrs = FixedCompactStrings::new();
    /// assert!(cmpstrs.is_empty());
    ///
    /// cmpstrs.push("One");
    ///
    /// assert!(!cmpstrs.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of bytes the data vector can store without reallocating.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::FixedCompactStrings;
    /// let mut cmpstrs = FixedCompactStrings::with_capacity(20, 3);
    ///
    /// cmpstrs.push("One");
    ///
    /// assert!(cmpstrs.capacity() >= 20);
    /// ```
    #[inline]
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// Returns the number of starting indices can store without reallocating.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::FixedCompactStrings;
    /// let mut cmpstrs = FixedCompactStrings::with_capacity(20, 3);
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
    #[must_use]
    pub fn capacity_meta(&self) -> usize {
        self.0.capacity_meta()
    }

    /// Clears the [`FixedCompactStrings`], removing all strings.
    ///
    /// Note that this method has no effect on the allocated capacity of the vectors.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::FixedCompactStrings;
    /// let mut cmpstrs = FixedCompactStrings::new();
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
    /// # use compact_strings::FixedCompactStrings;
    /// let mut cmpstrs = FixedCompactStrings::with_capacity(20, 3);
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

    /// Shrinks the capacity of the info vector, which stores the starting indices of
    /// the held strings, as much as possible.
    ///
    /// It will drop down as close as possible to the length but the allocator
    /// may still inform the vector that there is space for a few more elements.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::FixedCompactStrings;
    /// let mut cmpstrs = FixedCompactStrings::with_capacity(20, 10);
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
    /// # use compact_strings::FixedCompactStrings;
    /// let mut cmpstrs = FixedCompactStrings::with_capacity(20, 4);
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

    /// Shrinks the capacity of the meta vector, which starting indices of the held strings,
    /// with a lower bound.
    ///
    /// The capacity will remain at least as large as both the length and the supplied value.
    ///
    /// If the current capacity is less than the lower limit, this is a no-op.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::FixedCompactStrings;
    /// let mut cmpstrs = FixedCompactStrings::with_capacity(20, 10);
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

    /// Removes the bytes of the string and data pointing to the string is stored.
    ///
    /// Note: This does not shrink the vectors where the bytes of the string and data to the string
    /// are stored. You may shrink the data vector with [`shrink_to`] and [`shrink_to_fit`] and the
    /// meta vector with [`shrink_meta_to`] and [`shrink_meta_to_fit`].
    ///
    /// Note: Because this shifts over the remaining elements in both data and meta vectors, it
    /// has a worst-case performance of *O*(*n*).
    ///
    /// [`shrink_to`]: FixedCompactStrings::shrink_to
    /// [`shrink_to_fit`]: FixedCompactStrings::shrink_to_fit
    /// [`shrink_meta_to`]: FixedCompactStrings::shrink_meta_to
    /// [`shrink_meta_to_fit`]: FixedCompactStrings::shrink_meta_to_fit
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::FixedCompactStrings;
    /// let mut cmpstrs = FixedCompactStrings::with_capacity(20, 3);
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
        self.0.remove(index);
    }

    /// Returns an iterator over the slice.
    ///
    /// The iterator yields all items from start to end.
    ///
    /// # Examples
    ///
    /// ```
    /// # use compact_strings::FixedCompactStrings;
    /// let mut cmpstrs = FixedCompactStrings::with_capacity(20, 3);
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
    #[must_use]
    pub fn iter(&self) -> Iter<'_> {
        Iter(self.0.iter())
    }
}

impl PartialEq for FixedCompactStrings {
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

impl Debug for FixedCompactStrings {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<S> Extend<S> for FixedCompactStrings
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

impl Index<usize> for FixedCompactStrings {
    type Output = str;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

/// Iterator over strings in a [`FixedCompactStrings`]
///
/// # Examples
/// ```
/// # use compact_strings::FixedCompactStrings;
/// let mut cmpstrs = FixedCompactStrings::new();
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
pub struct Iter<'a>(crate::fixed_compact_bytestrings::Iter<'a>);

impl<'a> Iter<'a> {
    pub fn new(inner: &'a FixedCompactStrings) -> Self {
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

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth(n).and_then(Self::from_utf8_maybe_checked)
    }

    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.len()
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.0.next_back().and_then(Self::from_utf8_maybe_checked)
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

impl<'a> IntoIterator for &'a FixedCompactStrings {
    type Item = &'a str;

    type IntoIter = Iter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<S> FromIterator<S> for FixedCompactStrings
where
    S: Deref<Target = str>,
{
    fn from_iter<I: IntoIterator<Item = S>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let meta_capacity = match iter.size_hint() {
            (a, Some(b)) if a == b => a,
            _ => 0,
        };

        let mut out = FixedCompactStrings::with_capacity(0, meta_capacity);
        for s in iter {
            out.push(s);
        }

        out
    }
}

impl<S, I> From<I> for FixedCompactStrings
where
    S: Deref<Target = str>,
    I: IntoIterator<Item = S>,
{
    #[inline]
    fn from(value: I) -> Self {
        FromIterator::from_iter(value)
    }
}

impl TryFrom<FixedCompactBytestrings> for FixedCompactStrings {
    type Error = core::str::Utf8Error;

    fn try_from(value: FixedCompactBytestrings) -> Result<Self, Self::Error> {
        for bstr in &value {
            let _ = core::str::from_utf8(bstr)?;
        }

        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use crate::FixedCompactStrings;

    #[test]
    fn exact_size_iterator() {
        let mut cmpstrs = FixedCompactStrings::new();

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
        let mut cmpbytes = FixedCompactStrings::new();

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

    use crate::FixedCompactStrings;

    impl Serialize for FixedCompactStrings {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serializer.collect_seq(self)
        }
    }

    impl<'de> Deserialize<'de> for FixedCompactStrings {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_seq(FixedCompactStringsVisitor)
        }
    }

    struct FixedCompactStringsVisitor;

    impl<'de> Visitor<'de> for FixedCompactStringsVisitor {
        type Value = FixedCompactStrings;

        fn expecting(&self, formatter: &mut alloc::fmt::Formatter) -> alloc::fmt::Result {
            formatter.write_str("an array of strings")
        }

        #[inline]
        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut out =
                FixedCompactStrings::with_capacity(0, seq.size_hint().unwrap_or_default());
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
