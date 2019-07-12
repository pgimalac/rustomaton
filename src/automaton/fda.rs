pub mod fda {
    use crate::automaton::fnda::fnda::FNDA;
    use std::collections::{HashMap, HashSet};
    use std::fmt::Display;
    use std::hash::Hash;

    #[derive(Debug)]
    pub struct FDA<V: Eq + Hash + Display + Copy + Clone> {
        pub(crate) alphabet: HashSet<V>,
        pub(crate) initial: usize,
        pub(crate) finals: HashSet<usize>,
        pub(crate) transitions: Vec<HashMap<V, usize>>,
    }

    impl<V: Eq + Hash + Display + Copy + Clone> FDA<V> {
        pub fn to_fnda(&self) -> FNDA<V> {
            let mut initials = HashSet::new();
            initials.insert(self.initial);
            let transitions = Vec::new();
            FNDA {
                alphabet: self.alphabet.clone(),
                initials,
                finals: self.finals.clone(),
                transitions,
            }
        }

        pub fn intersect(self, b: FDA<V>) -> FNDA<V> {
            self.to_fnda().intersect(b.to_fnda())
        }

        pub fn union(self, b: FDA<V>) -> FNDA<V> {
            self.to_fnda().union(b.to_fnda())
        }

        pub fn concatenate(self, b: FDA<V>) -> FNDA<V> {
            self.to_fnda().concatenate(b.to_fnda())
        }

        pub fn negate(&self) {
            self.to_fnda().negate();
        }

        pub fn minimise(&self) {
            self.to_fnda().minimise();
        }

        pub fn kleene(&self) {
            self.to_fnda().kleene();
        }

        pub fn complete(&self) {
            self.to_fnda().complete();
        }

        pub fn make_reachable(&self) {
            self.to_fnda().make_reachable();
        }

        pub fn make_coreachable(&self) {
            self.to_fnda().make_coreachable();
        }

        pub fn trim(&self) {
            self.to_fnda().trim();
        }

        pub fn reverse(&self) {
            self.to_fnda().reverse();
        }

        pub fn contains(&self, b: &FDA<V>) -> bool {
            self.to_fnda().contains(&b.to_fnda())
        }

        pub fn is_complete(&self) -> bool {
            self.to_fnda().is_complete()
        }

        pub fn is_reachable(&self) -> bool {
            self.to_fnda().is_reachable()
        }

        pub fn is_coreachable(&self) -> bool {
            self.to_fnda().is_coreachable()
        }

        pub fn is_trimmed(&self) -> bool {
            self.to_fnda().is_trimmed()
        }

        pub fn is_empty(&self) -> bool {
            self.to_fnda().is_empty()
        }

        pub fn is_full(&self) -> bool {
            self.to_fnda().is_full()
        }
    }
}
