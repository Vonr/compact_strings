use core::{fmt::Debug, ops::Index};

use alloc::vec::Vec;

use crate::FixedCompactStrings;

/// An even more compact but limited representation of a list of bytestrings.
///
/// Bytestrings are stored contiguously in a vector of bytes, with their starting indices
/// being stored separately.
///
/// Bytestrings smaller than 8 bytes are stored inline in the indices.
///
/// Limitations include being unable to mutate bytestrings stored in the vector.
///
/// # Examples
/// ```
/// # use compact_strings::FixedCompactBytestrings;
/// let mut cmpbytes = FixedCompactBytestrings::with_capacity(20, 3);
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
pub struct FixedCompactBytestrings {
    pub(crate) data: Vec<u8>,
    pub(crate) starts: Vec<usize>,
}

impl FixedCompactBytestrings {
    /// Constructs a new, empty [`FixedCompactBytestrings`].
    ///
    /// The [`FixedCompactBytestrings`] will not allocate until bytestrings are pushed into it.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::FixedCompactBytestrings;
    /// let mut cmpbytes = FixedCompactBytestrings::new();
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {
            data: Vec::new(),
            starts: Vec::new(),
        }
    }

    /// Constructs a new, empty [`FixedCompactBytestrings`] with at least the specified capacities in each
    /// vector.
    ///
    /// - `data_capacity`: The capacity of the data vector where the bytes of the bytestrings are stored.
    /// - `capacity_meta`: The capacity of the meta vector where the starting indices
    ///   of the bytestrings are stored.
    ///
    /// The [`FixedCompactBytestrings`] will be able to hold at least *`data_capacity`* bytes worth of bytestrings
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
    /// [`capacity`]: FixedCompactBytestrings::capacity
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::FixedCompactBytestrings;
    /// let mut cmpbytes = FixedCompactBytestrings::with_capacity(20, 3);
    ///
    /// assert_eq!(cmpbytes.len(), 0);
    /// assert!(cmpbytes.capacity() >= 20);
    /// assert!(cmpbytes.capacity_meta() >= 3);
    /// ```
    #[must_use]
    pub fn with_capacity(data_capacity: usize, capacity_meta: usize) -> Self {
        Self {
            data: Vec::with_capacity(data_capacity),
            starts: Vec::with_capacity(capacity_meta),
        }
    }

    /// Appends a bytestring to the back of the [`FixedCompactBytestrings`].
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::FixedCompactBytestrings;
    /// let mut cmpbytes = FixedCompactBytestrings::new();
    /// cmpbytes.push(b"One");
    /// cmpbytes.push(b"Two");
    /// cmpbytes.push(b"Three");
    ///
    /// assert_eq!(cmpbytes.get(0), Some(b"One".as_slice()));
    /// assert_eq!(cmpbytes.get(1), Some(b"Two".as_slice()));
    /// assert_eq!(cmpbytes.get(2), Some(b"Three".as_slice()));
    /// assert_eq!(cmpbytes.get(3), None);
    /// ```
    #[allow(clippy::cast_possible_truncation)]
    pub fn push<S>(&mut self, bytestring: S)
    where
        S: AsRef<[u8]>,
    {
        const BYTES: usize = (usize::BITS / 8) as usize;

        let bytestr = bytestring.as_ref();
        if bytestr.len() < BYTES {
            let mut data = [0u8; BYTES];
            data[0] = 0b1000_0000 | ((bytestr.len() as u8) << (u8::BITS - 4));
            for (i, &b) in bytestr.iter().enumerate() {
                data[i + 1] = b;
            }

            self.starts.push(bytemuck::cast::<_, usize>(data));
            return;
        }

        self.starts.push(self.data.len());
        self.data.extend_from_slice(bytestr);
    }

    /// Returns a reference to the bytestring stored in the [`FixedCompactBytestrings`] at that position.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::FixedCompactBytestrings;
    /// let mut cmpbytes = FixedCompactBytestrings::new();
    /// cmpbytes.push(b"One");
    /// cmpbytes.push(b"Two");
    /// cmpbytes.push(b"Three");
    /// cmpbytes.push(b"Seventeen");
    ///
    /// assert_eq!(cmpbytes.get(0), Some(b"One".as_slice()));
    /// assert_eq!(cmpbytes.get(1), Some(b"Two".as_slice()));
    /// assert_eq!(cmpbytes.get(2), Some(b"Three".as_slice()));
    /// assert_eq!(cmpbytes.get(3), Some(b"Seventeen".as_slice()));
    /// assert_eq!(cmpbytes.get(4), None);
    /// ```
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&[u8]> {
        let start = self.starts.get(index)?;
        let data = bytemuck::bytes_of(start);
        if data[0] & 0b1000_1111 == 0b1000_0000 {
            let len = ((data[0] & (0b111 << (u8::BITS - 4))) >> (u8::BITS - 4)) as usize;

            return Some(&data[1..=len]);
        }

        let next = self
            .starts
            .iter()
            .skip(index + 1)
            .copied()
            .find(|n| bytemuck::bytes_of(n)[0] & 0b1000_1111 != 0b1000_0000)
            .unwrap_or(self.data.len());

        if cfg!(feature = "no_unsafe") {
            self.data.get(*start..next)
        } else {
            unsafe { Some(self.data.get_unchecked(*start..next)) }
        }
    }

    /// Returns a reference to the bytestring stored in the [`FixedCompactBytestrings`] at that position, without
    /// doing bounds checking.
    ///
    /// # Safety
    /// Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::FixedCompactBytestrings;
    /// let mut cmpbytes = FixedCompactBytestrings::new();
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
    #[must_use]
    #[cfg(not(feature = "no_unsafe"))]
    pub unsafe fn get_unchecked(&self, index: usize) -> &[u8] {
        let start = self.starts.get_unchecked(index);
        let data = bytemuck::bytes_of(start);
        if data[0] & 0b1000_1111 == 0b1000_0000 {
            let len = ((data[0] & (0b111 << (u8::BITS - 4))) >> (u8::BITS - 4)) as usize;

            return &data[1..=len];
        }

        let next = self
            .starts
            .iter()
            .skip(index + 1)
            .copied()
            .find(|n| bytemuck::bytes_of(n)[0] & 0b1000_1111 != 0b1000_0000)
            .unwrap_or(self.data.len());

        self.data.get_unchecked(*start..next)
    }

    /// Returns the number of bytestrings in the [`FixedCompactBytestrings`], also referred to as its 'length'.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::FixedCompactBytestrings;
    /// let mut cmpbytes = FixedCompactBytestrings::new();
    ///
    /// cmpbytes.push(b"One");
    /// cmpbytes.push(b"Two");
    /// cmpbytes.push(b"Three");
    ///
    /// assert_eq!(cmpbytes.len(), 3);
    /// ```
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.starts.len()
    }

    /// Returns true if the [`FixedCompactBytestrings`] contains no bytestrings.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::FixedCompactBytestrings;
    /// let mut cmpbytes = FixedCompactBytestrings::new();
    /// assert!(cmpbytes.is_empty());
    ///
    /// cmpbytes.push(b"One");
    ///
    /// assert!(!cmpbytes.is_empty());
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
    /// # use compact_strings::FixedCompactBytestrings;
    /// let mut cmpbytes = FixedCompactBytestrings::with_capacity(20, 3);
    ///
    /// cmpbytes.push(b"One");
    ///
    /// assert!(cmpbytes.capacity() >= 20);
    /// ```
    #[inline]
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Returns the number of starting indices can store without reallocating.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::FixedCompactBytestrings;
    /// let mut cmpbytes = FixedCompactBytestrings::with_capacity(20, 3);
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
    #[must_use]
    pub fn capacity_meta(&self) -> usize {
        self.starts.capacity()
    }

    /// Clears the [`FixedCompactBytestrings`], removing all bytestrings.
    ///
    /// Note that this method has no effect on the allocated capacity of the vectors.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::FixedCompactBytestrings;
    /// let mut cmpbytes = FixedCompactBytestrings::new();
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
        self.starts.clear();
    }

    /// Shrinks the capacity of the data vector, which stores the bytes of the held bytestrings, as much as possible.
    ///
    /// It will drop down as close as possible to the length but the allocator
    /// may still inform the vector that there is space for a few more elements.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::FixedCompactBytestrings;
    /// let mut cmpbytes = FixedCompactBytestrings::with_capacity(40, 3);
    ///
    /// // Padding used as bytestrings smaller than 8 bytes are stored inline,
    /// // thus not affecting needed capacity.
    /// cmpbytes.push(b"Padding One");
    /// cmpbytes.push(b"Padding Two");
    /// cmpbytes.push(b"Padding Three");
    ///
    /// assert!(cmpbytes.capacity() >= 40);
    /// cmpbytes.shrink_to_fit();
    /// assert!(cmpbytes.capacity() >= 26);
    /// ```
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit();
    }

    /// Shrinks the capacity of the info vector, which stores the starting indices of
    /// the held bytestrings, as much as possible.
    ///
    /// It will drop down as close as possible to the length but the allocator
    /// may still inform the vector that there is space for a few more elements.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::FixedCompactBytestrings;
    /// let mut cmpbytes = FixedCompactBytestrings::with_capacity(20, 10);
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
        self.starts.shrink_to_fit();
    }

    /// Shrinks the capacity of the data vector, which stores the bytes of the held bytestrings, with a lower bound.
    ///
    /// The capacity will remain at least as large as both the length and the supplied value.
    ///
    /// If the current capacity is less than the lower limit, this is a no-op.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::FixedCompactBytestrings;
    /// let mut cmpbytes = FixedCompactBytestrings::with_capacity(20, 4);
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

    /// Shrinks the capacity of the meta vector, which starting indices of the held bytestrings,
    /// with a lower bound.
    ///
    /// The capacity will remain at least as large as both the length and the supplied value.
    ///
    /// If the current capacity is less than the lower limit, this is a no-op.
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::FixedCompactBytestrings;
    /// let mut cmpbytes = FixedCompactBytestrings::with_capacity(20, 10);
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
        self.starts.shrink_to(min_capacity);
    }

    /// Removes the bytes of the bytestring and data pointing to the bytestring is stored.
    ///
    /// Note: This does not shrink the vectors where the bytes of the bytestring and data to the bytestring
    /// are stored. You may shrink the data vector with [`shrink_to`] and [`shrink_to_fit`] and the
    /// meta vector with [`shrink_meta_to`] and [`shrink_meta_to_fit`].
    ///
    /// Note: Because this shifts over the remaining elements in both data and meta vectors, it
    /// has a worst-case performance of *O*(*n*).
    ///
    /// [`shrink_to`]: FixedCompactBytestrings::shrink_to
    /// [`shrink_to_fit`]: FixedCompactBytestrings::shrink_to_fit
    /// [`shrink_meta_to`]: FixedCompactBytestrings::shrink_meta_to
    /// [`shrink_meta_to_fit`]: FixedCompactBytestrings::shrink_meta_to_fit
    ///
    /// # Examples
    /// ```
    /// # use compact_strings::FixedCompactBytestrings;
    /// let mut cmpbytes = FixedCompactBytestrings::with_capacity(20, 3);
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

        let inner_len = self.data.len();
        let start = self.starts.remove(index);
        let data = bytemuck::bytes_of(&start);
        if data[0] & 0b1000_1111 == 0b1000_0000 {
            return;
        }

        let next = self
            .starts
            .iter()
            .skip(index + 1)
            .copied()
            .find(|n| bytemuck::bytes_of(n)[0] & 0b1000_1111 != 0b1000_0000)
            .unwrap_or(self.data.len());

        let len = next - start;

        for s in self.starts.iter_mut().skip(index + 1) {
            *s -= len;
        }

        if cfg!(feature = "no_unsafe") {
            self.data.copy_within(start + len..inner_len, start);
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
    /// # use compact_strings::FixedCompactBytestrings;
    /// let mut cmpbytes = FixedCompactBytestrings::with_capacity(20, 3);
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

impl Default for FixedCompactBytestrings {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for FixedCompactBytestrings {
    fn clone(&self) -> Self {
        let mut data = Vec::with_capacity(self.starts.windows(2).map(|s| s[1] - s[0]).sum());
        let mut meta = Vec::with_capacity(self.starts.len());

        for bytes in self {
            meta.push(bytes.len());
            data.extend_from_slice(bytes);
        }

        Self { data, starts: meta }
    }
}

impl PartialEq for FixedCompactBytestrings {
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

impl Debug for FixedCompactBytestrings {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<S> Extend<S> for FixedCompactBytestrings
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

impl Index<usize> for FixedCompactBytestrings {
    type Output = [u8];

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

/// Iterator over bytestrings in a [`FixedCompactBytestrings`]
///
/// # Examples
/// ```
/// # use compact_strings::FixedCompactBytestrings;
/// let mut cmpbytes = FixedCompactBytestrings::new();
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
#[must_use = "Iterators are lazy and do nothing unless consumed"]
pub struct Iter<'a> {
    data: &'a [u8],
    starts: core::slice::Iter<'a, usize>,
}

impl<'a> Iter<'a> {
    #[inline]
    pub fn new(inner: &'a FixedCompactBytestrings) -> Self {
        Self {
            data: &inner.data,
            starts: inner.starts.iter(),
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.starts.next()?;
        let data = bytemuck::bytes_of(start);
        if data[0] & 0b1000_1111 == 0b1000_0000 {
            let len = ((data[0] & (0b111 << (u8::BITS - 4))) >> (u8::BITS - 4)) as usize;

            return Some(&data[1..=len]);
        }

        let end = self
            .starts
            .clone()
            .copied()
            .find(|n| bytemuck::bytes_of(n)[0] & 0b1000_1111 != 0b1000_0000)
            .unwrap_or(self.data.len());

        if cfg!(feature = "no_unsafe") {
            self.data.get(*start..end)
        } else {
            unsafe { Some(self.data.get_unchecked(*start..end)) }
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let start = self.starts.nth(n)?;
        let data = bytemuck::bytes_of(start);
        if data[0] & 0b1000_1111 == 0b1000_0000 {
            let len = ((data[0] & (0b111 << (u8::BITS - 4))) >> (u8::BITS - 4)) as usize;

            return Some(&data[1..=len]);
        }

        let end = self
            .starts
            .clone()
            .copied()
            .find(|n| bytemuck::bytes_of(n)[0] & 0b1000_1111 != 0b1000_0000)
            .unwrap_or(self.data.len());

        if cfg!(feature = "no_unsafe") {
            self.data.get(*start..end)
        } else {
            unsafe { Some(self.data.get_unchecked(*start..end)) }
        }
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
        self.next_back()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.starts.size_hint()
    }
}

impl<'a> DoubleEndedIterator for Iter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let start = self.starts.next_back()?;
        let data = bytemuck::bytes_of(start);
        if data[0] & 0b1000_0000 != 0 {
            let len = ((data[0] & (0b111 << (u8::BITS - 4))) >> (u8::BITS - 4)) as usize;

            return Some(&data[1..=len]);
        }

        let out = if cfg!(feature = "no_unsafe") {
            self.data.get(*start..)
        } else {
            unsafe { Some(self.data.get_unchecked(*start..)) }
        };
        self.data = &self.data[..*start];

        out
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let mut fork = self.starts.clone();
        let start = self.starts.nth_back(n)?;
        let data = bytemuck::bytes_of(start);
        if data[0] & 0b1000_0000 != 0 {
            let len = ((data[0] & (0b111 << (u8::BITS - 4))) >> (u8::BITS - 4)) as usize;

            return Some(&data[1..=len]);
        }
        let end = if n == 0 {
            self.data.len()
        } else {
            *fork.nth_back(n - 1)?
        };

        let out = if cfg!(feature = "no_unsafe") {
            self.data.get(*start..end)
        } else {
            unsafe { Some(self.data.get_unchecked(*start..end)) }
        };
        self.data = &self.data[..*start];

        out
    }
}

impl ExactSizeIterator for Iter<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.starts.len()
    }
}

impl<'a> IntoIterator for &'a FixedCompactBytestrings {
    type Item = &'a [u8];

    type IntoIter = Iter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<S> FromIterator<S> for FixedCompactBytestrings
where
    S: AsRef<[u8]>,
{
    fn from_iter<I: IntoIterator<Item = S>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let meta_capacity = match iter.size_hint() {
            (a, Some(b)) if a == b => a,
            _ => 0,
        };

        let mut out = FixedCompactBytestrings::with_capacity(0, meta_capacity);
        for s in iter {
            out.push(s);
        }

        out
    }
}

impl<S, I> From<I> for FixedCompactBytestrings
where
    S: AsRef<[u8]>,
    I: IntoIterator<Item = S>,
{
    #[inline]
    fn from(value: I) -> Self {
        FromIterator::from_iter(value)
    }
}

impl From<FixedCompactStrings> for FixedCompactBytestrings {
    fn from(value: FixedCompactStrings) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use crate::FixedCompactBytestrings;

    #[test]
    fn exact_size_iterator() {
        let mut cmpbytes = FixedCompactBytestrings::new();

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
        let mut cmpbytes = FixedCompactBytestrings::new();

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
