use rand::prelude::*;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Generator {
    alphabet: Vec<char>,
    max_depth: u8,
    actual_depth: u8,
    rng: ThreadRng,
}

pub fn new_generator(alphabet: HashSet<char>, max_depth: u8) -> Generator {
    Generator {
        alphabet: alphabet.into_iter().collect(),
        max_depth,
        actual_depth: 0,
        rng: rand::thread_rng(),
    }
}

impl Generator {
    pub fn with_max_depth(mut self, max_depth: u8) -> Self {
        self.max_depth = max_depth;
        self
    }

    pub fn random(alphabet: &HashSet<char>) -> String {
        Self::random_with_rng(
            &alphabet.iter().map(|x| *x).collect(),
            &mut rand::thread_rng(),
        )
    }

    fn random_with_rng(alphabet: &Vec<char>, rng: &mut ThreadRng) -> String {
        let alphalen = alphabet.len();
        let n = rng.gen_range(0, alphalen + 2);
        return if n == alphalen {
            ".".to_string()
        } else if n < alphalen {
            alphabet[n].to_string()
        } else {
            "𝜀".to_string()
        };
    }

    pub fn letter(&mut self) -> String {
        Self::random_with_rng(&self.alphabet, &mut self.rng)
    }

    pub fn run(&mut self) -> String {
        if self.actual_depth == self.max_depth {
            return self.letter();
        }

        const TOTAL: u8 = 7;
        let choice = self.rng.gen_range(0, TOTAL);
        self.actual_depth += 1;
        let rec1 = self.run();

        let ret = if choice < 5 {
            if choice == 0 {
                format!("({})", rec1)
            } else if choice == 1 {
                format!("{}*", rec1)
            } else if choice == 2 {
                format!("{}+", rec1)
            } else if choice == 3 {
                format!("{}?", rec1)
            } else {
                self.letter()
            }
        } else {
            let rec2 = self.run();
            if choice == 5 {
                format!("{}{}", rec1, rec2)
            } else {
                format!("{}|{}", rec1, rec2)
            }
        };
        self.actual_depth -= 1;

        return ret;
    }
}
