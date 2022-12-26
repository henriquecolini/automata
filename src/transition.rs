use std::fmt::Display;

use crate::{state::State, symbol::Symbol};

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Transition {
    pub from: State,
    pub to: State,
    pub when: Symbol,
}

impl Display for Transition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "δ({},{}) ⊇ {{{}}}", self.from, self.when, self.to)
    }
}