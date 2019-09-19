/* AUXILIARY FUNCTIONS */

use crate::nfa::NFA;
//use std::iter::repeat;
// use std::ops::{Add, AddAssign, Mul};
//use ndarray::arr2;
//use ndarray::ArrayBase;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::hash::Hash;

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

// pub fn mul<V: Clone + AddAssign + Mul<Output = V>>(
//     arr1: &Vec<Vec<V>>,
//     arr2: &Vec<Vec<V>>,
// ) -> Vec<Vec<V>> {
//     let a = arr1.len();
//     let b = arr2.len();
//     let c = arr2[0].len();
//     let mut res: Vec<Vec<V>> = repeat(Vec::with_capacity(a)).take(c).collect();

//     for i in 0..a {
//         for j in 0..c {
//             res[i].push(arr1[i][j].clone() * arr2[0][j].clone());
//             for k in 1..b {
//                 res[i][j] += arr1[i][j].clone() * arr2[k][j].clone();
//             }
//         }
//     }

//     res
// }

// pub fn add<V: Clone + Add<Output = V>>(arr1: &Vec<Vec<V>>, arr2: &Vec<Vec<V>>) -> Vec<Vec<V>> {
//     let a = arr1.len();
//     let b = arr2[0].len();
//     let mut res: Vec<Vec<V>> = repeat(Vec::with_capacity(a)).take(b).collect();

//     for i in 0..a {
//         for j in 0..b {
//             res[i].push(arr1[i][j].clone() + arr2[0][j].clone());
//         }
//     }

//     res
// }

// pub fn pow<V: Clone + Mul<Output = V> + AddAssign>(arr: &Vec<Vec<V>>, n: usize) -> Vec<Vec<V>> {
//     assert!(n != 0);
//     if n == 1 {
//         return arr.clone();
//     }

//     let sq = pow(arr, n / 2);
//     let sq = mul(&sq, &sq);
//     if n % 2 == 1 {
//         return mul(&arr, &sq);
//     } else {
//         return sq;
//     }
// }
