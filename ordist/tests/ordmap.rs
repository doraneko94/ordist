use ordist::error::*;
use ordist::ordmap::*;

use std::iter::FromIterator;

#[test]
fn error() {
    let v1 = vec![1, 2, 3, 4, 5];
    let om1 = OrdMap::from_iter(v1);

    let v2 = vec![1, 3, 5, 2, 4];
    let om2 = OrdMap::from_iter(v2);
    assert_eq!(om1.identical(&om2).unwrap(), ());

    let v3 = vec![1, 2, 3, 4, 5, 6];
    let om3 = OrdMap::from_iter(v3);
    match om1.identical(&om3) {
        Err(OrDistError::DifferentLength { left: _, right: _ }) => {}
        _ => assert!(false),
    }

    let v4 = vec![1, 2, 3, 4, 6];
    let om4 = OrdMap::from_iter(v4);
    match om1.identical(&om4) {
        Err(OrDistError::ElementNotFound(_)) => {}
        _ => assert!(false),
    }

    let v5 = vec![1, 2, 3, 4, 5];
    let mut om5 = OrdMap::from_iter(v5);
    *(om5.data.get_mut(&2).unwrap()) = 10;
    match om1.identical(&om5) {
        Err(OrDistError::RankOutOfLength {
            length: _,
            rank: _,
            elem: _,
        }) => {}
        _ => assert!(false),
    }

    *(om5.data.get_mut(&2).unwrap()) = 0;
    match om1.identical(&om5) {
        Err(OrDistError::DuplicateRank(_)) => {}
        _ => assert!(false),
    }
}
