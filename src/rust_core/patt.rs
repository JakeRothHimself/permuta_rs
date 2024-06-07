use super::perm::Perm;

pub type PattOccurrence = Vec<usize>;

pub trait Patt {
    fn patt_iter(&self, patt: &impl Patt) -> impl Iterator<Item = PattOccurrence>
    where
        Self: Sized;

    fn occurrences_in(&self, pi: &Self) -> Vec<PattOccurrence>
    where
        Self: Sized,
    {
        self.patt_iter(pi).collect()
    }

    fn len(&self) -> usize;

    fn get_perm(&self) -> &Perm;

    fn contains(&self, pi: &Self) -> bool
    where
        Self: Sized,
    {
        matches!(self.patt_iter(pi).next(), Some(_))
    }
}
