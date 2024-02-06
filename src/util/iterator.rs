/// An iterator wither a `lookahead(n)` that returns an optional
/// reference to the `n`th item (where `n==0` is the `next()` item).
pub struct LookaheadIterator<I>
where I:Iterator {
    /// The underlying iterator from which this iterator is based.
    iter: I,
    /// Stores items which have been read out of the iterator already.
    items: Vec<I::Item>,
    /// Determines offset within original stream.
    offset: usize
}

impl<I:Iterator> LookaheadIterator<I> {
    /// Construct a lookahead iterator from an arbitrary iterator.
    pub fn new(iter:I) -> Self { Self{iter, items: Vec::new(), offset:0 } }

    /// Extract the _nth_ item in the iterator.
    pub fn lookahead(&mut self, n: usize) -> Option<&<I as Iterator>::Item> {
        self.expand(n);
        // Read out the nth item
        if n >= self.items.len() {
            None
        } else {
            Some(&self.items[n])
        }
    }

    pub fn offset(&self) -> usize { self.offset }

    /// Skip over the next `n` items
    pub fn skip(&mut self, n:usize) {
        // For now
        assert!(self.items.len() >= n);
        // Update count
        self.offset += n;
        // Trim down buffer
        self.items.drain(0..n);
    }

    /// Slice out `n` items from the iterator.
    pub fn slice(&mut self, n: usize) -> &[<I as Iterator>::Item] {
        self.expand(n);
        // Determine how large the slice actually is
        let m = usize::min(self.items.len(),n);
        // Make the slice!
        &self.items[0..m]
    }

    /// Slice out `n` items matching a given predicate.
    pub fn slice_while(&mut self, predicate: fn(&I::Item)->bool) -> &[<I as Iterator>::Item] {
        let mut i = 0;
        // Search forward
        loop {
            match self.lookahead(i) {
                Some(c) if predicate(c) => { i = i + 1; }
                _ => { return self.slice(i); }
            }
        }
    }

    /// Ensure buffer has `n` elements (unless there are insufficient elements).
    fn expand(&mut self, n: usize) {
        while self.items.len() <= n {
            match self.iter.next() {
                Some(item) => { self.items.push(item); }
                None => { break; }
            }
        }
    }
}
