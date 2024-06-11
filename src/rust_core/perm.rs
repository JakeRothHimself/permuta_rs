use super::patt::{Patt, PattOccurrence};
use std::collections::VecDeque;

type Element = usize;

#[derive(Clone)]
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

    pub fn pattern_details(&self) -> PattDetails {
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
    
    fn patt_iter(&self, patt: &impl Patt) -> IntoPattIter<Self>
    where
        Self: Sized
    {
        let patt = patt.get_perm().clone();
        let patt_details = patt.pattern_details();
        IntoPattIter {
            perm: self.clone(),
            patt,
            patt_details,
            curr: None,
        }
    }
}

type PattDetails = Vec<(Option<usize>, Option<usize>, usize, usize)>;

pub struct IntoPattIter<T: Patt + Sized>{
    perm: T,
    patt: Perm,
    patt_details: PattDetails,
    curr: Option<PattOccurrence>,
}

impl<T: Patt + Sized> Iterator for IntoPattIter<T> {
    type Item = PattOccurrence;

    fn next(&mut self) -> Option<Self::Item> {
        if self.patt.len() == 0 || self.patt.len() > self.perm.get_perm().len() {
            return None;
        }

        let (mut i, mut next_occ) = match self.curr.clone() {
            Some(mut occ) => {
                (occ.pop().unwrap() + 1, occ)
            },
            None => (0, vec![]),
        };

        self.curr = loop {
            match self.occurrences(i, next_occ.clone()) {
                Some(occ) => break Some(occ),
                None => {
                    if next_occ.is_empty() {
                        break None;
                    } else {
                        (i, next_occ) = (next_occ.pop().unwrap() + 1, next_occ);
                    }
                }
            }
        };

        self.curr.clone()
    }
}

impl<T: Patt> IntoPattIter<T> {
    //TODO: better name
        // i = position in pi to search from
        // next = vec of indecies in occurrence we have so far
        fn occurrences(&self, mut i: usize, mut next_occ: Vec<usize>) -> Option<PattOccurrence> {
            let pi = self.perm.get_perm();

            let n = self.patt.len();
            let k = next_occ.len();

            let (left_inf, left_sup, floor_dist, ceil_dist) = self.patt_details[k];
            let lower_bound = match left_inf {
                //smallest element of the pattern parsed so far
                None => {
                    //if we need to smallest element of the occurrence so far, then it can be at most the kth element of the pattern
                    //e.g. if we need to find the second element of a 210 occurrence, then that element must be at most 1, because if
                    //it was 0, then there wouldn't be a smaller element for the third element
                    floor_dist
                },
                Some(inf) => {
                    //next element must be at most as far from its left inf as patt[k] is from its left inf
                    //so we are leaving room for the next elements in the occurrence
                    //e.g. 3021 in 41032, if 4 and 1 is part of the occurrence, then the third element must be at most 
                    //3, because if it was 2 or less, there wouldn't be an element between the second and third elements (1 and 2)
                    //to form the fourth element in the pattern

                    let occ_left_inf = pi.val[next_occ[inf]];
                    occ_left_inf + floor_dist
                },
            };
            let upper_bound = match left_sup {
                //largest element of the pattern parsed so far
                None => {
                    pi.len() - ceil_dist
                },
                Some(sup) => {
                    let occ_left_sup = pi.val[next_occ[sup]];
                    occ_left_sup - ceil_dist
                },
            };

            loop {
                let elmts_needed = n - k;
                let perm_elmts_left = pi.len() - i;

                if perm_elmts_left < elmts_needed {
                    return None;
                }
                
                let element = pi.val[i];
                if (lower_bound..=upper_bound).contains(&element) {
                    next_occ.push(i);

                    if elmts_needed == 1 {
                        return Some(next_occ);
                    }
                    else {
                        if let Some(occ) = self.occurrences(i + 1, next_occ.clone()) {
                            return Some(occ);
                        }
                    }
                }

                i += 1;
            }
        }
}
