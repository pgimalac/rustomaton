pub mod automaton;

#[cfg(test)]
mod tests {
    use crate::automaton::nfa::nfa::NFA;
    use std::collections::{HashMap, HashSet};
    use std::iter::repeat;

    // empty automaton
    // this automaton is deterministic
    fn automaton0() -> NFA<u8> {
        NFA {
            alphabet: (0..10).collect(),
            initials: HashSet::new(),
            finals: HashSet::new(),
            transitions: Vec::new(),
        }
    }
    fn automaton0_accept() -> Vec<Vec<u8>> {
        vec![]
    }
    fn automaton0_reject() -> Vec<Vec<u8>> {
        let mut v = vec![vec![], vec![1, 2, 3, 4, 5, 6]];
        (0..10).for_each(|x| v.push(vec![x]));
        return v;
    }

    // full automaton (one state, both initial and final, accepting everything)
    // this automaton is deterministic
    fn automaton1() -> NFA<u8> {
        let mut map = HashMap::new();
        for x in 0..10 {
            map.insert(x, vec![0]);
        }
        NFA {
            alphabet: (0..10).collect(),
            initials: (0..=0).collect(),
            finals: (0..=0).collect(),
            transitions: vec![map],
        }
    }
    fn automaton1_accept() -> Vec<Vec<u8>> {
        let mut v = vec![vec![], vec![1, 2, 3, 4, 5, 6]];
        (0..10).for_each(|x| v.push(vec![x]));
        return v;
    }
    fn automaton1_reject() -> Vec<Vec<u8>> {
        vec![]
    }

    fn automaton_mult(a: usize, b: usize) -> NFA<u8> {
        let mut transitions: Vec<HashMap<u8, Vec<usize>>> =
            repeat(HashMap::new()).take(a).collect();
        for i in 0..a {
            for t in 0..b {
                transitions[i].insert(t as u8, vec![(i * b + t) % a]);
            }
        }

        NFA {
            alphabet: (0..10).collect(),
            initials: (0..=0).collect(),
            finals: (0..=0).collect(),
            transitions,
        }
    }

    // automaton accepting all multiples of a in base b (as well as the empty word)
    // b is at most 10
    // this automaton is deterministic
    fn automaton2() -> NFA<u8> {
        automaton_mult(3, 2)
    }
    fn automaton2_accept() -> Vec<Vec<u8>> {
        vec![
            vec![],
            vec![0],
            vec![1, 1],
            vec![1, 0, 0, 1],
            vec![1, 1, 0, 0],
        ]
    }
    fn automaton2_reject() -> Vec<Vec<u8>> {
        vec![vec![2], vec![5], vec![1], vec![1, 0], vec![1, 0, 1]]
    }

    // a weird automaton
    fn automaton3() -> NFA<u8> {
        let mut transitions: Vec<HashMap<u8, Vec<usize>>> =
            repeat(HashMap::new()).take(10).collect();
        transitions[0].insert(0, vec![6]);
        transitions[0].insert(2, vec![7]);
        transitions[2].insert(1, vec![2]);
        transitions[2].insert(7, vec![6]);
        transitions[4].insert(8, vec![5]);
        transitions[4].insert(9, vec![8]);
        transitions[4].insert(5, vec![7]);
        transitions[5].insert(3, vec![6]);
        transitions[5].insert(1, vec![8]);
        transitions[6].insert(0, vec![0]);
        transitions[6].insert(2, vec![4]);
        transitions[6].insert(4, vec![7]);
        transitions[7].insert(6, vec![4]);
        transitions[7].insert(9, vec![6, 7]);
        transitions[7].insert(5, vec![4]);
        transitions[8].insert(7, vec![5]);
        transitions[8].insert(4, vec![0]);
        transitions[8].insert(3, vec![2]);
        NFA {
            alphabet: (0..10).collect(),
            initials: (0..=3).into_iter().collect(),
            finals: vec![2, 3, 4, 5, 9].into_iter().collect(),
            transitions,
        }
    }
    fn automaton3_accept() -> Vec<Vec<u8>> {
        vec![
            vec![],
            vec![1, 1, 1],
            vec![7, 2],
            vec![2, 6],
            vec![0, 4, 5],
            vec![2, 6, 9, 7],
        ]
    }
    fn automaton3_reject() -> Vec<Vec<u8>> {
        vec![
            vec![7],
            vec![5],
            vec![0],
            vec![0, 2, 9, 4],
            vec![0, 2, 5, 9],
        ]
    }

    fn automaton_list() -> Vec<(NFA<u8>, Vec<Vec<u8>>, Vec<Vec<u8>>)> {
        vec![
            (automaton0(), automaton0_accept(), automaton0_reject()),
            (automaton1(), automaton1_accept(), automaton1_reject()),
            (automaton2(), automaton2_accept(), automaton2_reject()),
            (automaton3(), automaton3_accept(), automaton3_reject()),
        ]
    }

    #[ignore]
    #[test]
    fn test_dot() {
        automaton0().write_dot(0).unwrap();
        automaton1().write_dot(1).unwrap();
        automaton2().write_dot(2).unwrap();
        automaton3().write_dot(3).unwrap();
    }

    #[test]
    fn test_is_complete() {
        assert!(!automaton0().is_complete());
        assert!(automaton1().is_complete());
        assert!(!automaton2().is_complete());
        assert!(!automaton3().is_complete());
    }

    #[test]
    fn test_is_reachable() {
        assert!(automaton0().is_reachable());
        assert!(automaton1().is_reachable());
        assert!(automaton2().is_reachable());
        assert!(!automaton3().is_reachable());
    }

    #[test]
    fn test_is_coreachable() {
        assert!(automaton0().is_coreachable());
        assert!(automaton1().is_coreachable());
        assert!(automaton2().is_coreachable());
        assert!(!automaton3().is_coreachable());
    }

    #[test]
    fn test_is_empty() {
        assert!(automaton0().is_empty());
        assert!(!automaton1().is_empty());
        assert!(!automaton2().is_empty());
        assert!(!automaton3().is_empty());
    }

    #[test]
    fn test_is_full() {
        assert!(!automaton0().is_full());
        assert!(automaton1().is_full());
        assert!(!automaton2().is_full());
        assert!(!automaton3().is_full());
    }

    #[test]
    fn test_run() {
        for (aut, acc, rej) in automaton_list() {
            if let Some(e) = acc.iter().find(|x| !aut.run(x)) {
                panic!("{:?}", e);
            }
            if let Some(e) = rej.iter().find(|x| aut.run(x)) {
                panic!("{:?}", e);
            }
        }
    }

    #[test]
    fn test_union() {
        let list = automaton_list();
        for (i, (aut1, acc1, _)) in list.iter().enumerate() {
            for (j, (aut2, acc2, _)) in list.iter().enumerate() {
                let aut = aut1.clone().union(aut2.clone());
                if let Some(e) = acc1.iter().chain(acc2.iter()).find(|x| !aut.run(x)) {
                    aut.write_dot(9).unwrap();
                    panic!("union of {} and {}: elem {:?}", i, j, e);
                }
            }
        }
    }

    #[test]
    fn test_concatenate() {
        let list = automaton_list();
        for (i, (aut1, acc1, _)) in list.iter().enumerate() {
            for (j, (aut2, acc2, _)) in list.iter().enumerate() {
                let aut = aut1.clone().concatenate(aut2.clone());
                for post in acc2 {
                    let mut acc = acc1.clone();
                    acc.iter_mut().for_each(|x| x.append(&mut post.clone()));
                    if let Some(e) = acc.into_iter().find(|x| !aut.run(x)) {
                        aut.write_dot(9).unwrap();
                        panic!("concat of {} and {}: elem {:?}", i, j, e);
                    }
                }
            }
        }
    }

    #[test]
    fn test_negate() {
        for (i, (mut aut, acc, rej)) in automaton_list().into_iter().enumerate() {
            let aut = aut.negate();
            if let Some(e) = acc.iter().find(|x| aut.run(x)) {
                aut.write_dot(9).unwrap();
                panic!(
                    "negation of {} : elem {:?} wasn't supposed to be accepted",
                    i, e
                );
            }
            if let Some(e) = rej.iter().find(|x| !aut.run(x)) {
                aut.write_dot(9).unwrap();
                panic!(
                    "negation of {} : elem {:?} was supposed to be accepted",
                    i, e
                );
            }
        }
    }

}
