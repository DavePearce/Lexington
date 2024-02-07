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

    /// Construct a given matcher that matches one or some items.
    fn many(self) -> Many<Self> { Many(self) }

    /// Construct a matcher from two matchers.
    fn or<Rhs:Matcher<Item=Self::Item>>(self, other: Rhs) -> Or<Self,Rhs> {
        Or(self,other)
    }
}

impl<T:PartialEq+Copy> Matcher for T
{
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

/// A matcher which matches one or more occurences of a given item.
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

/// A `Matcher` which combines two `Matchers` together, such that it
/// matches if either matches.
pub struct Or<Lhs:Matcher,Rhs:Matcher<Item=Lhs::Item>>(Lhs,Rhs);

impl<Lhs:Matcher,Rhs:Matcher<Item=Lhs::Item>> Matcher for Or<Lhs,Rhs> {
    type Item = Lhs::Item;

    fn matches<I:Iterator<Item=Lhs::Item>>(&self, input: &mut ResetIterator<I>) -> bool {
        self.0.matches(input) || self.1.matches(input)
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
        let matcher = '('.many();
        assert!(matcher.matches(&mut input));
        assert_eq!(input.next(),None);
    }

    #[test]
    fn test_06() {
        let mut input = ResetIterator::new(")".chars());
        let matcher = '('.or(')').many();
        assert!(matcher.matches(&mut input));
        assert_eq!(input.next(),None);        
    }       
}
