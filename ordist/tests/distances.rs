use ordist::distances::*;
use ordist::random::*;

#[test]
fn diaconis_graham() {
    let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let v1 = v.shuffle();
    let v2 = v.shuffle();

    let d_ken = kendall_dist(&v1, &v2).unwrap();
    let d_foot = footrule_dist(&v1, &v2).unwrap();
    let d_cay = cayley_dist(&v1, &v2).unwrap();
    assert!(d_ken + d_cay <= d_foot);
    assert!(d_foot <= d_ken * 2);
}

#[test]
fn durbin_stuart() {
    let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let v1 = v.shuffle();
    let v2 = v.shuffle();

    let m = v1.len() as f64;
    let d_spear = spearman_dist(&v1, &v2).unwrap() as f64;
    let d_ken = kendall_dist(&v1, &v2).unwrap() as f64;
    assert!(d_spear >= 4.0 / 3.0 * d_ken * (1.0 + d_ken / m));
}