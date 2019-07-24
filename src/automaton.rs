use crate::automaton::Automaton::*;
use crate::regex::Regex;
use crate::{
    dfa::DFA,
    nfa::{ToNfa, NFA},
};
use std::cmp::{Ordering, Ordering::*, PartialEq, PartialOrd};
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

pub trait Runnable<V: Eq + Hash + Display + Copy + Clone + Debug, Rhs = Self> {
    fn run(&self, v: &Vec<V>) -> bool;

    fn is_complete(&self) -> bool;
    fn is_reachable(&self) -> bool;
    fn is_coreachable(&self) -> bool;
    fn is_trimmed(&self) -> bool;
    fn is_empty(&self) -> bool;
    fn is_full(&self) -> bool;

    fn negate(self) -> Rhs;
    fn complete(self) -> Rhs;
    fn make_reachable(self) -> Rhs;
    fn make_coreachable(self) -> Rhs;
    fn trim(self) -> Rhs;
    fn reverse(self) -> Rhs;
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug> Automaton<V> {
    pub fn contains(&self, other: &Automaton<V>) -> bool {
        let a = match self {
            DFA(a) => a.to_nfa(),
            NFA(a) => a.to_nfa(),
            REG(a) => a.to_nfa(),
        };
        let b = match other {
            DFA(b) => b.to_nfa(),
            NFA(b) => b.to_nfa(),
            REG(b) => b.to_nfa(),
        };

        a.contains(&b)
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug> PartialEq<Automaton<V>> for Automaton<V> {
    fn eq(&self, b: &Automaton<V>) -> bool {
        self.le(b) && self.ge(b)
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug> PartialEq<DFA<V>> for Automaton<V> {
    fn eq(&self, b: &DFA<V>) -> bool {
        self.eq(&b.to_nfa())
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug> PartialEq<Regex<V>> for Automaton<V> {
    fn eq(&self, b: &Regex<V>) -> bool {
        self.eq(&b.to_nfa())
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug> PartialEq<NFA<V>> for Automaton<V> {
    fn eq(&self, b: &NFA<V>) -> bool {
        match self {
            Automaton::DFA(v) => b.eq(&**v),
            Automaton::NFA(v) => b.eq(&**v),
            Automaton::REG(v) => b.eq(&**v),
        }
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug> PartialOrd for Automaton<V> {
    fn partial_cmp(&self, other: &Automaton<V>) -> Option<Ordering> {
        match (self.ge(&other), self.le(&other)) {
            (true, true) => Some(Equal),
            (true, false) => Some(Greater),
            (false, true) => Some(Less),
            (false, false) => None,
        }
    }

    fn lt(&self, other: &Automaton<V>) -> bool {
        other.contains(&self) && !self.contains(&other)
    }

    fn le(&self, other: &Automaton<V>) -> bool {
        other.contains(&self)
    }

    fn gt(&self, other: &Automaton<V>) -> bool {
        self.contains(&other) && !other.contains(&self)
    }

    fn ge(&self, other: &Automaton<V>) -> bool {
        self.contains(&other)
    }
}
