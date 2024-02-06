use std::ops::Range;

/// Basically the same as `std::ops::Range`, but implements `Copy`.
/// Note, like `Range`, this is _half open_.  That means `start`
/// identifies the first index in the region, whilst `end` is one past
/// the last index.
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Region {
    pub start: usize,
    pub end: usize
}

impl Region {
    pub fn new(start: usize, end: usize) -> Self {
        Self {start,end}
    }
    /// Determine the number of items this region covers.
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn shift(&mut self, delta: usize) {
        self.start += delta;
        self.end += delta;
    }
}

/// Simple mechanism for constructing a `Region` from a `Range`.
impl From<Range<usize>> for Region {
    fn from(r: Range<usize>) -> Region {
       Region{start:r.start,end:r.end}
    }
}

impl Into<Range<usize>> for Region {
    fn into(self) -> Range<usize> { self.start .. self.end }
}
