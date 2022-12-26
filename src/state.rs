use std::{collections::BTreeSet, fmt::{Display, Debug}};

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct State(pub String);

pub type StateSet = BTreeSet<State>;

impl From<State> for String {
    fn from(value: State) -> Self {
        value.0
    }
}

impl From<String> for State {
    fn from(value: String) -> Self {
        State(value)
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>", self.0)
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

// pub fn state_set_str (set: &StateSet) -> String {

//}