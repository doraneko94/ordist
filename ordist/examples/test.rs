use ordist::corrcoef::*;
use ordist::distances::*;
use ordist::lcs::*;
use ordist::permutation::*;
use ordist::random::*;

fn main() {
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![3, 4, 1, 2, 5];
    println!("{}", kendall_dist(&v1, &v2).unwrap());

    println!("{}", n_circle(&v1, &v2).unwrap());

    let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let v1 = v.shuffle();
    let v2 = v.shuffle();
    //let mut v2 = v.clone();
    //v2.reverse();
    println!("{}", v.len() - n_circle(&v1, &v2).unwrap());
    println!("{}", cayley_dist(&v1, &v2).unwrap());

    let d_ken = kendall_dist(&v1, &v2).unwrap();
    let d_foot = footrule_dist(&v1, &v2).unwrap();
    let d_cay = cayley_dist(&v1, &v2).unwrap();
    println!("{} <= {} <= {}", d_ken + d_cay, d_foot, d_ken * 2);

    println!("{:?}", v1);
    println!("{:?}", v2);
    println!("{} {}", v.len() - lcs(&v1, &v2), ulam_dist(&v1, &v2).unwrap());

    let lo: f64 = spearman_corrcoef(&v1, &v2).unwrap();
    let tau: f64 = kendall_corrcoef(&v1, &v2).unwrap();
    let m = v1.len() as f64;
    let left = 3.0 * (m + 2.0) * tau / (m - 2.0);
    let right = 2.0 * (m + 1.0) * lo / (m - 2.0);
    println!("-1 <= {} <= 1", left - right);

    let d_spear = spearman_dist(&v1, &v2).unwrap() as f64;
    let d_ken = kendall_dist(&v1, &v2).unwrap() as f64;
    println!("{} >= {}", d_spear, 4.0 / 3.0 * d_ken * (1.0 + d_ken / m));
}