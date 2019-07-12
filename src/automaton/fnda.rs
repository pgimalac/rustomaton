pub mod fnda {
    use crate::automaton::automaton::Automaton;
    use crate::automaton::fda::fda::FDA;
    use std::cmp::PartialEq;
    use std::collections::{HashMap, HashSet};
    use std::fmt::Display;
    use std::hash::Hash;
    use std::iter::repeat;

    /* AUXILIARY FUNCTIONS */

    fn append_hashset<V: Eq + Hash>(a: &mut HashSet<V>, b: HashSet<V>) {
        for v in b {
            a.insert(v);
        }
    }

    fn shift_hashset(a: &mut HashSet<usize>, l: usize) {
        for e in a.drain().collect::<Vec<usize>>() {
            a.insert(e + l);
        }
    }

    fn shift_fnda<V: Eq + Hash + Display + Copy + Clone>(a: &mut FNDA<V>, l: usize) {
        shift_hashset(&mut a.initials, l);
        shift_hashset(&mut a.finals, l);
        shift_transitions(&mut a.transitions, l);
    }

    fn shift_transitions<V: Eq + Hash>(a: &mut Vec<HashMap<V, Vec<usize>>>, l: usize) {
        for map in a {
            for (_, v) in map {
                v.iter_mut().for_each(|t| *t += l);
            }
        }
    }

    fn append_transitions<V: Eq + Hash>(
        a: &mut Vec<HashMap<V, Vec<usize>>>,
        mut b: Vec<HashMap<V, Vec<usize>>>,
    ) {
        shift_transitions(&mut b, a.len());
        a.append(&mut b);
    }

    #[derive(Debug, Clone)]
    pub struct FNDA<V: Eq + Hash + Display + Copy + Clone> {
        pub(crate) alphabet: HashSet<V>,
        pub(crate) initials: HashSet<usize>,
        pub(crate) finals: HashSet<usize>,
        pub(crate) transitions: Vec<HashMap<V, Vec<usize>>>,
    }

    /* IMPLEMENTATION OF FNDA */

    impl<V: Eq + Hash + Display + Copy + Clone> FNDA<V> {
        pub fn intersect(self, mut _b: FNDA<V>) -> FNDA<V> {
            unimplemented!()
        }

        pub fn union(mut self, b: FNDA<V>) -> FNDA<V> {
            let FNDA {
                alphabet,
                initials,
                finals,
                transitions,
            } = b;

            append_hashset(&mut self.alphabet, alphabet);
            append_hashset(&mut self.initials, initials);
            append_transitions(&mut self.transitions, transitions);
            append_hashset(&mut self.finals, finals);

            self
        }

        pub fn concatenate(mut self, mut b: FNDA<V>) -> FNDA<V> {
            let l = self.transitions.len();
            shift_fnda(&mut b, l);
            let FNDA {
                alphabet,
                mut initials,
                finals,
                mut transitions,
            } = b;

            append_hashset(&mut self.alphabet, alphabet);

            for e in &initials {
                for (v, mut t) in &mut transitions[e - l] {
                    // e - l because of the shift above
                    for f in &self.finals {
                        self.transitions[*f]
                            .entry(*v)
                            .or_insert(Vec::new())
                            .append(&mut t);
                    }
                }
            }

            if finals.is_disjoint(&mut initials) {
                self.finals = finals;
            } else {
                append_hashset(&mut self.finals, finals);
            }
            self.transitions.append(&mut transitions);

            self
        }

        pub fn negate(&mut self) {
            let mut finals = HashSet::new();
            self.complete();
            for u in 0..self.transitions.len() {
                if !self.finals.contains(&u) {
                    finals.insert(u);
                }
            }
            self.finals = finals;
        }

        pub fn minimise(&mut self) {
            unimplemented!()
        }

        pub fn kleene(&mut self) {
            unimplemented!()
        }

        pub fn complete(&mut self) {
            if self.is_complete() {
                return;
            }

            let l = self.transitions.len();
            self.transitions.push(HashMap::new());
            for m in &mut self.transitions {
                for v in &self.alphabet {
                    let t = m.entry(*v).or_insert(Vec::new());
                    if t.is_empty() {
                        t.push(l);
                    }
                }
            }
        }

        pub fn make_reachable(&mut self) {
            let mut acc: HashSet<usize> = self.initials.clone().into_iter().collect();
            let mut stack: Vec<usize> = self.initials.iter().cloned().collect();
            loop {
                if let Some(e) = stack.pop() {
                    for (_, v) in &self.transitions[e] {
                        for t in v {
                            if !acc.contains(t) {
                                acc.insert(*t);
                                stack.push(*t);
                            }
                        }
                    }
                } else {
                    break;
                }
            }

            let mut map = HashMap::new();
            let mut ind = 0;
            let l = self.transitions.len();
            for i in 0..l {
                if acc.contains(&i) {
                    map.insert(i, ind);
                    self.transitions.swap(i, ind);
                    ind += 1;
                }
            }
            self.transitions.truncate(ind);

            self.finals = self
                .finals
                .iter()
                .filter(|x| acc.contains(&x))
                .map(|x| *map.get(x).unwrap())
                .collect();
            // no need to filter the initials since they must be reachable
            self.initials = self.initials.iter().map(|x| *map.get(x).unwrap()).collect();
            for m in &mut self.transitions {
                for v in m.values_mut() {
                    for t in v {
                        *t = *map.get(t).unwrap();
                    }
                }
            }
        }

        pub fn make_coreachable(&mut self) {
            self.reverse();
            self.make_reachable();
            self.reverse();
        }

        pub fn trim(&mut self) {
            self.make_reachable();
            self.make_coreachable();
        }

        pub fn reverse(&mut self) {
            let mut transitions: Vec<HashMap<V, Vec<usize>>> = repeat(HashMap::new())
                .take(self.transitions.len())
                .collect();

            for i in 0..self.transitions.len() {
                for (k, v) in &self.transitions[i] {
                    for e in v {
                        transitions[*e].entry(*k).or_insert(Vec::new()).push(i);
                    }
                }
            }

            self.transitions = transitions;
            std::mem::swap(&mut self.initials, &mut self.finals);
        }

        pub fn contains(&self, b: &FNDA<V>) -> bool {
            let mut cpy_a = self.clone();
            let cpy_b = b.clone();
            cpy_a.negate();
            cpy_a.intersect(cpy_b).is_empty()
        }

        pub fn is_complete(&self) -> bool {
            for m in &self.transitions {
                for v in &self.alphabet {
                    if match m.get(v) {
                        None => true,
                        Some(v) => v.is_empty(),
                    } {
                        return false;
                    }
                }
            }
            return true;
        }

        pub fn is_reachable(&self) -> bool {
            let mut acc: HashSet<usize> = self.initials.clone().into_iter().collect();
            let mut stack: Vec<usize> = self.initials.iter().cloned().collect();
            loop {
                if let Some(e) = stack.pop() {
                    for (_, v) in &self.transitions[e] {
                        for t in v {
                            if !acc.contains(t) {
                                acc.insert(*t);
                                stack.push(*t);
                            }
                        }
                    }
                } else {
                    break;
                }
            }
            acc.len() == self.transitions.len()
        }

        pub fn is_coreachable(&self) -> bool {
            let mut rev = self.clone();
            rev.reverse();
            rev.is_reachable()
        }

        pub fn is_trimmed(&self) -> bool {
            self.is_reachable() && self.is_coreachable()
        }

        pub fn is_empty(&self) -> bool {
            if self.initials.is_disjoint(&self.finals) {
                return false;
            }

            let mut acc: HashSet<usize> = self.initials.clone().into_iter().collect();
            let mut stack: Vec<usize> = self.initials.clone().into_iter().collect();

            loop {
                if let Some(e) = stack.pop() {
                    for (_, v) in &self.transitions[e] {
                        for t in v {
                            if self.finals.contains(t) {
                                return false;
                            }
                            if !acc.contains(t) {
                                acc.insert(*t);
                                stack.push(*t);
                            }
                        }
                    }
                } else {
                    break;
                }
            }
            return true;
        }

        pub fn is_full(&self) -> bool {
            let mut cpy = self.clone();
            cpy.negate();
            cpy.is_empty()
        }
    }

    impl<V: Eq + Hash + Display + Copy + Clone> PartialEq<FNDA<V>> for FNDA<V> {
        fn eq(&self, b: &FNDA<V>) -> bool {
            self.contains(&b) && b.contains(self)
        }
    }

    impl<V: Eq + Hash + Display + Copy + Clone> PartialEq<FDA<V>> for FNDA<V> {
        fn eq(&self, b: &FDA<V>) -> bool {
            self.eq(&b.to_fnda())
        }
    }

    impl<V: Eq + Hash + Display + Copy + Clone> PartialEq<Automaton<V>> for FNDA<V> {
        fn eq(&self, b: &Automaton<V>) -> bool {
            match b {
                Automaton::FDA(v) => self.eq(&**v),
                Automaton::FNDA(v) => self.eq(&**v),
            }
        }
    }
}
