use super::patt::{Patt, PattOccurrence};
use std::collections::VecDeque;

type Element = usize;

pub struct Perm {
    val: Box<[Element]>,
}

impl Perm {
    //TODO iterator element should not be Element, but should instead implement Into<Element>
    pub fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = Element>,
    {
        Perm {
            val: iter.collect(),
        }
    }

    pub fn pattern_details(&self) -> Vec<(Option<usize>, Option<usize>, usize, usize)> {
        //TODO: this should probably be one function with left_inf_sup
        self.val.iter().zip(self.left_inf_sup()).map(|(val, left_inf_sup)| {
            let space_below = match left_inf_sup.0 {
                Some(inf) => val - self.val[inf],
                None => *val,
            };

            let space_above = match left_inf_sup.1 {
                Some(sup) => self.val[sup] - val,
                None => self.len() - val,
            };

            (left_inf_sup.0, left_inf_sup.1, space_below, space_above)
        }).collect()
    }

    pub fn left_inf_sup(&self) -> Vec<(Option<usize>, Option<usize>)> {
        let mut vec = vec![];
    
        //deq in permuta is a list ordered by values that also keeps the indecies

        //TODO: since we know all the values ahead of time (0 - n-1), maybe we can be clever
        //while adding to the deq to avoid needing to shuffle elements
        //PREMATURE OPTIMIZATION... but it'd be fun :)

        let mut deq = VecDeque::with_capacity(self.len());
        let mut left_floor_ceiling = None;

        for (idx, val) in self.val.iter().enumerate() {
            match left_floor_ceiling {
                None => {
                    deq.push_back((*val, idx));
                    left_floor_ceiling = Some((val, val));
                    vec.push((None, None));
                },
                Some((floor, _)) if val < floor => {
                    while deq.front().unwrap().0 != *floor {
                        deq.rotate_left(1);
                    }
                    vec.push((None, Some(deq.front().unwrap().1)));
                    deq.push_front((*val, idx));
                    left_floor_ceiling = Some((val, left_floor_ceiling.unwrap().1)); //TODO: this is ugly
                },
                Some((_, ceiling)) if val > ceiling => {
                    while deq.back().unwrap().0 != *ceiling {
                        deq.rotate_left(1);
                    }
                    vec.push((Some(deq.back().unwrap().1), None));
                    deq.push_back((*val, idx));
                    left_floor_ceiling = Some((left_floor_ceiling.unwrap().0, val));
                },
                Some((_, _)) => {
                    // Rotate until deq.back.unwrap.0 <= val <= deq.front.unwrap.0
                    while deq.back().unwrap().0 > *val || deq.front().unwrap().0 < *val {
                        deq.rotate_right(1);
                    }
                    vec.push((Some(deq.back().unwrap().1), Some(deq.front().unwrap().1)));
                    deq.push_front((*val, idx));
                },
            }
        }

        vec
    }
}

impl Patt for Perm {
    fn len(&self) -> usize {
        self.val.len()
    }

    fn get_perm(&self) -> &Perm {
        self
    }
    
    fn patt_iter(&self, patt: &impl Patt) -> impl Iterator<Item = PattOccurrence>
    where
        Self: Sized
    {
        [vec![0]].into_iter() //todo! macro doesn't work with existential types for some reason
        // //https://github.com/rust-lang/rust/issues/68610
        
    }
}

struct IntoPattIter<T: Patt + Sized>{
    perm: T,
    patt: Perm,
    curr: PattOccurrence,
}

impl<T: Patt + Sized> Iterator for IntoPattIter<T> {
    type Item = PattOccurrence;

    fn next(&mut self) -> Option<Self::Item> {
        let pi = self.perm.get_perm();
        let n = self.patt.len();

        if n == 0 || n > pi.len() {
            return None;
        }

        let patt_details = self.patt.pattern_details();

        let prev = &self.curr;

        todo!()
    }
}
