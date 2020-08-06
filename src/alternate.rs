/// An iterator that iterates over two iterators alternately.
#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Alternate<A, B> {
    a: A,
    b: B,
    odd: bool,
    done: bool,
}

impl<
    A: Iterator<Item = T>, 
    B: Iterator<Item = T>, 
    T
    > Alternate<A, B> {
    pub(crate) fn new(a: A, b: B) -> Self {
        Self {
            a,
            b,
            odd: false,
            done: false,
        }
    }
}

impl<A: Iterator<Item = T>, B: Iterator<Item = T>, T> Iterator for Alternate<A, B> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.done { return None }

        let item = if self.odd {
            self.b.next()
        } else {
            self.a.next()
        };

        if item.is_none() {
            self.done = true;
        }
        
        self.odd = !self.odd;
        item
    }
}