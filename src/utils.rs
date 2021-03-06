/* AUXILIARY FUNCTIONS */

use crate::{
    nfa::NFA,
    regex::{Operations, Operations::Letter},
};
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fmt::{Debug, Display},
    hash::Hash,
};

pub fn append_hashset<V: Eq + Hash>(a: &mut HashSet<V>, b: HashSet<V>) {
    a.extend(b.into_iter())
}

pub fn append_shift_hashset(a: &mut HashSet<usize>, b: HashSet<usize>, l: usize) {
    a.extend(b.into_iter().map(|x| x + l))
}

pub fn shift_hashset(a: &mut HashSet<usize>, l: usize) {
    for e in a.drain().collect::<Vec<usize>>() {
        a.insert(e + l);
    }
}

pub fn shift_fnda<V: Eq + Hash + Display + Copy + Clone + Debug + Ord>(a: &mut NFA<V>, l: usize) {
    shift_hashset(&mut a.initials, l);
    shift_hashset(&mut a.finals, l);
    shift_transitions(&mut a.transitions, l);
}

pub fn shift_transitions<V: Eq + Hash>(a: &mut Vec<HashMap<V, Vec<usize>>>, l: usize) {
    for map in a {
        for v in map.values_mut() {
            v.iter_mut().for_each(|t| *t += l);
        }
    }
}

pub fn append_shift_transitions<V: Eq + Hash>(
    a: &mut Vec<HashMap<V, Vec<usize>>>,
    mut b: Vec<HashMap<V, Vec<usize>>>,
) {
    shift_transitions(&mut b, a.len());
    a.append(&mut b);
}

pub(crate) fn contains_dot<V: Eq + Hash + Display + Copy + Clone + Debug + Ord>(
    set: &BTreeSet<Operations<V>>,
    alphabet: &HashSet<V>,
) -> bool {
    alphabet.iter().all(|x| set.contains(&Letter(*x)))
}

macro_rules! paren {
    ($x: expr) => {
        if $x.len() == 1 {
            $x
        } else {
            format!("({})", $x)
        }
    };
}
