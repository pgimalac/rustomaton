extern crate logos;

#[macro_use]
mod utils;

pub mod automaton;
pub mod dfa;
pub mod generator;
pub mod nfa;
pub mod regex;

mod parser;

/// https://en.wikipedia.org/wiki/Regular_language
/// https://en.wikipedia.org/wiki/Finite-state_machine

#[cfg(test)]
mod tests {
    use crate::automaton::{Automata, Buildable};
    use crate::dfa::ToDfa;
    use crate::generator::new_generator;
    use crate::nfa::{ToNfa, NFA};
    use crate::regex::{Regex, ToRegex};
    use std::collections::{HashMap, HashSet};
    use std::iter::repeat;

    // empty automaton
    // this automaton is deterministic
    fn automaton0() -> NFA<char> {
        NFA {
            alphabet: (b'0'..=b'9').map(char::from).collect(),
            initials: HashSet::new(),
            finals: HashSet::new(),
            transitions: Vec::new(),
        }
    }
    fn automaton0_accept() -> Vec<Vec<char>> {
        vec![]
    }
    fn automaton0_reject() -> Vec<Vec<char>> {
        let mut v = vec![vec![], vec!['1', '2', '3', '4', '5', '6']];
        (b'0'..=b'9').map(char::from).for_each(|x| v.push(vec![x]));
        return v;
    }

    // full automaton (one state, both initial and final, accepting everything)
    // this automaton is deterministic
    fn automaton1() -> NFA<char> {
        let mut map = HashMap::new();
        for x in (b'0'..=b'9').map(char::from) {
            map.insert(x, vec![0]);
        }
        NFA {
            alphabet: (b'0'..=b'9').map(char::from).collect(),
            initials: (0..=0).collect(),
            finals: (0..=0).collect(),
            transitions: vec![map],
        }
    }
    fn automaton1_accept() -> Vec<Vec<char>> {
        let mut v = vec![vec![], vec!['1', '2', '3', '4', '5', '6']];
        (b'0'..=b'9').map(char::from).for_each(|x| v.push(vec![x]));
        return v;
    }
    fn automaton1_reject() -> Vec<Vec<char>> {
        vec![]
    }

    // automaton accepting all numbers in base b that have c for remainder when divided by a (as well as the empty word if c == 0)
    // b is at most 10
    // this automaton is deterministic
    fn automaton_mult(a: usize, b: usize, c: usize) -> NFA<char> {
        assert!(b <= 10);

        let mut transitions: Vec<_> = repeat(HashMap::new()).take(a).collect();
        for i in 0..a {
            for t in 0..b as u8 {
                transitions[i].insert((t + '0' as u8) as char, vec![(i * b + t as usize) % a]);
            }
        }

        NFA {
            alphabet: (b'0'..=b'9').map(char::from).collect(),
            initials: (0..=0).collect(),
            finals: (c..=c).collect(),
            transitions,
        }
    }

    fn automaton2() -> NFA<char> {
        automaton_mult(3, 2, 0)
    }
    fn automaton2_accept() -> Vec<Vec<char>> {
        vec![
            vec![],
            vec!['0'],
            vec!['1', '1'],
            vec!['1', '0', '0', '1'],
            vec!['1', '1', '0', '0'],
        ]
    }
    fn automaton2_reject() -> Vec<Vec<char>> {
        vec![
            vec!['2'],
            vec!['5'],
            vec!['1'],
            vec!['1', '0'],
            vec!['1', '0', '1'],
        ]
    }

    // a weird automaton
    fn automaton3() -> NFA<char> {
        let mut transitions: Vec<HashMap<char, Vec<usize>>> =
            repeat(HashMap::new()).take(10).collect();
        transitions[0].insert('0', vec![6]);
        transitions[0].insert('2', vec![7]);
        transitions[2].insert('1', vec![2]);
        transitions[2].insert('7', vec![6]);
        transitions[4].insert('8', vec![5]);
        transitions[4].insert('9', vec![8]);
        transitions[4].insert('5', vec![7]);
        transitions[5].insert('3', vec![6]);
        transitions[5].insert('1', vec![8]);
        transitions[6].insert('0', vec![0]);
        transitions[6].insert('2', vec![4]);
        transitions[6].insert('4', vec![7]);
        transitions[7].insert('6', vec![4]);
        transitions[7].insert('9', vec![6, 7]);
        transitions[7].insert('5', vec![4]);
        transitions[8].insert('7', vec![5]);
        transitions[8].insert('4', vec![0]);
        transitions[8].insert('3', vec![2]);
        NFA {
            alphabet: (b'0'..=b'9').map(char::from).collect(),
            initials: (0..=3).into_iter().collect(),
            finals: vec![2, 3, 4, 5, 9].into_iter().collect(),
            transitions,
        }
    }
    fn automaton3_accept() -> Vec<Vec<char>> {
        vec![
            vec![],
            vec!['1', '1', '1'],
            vec!['7', '2'],
            vec!['2', '6'],
            vec!['0', '4', '5'],
            vec!['2', '6', '9', '7'],
        ]
    }
    fn automaton3_reject() -> Vec<Vec<char>> {
        vec![
            vec!['7'],
            vec!['5'],
            vec!['0'],
            vec!['0', '2', '9', '4'],
            vec!['0', '2', '5', '9'],
        ]
    }

    fn automaton4() -> NFA<char> {
        Regex::parse_with_alphabet(
            (b'0'..=b'9').map(char::from).collect(),
            "(018)*4(5+|6|7*)?3+.29?|𝜀",
        )
        .unwrap()
        .to_nfa()
    }

    fn automaton4_accept() -> Vec<Vec<char>> {
        vec![
            vec![],
            vec!['4', '3', '1', '2'],
            vec![
                '0', '1', '8', '0', '1', '8', '0', '1', '8', '4', '5', '5', '5', '3', '3', '3',
                '3', '2', '9',
            ],
            vec!['0', '1', '8', '4', '6', '3', '2', '2'],
            vec!['4', '7', '7', '7', '7', '3', '3', '4', '2', '9'],
            vec!['0', '1', '8', '0', '1', '8', '4', '6', '3', '2', '2'],
        ]
    }

    fn automaton4_reject() -> Vec<Vec<char>> {
        vec![
            vec!['1', '2', '3'],
            vec!['0', '1', '8', '4', '4'],
            vec!['4', '5', '5', '3', '3', '2', '9', '1'],
            vec!['4', '7', '7', '7', '3', '2'],
        ]
    }

    fn automaton5() -> NFA<char> {
        Regex::parse_with_alphabet(
            (b'0'..=b'9').map(char::from).collect(),
            "2|5+|6|9*|(𝜀42?78+3|2+|71+)+",
        )
        .unwrap()
        .to_nfa()
    }

    fn automaton5_accept() -> Vec<Vec<char>> {
        vec![
            vec![],
            vec!['2'],
            vec!['5', '5', '5'],
            vec!['9'],
            vec!['4', '2', '7', '8', '8', '3', '7', '1', '1', '1', '2'],
            vec![
                '4', '7', '8', '3', '2', '7', '1', '1', '2', '7', '1', '4', '2', '7', '8', '3',
            ],
        ]
    }

    fn automaton5_reject() -> Vec<Vec<char>> {
        vec![
            vec!['4'],
            vec!['2', '2', '7'],
            vec!['9', '6'],
            vec!['4', '2', '2', '7', '8', '3'],
            vec!['7', '1', '2', '2', '4', '2', '7', '3'],
        ]
    }

    fn automaton6() -> NFA<char> {
        Regex::parse_with_alphabet(
            (b'0'..=b'9').map(char::from).collect(),
            "(3*8*|4(1|4)*)(9+|7*)5*6|18|8*5|4|12|9+",
        )
        .unwrap()
        .to_nfa()
    }

    fn automaton6_accept() -> Vec<Vec<char>> {
        vec![
            vec!['1', '2'],
            vec!['4'],
            vec!['8', '8', '8', '8', '8', '5'],
            vec!['9', '9'],
            vec!['6'],
            vec!['3', '3', '3', '8', '8', '9', '9', '9', '5', '5', '5', '6'],
            vec!['4', '6'],
            vec!['4', '1', '4', '1', '4', '7', '7', '7', '5', '6'],
        ]
    }

    fn automaton6_reject() -> Vec<Vec<char>> {
        vec![
            vec![],
            vec!['4', '9', '7', '6'],
            vec!['1', '4', '4', '1', '6'],
            vec!['2'],
            vec!['3', '4', '6'],
            vec!['3', '9', '5', '5', '5'],
        ]
    }

    fn automaton7() -> NFA<char> {
        Regex::parse_with_alphabet(
            (b'0'..=b'9').map(char::from).collect(),
            "0(8+4*3*)*|86+(3+|578)((3*|4?6?)+|(4*|86+|2)37*|54|.|5*)|.8*|(3*0*)+|2*|7*2|.3|3*5*|(50|7)1|21|4+|(30*|6|9*2*)*|1+(608*)*",
        )
        .unwrap()
        .to_nfa()
    }

    fn automaton7_accept() -> Vec<Vec<char>> {
        vec![
            vec![],
            vec!['3'],
            vec!['5'],
            vec!['2', '1'],
            vec!['1', '1', '6', '0', '6', '0', '6', '0', '8', '8', '8', '8'],
            vec!['0'],
            vec!['8', '6', '6', '6', '3', '3', '3', '5', '4'],
            vec!['3', '0', '0', '0', '6', '9', '2', '2'],
            vec!['6', '2'],
        ]
    }

    fn automaton7_reject() -> Vec<Vec<char>> {
        vec![
            vec!['8', '9', '6', '5', '3', '5', '2'],
            vec!['3', '3', '3', '0', '0', '0', '3', '3', '1'],
            vec!['9', '9', '9', '5'],
            vec!['5', '5', '5', '3'],
        ]
    }

    fn automaton_list() -> Vec<(NFA<char>, Vec<Vec<char>>, Vec<Vec<char>>)> {
        vec![
            (automaton0(), automaton0_accept(), automaton0_reject()),
            (automaton1(), automaton1_accept(), automaton1_reject()),
            (automaton2(), automaton2_accept(), automaton2_reject()),
            (automaton3(), automaton3_accept(), automaton3_reject()),
            (automaton4(), automaton4_accept(), automaton4_reject()),
            (automaton5(), automaton5_accept(), automaton5_reject()),
            (automaton6(), automaton6_accept(), automaton6_reject()),
            (automaton7(), automaton7_accept(), automaton7_reject()),
        ]
    }

    #[ignore]
    #[test]
    fn test_dot() {
        for (i, (aut, _, _)) in automaton_list().into_iter().enumerate() {
            aut.write_dot(i as u8).unwrap();
        }
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
        assert!(!automaton4().is_empty());
    }

    #[test]
    fn test_is_full() {
        assert!(!automaton0().is_full());
        assert!(automaton1().is_full());
        assert!(!automaton2().is_full());
        assert!(!automaton3().is_full());
        assert!(!automaton4().is_full());
    }

    #[test]
    fn test_run() {
        for (i, (aut, acc, rej)) in automaton_list().into_iter().enumerate() {
            if let Some(e) = acc.iter().find(|x| !aut.run(x)) {
                panic!("{} should have accepted {:?}", i, e);
            }
            if let Some(e) = rej.iter().find(|x| aut.run(x)) {
                panic!("{} shouldn't have accepted {:?}", i, e);
            }
        }
    }

    #[test]
    fn test_unite() {
        let list = automaton_list();
        for (i, (aut1, acc1, _)) in list.iter().enumerate() {
            for (j, (aut2, acc2, _)) in list.iter().enumerate() {
                let aut = aut1.clone().unite(aut2.clone());
                if let Some(e) = acc1.iter().chain(acc2.iter()).find(|x| !aut.run(x)) {
                    aut.write_dot(9).unwrap();
                    panic!("unite of {} and {}: elem {:?}", i, j, e);
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
        for (i, (aut, acc, rej)) in automaton_list().into_iter().enumerate() {
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

    #[test]
    fn test_intersect() {
        let list = automaton_list();
        for (i, (aut1, _, rej1)) in list.iter().enumerate() {
            for (j, (aut2, _, rej2)) in list.iter().enumerate() {
                let aut = aut1.clone().intersect(aut2.clone());
                if let Some(e) = rej1.iter().chain(rej2.iter()).find(|x| aut.run(x)) {
                    aut.write_dot(9).unwrap();
                    panic!("intersection of {} and {}: elem {:?}", i, j, e);
                }
            }
        }
    }

    #[test]
    fn test_equals() {
        for (i, (aut, _, _)) in automaton_list().into_iter().enumerate() {
            if !aut.eq(&aut) {
                panic!("{} is supposed to be equal to itself", i);
            }

            if !aut.clone().complete().eq(&aut) {
                panic!("{} is supposed to be equal to itself completed", i);
            }

            if !aut.clone().reverse().reverse().eq(&aut) {
                panic!("{} is supposed to be equal to itself reversed twice", i);
            }

            if !aut.clone().trim().eq(&aut) {
                panic!("{} is supposed to be equal to itself trimmed", i);
            }

            if !aut.eq(&aut.clone().negate().negate()) {
                panic!("{} is supposed to be equal to itself negated twice", i);
            }

            if !aut.eq(&aut.clone().unite(aut.clone())) {
                panic!("{} is supposed to be equal to itself united with itself", i);
            }

            if !aut.eq(&aut.clone().intersect(aut.clone())) {
                panic!(
                    "{} is supposed to be equal to itself intersected with itself",
                    i
                );
            }
        }
    }

    #[test]
    fn test_to_dfa() {
        for (i, (aut, acc, rej)) in automaton_list().into_iter().enumerate() {
            let aut = aut.to_dfa();
            if let Some(e) = acc.iter().find(|x| !aut.run(x)) {
                aut.write_dot(8).unwrap();
                panic!("{} is supposed to accept {:?}", i, e);
            }
            if let Some(e) = rej.iter().find(|x| aut.run(x)) {
                aut.write_dot(8).unwrap();
                panic!("{} isn't supposed to accept {:?}", i, e);
            }
        }
    }

    #[test]
    fn test_kleene() {
        for (i, (aut, acc, _)) in automaton_list().into_iter().enumerate() {
            let aut1 = aut.clone().kleene();

            if !aut1.run(&Vec::new()) {
                aut1.write_dot(9).unwrap();
                panic!("{} kleened should accept []", i);
            }
            for a1 in &acc {
                for a2 in &acc {
                    let mut e = a1.clone();
                    e.append(&mut a2.clone());
                    if !aut1.run(&e) {
                        aut1.write_dot(9).unwrap();
                        panic!(
                            "{} kleened should accept the concatenation of {:?} and {:?}",
                            i, a1, a2
                        );
                    }
                }
            }

            if !aut1.contains(&aut) {
                aut1.write_dot(9).unwrap();
                panic!("{} kleened should contain itself", i);
            }
        }
    }

    #[test]
    fn test_minimize() {
        for (i, (aut, acc, rej)) in automaton_list().into_iter().enumerate() {
            let aut1 = aut.to_dfa().minimize();

            if let Some(e) = acc.iter().find(|x| !aut1.run(x)) {
                panic!("{} minimized should accept {:?}", i, e);
            }
            if let Some(e) = rej.iter().find(|x| aut1.run(x)) {
                panic!("{} minimized should accept {:?}", i, e);
            }

            if !aut.eq(&aut1) {
                panic!("{} should be equal to itself minimized", i);
            }
        }
    }

    #[test]
    #[ignore]
    fn test_generator() {
        let mut gen = new_generator((b'0'..=b'9').map(char::from).collect(), 20);
        for _ in 0..10 {
            println!("{}", gen.run());
        }
    }

    #[test]
    fn test_to_regex() {
        for (i, (aut, _, _)) in automaton_list().into_iter().enumerate() {
            println!("{} : {}", i, aut.to_regex().simplify().to_string());
        }
    }

    #[test]
    fn test_simplify() {
        let list = [
            "",
            "𝜀",
            "𝜀𝜀((𝜀))𝜀𝜀",
            "0|1|0|(0|1)",
            "(0|1|2|3|𝜀)?",
            "10|11|12|13",
            "1𝜀2𝜀3𝜀",
            "(1|3|4|𝜀)*",
            "1|𝜀",
            "1*|𝜀",
            "1+|𝜀",
        ];

        for e in &list {
            println!(
                "{}  :  {}",
                e,
                Regex::parse_with_alphabet((b'0'..=b'9').map(char::from).collect(), e)
                    .unwrap()
                    .simplify()
                    .to_string()
            );
        }
    }
}
