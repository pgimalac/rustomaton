pub mod nfa {
    use crate::automaton::automaton::Automaton;
    use crate::automaton::dfa::dfa::DFA;
    use std::cmp::PartialEq;
    use std::collections::{HashMap, HashSet, VecDeque};
    use std::fmt::Display;
    use std::hash::Hash;
    use std::iter::repeat;
    use std::iter::FromIterator;

    /* AUXILIARY FUNCTIONS */

    fn append_hashset<V: Eq + Hash>(a: &mut HashSet<V>, b: HashSet<V>) {
        for v in b {
            a.insert(v);
        }
    }

    fn append_shift_hashset(a: &mut HashSet<usize>, b: HashSet<usize>, l: usize) {
        for v in b {
            a.insert(v + l);
        }
    }

    fn shift_hashset(a: &mut HashSet<usize>, l: usize) {
        for e in a.drain().collect::<Vec<usize>>() {
            a.insert(e + l);
        }
    }

    fn shift_fnda<V: Eq + Hash + Display + Copy + Clone>(a: &mut NFA<V>, l: usize) {
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

    fn append_shift_transitions<V: Eq + Hash>(
        a: &mut Vec<HashMap<V, Vec<usize>>>,
        mut b: Vec<HashMap<V, Vec<usize>>>,
    ) {
        shift_transitions(&mut b, a.len());
        a.append(&mut b);
    }

    #[derive(Debug, Clone)]
    pub struct NFA<V: Eq + Hash + Display + Copy + Clone> {
        pub(crate) alphabet: HashSet<V>,
        pub(crate) initials: HashSet<usize>,
        pub(crate) finals: HashSet<usize>,
        pub(crate) transitions: Vec<HashMap<V, Vec<usize>>>,
    }

    /* IMPLEMENTATION OF NFA */

    impl<V: Eq + Hash + Display + Copy + Clone> NFA<V> {
        pub fn intersect(self, mut _b: NFA<V>) -> NFA<V> {
            unimplemented!()
        }

        pub fn union(mut self, b: NFA<V>) -> NFA<V> {
            let NFA {
                alphabet,
                initials,
                finals,
                transitions,
            } = b;

            append_hashset(&mut self.alphabet, alphabet);
            append_shift_hashset(&mut self.initials, initials, self.transitions.len());
            append_shift_hashset(&mut self.finals, finals, self.transitions.len());
            append_shift_transitions(&mut self.transitions, transitions);

            self
        }

        pub fn concatenate(mut self, mut b: NFA<V>) -> NFA<V> {
            let l = self.transitions.len();
            shift_fnda(&mut b, l);
            let NFA {
                alphabet,
                mut initials,
                finals,
                mut transitions,
            } = b;

            append_hashset(&mut self.alphabet, alphabet);

            for e in &initials {
                for (v, t) in &mut transitions[e - l] {
                    // e - l because of the shift above
                    for f in &self.finals {
                        self.transitions[*f]
                            .entry(*v)
                            .or_insert(Vec::new())
                            .append(&mut t.clone());
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

        pub fn negate(&mut self) -> DFA<V> {
            let mut aut = self.to_dfa();
            aut.negate();
            return aut;
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

            if self.initials.is_empty() {
                self.initials.insert(l);
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

        pub fn contains(&self, b: &NFA<V>) -> bool {
            let mut cpy_a = self.clone();
            let cpy_b = b.clone();
            cpy_a.negate();
            cpy_a.intersect(cpy_b).is_empty()
        }

        pub fn is_complete(&self) -> bool {
            if self.initials.is_empty() {
                return false;
            }

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
            if !self.initials.is_disjoint(&self.finals) {
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
            if self.initials.is_disjoint(&self.finals) {
                return false;
            }

            let mut acc: HashSet<usize> = self.initials.clone().into_iter().collect();
            let mut stack: Vec<usize> = self.initials.clone().into_iter().collect();

            loop {
                if let Some(e) = stack.pop() {
                    for (_, v) in &self.transitions[e] {
                        for t in v {
                            if !self.finals.contains(t) {
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

        pub fn to_dfa(&self) -> DFA<V> {
            if self.is_empty() {
                DFA {
                    alphabet: self.alphabet.clone(),
                    initial: 0,
                    finals: HashSet::new(),
                    transitions: vec![HashMap::new()],
                }
            } else if self.transitions.len() < 128 {
                self.small_to_dfa()
            } else {
                self.big_to_dfa()
            }
        }

        fn small_to_dfa(&self) -> DFA<V> {
            let mut map = HashMap::new();
            let mut stack = VecDeque::new();

            let mut dfa = DFA {
                alphabet: self.alphabet.clone(),
                initial: 0,
                finals: HashSet::new(),
                transitions: vec![HashMap::new()],
            };

            let i: u128 = self.initials.iter().fold(0, |acc, x| acc | (1 << *x));
            if self.finals.iter().any(|x| self.finals.contains(x)) {
                dfa.finals.insert(0);
            }

            map.insert(i, 0);
            stack.push_back((i, HashSet::from_iter(self.initials.clone().into_iter())));

            loop {
                if let Some((elem, iter)) = stack.pop_front() {
                    let elem_num = *map.get(&elem).unwrap();
                    for v in &self.alphabet {
                        let mut it = HashSet::new();
                        for state in &iter {
                            if let Some(transitions) = self.transitions[*state].get(&v) {
                                for t in transitions {
                                    it.insert(*t);
                                }
                            }
                        }
                        if it.is_empty() {
                            continue;
                        }

                        let other = it.iter().fold(0, |acc, x| acc | 1 << *x);
                        if !map.contains_key(&other) {
                            map.insert(other, map.len());
                            if it.iter().any(|x| self.finals.contains(x)) {
                                dfa.finals.insert(map.len() - 1);
                            }
                            stack.push_back((other, it));
                            dfa.transitions.push(HashMap::new());
                        }
                        dfa.transitions[elem_num].insert(*v, *map.get(&other).unwrap());
                    }
                } else {
                    break;
                }
            }

            dfa
        }

        fn big_to_dfa(&self) -> DFA<V> {
            unimplemented!()
        }

        pub fn run(&self, v: &Vec<V>) -> bool {
            if self.initials.is_empty() {
                return false;
            }

            let mut actuals = self.initials.clone();
            let mut next = HashSet::new();

            for l in v {
                for st in &actuals {
                    if let Some(tr) = self.transitions[*st].get(l) {
                        for t in tr {
                            next.insert(*t);
                        }
                    }
                }

                std::mem::swap(&mut actuals, &mut next);
                if actuals.is_empty() {
                    return false;
                }
                next.clear();
            }

            return actuals.iter().any(|x| self.finals.contains(x));
        }

        pub fn write_dot(&self, i: u8) -> Result<(), std::io::Error> {
            use std::fs::File;
            use std::io::Write;
            use std::path::Path;

            let mut name = "dots/automaton".to_string();
            name.push_str(&i.to_string());
            name.push_str(".dot");
            let name = Path::new(&name);

            let mut file = File::create(&name)?;
            file.write(b"digraph {\n")?;

            if !self.finals.is_empty() {
                file.write(b"    node [shape = doublecircle];")?;
                for e in &self.finals {
                    write!(file, " S_{}", e)?;
                }
                file.write(b";\n")?;
            }

            if !self.initials.is_empty() {
                file.write(b"    node [shape = point];")?;
                for e in &self.initials {
                    write!(file, " I_{}", e)?;
                }
                file.write(b";\n")?;
            }

            file.write(b"    node [shape = circle];\n")?;
            let mut tmp_map = HashMap::new();
            for (i, map) in self.transitions.iter().enumerate() {
                if map.is_empty() {
                    write!(file, "    S_{};\n", i)?;
                }
                for (k, v) in map {
                    for e in v {
                        tmp_map.entry(e).or_insert(Vec::new()).push(k);
                    }
                }
                for (e, v) in tmp_map.drain() {
                    let mut vs = v.into_iter().fold(String::new(), |mut acc, x| {
                        acc.push_str(&x.to_string());
                        acc.push_str(", ");
                        acc
                    });
                    vs.pop();
                    vs.pop();
                    write!(file, "    S_{} -> S_{} [label = \"{}\"];\n", i, e, vs)?;
                }
            }

            for e in &self.initials {
                write!(file, "    I_{} -> S_{};\n", e, e)?;
            }

            file.write(b"}")?;

            Ok(())
        }
    }

    impl std::str::FromStr for NFA<char> {
        type Err = String;

        fn from_str(_s: &str) -> Result<Self, Self::Err> {
            unimplemented!()
        }
    }

    impl<V: Eq + Hash + Display + Copy + Clone> PartialEq<NFA<V>> for NFA<V> {
        fn eq(&self, b: &NFA<V>) -> bool {
            self.contains(&b) && b.contains(self)
        }
    }

    impl<V: Eq + Hash + Display + Copy + Clone> PartialEq<DFA<V>> for NFA<V> {
        fn eq(&self, b: &DFA<V>) -> bool {
            self.eq(&b.to_nfa())
        }
    }

    impl<V: Eq + Hash + Display + Copy + Clone> PartialEq<Automaton<V>> for NFA<V> {
        fn eq(&self, b: &Automaton<V>) -> bool {
            match b {
                Automaton::DFA(v) => self.eq(&**v),
                Automaton::NFA(v) => self.eq(&**v),
            }
        }
    }
}
