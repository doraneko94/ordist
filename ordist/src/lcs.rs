use ndarray::*;
use std::cmp::max;

use crate::traits::OrDistElement;

macro_rules! lcs_core {
    (
        $dp:ident,
        $v1:ident, $v2:ident,
        $n1:ident, $n2:ident
    ) => {
        let mut $dp = Array2::zeros(($n1+1, $n2+1));

        for i1 in 0..$n1 {
            for i2 in 0..$n2 {
                $dp[[i1+1, i2+1]] = if $v1[i1] == $v2[i2] {
                    $dp[[i1, i2]] + 1
                } else {
                    max($dp[[i1+1, i2]], $dp[[i1, i2+1]])
                }
            }
        }
    };
} // lcs_core!

pub fn lcs<T: OrDistElement>(v1: &[T], v2: &[T]) -> usize {
    let (n1, n2) = (v1.len(), v2.len());
    lcs_core!(dp, v1, v2, n1, n2);
    dp[[n1, n2]]
}

pub fn lcs_with_vec<T: OrDistElement>(v1: &[T], v2: &[T]) -> (usize, Vec<T>) {
    let (n1, n2) = (v1.len(), v2.len());
    lcs_core!(dp, v1, v2, n1, n2);
    let n_lcs = dp[[n1, n2]];
    let mut v = vec![v1[0].clone(); n_lcs];
    let mut pos = n_lcs - 1;
    let (mut i1, mut i2) = (n1 - 1, n2 - 1);
    loop {
        if v1[i1] == v2[i2] {
            v[pos] = v1[i1].clone();
            if pos == 0 {
                break;
            } else {
                pos -= 1;
                i1 -= 1;
                i2 -= 1;
            }
        } else {
            if dp[[i1, i2 + 1]] > dp[[i1 + 1, i2]] {
                i1 -= 1;
            } else {
                i2 -= 1;
            }
        }
    }
    (dp[[n1, n2]], v)
}

pub trait OrDistLCS {
    fn lcs(&self, v: &Self) -> usize;
    fn lcs_with_vec(&self, v: &Self) -> (usize, Self);
}

impl<T: OrDistElement> OrDistLCS for Vec<T> {
    fn lcs(&self, v: &Self) -> usize {
        lcs(self, v)
    }
    fn lcs_with_vec(&self, v: &Self) -> (usize, Self) {
        lcs_with_vec(self, v)
    }
}

impl<A, S> OrDistLCS for ArrayBase<S, Ix1>
where
    A: OrDistElement,
    S: Data<Elem = A> + DataOwned,
{
    fn lcs(&self, v: &Self) -> usize {
        self.to_vec().lcs(&v.to_vec())
    }
    fn lcs_with_vec(&self, v: &Self) -> (usize, Self) {
        let (n_lcs, v_lcs) = self.to_vec().lcs_with_vec(&v.to_vec());
        (n_lcs, ArrayBase::from(v_lcs))
    }
}