//! Order distances
use ndarray::*;
use std::cmp::min;

use crate::error::OrDistError;
use crate::traits::OrDistElement;
use crate::utils::*;

/// Calculation mode in `spearman_footrule()`.
pub enum ModeSF {
    /// Calculating Spearman distance
    Spearman,
    /// Calculating Footrule distance
    Footrule,
}

/// The core function of Spearman & Footrule distance.
pub fn spearman_footrule<K: OrDistElement>(
    v1: &[K],
    v2: &[K],
    m: ModeSF,
) -> Result<usize, OrDistError<K>> {
    let (om1, om2) = ordmaps_identical(v1, v2)?;
    Ok(om1
        .data
        .keys()
        .map(|k| (om1.get(k), om2.get(k)))
        .map(|(r1, r2)| {
            let r = sub_abs(r1, r2);
            match m {
                ModeSF::Spearman => r * r,
                ModeSF::Footrule => r,
            }
        })
        .sum())
}

/// Spearman distance between 2 orders.
/// ```latex
/// d_{Spear}(x,y) = \sum_{i=1}^{n}(r_{xi}-r_{yi})^{2}
/// ```
/// `r_{xi}` is the rank in `x` of element `i`.
pub fn spearman_dist<K: OrDistElement>(v1: &[K], v2: &[K]) -> Result<usize, OrDistError<K>> {
    spearman_footrule(v1, v2, ModeSF::Spearman)
}

/// Footrule distance between 2 orders.
/// ```latex
/// d_{Foot}(x,y) = \sum_{i=1}^{n}|r_{xi}-r_{yi}|
/// ```
/// `r_{xi}` is the rank in `x` of element `i`.
pub fn footrule_dist<K: OrDistElement>(v1: &[K], v2: &[K]) -> Result<usize, OrDistError<K>> {
    spearman_footrule(v1, v2, ModeSF::Footrule)
}

/// Kendall distance between 2 orders.
/// 
/// The number of pairs of elements that are in mismatched order.
pub fn kendall_dist<K: OrDistElement>(v1: &[K], v2: &[K]) -> Result<usize, OrDistError<K>> {
    let (om1, om2) = ordmaps_identical(v1, v2)?;
    let keys: Vec<&&K> = om1.data.keys().collect();
    let n = keys.len();
    Ok((0..n)
        .map(|i| {
            let ri1 = om1.get(keys[i]);
            let ri2 = om2.get(keys[i]);
            (i + 1..n)
                .map({
                    |j| {
                        let rj1 = om1.get(keys[j]);
                        let rj2 = om2.get(keys[j]);
                        if ri1 > rj1 {
                            if ri2 > rj2 {
                                0
                            } else {
                                1
                            }
                        } else {
                            if ri2 < rj2 {
                                0
                            } else {
                                1
                            }
                        }
                    }
                })
                .sum::<usize>()
        })
        .sum())
}

/// Cayley distance between 2 orders.
/// 
/// The minimum number of procedures to exchange any element of `v2` 
/// that is required to convert `v2` to `v1`.
pub fn cayley_dist<K: OrDistElement>(v1: &[K], v2: &[K]) -> Result<usize, OrDistError<K>> {
    let (_, mut om2) = ordmaps_identical_owned(v1, v2)?;
    let mut v2 = Vec::from(v2);
    let mut ans = 0;
    for (i, e) in v1.iter().enumerate() {
        let exchange_val = om2.get(&e);
        if exchange_val != i {
            let exchange_key = v2[i].to_owned();
            swap_ordmap(&mut om2, &e, &&exchange_key);
            swap_vec(&mut v2, i, exchange_val);
            ans += 1;
        }
    }
    Ok(ans)
}

/// The core function of dynamic programming.
/// 
/// The value of dp[[i+1, j+1]] is the minimum value of
/// ```
/// dp[[i+1, j  ]] + 1
/// dp[[i  , j+1]] + 1
/// dp[[i  , j  ]] + rewrite_cost
/// ```
pub fn dp_core<T: OrDistElement>(v1: &[T], v2: &[T], rewrite_cost: usize) -> usize {
    let (n1, n2) = (v1.len(), v2.len());
    let mut dp = Array2::zeros((n1 + 1, n2 + 1));
    for i in 0..n1 + 1 {
        dp[[i, 0]] = i;
    }
    for i in 0..n2 + 1 {
        dp[[0, i]] = i;
    }

    for i1 in 0..n1 {
        for i2 in 0..n2 {
            let cost = if v1[i1] == v2[i2] { 0 } else { rewrite_cost };
            dp[[i1 + 1, i2 + 1]] = min(
                min(dp[[i1, i2 + 1]] + 1, dp[[i1 + 1, i2]] + 1),
                dp[[i1, i2]] + cost,
            );
        }
    }
    dp[[n1, n2]]
}

pub fn levenshtein_dist<T: OrDistElement>(v1: &[T], v2: &[T]) -> Result<usize, OrDistError<T>> {
    Ok(dp_core(v1, v2, 1))
}

pub fn ulam_dist<K: OrDistElement>(v1: &[K], v2: &[K]) -> Result<usize, OrDistError<K>> {
    let (_, _) = ordmaps_identical(v1, v2)?;
    Ok(dp_core(v1, v2, 2) / 2)
}

pub trait Ordist<T: OrDistElement> {
    fn spearman_dist(&self, v: &Self) -> Result<usize, OrDistError<T>>;
    fn footrule_dist(&self, v: &Self) -> Result<usize, OrDistError<T>>;
    fn kendall_dist(&self, v: &Self) -> Result<usize, OrDistError<T>>;
    fn cayley_dist(&self, v: &Self) -> Result<usize, OrDistError<T>>;
    fn levenshtein_dist(&self, v: &Self) -> Result<usize, OrDistError<T>>;
    fn ulam_dist(&self, v: &Self) -> Result<usize, OrDistError<T>>;
}

impl<T: OrDistElement> Ordist<T> for Vec<T> {
    fn spearman_dist(&self, v: &Self) -> Result<usize, OrDistError<T>> {
        spearman_dist(self, v)
    }
    fn footrule_dist(&self, v: &Self) -> Result<usize, OrDistError<T>> {
        footrule_dist(self, v)
    }
    fn kendall_dist(&self, v: &Self) -> Result<usize, OrDistError<T>> {
        kendall_dist(self, v)
    }
    fn cayley_dist(&self, v: &Self) -> Result<usize, OrDistError<T>> {
        cayley_dist(self, v)
    }
    fn levenshtein_dist(&self, v: &Self) -> Result<usize, OrDistError<T>> {
        levenshtein_dist(self, v)
    }
    fn ulam_dist(&self, v: &Self) -> Result<usize, OrDistError<T>> {
        ulam_dist(self, v)
    }
}

impl<A, S> Ordist<A> for ArrayBase<S, Ix1>
where
    A: OrDistElement,
    S: Data<Elem = A>,
{
    fn spearman_dist(&self, v: &Self) -> Result<usize, OrDistError<A>> {
        self.to_vec().spearman_dist(&v.to_vec())
    }
    fn footrule_dist(&self, v: &Self) -> Result<usize, OrDistError<A>> {
        self.to_vec().footrule_dist(&v.to_vec())
    }
    fn kendall_dist(&self, v: &Self) -> Result<usize, OrDistError<A>> {
        self.to_vec().kendall_dist(&v.to_vec())
    }
    fn cayley_dist(&self, v: &Self) -> Result<usize, OrDistError<A>> {
        self.to_vec().cayley_dist(&v.to_vec())
    }
    fn levenshtein_dist(&self, v: &Self) -> Result<usize, OrDistError<A>> {
        self.to_vec().levenshtein_dist(&v.to_vec())
    }
    fn ulam_dist(&self, v: &Self) -> Result<usize, OrDistError<A>> {
        self.to_vec().ulam_dist(&v.to_vec())
    }
}
