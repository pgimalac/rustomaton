pub mod dfa {
    use crate::automaton::nfa::nfa::NFA;
    use std::collections::{HashMap, HashSet};
    use std::fmt::{Debug, Display};
    use std::hash::Hash;

    #[derive(Debug)]
    pub struct DFA<V: Eq + Hash + Display + Copy + Clone + Debug> {
        pub(crate) alphabet: HashSet<V>,
        pub(crate) initial: usize,
        // in case the automaton is empty
        pub(crate) finals: HashSet<usize>,
        pub(crate) transitions: Vec<HashMap<V, usize>>,
    }

    impl<V: Eq + Hash + Display + Copy + Clone + Debug> DFA<V> {
        pub fn to_nfa(&self) -> NFA<V> {
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

        pub fn intersect(mut self, mut b: DFA<V>) -> DFA<V> {
            self.negate();
            b.negate();
            let mut aut = self.unite(b);
            aut.negate()
        }

        pub fn unite(self, b: DFA<V>) -> NFA<V> {
            self.to_nfa().unite(b.to_nfa())
        }

        pub fn concatenate(self, b: DFA<V>) -> NFA<V> {
            self.to_nfa().concatenate(b.to_nfa())
        }

        pub fn negate(&mut self) {
            self.complete();
            self.finals = (0..self.transitions.len())
                .into_iter()
                .filter(|x| !self.finals.contains(&x))
                .collect();
        }

        pub fn minimise(&self) {
            unimplemented!()
        }

        pub fn kleene(&self) -> NFA<V> {
            let mut aut = self.to_nfa();
            aut.kleene();
            aut
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
            let mut aut = self.to_nfa();
            aut.make_reachable();
            aut
        }

        pub fn make_coreachable(&self) -> NFA<V> {
            let mut aut = self.to_nfa();
            aut.make_coreachable();
            aut
        }

        pub fn trim(&self) -> NFA<V> {
            let mut aut = self.to_nfa();
            aut.trim();
            aut
        }

        pub fn reverse(&self) -> NFA<V> {
            let mut aut = self.to_nfa();
            aut.reverse();
            aut
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
            loop {
                if let Some(e) = stack.pop() {
                    for (_, v) in &self.transitions[e] {
                        if !acc.contains(&v) {
                            acc.insert(*v);
                            stack.push(*v);
                        }
                    }
                } else {
                    break;
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

        pub fn run(&self, v: &Vec<V>) -> bool {
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
}
