use super::perm::Perm;

pub type PattOccurrence = Vec<usize>;

pub trait Patt: Sized {
    fn patt_iter(&self, patt: &impl Patt) -> impl Iterator<Item = PattOccurrence>;

    fn occurrences_in(&self, pi: &Self) -> Vec<PattOccurrence>{
        self.patt_iter(pi).collect()
    }

    fn len(&self) -> usize;

    fn get_perm(&self) -> &Perm;

    fn contains(&self, pi: &Self) -> bool
    {
        self.patt_iter(pi).next().is_some()
    }
}
