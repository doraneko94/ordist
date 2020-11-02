use std::fmt;
use std::hash::Hash;

pub trait OrDistElement: Hash + Eq + Clone + fmt::Debug {}
impl<T: Hash + Eq + Clone + fmt::Debug> OrDistElement for T {}
