use crate::error::OrDistError;
use crate::traits::OrDistElement;
use crate::utils::ordmaps_identical;

pub fn n_circle<K: OrDistElement>(v1: &[K], v2: &[K]) -> Result<usize, OrDistError<K>> {
    let (_, om2) = ordmaps_identical(v1, v2)?;
    let mut ans = 0;
    let n = v1.len();
    let mut v = vec![false; n];
    for i in 0..n {
        if !v[i] {
            v[i] = true;
            let mut e = &v1[i];
            loop {
                let p = om2.get(&e);
                if v[p] {
                    break;
                } else {
                    v[p] = true;
                    e = &v1[p];
                }
            }
            ans += 1;
        }
    }
    Ok(ans)
}