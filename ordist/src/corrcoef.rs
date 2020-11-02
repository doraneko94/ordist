use ndarray::*;
use num_traits::Float;

use crate::distances::*;
use crate::error::OrDistError;
use crate::traits::OrDistElement;

pub fn spearman_corrcoef<T: OrDistElement, U: Float>(v1: &[T], v2: &[T])
-> Result<U, OrDistError<T>> {
    let d_spear = U::from(spearman_dist(v1, v2)?).unwrap();
    let m = U::from(v1.len()).unwrap();
    let one = U::one();
    Ok(one - U::from(6).unwrap() * d_spear / (m * (m * m - one)))
}

pub fn kendall_corrcoef<T: OrDistElement, U: Float>(v1: &[T], v2: &[T])
-> Result<U, OrDistError<T>> {
    let d_ken = U::from(kendall_dist(v1, v2)?).unwrap();
    let m = U::from(v1.len()).unwrap();
    let one = U::one();
    Ok(one - U::from(4).unwrap() * d_ken / (m * (m - one)))
}

pub trait OrDistCorrCoef<T: OrDistElement, U: Float> {
    fn spearman_corrcoef(&self, v: &Self) -> Result<U, OrDistError<T>>;
    fn kendall_corrcoef(&self, v: &Self) -> Result<U, OrDistError<T>>;
}

impl<T, U> OrDistCorrCoef<T, U> for Vec<T>
where
    T: OrDistElement,
    U: Float,
{
    fn spearman_corrcoef(&self, v: &Self) -> Result<U, OrDistError<T>> {
        spearman_corrcoef(self, v)
    }
    fn kendall_corrcoef(&self, v: &Self) -> Result<U, OrDistError<T>> {
        kendall_corrcoef(self, v)
    }
}

impl<A, S, U> OrDistCorrCoef<A, U> for ArrayBase<S, Ix1>
where
    A: OrDistElement,
    S: Data<Elem = A>,
    U: Float,
{
    fn spearman_corrcoef(&self, v: &Self) -> Result<U, OrDistError<A>> {
        self.to_vec().spearman_corrcoef(&v.to_vec())
    }
    fn kendall_corrcoef(&self, v: &Self) -> Result<U, OrDistError<A>> {
        self.to_vec().kendall_corrcoef(&v.to_vec())
    }
}