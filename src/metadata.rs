#[derive(Clone, Copy)]
pub(crate) struct Metadata {
    pub(crate) start: usize,
    pub(crate) len: usize,
}

impl Metadata {
    #[inline]
    pub(crate) const fn new(start: usize, len: usize) -> Self {
        Self { start, len }
    }

    #[inline]
    pub(crate) const fn as_tuple(&self) -> (usize, usize) {
        (self.start, self.len)
    }
}
