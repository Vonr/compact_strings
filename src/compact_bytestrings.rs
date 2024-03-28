use core::{fmt::Debug, ops::Index};

use alloc::vec::Vec;

use crate::{metadata::Metadata, CompactStrings};

/// A more compact but limited representation of a list of bytestrings.
///
/// Strings are stored contiguously in a vector of bytes, with their lengths and starting indices
/// being stored separately.
///
/// Limitations include being unable to mutate bytestrings stored in the vector.
///
/// # Examples
/// ```
/// # use compact_strings::CompactBytestrings;
/// let mut cmpbytes = CompactBytestrings::with_capacity(20, 3);
///
/// cmpbytes.push(b"One");
/// cmpbytes.push(b"Two");
/// cmpbytes.push(b"Three");
///
/// cmpbytes.remove(1);
///
/// assert_eq!(cmpbytes.get(0), Some(b"One".as_slice()));
/// assert_eq!(cmpbytes.get(1), Some(b"Three".as_slice()));
/// assert_eq!(cmpbytes.get(2), None);
/// ```
pub struct CompactBytestrings {
    pub(crate) data: Vec<u8>,
    pub(crate) meta: Vec<Metadata>,
}

impl CompactBytestrings {
    /// Constructs a new, empty [`CompactBytestrings`].
    ///
    /// The [`CompactBytestrings`] will not allocate until bytestrings are pushed into it.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactBytestrings;
    /// let mut cmpbytes = CompactBytestrings::new();
    /// ```
    pub const fn new() -> Self {
        Self {
            data: Vec::new(),
            meta: Vec::new(),
        }
    }

    /// Constructs a new, empty [`CompactBytestrings`] with at least the specified capacities in each
    /// vector.
    ///
    /// - `data_capacity`: The capacity of the data vector where the bytes of the bytestrings are stored.
    /// - `capacity_meta`: The capacity of the meta vector where the starting indices and lengths
    /// of the bytestrings are stored.
    ///
    /// The [`CompactBytestrings`] will be able to hold at least *data_capacity* bytes worth of bytestrings
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
    /// [`capacity`]: CompactBytestrings::capacity
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactBytestrings;
    /// let mut cmpbytes = CompactBytestrings::with_capacity(20, 3);
    ///
    /// assert_eq!(cmpbytes.len(), 0);
    /// assert!(cmpbytes.capacity() >= 20);
    /// assert!(cmpbytes.capacity_meta() >= 3);
    /// ```
    pub fn with_capacity(data_capacity: usize, capacity_meta: usize) -> Self {
        Self {
            data: Vec::with_capacity(data_capacity),
            meta: Vec::with_capacity(capacity_meta),
        }
    }

    /// Appends a bytestring to the back of the [`CompactBytestrings`].
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactBytestrings;
    /// let mut cmpbytes = CompactBytestrings::new();
    /// cmpbytes.push(b"One");
    /// cmpbytes.push(b"Two");
    /// cmpbytes.push(b"Three");
    ///
    /// assert_eq!(cmpbytes.get(0), Some(b"One".as_slice()));
    /// assert_eq!(cmpbytes.get(1), Some(b"Two".as_slice()));
    /// assert_eq!(cmpbytes.get(2), Some(b"Three".as_slice()));
    /// assert_eq!(cmpbytes.get(3), None);
    /// ```
    pub fn push<S>(&mut self, bytestring: S)
    where
        S: AsRef<[u8]>,
    {
        let bytestr = bytestring.as_ref();
        self.meta
            .push(Metadata::new(self.data.len(), bytestr.len()));
        self.data.extend_from_slice(bytestr);
    }

    /// Returns a reference to the bytestring stored in the [`CompactBytestrings`] at that position.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactBytestrings;
    /// let mut cmpbytes = CompactBytestrings::new();
    /// cmpbytes.push(b"One");
    /// cmpbytes.push(b"Two");
    /// cmpbytes.push(b"Three");
    ///
    /// assert_eq!(cmpbytes.get(0), Some(b"One".as_slice()));
    /// assert_eq!(cmpbytes.get(1), Some(b"Two".as_slice()));
    /// assert_eq!(cmpbytes.get(2), Some(b"Three".as_slice()));
    /// assert_eq!(cmpbytes.get(3), None);
    /// ```
    pub fn get(&self, index: usize) -> Option<&[u8]> {
        let (start, len) = self.meta.get(index)?.as_tuple();
        if cfg!(feature = "no_unsafe") {
            self.data.get(start..start + len)
        } else {
            unsafe { Some(self.data.get_unchecked(start..start + len)) }
        }
    }

    /// Returns a reference to the bytestring stored in the [`CompactBytestrings`] at that position, without
    /// doing bounds checking.
    ///
    /// # Safety
    /// Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactBytestrings;
    /// let mut cmpbytes = CompactBytestrings::new();
    /// cmpbytes.push(b"One");
    /// cmpbytes.push(b"Two");
    /// cmpbytes.push(b"Three");
    ///
    /// unsafe {
    ///     assert_eq!(cmpbytes.get_unchecked(0), b"One".as_slice());
    ///     assert_eq!(cmpbytes.get_unchecked(1), b"Two".as_slice());
    ///     assert_eq!(cmpbytes.get_unchecked(2), b"Three".as_slice());
    /// }
    /// ```
    #[cfg(not(feature = "no_unsafe"))]
    pub unsafe fn get_unchecked(&self, index: usize) -> &[u8] {
        let (start, len) = self.meta.get_unchecked(index).as_tuple();
        self.data.get_unchecked(start..start + len)
    }

    /// Returns the number of bytestrings in the [`CompactBytestrings`], also referred to as its 'length'.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactBytestrings;
    /// let mut cmpbytes = CompactBytestrings::new();
    ///
    /// cmpbytes.push(b"One");
    /// cmpbytes.push(b"Two");
    /// cmpbytes.push(b"Three");
    ///
    /// assert_eq!(cmpbytes.len(), 3);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.meta.len()
    }

    /// Returns true if the [`CompactBytestrings`] contains no bytestrings.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactBytestrings;
    /// let mut cmpbytes = CompactBytestrings::new();
    /// assert!(cmpbytes.is_empty());
    ///
    /// cmpbytes.push(b"One");
    ///
    /// assert!(!cmpbytes.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of bytes the data vector can store without reallocating.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactBytestrings;
    /// let mut cmpbytes = CompactBytestrings::with_capacity(20, 3);
    ///
    /// cmpbytes.push(b"One");
    ///
    /// assert!(cmpbytes.capacity() >= 20);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Returns the number of starting indices and lengths can store without reallocating.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactBytestrings;
    /// let mut cmpbytes = CompactBytestrings::with_capacity(20, 3);
    ///
    /// cmpbytes.push(b"One");
    /// cmpbytes.push(b"Two");
    /// cmpbytes.push(b"Three");
    /// assert!(cmpbytes.capacity_meta() >= 3);
    ///
    /// cmpbytes.push(b"Three");
    /// assert!(cmpbytes.capacity_meta() > 3);
    /// ```
    #[inline]
    pub fn capacity_meta(&self) -> usize {
        self.meta.capacity()
    }

    /// Clears the [`CompactBytestrings`], removing all bytestrings.
    ///
    /// Note that this method has no effect on the allocated capacity of the vectors.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactBytestrings;
    /// let mut cmpbytes = CompactBytestrings::new();
    ///
    /// cmpbytes.push(b"One");
    /// cmpbytes.push(b"Two");
    /// cmpbytes.push(b"Three");
    /// cmpbytes.clear();
    ///
    /// assert!(cmpbytes.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.data.clear();
        self.meta.clear();
    }

    /// Shrinks the capacity of the data vector, which stores the bytes of the held bytestrings, as much as possible.
    ///
    /// It will drop down as close as possible to the length but the allocator
    /// may still inform the vector that there is space for a few more elements.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactBytestrings;
    /// let mut cmpbytes = CompactBytestrings::with_capacity(20, 3);
    ///
    /// cmpbytes.push(b"One");
    /// cmpbytes.push(b"Two");
    /// cmpbytes.push(b"Three");
    ///
    /// assert!(cmpbytes.capacity() >= 20);
    /// cmpbytes.shrink_to_fit();
    /// assert!(cmpbytes.capacity() >= 3);
    /// ```
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit();
    }

    /// Shrinks the capacity of the info vector, which stores the starting indices and lengths of
    /// the held bytestrings, as much as possible.
    ///
    /// It will drop down as close as possible to the length but the allocator
    /// may still inform the vector that there is space for a few more elements.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactBytestrings;
    /// let mut cmpbytes = CompactBytestrings::with_capacity(20, 10);
    ///
    /// cmpbytes.push(b"One");
    /// cmpbytes.push(b"Two");
    /// cmpbytes.push(b"Three");
    ///
    /// assert!(cmpbytes.capacity_meta() >= 10);
    /// cmpbytes.shrink_to_fit();
    /// assert!(cmpbytes.capacity_meta() >= 3);
    /// ```
    #[inline]
    pub fn shrink_meta_to_fit(&mut self) {
        self.meta.shrink_to_fit();
    }

    /// Shrinks the capacity of the data vector, which stores the bytes of the held bytestrings, with a lower bound.
    ///
    /// The capacity will remain at least as large as both the length and the supplied value.
    ///
    /// If the current capacity is less than the lower limit, this is a no-op.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactBytestrings;
    /// let mut cmpbytes = CompactBytestrings::with_capacity(20, 4);
    ///
    /// cmpbytes.push(b"One");
    /// cmpbytes.push(b"Two");
    /// cmpbytes.push(b"Three");
    ///
    /// assert!(cmpbytes.capacity() >= 20);
    /// cmpbytes.shrink_to(4);
    /// assert!(cmpbytes.capacity() >= 4);
    /// ```
    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.data.shrink_to(min_capacity);
    }

    /// Shrinks the capacity of the meta vector, which starting indices and lengths of the held bytestrings,
    /// with a lower bound.
    ///
    /// The capacity will remain at least as large as both the length and the supplied value.
    ///
    /// If the current capacity is less than the lower limit, this is a no-op.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactBytestrings;
    /// let mut cmpbytes = CompactBytestrings::with_capacity(20, 10);
    ///
    /// cmpbytes.push(b"One");
    /// cmpbytes.push(b"Two");
    /// cmpbytes.push(b"Three");
    ///
    /// assert!(cmpbytes.capacity_meta() >= 10);
    /// cmpbytes.shrink_meta_to(4);
    /// assert!(cmpbytes.capacity_meta() >= 4);
    /// ```
    #[inline]
    pub fn shrink_meta_to(&mut self, min_capacity: usize) {
        self.meta.shrink_to(min_capacity);
    }

    /// Removes the data pointing to where the bytestring at the specified index is stored.
    ///
    /// Note: This does not remove the bytes of the bytestring from memory, you may want to use
    /// [`remove`] if you desire that behavior.
    ///
    /// Note: Because this shifts over the remaining elements in the meta vector, it has a
    /// worst-case performance of *O*(*n*).
    ///
    /// [`remove`]: CompactBytestrings::remove
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactBytestrings;
    /// let mut cmpbytes = CompactBytestrings::with_capacity(20, 3);
    ///
    /// cmpbytes.push(b"One");
    /// cmpbytes.push(b"Two");
    /// cmpbytes.push(b"Three");
    ///
    /// cmpbytes.ignore(1);
    ///
    /// assert_eq!(cmpbytes.get(0), Some(b"One".as_slice()));
    /// assert_eq!(cmpbytes.get(1), Some(b"Three".as_slice()));
    /// assert_eq!(cmpbytes.get(2), None);
    /// ```
    #[track_caller]
    pub fn ignore(&mut self, index: usize) {
        #[cold]
        #[inline(never)]
        #[track_caller]
        fn assert_failed(index: usize, len: usize) -> ! {
            panic!("removal index (is {index}) should be < len (is {len})");
        }

        let len = self.len();
        if index >= len {
            assert_failed(index, len);
        }

        self.meta.remove(index);
    }

    /// Removes the bytes of the bytestring and data pointing to the bytestring is stored.
    ///
    /// Note: This does not shrink the vectors where the bytes of the bytestring and data to the bytestring
    /// are stored. You may shrink the data vector with [`shrink_to`] and [`shrink_to_fit`] and the
    /// meta vector with [`shrink_meta_to`] and [`shrink_meta_to_fit`].
    ///
    /// Note: Because this shifts over the remaining elements in both data and meta vectors, it
    /// has a worst-case performance of *O*(*n*). If you don't need the bytes of the bytestring to
    /// be removed, use [`ignore`] instead.
    ///
    /// [`shrink_to`]: CompactBytestrings::shrink_to
    /// [`shrink_to_fit`]: CompactBytestrings::shrink_to_fit
    /// [`shrink_meta_to`]: CompactBytestrings::shrink_meta_to
    /// [`shrink_meta_to_fit`]: CompactBytestrings::shrink_meta_to_fit
    /// [`ignore`]: CompactBytestrings::ignore
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::CompactBytestrings;
    /// let mut cmpbytes = CompactBytestrings::with_capacity(20, 3);
    ///
    /// cmpbytes.push(b"One");
    /// cmpbytes.push(b"Two");
    /// cmpbytes.push(b"Three");
    ///
    /// cmpbytes.remove(1);
    ///
    /// assert_eq!(cmpbytes.get(0), Some(b"One".as_slice()));
    /// assert_eq!(cmpbytes.get(1), Some(b"Three".as_slice()));
    /// assert_eq!(cmpbytes.get(2), None);
    /// ```
    #[track_caller]
    pub fn remove(&mut self, index: usize) {
        #[cold]
        #[inline(never)]
        #[track_caller]
        fn assert_failed(index: usize, len: usize) -> ! {
            panic!("removal index (is {index}) should be < len (is {len})");
        }

        let len = self.len();
        if index >= len {
            assert_failed(index, len);
        }

        let (start, len) = self.meta.remove(index).as_tuple();
        let inner_len = self.data.len();

        for meta in self.meta.iter_mut().skip(index) {
            meta.start -= start;
        }

        if cfg!(feature = "no_unsafe") {
            self.data.copy_within(start + len..inner_len, start)
        } else {
            unsafe {
                let ptr = self.data.as_mut_ptr().add(start);

                core::ptr::copy(ptr.add(len), ptr, inner_len - start - len);

                self.data.set_len(inner_len - len);
            }
        }
    }

    /// Returns an iterator over the slice.
    ///
    /// The iterator yields all items from start to end.
    ///
    /// # Examples
    ///
    /// ```
    /// # use compact_strings::CompactBytestrings;
    /// let mut cmpbytes = CompactBytestrings::with_capacity(20, 3);
    /// cmpbytes.push(b"One");
    /// cmpbytes.push(b"Two");
    /// cmpbytes.push(b"Three");
    /// let mut iterator = cmpbytes.iter();
    ///
    /// assert_eq!(iterator.next(), Some(b"One".as_slice()));
    /// assert_eq!(iterator.next(), Some(b"Two".as_slice()));
    /// assert_eq!(iterator.next(), Some(b"Three".as_slice()));
    /// assert_eq!(iterator.next(), None);
    /// ```
    #[inline]
    pub fn iter(&self) -> Iter<'_> {
        Iter::new(self)
    }
}

impl Clone for CompactBytestrings {
    fn clone(&self) -> Self {
        let mut data = Vec::with_capacity(self.meta.iter().map(|m| m.len).sum());
        let mut meta = Vec::with_capacity(self.meta.len());

        for bytes in self.iter() {
            meta.push(Metadata {
                start: data.len(),
                len: bytes.len(),
            });
            data.extend_from_slice(bytes);
        }

        Self { data, meta }
    }
}

impl PartialEq for CompactBytestrings {
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

impl Debug for CompactBytestrings {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<S> Extend<S> for CompactBytestrings
where
    S: AsRef<[u8]>,
{
    #[inline]
    fn extend<I: IntoIterator<Item = S>>(&mut self, iter: I) {
        for s in iter {
            self.push(&s);
        }
    }
}

impl Index<usize> for CompactBytestrings {
    type Output = [u8];

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

/// Iterator over bytestrings in a [`CompactBytestrings`]
///
/// # Examples
/// ```
/// # use compact_strings::CompactBytestrings;
/// let mut cmpbytes = CompactBytestrings::new();
/// cmpbytes.push(b"One");
/// cmpbytes.push(b"Two");
/// cmpbytes.push(b"Three");
///
/// let mut iter = cmpbytes.into_iter();
/// assert_eq!(iter.next(), Some(b"One".as_slice()));
/// assert_eq!(iter.next(), Some(b"Two".as_slice()));
/// assert_eq!(iter.next(), Some(b"Three".as_slice()));
/// assert_eq!(iter.next(), None);
/// ```
pub struct Iter<'a> {
    data: &'a [u8],
    iter: core::slice::Iter<'a, Metadata>,
}

impl<'a> Iter<'a> {
    #[inline]
    pub fn new(inner: &'a CompactBytestrings) -> Self {
        Self {
            data: &inner.data,
            iter: inner.meta.iter(),
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let (start, len) = self.iter.next()?.as_tuple();

        if cfg!(feature = "no_unsafe") {
            self.data.get(start..start + len)
        } else {
            unsafe { Some(self.data.get_unchecked(start..start + len)) }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> DoubleEndedIterator for Iter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let (start, len) = self.iter.next_back()?.as_tuple();

        if cfg!(feature = "no_unsafe") {
            self.data.get(start..start + len)
        } else {
            unsafe { Some(self.data.get_unchecked(start..start + len)) }
        }
    }
}

impl ExactSizeIterator for Iter<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a> IntoIterator for &'a CompactBytestrings {
    type Item = &'a [u8];

    type IntoIter = Iter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<S> FromIterator<S> for CompactBytestrings
where
    S: AsRef<[u8]>,
{
    fn from_iter<I: IntoIterator<Item = S>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let meta_capacity = match iter.size_hint() {
            (a, Some(b)) if a == b => a,
            _ => 0,
        };

        let mut out = CompactBytestrings::with_capacity(0, meta_capacity);
        for s in iter {
            out.push(s);
        }

        out
    }
}

impl<S, I> From<I> for CompactBytestrings
where
    S: AsRef<[u8]>,
    I: IntoIterator<Item = S>,
{
    #[inline]
    fn from(value: I) -> Self {
        FromIterator::from_iter(value)
    }
}

impl From<CompactStrings> for CompactBytestrings {
    fn from(value: CompactStrings) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use crate::CompactBytestrings;

    #[test]
    fn exact_size_iterator() {
        let mut cmpbytes = CompactBytestrings::new();

        cmpbytes.push(b"One");
        cmpbytes.push(b"Two");
        cmpbytes.push(b"Three");

        let mut iter = cmpbytes.iter();
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
        let mut cmpbytes = CompactBytestrings::new();

        cmpbytes.push(b"One");
        cmpbytes.push(b"Two");
        cmpbytes.push(b"Three");
        cmpbytes.push(b"Four");

        let mut iter = cmpbytes.iter();
        assert_eq!(iter.next(), Some(b"One".as_slice()));
        assert_eq!(iter.next_back(), Some(b"Four".as_slice()));
        assert_eq!(iter.next(), Some(b"Two".as_slice()));
        assert_eq!(iter.next_back(), Some(b"Three".as_slice()));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }
}
