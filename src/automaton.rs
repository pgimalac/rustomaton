use crate::regex::Regex;
use crate::{dfa::DFA, nfa::NFA};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::RangeBounds;

#[derive(Debug)]
pub enum Automaton<V: Eq + Hash + Display + Copy + Clone + Debug> {
    DFA(Box<DFA<V>>),
    NFA(Box<NFA<V>>),
    REG(Box<Regex<V>>),
}

pub trait Automata<V: Eq + Hash + Display + Copy + Clone + Debug, T> {
    fn unite(self, b: T) -> T;
    fn concatenate(self, b: T) -> T;
    fn kleene(self) -> T;
    fn at_most(self, u: usize) -> T;
    fn at_least(self, u: usize) -> T;
    fn repeat<R: RangeBounds<usize>>(self, r: R) -> T;
}

pub trait Runnable<V: Eq + Hash + Display + Copy + Clone + Debug> {
    fn run(&self, v: &Vec<V>) -> bool;
}
