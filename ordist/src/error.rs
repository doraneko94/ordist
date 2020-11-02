use std::error;
use std::fmt;

use crate::traits::OrDistElement;

#[derive(Debug, PartialEq)]
pub enum OrDistError<K: OrDistElement> {
    DifferentLength { left: usize, right: usize },
    ElementNotFound(K),
    RankOutOfLength { length: usize, rank: usize, elem: K },
    DuplicateRank(usize),

}

impl<K: OrDistElement> fmt::Display for OrDistError<K> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OrDistError::DifferentLength { left, right } => {
                write!(f, "Different length!: left={}, right={}",
                left, right)
            }
            OrDistError::ElementNotFound(e) => {
                write!(f, "Element not found!: element={:?}", e)
            }
            OrDistError::RankOutOfLength { length, rank, elem } => {
                write!(f, "Rank out of length!: length={}, rank={}, element={:?}",
                length, rank, elem)
            }
            OrDistError::DuplicateRank(rank) => {
                write!(f, "Duplicate rank!: rank={}", rank)
            }
        }
    }
}

impl<K: OrDistElement> error::Error for OrDistError<K>
{
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl<K: OrDistElement> From<OrDistError<&K>> for OrDistError<K> {
    fn from(error: OrDistError<&K>) -> Self {
        match error {
            OrDistError::DifferentLength { left, right } => {
                OrDistError::DifferentLength { left, right }
            }
            OrDistError::ElementNotFound(e) => {
                OrDistError::ElementNotFound(e.clone())
            }
            OrDistError::RankOutOfLength { length, rank, elem } => {
                OrDistError::RankOutOfLength { length, rank, elem: elem.clone() }
            }
            OrDistError::DuplicateRank(rank) => {
                OrDistError::DuplicateRank(rank)
            }
        }
    }
}