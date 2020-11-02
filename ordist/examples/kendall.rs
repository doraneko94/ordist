use ordist::distances::*;
use ordist::permutation::*;
fn main() {
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![3, 4, 1, 2, 5];
    println!("{} == {}", kendall_dist(&v1, &v2).unwrap(), n_circle(&v1, &v2).unwrap());
}