use crate::automaton::{Automata, Automaton};
use crate::utils::append_hashset;
use crate::{
    dfa::{ToDfa, DFA},
    nfa::{ToNfa, NFA},
};
use std::cmp::{Ordering, Ordering::*};
use std::collections::HashSet;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::{Bound::*, RangeBounds};
use Operations::*;

#[derive(Debug, Clone)]
pub struct Regex<V: Eq + Hash + Display + Copy + Clone + Debug> {
    pub(crate) alphabet: HashSet<V>,
    pub(crate) regex: Operations<V>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Operations<V: Eq + Hash + Display + Copy + Clone + Debug> {
    Union(Vec<Operations<V>>),
    Concat(Vec<Operations<V>>),
    Repeat(Box<Operations<V>>, usize, Option<usize>),
    Letter(V),
    Epsilon,
    Dot,
}

pub trait ToRegex<V: Eq + Hash + Display + Copy + Clone + Debug> {
    fn to_regex(&self) -> Regex<V>;
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug> ToDfa<V> for Regex<V> {
    fn to_dfa(&self) -> DFA<V> {
        self.to_nfa().to_dfa()
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug> ToNfa<V> for Regex<V> {
    fn to_nfa(&self) -> NFA<V> {
        self.regex.to_nfa(&self.alphabet)
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug> ToRegex<V> for Regex<V> {
    fn to_regex(&self) -> Regex<V> {
        self.clone()
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug> Regex<V> {
    pub fn simplify(&mut self) {
        self.regex = self.to_dfa().minimize().to_regex().regex
    }

    pub fn contains(&self, other: &Regex<V>) -> bool {
        self.to_nfa().contains(&other.to_nfa())
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug> Operations<V> {
    fn to_nfa(&self, alphabet: &HashSet<V>) -> NFA<V> {
        match self {
            Union(v) => v
                .iter()
                .fold(NFA::new_length(alphabet.clone(), 0), |acc, x| {
                    acc.unite(x.to_nfa(alphabet))
                }),
            Concat(v) => v
                .iter()
                .fold(NFA::new_length(alphabet.clone(), 0), |acc, x| {
                    acc.concatenate(x.to_nfa(alphabet))
                }),
            Repeat(a, min, max) => {
                if let Some(max) = max {
                    a.to_nfa(alphabet).repeat(*min..=(*max))
                } else {
                    a.to_nfa(alphabet).repeat((*min)..)
                }
            }
            Letter(a) => NFA::new_matching(alphabet.clone(), &vec![*a]),
            Epsilon => NFA::new_length(alphabet.clone(), 0),
            Dot => NFA::new_length(alphabet.clone(), 1),
        }
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug> ToString for Operations<V> {
    fn to_string(&self) -> String {
        match self {
            Union(v) => v.iter().fold(String::new(), |mut acc, x| {
                acc.push('|');
                acc.push_str(x.to_string().as_str());
                acc
            }),
            Concat(v) => {
                let mut acc = String::new();
                v.iter().for_each(|x| acc.push_str(x.to_string().as_str()));
                acc
            }
            Repeat(a, 0, None) => format!("({})*", a.to_string()),
            Repeat(a, 1, None) => format!("({})+", a.to_string()),
            Repeat(a, 0, Some(1)) => format!("({})?", a.to_string()),
            Repeat(a, 0, max) => {
                if let Some(max) = max {
                    format!("({}){{,{}}}", a.to_string(), max)
                } else {
                    format!("({})*", a.to_string())
                }
            }
            Repeat(a, min, max) => {
                if let Some(max) = max {
                    if min == max {
                        format!("({}){{{}}}", a.to_string(), min)
                    } else {
                        format!("({}){{{},{}}}", a.to_string(), min, max)
                    }
                } else {
                    format!("({}){{{},}}", a.to_string(), min)
                }
            }
            Letter(a) => format!("{}", a),
            Epsilon => format!("𝜀"),
            Dot => format!("."),
        }
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug> Automata<V, Regex<V>> for Regex<V> {
    fn unite(mut self, b: Regex<V>) -> Regex<V> {
        append_hashset(&mut self.alphabet, b.alphabet);
        self.regex = Union(vec![self.regex, b.regex]);
        self
    }

    fn concatenate(mut self, b: Regex<V>) -> Regex<V> {
        append_hashset(&mut self.alphabet, b.alphabet);
        self.regex = Concat(vec![self.regex, b.regex]);
        self
    }

    fn kleene(mut self) -> Regex<V> {
        self.regex = Repeat(Box::new(self.regex), 0, None);
        self
    }

    fn at_most(mut self, u: usize) -> Regex<V> {
        self.regex = Repeat(Box::new(self.regex), 0, Some(u));
        self
    }

    fn at_least(mut self, u: usize) -> Regex<V> {
        self.regex = Repeat(Box::new(self.regex), u, None);
        self
    }

    fn repeat<R: RangeBounds<usize>>(mut self, r: R) -> Regex<V> {
        let start = match r.start_bound() {
            Included(&a) => a,
            Excluded(&a) => a + 1,
            Unbounded => 0,
        };

        let end = match r.end_bound() {
            Included(&a) => Some(a),
            Excluded(&a) => Some(a - 1),
            Unbounded => None,
        };

        self.regex = Repeat(Box::new(self.regex), start, end);
        self
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug> PartialEq<Regex<V>> for Regex<V> {
    fn eq(&self, b: &Regex<V>) -> bool {
        self.le(&b) && self.ge(&b)
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug> PartialEq<NFA<V>> for Regex<V> {
    fn eq(&self, b: &NFA<V>) -> bool {
        self.to_nfa().eq(b)
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug> PartialEq<DFA<V>> for Regex<V> {
    fn eq(&self, b: &DFA<V>) -> bool {
        self.to_nfa().eq(&b.to_nfa())
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug> PartialEq<Automaton<V>> for Regex<V> {
    fn eq(&self, b: &Automaton<V>) -> bool {
        match b {
            Automaton::DFA(v) => self.eq(&**v),
            Automaton::NFA(v) => self.eq(&**v),
            Automaton::REG(v) => self.eq(&**v),
        }
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug> PartialOrd for Regex<V> {
    fn partial_cmp(&self, other: &Regex<V>) -> Option<Ordering> {
        match (self.ge(&other), self.le(&other)) {
            (true, true) => Some(Equal),
            (true, false) => Some(Greater),
            (false, true) => Some(Less),
            (false, false) => None,
        }
    }

    fn lt(&self, other: &Regex<V>) -> bool {
        other.contains(&self) && !self.contains(&other)
    }

    fn le(&self, other: &Regex<V>) -> bool {
        other.contains(&self)
    }

    fn gt(&self, other: &Regex<V>) -> bool {
        self.contains(&other) && !other.contains(&self)
    }

    fn ge(&self, other: &Regex<V>) -> bool {
        self.contains(&other)
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug> ToString for Regex<V> {
    fn to_string(&self) -> String {
        self.regex.to_string()
    }
}
