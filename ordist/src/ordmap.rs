use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use crate::error::OrDistError;
use crate::traits::OrDistElement;

pub struct OrdMap<K: OrDistElement> {
    pub data: HashMap<K, usize>,
}

impl<K: OrDistElement> FromIterator<K> for OrdMap<K> {
    fn from_iter<T: IntoIterator<Item = K>>(iter: T) -> Self {
        let mut data = HashMap::new();
        for (i, x) in iter.into_iter().enumerate() {
            data.insert(x.clone(), i);
        }
        Self { data }
    }
}

impl<K: OrDistElement> OrdMap<K> {
    pub fn get(&self, key: &K) -> usize {
        self.data.get(key).unwrap().clone()
    }

    pub fn identical(&self, rhs: &Self) -> Result<(), OrDistError<K>> {
        let n = self.data.len();
        if n != rhs.data.len() {
            return Err(OrDistError::DifferentLength {
                left: n,
                right: rhs.data.len(),
            });
        }
        let mut key = HashSet::new();
        let mut val = vec![0; n];
        for (k, &v) in self.data.iter() {
            key.insert(k);
            if v >= n {
                return Err(OrDistError::RankOutOfLength {
                    length: n,
                    rank: v,
                    elem: k.clone(),
                });
            }
            if val[v] == 1 {
                return Err(OrDistError::DuplicateRank(v));
            } else {
                val[v] = 1;
            }
        }
        for (k, &v) in rhs.data.iter() {
            if !key.remove(k) {
                return Err(OrDistError::ElementNotFound(k.clone()));
            }
            if v >= n {
                return Err(OrDistError::RankOutOfLength {
                    length: n,
                    rank: v,
                    elem: k.clone(),
                });
            }
            if val[v] == 0 {
                return Err(OrDistError::DuplicateRank(v));
            } else {
                val[v] = 0;
            }
        }
        Ok(())
    }
}
