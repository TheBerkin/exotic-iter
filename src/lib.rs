mod alternate;

pub use alternate::*;

/// Provides additional convenience methods to the `Iterator` trait and its implementors.
pub trait ExoticIteratorExt: Iterator + Sized {
    /// Consumes the iterator, counting the number of items that pass the predicate and returns true iff there were at least `n` passing items.
    /// 
    /// # Example
    /// ```rust
    /// use exotic_iter::*;
    /// let very_few_twos = vec![1, 2, 3, 4, 5, 6];
    /// let lots_of_twos = vec![2, 3, 2, 4, 2, 5];
    /// assert_eq!(false, very_few_twos.into_iter().at_least(3_usize, |n| *n == 2));
    /// assert_eq!(true, lots_of_twos.into_iter().at_least(3_usize, |n| *n == 2));
    /// ```
    fn at_least<P: FnMut(&Self::Item) -> bool>(self, n: usize, predicate: P) -> bool;

    /// Consumes the iterator, counting the number of items that pass the predicate and returns true iff there were no more than `n` passing items.
    ///
    /// # Example
    /// ```rust
    /// use exotic_iter::*;
    /// let just_enough_twos = vec![1, 1, 2, 2, 3, 3];
    /// let too_many_twos = vec![2, 2, 2, 2, 2, 2];
    /// assert_eq!(true, just_enough_twos.into_iter().at_most(2_usize, |n| *n == 2));
    /// assert_eq!(false, too_many_twos.into_iter().at_most(2_usize, |n| *n == 2));
    /// ```
    fn at_most<P: FnMut(&Self::Item) -> bool>(self, n: usize, predicate: P) -> bool;

    /// Consumes the iterator, counting the number of items that pass the predicate and returns true iff there were exactly `n` passing items.
    ///
    /// # Example
    /// ```rust
    /// use exotic_iter::*;
    /// let no_digits = "deadbeef";
    /// let two_digits = "deadb33f";
    /// let three_digits = "d3adb33f";
    /// assert_eq!(false, no_digits.chars().exactly_n(2_usize, |c| c.is_ascii_digit()));
    /// assert_eq!(true, two_digits.chars().exactly_n(2_usize, |c| c.is_ascii_digit()));
    /// assert_eq!(false, three_digits.chars().exactly_n(2_usize, |c| c.is_ascii_digit()));
    /// ```
    fn exactly_n<P: FnMut(&Self::Item) -> bool>(self, n: usize, predicate: P) -> bool;

    /// Consumes the iterator, counting the number of items that pass each predicate and returns true iff 
    /// there were exactly `m` passing items for the `predicate_m`, and exactly `n` passing items for `predicate_n`. 
    ///
    /// # Example
    /// ```rust
    /// use exotic_iter::*;
    /// let more_digits = "abc12345";
    /// let balanced_digits = "abcd1234";
    /// let more_letters = "abcde123";
    /// assert_eq!(false, more_digits.chars().exactly_m_n(4_usize, |c| c.is_ascii_alphabetic(), 4_usize, |c| c.is_ascii_digit()));
    /// assert_eq!(true, balanced_digits.chars().exactly_m_n(4_usize, |c| c.is_ascii_alphabetic(), 4_usize, |c| c.is_ascii_digit()));
    /// assert_eq!(false, more_letters.chars().exactly_m_n(4_usize, |c| c.is_ascii_alphabetic(), 4_usize, |c| c.is_ascii_digit()));
    /// ```
    fn exactly_m_n<Pm: FnMut(&Self::Item) -> bool, Pn: FnMut(&Self::Item) -> bool>(self, m: usize, predicate_m: Pm, n: usize, predicate_n: Pn) -> bool;

    /// Consumes the iterator and returns true iff either all of the items in the iterator passed the predicate, or none of them passed.
    /// Returns early on the first mixed result.
    ///
    /// # Example
    /// ```rust
    /// use exotic_iter::*;
    /// let all_true = vec![true, true, true];
    /// let some_true = vec![true, false, true];
    /// let none_true = vec![false, false, false];
    /// assert_eq!(true, all_true.into_iter().all_or_none(|b| *b));
    /// assert_eq!(false, some_true.into_iter().all_or_none(|b| *b));
    /// assert_eq!(true, none_true.into_iter().all_or_none(|b| *b));
    /// ```
    fn all_or_none<P: FnMut(&Self::Item) -> bool>(self, predicate: P) -> bool;

    /// Consumes the iterator and returns true if exactly half of the items passed the predicate; as all things should be.
    ///
    /// # Example
    /// ```rust
    /// use exotic_iter::*;
    /// let two_even = vec![1, 2, 3, 4];
    /// let three_even = vec![1, 2, 4, 6];
    /// assert_eq!(true, two_even.into_iter().perfectly_balanced(|n| *n % 2 == 0));
    /// assert_eq!(false, three_even.into_iter().perfectly_balanced(|n| *n % 2 == 0));
    /// ```
    fn perfectly_balanced<P: FnMut(&Self::Item) -> bool>(self, predicate: P) -> bool;

    /// Creates an iterator that alternates between items from `self` and `other`, with `self` providing the first value.
    /// Short-circuits at the first `None` returned, even if one of the iterators still has values left.
    ///
    /// # Example
    /// ```rust
    /// use exotic_iter::*;
    /// let odd_numbers = vec![1, 3];
    /// let even_numbers = vec![2, 4];
    /// let mut iter = odd_numbers.iter().alternate(even_numbers.iter());
    /// assert_eq!(Some(&1), iter.next());
    /// assert_eq!(Some(&2), iter.next());
    /// assert_eq!(Some(&3), iter.next());
    /// assert_eq!(Some(&4), iter.next());
    /// assert_eq!(None, iter.next());
    /// ```
    fn alternate<U: IntoIterator<Item = Self::Item>>(self, other: U) -> Alternate<Self, U::IntoIter>;
}

impl<T: Iterator> ExoticIteratorExt for T {    
    #[inline]
    fn at_least<P: FnMut(&Self::Item) -> bool>(self, n: usize, predicate: P) -> bool {
        self.filter(predicate).take(n).count() == n
    }
    
    #[inline]
    fn at_most<P: FnMut(&Self::Item) -> bool>(self, n: usize, predicate: P) -> bool {
        self.filter(predicate).skip(n).count() == 0
    }
    
    fn exactly_n<P: FnMut(&Self::Item) -> bool>(self, n: usize, predicate: P) -> bool {
        let mut predicate = predicate;
        let mut count: usize = 0;
        for item in self {
            if predicate(&item) {
                count += 1;
            }
            if count > n {
                return false
            }
        }
        count == n
    }
    
    fn exactly_m_n<
        Pm: FnMut(&Self::Item) -> bool, 
        Pn: FnMut(&Self::Item) -> bool,
    >(self, m: usize, predicate_m: Pm, n: usize, predicate_n: Pn) -> bool {
        let mut predicate_m = predicate_m;
        let mut predicate_n = predicate_n;
        let mut count_m: usize = 0;
        let mut count_n: usize = 0;
        for item in self {
            if predicate_m(&item) {
                count_m += 1;
            }
            if predicate_n(&item) {
                count_n += 1;
            }
            if count_m > m || count_n > n {
                return false
            }
        }
        (count_m, count_n) == (m, n)
    }
    
    fn perfectly_balanced<P: FnMut(&Self::Item) -> bool>(self, predicate: P) -> bool {
        let mut predicate = predicate;
        let mut count: usize = 0;
        let mut total: usize = 0;
        for item in self {
            total += 1;
            if predicate(&item) {
                count += 1;
            }
        }
        total % 2 == 0 && total / 2 == count
    }
    
    fn all_or_none<P: FnMut(&Self::Item) -> bool>(self, predicate: P) -> bool {
        let mut predicate = predicate;
        let mut has_pass = false;
        let mut has_fail = false;
        for item in self {
            if predicate(&item) {
                has_pass = true;
            } else {
                has_fail = true;
            }
            if has_pass && has_fail {
                return false
            }
        }
        true
    }

    fn alternate<U: IntoIterator<Item = Self::Item>>(self, other: U) -> Alternate<Self, <U as IntoIterator>::IntoIter> {
        Alternate::new(self, other.into_iter())
    }
}