use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;

#[derive(Debug)]
pub struct FDA<V: Eq + Hash + Display + Copy + Clone> {
    alphabet: HashSet<V>,
    initial: usize,
    finals: HashSet<usize>,
    transitions: Vec<HashMap<V, usize>>,
}
