# ordist
Order distance package.

## usage
```
use ordist::distances::*;
use ordist::permutation::*;
fn main() {
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![3, 4, 1, 2, 5];
    println!(
        "{} == {}",
        cayley_dist(&v1, &v2).unwrap(),
        v1.len() - n_circle(&v1, &v2).unwrap()
    );
}
```