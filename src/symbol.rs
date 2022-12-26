use std::fmt::Display;

#[derive(Hash, Eq, PartialEq)]
pub enum Symbol {
    Epsilon,
    Symbol(char),
}

impl Clone for Symbol {
    fn clone(&self) -> Self {
        match self {
            Self::Epsilon => Self::Epsilon,
            Self::Symbol(arg0) => Self::Symbol(arg0.clone()),
        }
    }
}

impl Copy for Symbol {}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c: char = char::from(*self);
        write!(f, "{}", c)
    }
}

impl From<Symbol> for char {
    fn from(value: Symbol) -> Self {
        match value {
            Symbol::Epsilon => 'ε',
            Symbol::Symbol(c) => c,
        }
    }
}