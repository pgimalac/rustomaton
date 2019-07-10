pub mod fnda {
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
                for u in v.iter_mut() {
                    *u += l;
                }
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

    #[derive(Debug)]
    pub struct FNDA<V: Eq + Hash + Display + Copy + Clone> {
        alphabet: HashSet<V>,
        initials: HashSet<usize>,
        finals: HashSet<usize>,
        transitions: Vec<HashMap<V, Vec<usize>>>,
    }

    /* IMPLEMENTATION OF FNDA */

    impl<V: Eq + Hash + Display + Copy + Clone> FNDA<V> {
        pub fn intersect(mut self, mut b: FNDA<V>) -> FNDA<V> {
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

        pub fn accessible(&mut self) {
            unimplemented!()
        }

        pub fn coaccessible(&mut self) {
            unimplemented!()
        }

        pub fn trim(&mut self) {
            self.accessible();
            self.coaccessible();
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

        pub fn copy(&self) -> FNDA<V> {
            let alphabet = self.alphabet.clone();
            let initials = self.initials.clone();
            let finals = self.finals.clone();
            let transitions = self.transitions.clone();
            FNDA {
                alphabet,
                initials,
                finals,
                transitions,
            }
        }

        pub fn equals(&self, b: &FNDA<V>) -> bool {
            self.contains(&b) && b.contains(self)
        }

        pub fn contains(&self, b: &FNDA<V>) -> bool {
            let mut cpy_a = self.copy();
            let mut cpy_b = b.copy();
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

        pub fn is_accessible(&self) -> bool {
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

        pub fn is_coaccessible(&self) -> bool {
            let mut rev = self.copy();
            rev.reverse();
            rev.is_accessible()
        }

        pub fn is_trimmed(&self) -> bool {
            self.is_accessible() && self.is_coaccessible()
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
            let mut cpy = self.copy();
            cpy.negate();
            cpy.is_empty()
        }
    }
}
