use std::ops::Range;
use crate::util::{Region};

/// A token constitutes a label and identifying region within the
/// original sequence.
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Token<T>
{
    /// Type of the token
    pub kind : T,
    /// Identifies the (half open) region in the sequence.
    pub region: Region
}

impl<T> Token<T>  {
    pub fn new(kind: T, range: Range<usize>) -> Self {
        Self { kind, region: Region::from(range) }
    }

    /// Get first index of this token.
    pub fn start(&self) -> usize {
	self.region.start
    }

    /// Get end of this token (that is one past its last character).
    pub fn end(&self) -> usize {
	self.region.end
    }

    /// Get the length (in chars) of this token.
    pub fn len(&self) -> usize {
        self.region.end - self.region.start
    }

    /// Extract the underlying region covered by this span as a
    /// `Range`.  This is really just for convenience.
    pub fn range(&self) -> Range<usize> { self.start() .. self.end() }

    /// Shift the span to a different position in the underlying
    /// sequence.  The position is taken as a delta from the current
    /// position (e.g. `delta==1` means we shift one up the sequence).
    pub fn shift(&mut self, delta: usize) {
        self.region.shift(delta);
    }
}

impl<T:PartialEq> PartialEq<(T,Range<usize>)> for Token<T> {
    //
    fn eq(&self, other: &(T,Range<usize>)) -> bool {
        self.kind == other.0 && self.range() == other.1
    }
}
