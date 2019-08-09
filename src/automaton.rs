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

///
/// Automaton<V> regroups [`NFA<V>`], [`DFA<V>`] and [`Regex<V>`] where `V` is the type of the [`alphabet`].
///
/// [`NFA<V>`]: dfa/struct.NFA.html
/// [`DFA<V>`]: dfa/struct.DFA.html
/// [`Regex<V>`]: dfa/struct.Regex.html
/// [`alphabet`]: https://en.wikipedia.org/wiki/Alphabet_(formal_languages)
///

#[derive(Debug)]
pub enum Automaton<V: Eq + Hash + Display + Copy + Clone + Debug> {
    /// This variant represents a [`DFA`](dfa/struct.DFA.html).
    DFA(DFA<V>),
    /// This variant represents a [`NFA`](dfa/struct.NFA.html).
    NFA(NFA<V>),
    /// This variant represents a [`Regex`](dfa/struct.Regex.html).
    REG(Regex<V>),
}

///
/// An interface to regroup functions used to build Automata.
///

pub trait Buildable<V: Eq + Hash + Display + Copy + Clone + Debug> {
    /// Returns the automata that accepts a word if and only if it is accepted by `self` or by `other`.
    fn unite(self, other: Self) -> Self;
    /// Returns the automata that accepts a word if and only if it is the concatenation of a word accepted by `self` and of a word accepted by `other`.
    fn concatenate(self, other: Self) -> Self;
    /// Returns the automata that accepts a word if and only if it is the concatenation of a finite number of words accepted by `self` (possibly 0).
    fn kleene(self) -> Self;
    /// Returns the automata that accepts a word if and only if it is the concatenation of at most `num` words accepted by `self`.
    fn at_most(self, num: usize) -> Self;
    /// Returns the automata that accepts a word if and only if it is the concatenation of at least `num` words accepted by `self`.
    fn at_least(self, num: usize) -> Self;
    /// Returns the automata that accepts a word if and only if it is the concatenation of a number in the range `r` of words accepted by `self`.
    fn repeat<R: RangeBounds<usize>>(self, r: R) -> Self;
}

///
/// An interface to regroup functions that can be called on an actual automaton, represented as an `alphabet`, a finite set of `states` (some of which are `initials` and/or `finals`) and a set of `transitions` from one `state` to another labeled by a `letter`.
///
/// # Complete automaton
/// An automaton is said `complete` if for each `state` and each `letter`, there is a `transition` from that `state` with that `letter`.
///
/// # Reachable automaton
/// An automaton is said `reachable` if for each `state`, there is a (possibly empty) `path` starting from that `state` and ending in a `final state`.
///
/// # Coreachable automaton
/// An automaton is said `coreachable` if for each `state`, there is a (possibly empty) `path` from a `starting` state to that `state`.
///
/// # Trimmed automaton
/// An automaton is said `trimmed` if it is [`reachable`] and [`coreachable`].
///
/// [`reachable`]: ./trait.Automata.html#reachable-automaton
/// [`coreachable`]: ./trait.Automata.html#reachable-automaton
///
/// # Empty automaton
/// An automaton is said `empty` if it doesn't accept any `word`.
///
/// # Full automaton
/// An automaton is said `full` if it accepts any `word`.
///

pub trait Automata<V: Eq + Hash + Display + Copy + Clone + Debug> {
    /// Returns `true` if and only if `word` is accepted by `self`.
    fn run(&self, word: &Vec<V>) -> bool;

    /// Returns `true` if and only if `self` is [`complete`](./trait.Automata.html#complete-automaton).
    fn is_complete(&self) -> bool;
    /// Returns `true` if and only if `self` is [`reachable`](./trait.Automata.html#reachable-automaton).
    fn is_reachable(&self) -> bool;
    /// Returns `true` if and only if `self` is [`coreachable`](./trait.Automata.html#coreachable-automaton).
    fn is_coreachable(&self) -> bool;
    /// Returns `true` if and only if `self` is [`trimmed`](./trait.Automata.html#trimmed-automaton).
    fn is_trimmed(&self) -> bool;
    /// Returns `true` if and only if `self` is [`empty`](./trait.Automata.html#empty-automaton).
    fn is_empty(&self) -> bool;
    /// Returns `true` if and only if `self` is [`full`](./trait.Automata.html#full-automaton).
    fn is_full(&self) -> bool;

    /// Returns an automaton that accepts the same words as `self` but is [`complete`](./trait.Automata.html#complete-automaton).
    fn complete(self) -> Self;
    /// Returns an automaton that accepts the same words as `self` but is [`reachable`](./trait.Automata.html#reachable-automaton).
    fn make_reachable(self) -> Self;
    /// Returns an automaton that accepts the same words as `self` but is [`coreachable`](./trait.Automata.html#coreachable-automaton).
    fn make_coreachable(self) -> Self;
    /// Returns an automaton that accepts the same words as `self` but is [`trimmed`](./trait.Automata.html#trimmed-automaton).
    fn trim(self) -> Self;
    /// Returns an automaton that accepts a word if and only if `self` doesn't accept this word.
    fn negate(self) -> Self;
    /// Returns an automaton that accepts a word if and only if `self` accepts the reversed word.
    fn reverse(self) -> Self;
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug> Automaton<V> {
    /// A contains B if and only if for each `word` w, if B `accepts` w then A `accepts` w.
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
    fn eq(&self, other: &Automaton<V>) -> bool {
        self.le(other) && self.ge(other)
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug> PartialEq<DFA<V>> for Automaton<V> {
    fn eq(&self, other: &DFA<V>) -> bool {
        self.eq(&other.to_nfa())
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug> PartialEq<Regex<V>> for Automaton<V> {
    fn eq(&self, other: &Regex<V>) -> bool {
        self.eq(&other.to_nfa())
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug> PartialEq<NFA<V>> for Automaton<V> {
    fn eq(&self, other: &NFA<V>) -> bool {
        match self {
            Automaton::DFA(v) => other.eq(&*v),
            Automaton::NFA(v) => other.eq(&*v),
            Automaton::REG(v) => other.eq(&*v),
        }
    }
}

/// The partial ordering on two automatons A and B is defined as A < B if and only if B [`contains`] A.
///
/// [`contains`]: ./enum.Automaton.html#method.contains
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
