use crate::automaton::{Automata, Runnable};
use crate::nfa::{ToNfa, NFA};
use crate::regex::{Regex, ToRegex};
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::RangeBounds;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct DFA<V: Eq + Hash + Display + Copy + Clone + Debug> {
    pub(crate) alphabet: HashSet<V>,
    pub(crate) initial: usize,
    // in case the automaton is empty
    pub(crate) finals: HashSet<usize>,
    pub(crate) transitions: Vec<HashMap<V, usize>>,
}

pub trait ToDfa<V: Eq + Hash + Display + Copy + Clone + Debug> {
    fn to_dfa(&self) -> DFA<V>;
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug> DFA<V> {
    pub fn intersect(self, b: DFA<V>) -> DFA<V> {
        self.negate().unite(b.negate()).negate()
    }

    pub fn negate(mut self) -> DFA<V> {
        self.complete();
        self.finals = (0..self.transitions.len())
            .into_iter()
            .filter(|x| !self.finals.contains(&x))
            .collect();
        self
    }

    // Brzozowski
    pub fn minimize(&self) -> DFA<V> {
        self.reverse().to_dfa().reverse().to_dfa()
    }

    pub fn complete(&mut self) {
        if self.is_complete() {
            return;
        }

        let l = self.transitions.len();
        self.transitions.push(HashMap::new());
        for map in &mut self.transitions {
            for v in &self.alphabet {
                if !map.contains_key(&v) {
                    map.insert(*v, l);
                }
            }
        }
    }

    pub fn make_reachable(&self) -> NFA<V> {
        self.to_nfa().make_reachable()
    }

    pub fn make_coreachable(&self) -> NFA<V> {
        self.to_nfa().make_coreachable()
    }

    pub fn trim(&self) -> NFA<V> {
        self.to_nfa().trim()
    }

    pub fn reverse(&self) -> NFA<V> {
        self.to_nfa().reverse()
    }

    pub fn contains(&self, b: &DFA<V>) -> bool {
        self.to_nfa().contains(&b.to_nfa())
    }

    pub fn is_complete(&self) -> bool {
        for map in &self.transitions {
            for v in &self.alphabet {
                if !map.contains_key(&v) {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn is_reachable(&self) -> bool {
        let mut stack = vec![self.initial];
        let mut acc = HashSet::new();
        acc.insert(self.initial);
        while let Some(e) = stack.pop() {
            for (_, v) in &self.transitions[e] {
                if !acc.contains(&v) {
                    acc.insert(*v);
                    stack.push(*v);
                }
            }
        }
        return acc.len() == self.transitions.len();
    }

    pub fn is_coreachable(&self) -> bool {
        self.to_nfa().is_coreachable()
    }

    pub fn is_trimmed(&self) -> bool {
        self.to_nfa().is_trimmed()
    }

    pub fn is_empty(&self) -> bool {
        self.to_nfa().is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.to_nfa().is_full()
    }

    pub fn write_dot(&self, n: u8) -> Result<(), std::io::Error> {
        self.to_nfa().write_dot(n)
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug> Runnable<V> for DFA<V> {
    fn run(&self, v: &Vec<V>) -> bool {
        let mut actual = self.initial;
        for l in v {
            if let Some(t) = self.transitions[actual].get(l) {
                actual = *t;
            } else {
                return false;
            }
        }
        return self.finals.contains(&actual);
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug> Automata<V, DFA<V>> for DFA<V> {
    fn unite(self, b: DFA<V>) -> DFA<V> {
        self.to_nfa().unite(b.to_nfa()).to_dfa()
    }

    fn concatenate(self, b: DFA<V>) -> DFA<V> {
        self.to_nfa().concatenate(b.to_nfa()).to_dfa()
    }

    fn kleene(self) -> DFA<V> {
        self.to_nfa().kleene().to_dfa()
    }

    fn at_most(self, u: usize) -> DFA<V> {
        self.to_nfa().at_most(u).to_dfa()
    }

    fn at_least(self, u: usize) -> DFA<V> {
        self.to_nfa().at_least(u).to_dfa()
    }

    fn repeat<R: RangeBounds<usize>>(self, r: R) -> DFA<V> {
        self.to_nfa().repeat(r).to_dfa()
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug> ToDfa<V> for DFA<V> {
    fn to_dfa(&self) -> DFA<V> {
        self.clone()
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug> ToRegex<V> for DFA<V> {
    fn to_regex(&self) -> Regex<V> {
        unimplemented!()
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug> ToNfa<V> for DFA<V> {
    fn to_nfa(&self) -> NFA<V> {
        let mut initials = HashSet::new();
        initials.insert(self.initial);
        let mut transitions = Vec::new();
        for map in &self.transitions {
            transitions.push(map.iter().map(|(k, v)| (*k, vec![*v])).collect());
        }
        NFA {
            alphabet: self.alphabet.clone(),
            initials,
            finals: self.finals.clone(),
            transitions,
        }
    }
}

impl FromStr for DFA<char> {
    type Err = String;

    fn from_str(s: &str) -> Result<DFA<char>, Self::Err> {
        NFA::from_str(s).map(|x| x.to_dfa())
    }
}
