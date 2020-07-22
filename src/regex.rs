use crate::{
    automaton::{Automaton, Buildable},
    dfa::{ToDfa, DFA},
    nfa::{ToNfa, NFA},
    parser::*,
    utils::*,
};
use std::{
    cmp::{Ordering, Ordering::*},
    collections::{BTreeSet, HashSet, VecDeque},
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, AddAssign, Bound::*, Mul, RangeBounds},
    str::FromStr,
};
use Operations::*;

/// Represents a regex.
#[derive(Debug, Clone)]
pub struct Regex<V: Eq + Hash + Display + Copy + Clone + Debug> {
    pub(crate) alphabet: HashSet<V>,
    pub(crate) regex: Operations<V>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum Operations<V: Eq + Hash + Display + Copy + Clone + Debug> {
    Union(BTreeSet<Operations<V>>),
    Concat(VecDeque<Operations<V>>),
    Repeat(Box<Operations<V>>, usize, Option<usize>),
    Letter(V),
    Epsilon,
    Empty,
    Dot,
}

/// An interface for structs that can be converted into a Regex.
pub trait ToRegex<V: Eq + Hash + Display + Copy + Clone + Debug> {
    fn to_regex(&self) -> Regex<V>;
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug + Ord> ToDfa<V> for Regex<V> {
    fn to_dfa(&self) -> DFA<V> {
        self.to_nfa().to_dfa()
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug + Ord> ToNfa<V> for Regex<V> {
    fn to_nfa(&self) -> NFA<V> {
        self.regex.to_nfa(&self.alphabet)
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug> ToRegex<V> for Regex<V> {
    fn to_regex(&self) -> Regex<V> {
        self.clone()
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug + Ord> Regex<V> {
    /// Simplify the regex.
    pub fn simplify(self) -> Regex<V> {
        let Regex { alphabet, regex } = self;
        Regex {
            regex: regex.simplify(&alphabet),
            alphabet,
        }
    }

    /// A contains B if and only if for each `word` w, if B `accepts` w then A `accepts` w.
    pub fn contains(&self, other: &Regex<V>) -> bool {
        self.to_nfa().contains(&other.to_nfa())
    }
}

impl Regex<char> {
    /// Returns the Regex<char> struct corresponding to the given regex.
    pub fn parse_with_alphabet(
        alphabet: HashSet<char>,
        regex: &str,
    ) -> Result<Regex<char>, String> {
        let mut tokens = tokens(regex);
        if tokens.is_empty() {
            return Ok(Regex {
                alphabet,
                regex: Operations::Empty,
            });
        }

        let regex = read_union(&mut tokens)?;
        if !tokens.is_empty() {
            Err("Trailing characters.".to_string())
        } else if let Some(x) = regex.alphabet().into_iter().find(|x| !alphabet.contains(x)) {
            Err(format!("Letter {} is not in the given alphabet", x))
        } else {
            Ok(Regex { alphabet, regex })
        }
    }
}

/// Returns the Regex<char> struct corresponding to the given regex, the alphabet is composed of the letter used in the regexp (without '+', '*', '?', '.', '(', ')', '|', '𝜀').
impl FromStr for Regex<char> {
    type Err = String;

    fn from_str(s: &str) -> Result<Regex<char>, String> {
        let unauthorized: HashSet<char> = vec!['+', '*', '?', '.', '(', ')', '|', '𝜀']
            .into_iter()
            .collect();

        let alphabet: HashSet<char> = s.chars().filter(|x| !unauthorized.contains(&x)).collect();

        Regex::parse_with_alphabet(alphabet, s)
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug + Ord> Operations<V> {
    fn simplify_union(t: BTreeSet<Operations<V>>, alphabet: &HashSet<V>) -> Operations<V> {
        if t.iter().all(|x| x == &Empty) {
            return Empty;
        }

        let mut set = BTreeSet::new();
        for e in t.into_iter() {
            match e.simplify(alphabet) {
                Empty => {}
                Union(t) => {
                    for e in t {
                        set.insert(e);
                    }
                }
                x => {
                    set.insert(x);
                }
            }
        }

        if set.is_empty() {
            return Epsilon;
        } else if set.len() == 1 {
            return set.into_iter().next().unwrap();
        } else if set.contains(&Epsilon) && set.len() == 2 {
            return Repeat(
                Box::new(set.into_iter().find(|x| x != &Epsilon).unwrap()),
                0,
                Some(1),
            )
            .simplify(alphabet);
        }

        if set.iter().any(|x| match x {
            Repeat(_, 0, _) => true,
            _ => false,
        }) {
            set.remove(&Epsilon);
        }

        let facto = match set.iter().next().unwrap() {
            Concat(t) => t.front().unwrap(),
            x => x,
        }
        .clone();

        if set.iter().all(|x| match x {
            Concat(t) => &facto == t.front().unwrap(),
            x => &facto == x,
        }) {
            let mut new_set = BTreeSet::new();
            for e in set {
                match e {
                    Concat(mut t) => {
                        t.pop_front();
                        new_set.insert(Concat(t));
                    }
                    _ => {
                        new_set.insert(Epsilon);
                    }
                }
            }
            Concat(vec![facto, Union(new_set)].into_iter().collect()).simplify(alphabet)
        } else {
            Union(set)
        }
    }

    fn simplify_concat(v: VecDeque<Operations<V>>, alphabet: &HashSet<V>) -> Operations<V> {
        if v.iter().all(|x| x == &Epsilon) {
            return Epsilon;
        }

        let mut vec = VecDeque::with_capacity(v.len());
        for e in v.into_iter() {
            match e.simplify(alphabet) {
                Epsilon => {}
                Concat(v) => {
                    for e in v {
                        vec.push_back(e);
                    }
                }
                x => {
                    vec.push_back(x);
                }
            }
        }

        if vec.is_empty() {
            Empty
        } else if vec.len() == 1 {
            vec.pop_back().unwrap()
        } else {
            Concat(vec)
        }
    }

    fn simplify_repeat(
        o: Operations<V>,
        min: usize,
        max: Option<usize>,
        alphabet: &HashSet<V>,
    ) -> Operations<V> {
        match (min, max, o.simplify(alphabet)) {
            (0, Some(0), _) | (_, _, Epsilon) => Epsilon,
            (min, Some(max), _) if max < min => Epsilon,
            (1, Some(1), x) => x,
            (0, _, Empty) => Union(vec![Empty, Epsilon].into_iter().collect()),
            (_, _, Empty) => Empty,
            (_, _, Repeat(o, 0, None)) => Repeat(o, 0, None).simplify(alphabet),
            (0, None, Repeat(o, _min @ 0..=1, _)) => Repeat(o, 0, None).simplify(alphabet),
            (0, Some(1), Repeat(o, 0, Some(1))) => Repeat(o, 0, Some(1)).simplify(alphabet),
            (0, Some(1), Union(mut u)) => {
                u.remove(&Epsilon);
                if u.iter().all(|x| match x {
                    Repeat(_, 0, _) => false,
                    _ => true,
                }) {
                    u.insert(Epsilon);
                }
                Union(u).simplify(alphabet)
            }
            (0, max, Union(mut u)) => {
                u.remove(&Epsilon);
                if u.is_empty() {
                    Epsilon
                } else if u.len() == 1 {
                    Repeat(Box::new(u.into_iter().next().unwrap()), 0, max).simplify(alphabet)
                } else {
                    Repeat(Box::new(Union(u)), 0, max)
                }
            }
            (1, None, Repeat(o, 0, _)) => Repeat(o, 0, None),
            (min, max, x) => Repeat(Box::new(x), min, max),
        }
    }

    pub fn simplify(self, alphabet: &HashSet<V>) -> Self {
        match self {
            Union(t) => Operations::simplify_union(t, alphabet),
            Concat(v) => Operations::simplify_concat(v, alphabet),
            Repeat(o, min, max) => Operations::simplify_repeat(*o, min, max, alphabet),
            x => x,
        }
    }

    fn to_nfa(&self, alphabet: &HashSet<V>) -> NFA<V> {
        match self {
            Union(v) => v.iter().fold(NFA::new_empty(alphabet.clone()), |acc, x| {
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
            Letter(a) => NFA::new_matching(alphabet.clone(), &[*a]),
            Epsilon => NFA::new_length(alphabet.clone(), 0),
            Empty => NFA::new_empty(alphabet.clone()),
            Dot => NFA::new_length(alphabet.clone(), 1),
        }
    }

    pub(crate) fn alphabet(&self) -> HashSet<V> {
        let mut stack = vec![self];
        let mut alphabet = HashSet::new();

        while let Some(x) = stack.pop() {
            match x {
                Union(v) => v.iter().for_each(|x| stack.push(x)),
                Concat(v) => v.iter().for_each(|x| stack.push(x)),
                Repeat(o, _, _) => stack.push(&**o),
                Letter(v) => {
                    alphabet.insert(*v);
                }
                _ => {}
            }
        }

        alphabet
    }

    fn to_string(&self, alphabet: &HashSet<V>) -> String {
        match self {
            Union(v) => {
                if v.contains(&Epsilon)
                    && v.len() == alphabet.len() + 1
                    && contains_dot(&v, alphabet)
                {
                    return ".?".to_string();
                }

                let mut acc = String::new();
                if alphabet.iter().all(|x| v.contains(&Letter(*x))) {
                    acc.push('.');
                    acc.push('|');
                    for x in v.iter().filter(|x| match x {
                        Letter(_) => false,
                        _ => true,
                    }) {
                        acc.push_str(x.to_string(alphabet).as_str());
                        acc.push('|');
                    }
                } else {
                    for x in v {
                        acc.push_str(x.to_string(alphabet).as_str());
                        acc.push('|');
                    }
                }
                acc.pop();
                acc
            }
            Concat(v) => {
                let mut acc = String::new();
                for e in v {
                    match e {
                        Union(_) => {
                            acc.push('(');
                            acc.push_str(e.to_string(alphabet).as_str());
                            acc.push(')');
                        }
                        _ => acc.push_str(e.to_string(alphabet).as_str()),
                    }
                }
                acc
            }
            Repeat(a, 0, None) => format!("{}*", paren!(a.to_string(alphabet))),
            Repeat(a, 1, None) => format!("{}+", paren!(a.to_string(alphabet))),
            Repeat(a, 0, Some(1)) => format!("{}?", paren!(a.to_string(alphabet))),
            Repeat(a, 0, max) => {
                if let Some(max) = max {
                    format!("{}{{,{}}}", paren!(a.to_string(alphabet)), max)
                } else {
                    format!("{}*", paren!(a.to_string(alphabet)))
                }
            }
            Repeat(a, min, max) => {
                if let Some(max) = max {
                    if min == max {
                        format!("{}{{{}}}", paren!(a.to_string(alphabet)), min)
                    } else {
                        format!("{}{{{},{}}}", paren!(a.to_string(alphabet)), min, max)
                    }
                } else {
                    format!("{}{{{},}}", paren!(a.to_string(alphabet)), min)
                }
            }
            Letter(a) => a.to_string(),
            Epsilon => "𝜀".to_string(),
            Empty => "∅".to_string(),
            Dot => ".".to_string(),
        }
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug + Ord> Buildable<V> for Regex<V> {
    fn unite(mut self, b: Regex<V>) -> Regex<V> {
        append_hashset(&mut self.alphabet, b.alphabet);
        self.regex += b.regex;
        self
    }

    fn concatenate(mut self, b: Regex<V>) -> Regex<V> {
        append_hashset(&mut self.alphabet, b.alphabet);
        self.regex = self.regex * b.regex;
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

impl<V: Eq + Hash + Display + Copy + Clone + Debug + Ord> PartialEq<Regex<V>> for Regex<V> {
    fn eq(&self, b: &Regex<V>) -> bool {
        self.le(&b) && self.ge(&b)
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug + Ord> PartialEq<NFA<V>> for Regex<V> {
    fn eq(&self, b: &NFA<V>) -> bool {
        self.to_nfa().eq(b)
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug + Ord> PartialEq<DFA<V>> for Regex<V> {
    fn eq(&self, b: &DFA<V>) -> bool {
        self.to_nfa().eq(&b.to_nfa())
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug + Ord> PartialEq<Automaton<V>> for Regex<V> {
    fn eq(&self, b: &Automaton<V>) -> bool {
        match b {
            Automaton::DFA(v) => self.eq(&*v),
            Automaton::NFA(v) => self.eq(&*v),
            Automaton::REG(v) => self.eq(&*v),
        }
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug + Ord> PartialOrd for Regex<V> {
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

impl<V: Eq + Hash + Display + Copy + Clone + Debug + Ord> ToString for Regex<V> {
    fn to_string(&self) -> String {
        self.regex.to_string(&self.alphabet)
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug + Ord> Add for Regex<V> {
    type Output = Self;

    fn add(self, other: Regex<V>) -> Regex<V> {
        self.unite(other)
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug + Ord> Mul for Regex<V> {
    type Output = Self;

    fn mul(self, other: Regex<V>) -> Regex<V> {
        self.concatenate(other)
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug + Ord> Add for Operations<V> {
    type Output = Self;

    fn add(self, other: Operations<V>) -> Operations<V> {
        match (self, other) {
            (Union(mut v1), Union(v2)) => {
                for e in v2 {
                    v1.insert(e);
                }
                Union(v1)
            }
            (Empty, op) => op,
            (op, Empty) => op,
            (Union(mut v), op) => {
                v.insert(op);
                Union(v)
            }
            (op, Union(mut v)) => {
                v.insert(op);
                Union(v)
            }
            (op1, op2) => Union(vec![op1, op2].into_iter().collect()),
        }
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug> Mul for Operations<V> {
    type Output = Self;

    fn mul(self, other: Operations<V>) -> Operations<V> {
        match (self, other) {
            (Concat(mut v1), Concat(mut v2)) => {
                v1.append(&mut v2);
                Concat(v1)
            }
            (Epsilon, op) => op,
            (op, Epsilon) => op,
            (Empty, _) => Empty,
            (_, Empty) => Empty,
            (Concat(mut v), op) => {
                v.push_back(op);
                Concat(v)
            }
            (op, Concat(mut v)) => {
                v.push_front(op);
                Concat(v)
            }
            (op1, op2) => Concat(vec![op1, op2].into_iter().collect()),
        }
    }
}

impl<V: Eq + Hash + Display + Copy + Clone + Debug + Ord> AddAssign for Operations<V> {
    fn add_assign(&mut self, op: Operations<V>) {
        let mut tmp = Operations::Epsilon;
        std::mem::swap(&mut tmp, self);
        *self = tmp + op;
    }
}
