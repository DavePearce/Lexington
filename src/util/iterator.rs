/// An iterator which can be "reset" after an arbitrary number of
/// calls to `next()`.  This is achieved using a
/// buffer which stores items as they are read.
pub struct ResetIterator<I>
where I:Iterator {
    /// The underlying iterator from which this iterator is based.
    iter: I,
    /// Stores items which have been read out of the iterator already.
    items: Vec<I::Item>,
    /// Determines offset of first element of `items` in original stream.
    start: usize,
    /// Determines offset within original stream.
    offset: usize
}

impl<I:Iterator> ResetIterator<I> {
    /// Construct a lookahead iterator from an arbitrary iterator.
    pub fn new(iter:I) -> Self { Self{iter, items: Vec::new(), start:0, offset:0 } }

    /// Get the current position within this iterator.
    pub fn offset(&self) -> usize {
        self.offset
    }
    
    pub fn backup(&mut self, n:usize) {
        assert!(n <= self.items.len());
        self.offset = self.offset - n;
    }

    /// Empty the internal lookahead buffer.
    pub fn reset(&mut self) {
        // Compute amount to reset.
        let n = self.offset - self.start;
        // Move start ptr along        
        self.start = self.offset;
        // Clean all items
        self.items.drain(0..n);
    }
}

impl<I:Iterator> Iterator for ResetIterator<I>
where I::Item : Copy {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        // Compute index within items
        let i = self.offset - self.start;        
        // Check whether item available
        if i >= self.items.len() {
            // Pull another item off.
            match self.iter.next() {
                Some(v) => {self.items.push(v);}
                None => {return None;}
            };
        }
        // Increment position
        self.offset += 1;                
        // Done
        Some(self.items[i])        
    }
}
