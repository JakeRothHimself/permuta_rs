use std::{cmp::min, collections::HashMap, ops::Index, sync::{Arc, Mutex}};
use indexmap::IndexMap;
use rayon::prelude::*;

use crate::rust_core::{
    patt::Patt,
    perm::{Element, Perm},
};

pub struct AvoidanceClass<P: Patt> {
    basis: Vec<P>,
    pub perm_cache: PermCache,
}

type PermCache = Vec<CacheLevel>;
type CacheLevel = CacheMap<Perm, Option<Vec<Element>>>;
type CacheMap<K, V> = IndexMap<K, V>;

impl<P: Patt + Send + Sync> AvoidanceClass<P> {
    pub fn new(basis: Vec<P>) -> Self {
        let mut map = CacheMap::new();
        map.insert(Perm::new([]), None);
        let perm_cache = vec![map];

        Self { basis, perm_cache }
    }

    pub fn build_perm_class(&mut self, n: usize) {
        let max_patt_len = self
            .basis
            .iter()
            .max_by(|a, b| a.len().cmp(&b.len()))
            .expect("patterns in the basis")
            .len();

        let num_threads = rayon::current_num_threads();
        //TODO: change the cache structure such that we hold all perms of a given length in a vector
        //      and we hold a map with references to those Perms as keys and valid appended values as values
        //      this way, we can pass "chunks" of the immutable perm vector to threads and only allowing those
        //      threads to mutate the map. We cannot do this with the current implementation because rayon's
        //      par_chunks() method requires that we use a collection that has indecies so it can use slices as
        //      chunks and also verify that all slices are disjoint

        while self.perm_cache.len() <= n {
            let curr_perm_len = self.perm_cache.len() - 1;
            let (left, right) = self.perm_cache.split_at_mut(curr_perm_len);
            let prev_cache_level = &mut left.last_mut();
            let curr_cache_level = &mut right[0];

            let perms = Arc::new(Mutex::new(CacheMap::new()));

            let chunk_size = min(curr_cache_level.len().div_ceil(num_threads), 500_000);
            curr_cache_level.par_iter_mut().chunks(chunk_size).for_each(|chunk| {
                for (perm, cached_elements) in chunk {
                    let mut appended_values = vec![];
                    let values = Self::valid_values(perm, max_patt_len, &prev_cache_level.as_deref());
                    for value in values {
                        let new_perm = perm.append(value);

                        if curr_perm_len + 1 > max_patt_len || !self.basis.iter().any(|patt| patt.get_perm() == &new_perm) {
                            perms.lock().unwrap().insert(new_perm, None);
                            appended_values.push(value);
                        }
                    }

                    cached_elements.replace(appended_values);
                }
            });

            //clear prev cache level now that we're done with it
            if let Some(level) = prev_cache_level {
                level.values_mut().for_each(|x| *x = None);
            }

            let lock = Arc::try_unwrap(perms).expect("lock should have no other owners");
            self.perm_cache.push(lock.into_inner().expect("no other threads holding lock"));
        }
    }

    fn valid_values(perm: &Perm, max_patt_len: usize, prev_cache_level: &Option<&CacheLevel>) -> Vec<Element> {
        //should only happen the first time through
        if prev_cache_level.is_none() {
            return vec![0];
        }

        let mut values: Option<Vec<Element>> = None;

        let n = perm.len();
        let start = n.saturating_sub(max_patt_len);

        for i in start..n {
            let val = perm.val[i];
            let sub_perm = perm.remove(i);

            let elements = prev_cache_level.as_ref().expect("non-empty cache")
                .get(&sub_perm)
                .expect("Permutation to be in cache")
                .as_ref()
                .expect("Permutation to have cached extensions");

            let mut potential_values: Vec<Element> = vec![];
            for &k in elements {
                if k <= val {
                    potential_values.push(k);
                }
                if k >= val {
                    potential_values.push(k + 1);
                }
            }

            //we're allocating new vectors a lot here
            //this might be fine since we're dropping them immediately, but might need to be improved later
            values = match values {
                Some(vec) => {
                    let mut tmp_vec = Vec::with_capacity(n+1);

                    for val in &potential_values {
                        if vec.contains(val) {
                            tmp_vec.push(*val);
                        }
                    }

                    Some(tmp_vec)
                }
                None => Some(potential_values),
            };
        }

        values.expect("values to be initialized")
    }

    pub fn perms_of_length(&self, n: usize) -> impl Iterator<Item = &Perm> {
        self.perm_cache[n].iter().map(|entry| entry.0)
    }
}
