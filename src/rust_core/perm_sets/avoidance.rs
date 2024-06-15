use std::collections::HashMap;

use crate::rust_core::{
    patt::Patt,
    perm::{Element, Perm},
};

pub struct AvoidanceClass<P: Patt> {
    basis: Vec<P>,
    pub perm_cache: Vec<HashMap<Perm, Option<Vec<usize>>>>,
}

impl<P: Patt> AvoidanceClass<P> {
    pub fn new(basis: Vec<P>) -> Self {
        let mut map = HashMap::new();
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

        while self.perm_cache.len() <= n {
            let curr_perm_len = self.perm_cache.len() - 1;
            let (left, right) = self.perm_cache.split_at_mut(curr_perm_len);
            let prev_cache_level = &mut left.last_mut();
            let curr_cache_level = &mut right[0];

            let mut perms = HashMap::new();

            for (perm, cached_elements) in curr_cache_level.iter_mut() {
                let values = Self::valid_values(&perm, max_patt_len, &prev_cache_level.as_deref());
                for value in values {
                    let new_perm = perm.append(value);

                    if self.basis.iter().any(|patt| patt.get_perm() != &new_perm) {
                        perms.insert(new_perm, None);
                        match cached_elements {
                            Some(vec) => vec.push(value),
                            None => {let _ = cached_elements.replace(vec![value]);},
                        }
                    }
                }
            }

            if let Some(level) = prev_cache_level {
                level.values_mut().for_each(|x| *x = None);
            }

            self.perm_cache.push(perms);
        }
    }

    fn valid_values(perm: &Perm, max_patt_len: usize, prev_cache_level: &Option<&HashMap<Perm, Option<Vec<usize>>>>) -> Vec<usize> {
        //should only happen the first time through
        if let None = prev_cache_level {
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

            let mut potential_values: Vec<usize> = vec![];
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
}
