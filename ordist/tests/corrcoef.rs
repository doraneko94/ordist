use ordist::corrcoef::*;
use ordist::random::*;

#[test]
fn daniels() {
    let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let v1 = v.shuffle();
    let v2 = v.shuffle();

    let lo: f64 = spearman_corrcoef(&v1, &v2).unwrap();
    let tau: f64 = kendall_corrcoef(&v1, &v2).unwrap();
    let m = v1.len() as f64;
    let left = 3.0 * (m + 2.0) * tau / (m - 2.0);
    let right = 2.0 * (m + 1.0) * lo / (m - 2.0);
    println!("-1 <= {} <= 1", left - right);
}