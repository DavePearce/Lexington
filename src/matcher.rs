use std::ops::RangeInclusive;
use crate::util::{ResetIterator};

/// Responsible for matching a certain pattern against a data stream
/// (e.g. a character stream).  This can be used, for example, for
/// lexing an input stream into tokens.
pub trait Matcher : Sized {
    /// Type of items being matched
    type Item;
    
    /// Determine how many consecutive characters in the input are
    /// matched by this matcher.
    fn matches<I:Iterator<Item=Self::Item>>(&self, input: &mut ResetIterator<I>) -> bool;

    /// Construct a given matcher that matches zero or some items.
    fn zero_or_more(self) -> ZeroOrMore<Self> { ZeroOrMore(self) }

    /// Construct a given matcher that matches one or some items.
    fn one_or_more(self) -> OneOrMore<Self> { OneOrMore(self) }
    
    /// Construct a matcher from two matchers.
    fn or<Rhs:Matcher<Item=Self::Item>>(self, other: Rhs) -> Or<Self,Rhs> {
        Or(self,other)
    }

    fn then<Rhs:Matcher<Item=Self::Item>>(self, other: Rhs) -> Then<Self,Rhs> {
        Then(self,other)
    }
}

/// A default implementation for any type T.
impl<T:PartialEq+Copy> Matcher for T {
    type Item = T;
    
    fn matches<I:Iterator<Item=T>>(&self, input: &mut ResetIterator<I>) -> bool {
        match input.next() {
            Some(t) if self == &t => true,
            Some(t) => {
                input.backup(1);
                false
            }
            _ => false
        }
    }
}

/// A matcher which matches any item from a fixed list of items.
#[derive(Clone,Copy,Debug)]
pub struct Any<T:PartialEq,const N:usize>(pub [T;N]);

impl<T:PartialEq+Copy,const N:usize> Matcher for Any<T,N> {
    type Item = T;

    fn matches<I:Iterator<Item=T>>(&self, input: &mut ResetIterator<I>) -> bool {
        match input.next() {
            Some(t) if self.0.contains(&t) => true,
            Some(t) => {
                input.backup(1);
                false
            }
            _ => false
        }
    }    
}

/// A matching which matches any item within a given range.
#[derive(Clone,Debug)]
pub struct Within<T:PartialOrd>(pub RangeInclusive<T>);

impl<T:PartialOrd+Copy> Matcher for Within<T> {
    type Item = T;

    fn matches<I:Iterator<Item=T>>(&self, input: &mut ResetIterator<I>) -> bool {
        match input.next() {
            Some(t) if self.0.contains(&t) => true,
            Some(t) => {
                input.backup(1);
                false
            }
            _ => false
        }
    }
}
            
/// A matcher which matches one or more occurences of a given item.
#[derive(Clone,Copy,Debug)]
pub struct Many<M:Matcher>(M);

impl<M:Matcher> Matcher for Many<M> {
    type Item = M::Item;

    fn matches<I:Iterator<Item=M::Item>>(&self, input: &mut ResetIterator<I>) -> bool {
        // Try the first match        
        let first = self.0.matches(input);
        // Continue whilst more
        while self.0.matches(input) {}
        //
        first
    }
}

/// A matcher which matches one or more occurences of a given item.
#[derive(Clone,Copy,Debug)]
pub struct OneOrMore<M:Matcher>(M);

impl<M:Matcher> Matcher for OneOrMore<M> {
    type Item = M::Item;

    fn matches<I:Iterator<Item=M::Item>>(&self, input: &mut ResetIterator<I>) -> bool {
        // Try the first match        
        let first = self.0.matches(input);
        // Continue whilst more
        while self.0.matches(input) {}
        //
        first
    }
}

/// A matcher which matches zero or more occurences of a given item.
#[derive(Clone,Copy,Debug)]
pub struct ZeroOrMore<M:Matcher>(M);

impl<M:Matcher> Matcher for ZeroOrMore<M> {
    type Item = M::Item;

    fn matches<I:Iterator<Item=M::Item>>(&self, input: &mut ResetIterator<I>) -> bool {
        // Continue whilst more
        while self.0.matches(input) {}
        // Always succeeds
        true
    }
}

/// A `Matcher` which combines two `Matchers` together, such that it
/// matches if either matches.
#[derive(Clone,Copy,Debug)]
pub struct Or<Lhs:Matcher,Rhs:Matcher<Item=Lhs::Item>>(Lhs,Rhs);

impl<Lhs:Matcher,Rhs:Matcher<Item=Lhs::Item>> Matcher for Or<Lhs,Rhs> {
    type Item = Lhs::Item;

    fn matches<I:Iterator<Item=Lhs::Item>>(&self, input: &mut ResetIterator<I>) -> bool {
        self.0.matches(input) || self.1.matches(input)
    }
}

#[derive(Clone,Copy,Debug)]
pub struct Then<Lhs:Matcher,Rhs:Matcher<Item=Lhs::Item>>(Lhs,Rhs);

impl<Lhs:Matcher,Rhs:Matcher<Item=Lhs::Item>> Matcher for Then<Lhs,Rhs> {
    type Item = Lhs::Item;

    fn matches<I:Iterator<Item=Lhs::Item>>(&self, input: &mut ResetIterator<I>) -> bool {
        let offset = input.offset();
        //
        if self.0.matches(input) {
            let n = input.offset() - offset;
            if self.1.matches(input) {
                return true;
            } else {
                input.backup(n);
            }
        }
        false
    }
}

/// =============================================================================
/// Tests
/// =============================================================================

#[cfg(test)]
mod tests {
    use super::{Matcher,ResetIterator};

    #[test]
    fn test_01() {
        let mut input = ResetIterator::new("(".chars());
        let matcher = '(';
        assert!(matcher.matches(&mut input));
        assert_eq!(input.next(),None);        
    }

    #[test]
    fn test_02() {
        let mut input = ResetIterator::new("(abc".chars());
        let matcher = '(';
        assert!(matcher.matches(&mut input));
        assert_eq!(input.next(),Some('a'));        
    }

    #[test]
    fn test_03() {
        let mut input = ResetIterator::new("(".chars());
        let matcher = '('.or(')');
        assert!(matcher.matches(&mut input));
        assert_eq!(input.next(),None);        
    }   

    #[test]
    fn test_04() {
        let mut input = ResetIterator::new(")".chars());
        let matcher = '('.or(')');
        assert!(matcher.matches(&mut input));
        assert_eq!(input.next(),None);        
    }   
    
    #[test]
    fn test_05() {
        let mut input = ResetIterator::new("((((".chars());
        let matcher = '('.one_or_more();
        assert!(matcher.matches(&mut input));
        assert_eq!(input.next(),None);
    }

    #[test]
    fn test_06() {
        let mut input = ResetIterator::new(")".chars());
        let matcher = '('.or(')').one_or_more();
        assert!(matcher.matches(&mut input));
        assert_eq!(input.next(),None);        
    }       
}
