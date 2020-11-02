use std::iter::FromIterator;

use crate::error::OrDistError;
use crate::ordmap::OrdMap;
use crate::traits::OrDistElement;

pub fn check_identical<K: OrDistElement>(
    om1: &OrdMap<K>,
    om2: &OrdMap<K>,
) -> Result<(), OrDistError<K>> {
    match om1.identical(&om2) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn ordmaps_identical<'a, K: OrDistElement>(
    v1: &'a [K],
    v2: &'a [K],
) -> Result<(OrdMap<&'a K>, OrdMap<&'a K>), OrDistError<K>> {
    let om1 = OrdMap::from_iter(v1);
    let om2 = OrdMap::from_iter(v2);
    check_identical(&om1, &om2)?;
    Ok((om1, om2))
}

pub fn ordmaps_identical_owned<K: OrDistElement>(
    v1: &[K],
    v2: &[K],
) -> Result<(OrdMap<K>, OrdMap<K>), OrDistError<K>> {
    let om1 = OrdMap::from_iter(Vec::from(v1));
    let om2 = OrdMap::from_iter(Vec::from(v2));
    check_identical(&om1, &om2)?;
    Ok((om1, om2))
}

pub fn sub_abs(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

pub fn swap_vec<T: OrDistElement>(v: &mut [T], i1: usize, i2: usize) {
    let tmp = v[i1].clone();
    v[i1] = v[i2].clone();
    v[i2] = tmp;
}

pub fn swap_ordmap<K: OrDistElement>(om: &mut OrdMap<K>, k1: &K, k2: &K) {
    let tmp = om.get(k1);
    *(om.data.get_mut(k1).unwrap()) = om.get(k2);
    *(om.data.get_mut(k2).unwrap()) = tmp;
}
