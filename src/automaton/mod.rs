pub mod dfa;
pub mod nfa;

pub mod automaton {
    use crate::automaton::dfa::dfa::DFA;
    use crate::automaton::nfa::nfa::NFA;
    use std::fmt::Display;
    use std::hash::Hash;

    #[derive(Debug)]
    pub enum Automaton<V: Eq + Hash + Display + Copy + Clone> {
        DFA(Box<DFA<V>>),
        NFA(Box<NFA<V>>),
    }

    // fn intersect(self, b: Automaton<V>) -> Automaton<V>;
    // fn union(self, b: Automaton<V>) -> Automaton<V>;
    // fn concatenate(self, b: Automaton<V>) -> Automaton<V>;

    // fn negate(self) -> Automaton<V>;
    // fn minimise(self) -> Automaton<V>;
    // fn kleene(self) -> Automaton<V>;
    // fn complete(self) -> Automaton<V>;
    // fn accessible(self) -> Automaton<V>;
    // fn coaccessible(self) -> Automaton<V>;
    // fn trim(self) -> Automaton<V>;
    // fn reverse(self) -> Automaton<V>;

    // fn copy(&self) -> Automaton<V>;

    // pub fn contains(&self, b: &Automaton<V>) -> bool {}
    // fn equals(&self, b: &Automaton<V>) -> bool;

    // fn is_complete(&self) -> bool;
    // fn is_accessible(&self) -> bool;
    // fn is_coaccessible(&self) -> bool;
    // fn is_trimmed(&self) -> bool {
    //     self.is_accessible() && self.is_coaccessible()
    // }
}
