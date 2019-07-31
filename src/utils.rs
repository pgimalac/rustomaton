/* AUXILIARY FUNCTIONS */

use crate::nfa::NFA;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::hash::Hash;

pub fn append_hashset<V: Eq + Hash>(a: &mut HashSet<V>, b: HashSet<V>) {
    for v in b {
        a.insert(v);
    }
}

pub fn append_shift_hashset(a: &mut HashSet<usize>, mut b: HashSet<usize>, l: usize) {
    shift_hashset(&mut b, l);
    append_hashset(a, b);
}

pub fn shift_hashset(a: &mut HashSet<usize>, l: usize) {
    for e in a.drain().collect::<Vec<usize>>() {
        a.insert(e + l);
    }
}

pub fn shift_fnda<V: Eq + Hash + Display + Copy + Clone + Debug>(a: &mut NFA<V>, l: usize) {
    shift_hashset(&mut a.initials, l);
    shift_hashset(&mut a.finals, l);
    shift_transitions(&mut a.transitions, l);
}

pub fn shift_transitions<V: Eq + Hash>(a: &mut Vec<HashMap<V, Vec<usize>>>, l: usize) {
    for map in a {
        for (_, v) in map {
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
