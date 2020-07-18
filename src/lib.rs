extern crate logos;

/// https://en.wikipedia.org/wiki/Regular_language
/// https://en.wikipedia.org/wiki/Finite-state_machine

#[macro_use]
mod utils;

pub mod automaton;
pub mod dfa;
pub mod nfa;
pub mod regex;

mod parser;
