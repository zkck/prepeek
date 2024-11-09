use std::usize;

/// Wrapper struct to an iterator, offering `peek` and `peek_nth`.
///
/// Internally holds the next `L` elements to allow for peeking without `mut`.
pub struct Prepeek<I: Iterator, const L: usize> {
    iter: I,
    ring: [Option<I::Item>; L],
    ring_index: usize,
}

impl<I: Iterator, const L: usize> Prepeek<I, L> {

    /// Creates a [`Prepeek`] object wrapping the given [`Iterator`].
    ///
    /// Calls next() `L` times on the iterator to fill up the internal buffer.
    pub fn new(iter: I) -> Self {
        let mut s = Self {
            iter,
            ring: [const { None }; L],
            ring_index: 0,
        };
        // fill ring buffer
        for _ in 0..L {
            s.next();
        }
        s
    }

    /// Returns a reference to the next() value without advancing the iterator.
    ///
    /// Like next, if there is a value, it is wrapped in a `Some(T)`. But if the iteration is over, `None` is returned.
    ///
    /// If `L` of this [`Prepeek`] object is 0, None is returned.
    pub fn peek(&self) -> Option<&I::Item> {
        self.peek_nth(0)
    }

    /// Returns a reference to the `nth` value without advancing the iterator.
    ///
    /// If `n` is greater or equal to `L`, None is returned.
    pub fn peek_nth(&self, n: usize) -> Option<&I::Item> {
        if n >= L {
            None
        } else {
            self.ring[(self.ring_index + n) % L].as_ref()
        }
    }
}

impl<I: Iterator, const L: usize> Iterator for Prepeek<I, L> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut v = self.iter.next();
        if L != 0 {
            v = std::mem::replace(&mut self.ring[self.ring_index], v);
            self.ring_index = (self.ring_index + 1) % L;
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let array = [1, 2, 3];
        let mut peekable = Prepeek::<_, 2>::new(array.into_iter());
        assert_eq!(peekable.peek().cloned(), Some(1));
        assert_eq!(peekable.peek_nth(1).cloned(), Some(2));
        assert_eq!(peekable.peek_nth(2).cloned(), None);

        assert_eq!(peekable.next(), Some(1));
        assert_eq!(peekable.peek().cloned(), Some(2));
        assert_eq!(peekable.peek_nth(1).cloned(), Some(3));
        assert_eq!(peekable.peek_nth(2).cloned(), None);

        assert_eq!(peekable.next(), Some(2));
        assert_eq!(peekable.peek().cloned(), Some(3));
        assert_eq!(peekable.peek_nth(1).cloned(), None);
        assert_eq!(peekable.peek_nth(2).cloned(), None);

        assert_eq!(peekable.next(), Some(3));
        assert_eq!(peekable.peek().cloned(), None);
        assert_eq!(peekable.peek_nth(1).cloned(), None);
        assert_eq!(peekable.peek_nth(2).cloned(), None);
    }

    #[test]
    fn test_no_prefetch() {
        let array = [1, 2, 3];
        let mut peekable = Prepeek::<_, 0>::new(array.into_iter());
        assert_eq!(peekable.peek().cloned(), None);
        assert_eq!(peekable.next(), Some(1));
    }

    #[test]
    fn test_overallocated() {
        let array = [1, 2, 3];
        let mut peekable = Prepeek::<_, 5>::new(array.into_iter());
        assert_eq!(peekable.next(), Some(1));
        assert_eq!(peekable.next(), Some(2));
        assert_eq!(peekable.next(), Some(3));
        assert_eq!(peekable.peek_nth(0).cloned(), None);
        assert_eq!(peekable.peek_nth(1).cloned(), None);
        assert_eq!(peekable.peek_nth(2).cloned(), None);
    }
}
