use super::perm::Perm;

pub trait Patt {
    fn occurrences_in(self, pi: Self) -> Vec<Vec<usize>>;
    fn len(self) -> usize;
    fn get_perm(self) -> Perm;
    fn contains(self, pi: Self) -> bool;
}