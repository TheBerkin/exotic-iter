/// Provides additional convenience methods to the `Iterator` trait and its implementors.
pub trait ExoticIterator: Iterator {
    fn at_least<P: FnMut(&Self::Item) -> bool>(self, n: usize, predicate: P) -> bool;
    fn at_most<P: FnMut(&Self::Item) -> bool>(self, n: usize, predicate: P) -> bool;
    fn any_n<P: FnMut(&Self::Item) -> bool>(self, n: usize, predicate: P) -> bool;
    fn all_or_none<P: FnMut(&Self::Item) -> bool>(self, predicate: P) -> bool;
    fn perfectly_balanced<P: FnMut(&Self::Item) -> bool>(self, predicate: P) -> bool;
}

impl<T: Iterator> ExoticIterator for T {
    /// Consumes the iterator, counting the number of items that pass the predicate and returns true iff there were at least `n` passing items.
    fn at_least<P: FnMut(&Self::Item) -> bool>(self, n: usize, predicate: P) -> bool {
        self.filter(predicate).take(n).count() == n
    }

    /// Consumes the iterator, counting the number of items that pass the predicate and returns true iff there were no more than `n` passing items.
    fn at_most<P: FnMut(&Self::Item) -> bool>(self, n: usize, predicate: P) -> bool {
        self.filter(predicate).take(n).count() <= n
    }

    /// Consumes the iterator, counting the number of items that pass the predicate and returns true iff there were exactly `n` passing items in the entire iterator.
    fn any_n<P: FnMut(&Self::Item) -> bool>(self, n: usize, predicate: P) -> bool {
        self.filter(predicate).count() == n
    }

    /// Consumes the iterator and returns true if exactly half of the items passed the predicate.
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

    /// Consumes the iterator and returns true iff either all of the items in the iterator passed the predicate, or none of them passed.
    /// Returns early on the first mixed result.
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
}