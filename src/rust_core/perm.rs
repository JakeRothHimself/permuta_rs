use super::patt::Patt;

type Element = usize;

pub struct Perm {
    val: Box<[Element]>,
}

impl Perm {
    //TODO iterator element should not be Element, but should instead implement Into<Element>
    fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = Element>,
    {
        Perm { val: iter.collect() }
    }
}

impl Patt for Perm {
    fn occurrences_in(self, pi: Self) -> Vec<Vec<usize>> {
        todo!()
    }

    fn len(self) -> usize {
        self.val.len()
    }

    fn get_perm(self) -> Perm {
        self
    }

    fn contains(self, pi: Self) -> bool {
        todo!()
    }
}