use crate::regex::RegularExpression;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Hash, Eq, PartialEq)]
enum Symbol {
    Epsilon,
    Symbol(char),
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Transition {
    from: u32,
    when: Symbol,
}

pub struct NFAe {
    pub increment: u32,
    initial: u32,
    states: HashMap<u32, bool>,
    transitions: HashMap<Transition, HashSet<u32>>,
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

impl Clone for Transition {
    fn clone(&self) -> Self {
        Self { from: self.from.clone(), when: self.when.clone() }
    }
}

impl Copy for Transition {}


impl NFAe {
    fn new() -> NFAe {
        let mut nfae = NFAe {
            increment: 0,
            initial: 0,
            states: HashMap::new(),
            transitions: HashMap::new(),
        };
        nfae.add_state(false);
        nfae
    }
    fn add_state(&mut self, end: bool) -> u32 {
        let id = self.increment;
        self.states.insert(id, end);
        self.increment += 1;
        id
    }
    fn add_transition(&mut self, from: u32, to: u32, when: Symbol) {
        let trans = Transition { from, when };
		let maybe = self.transitions.get_mut(&trans);
        let tos = match maybe {
            Some(a) => a,
            None => {
				self.transitions.insert(trans, HashSet::<u32>::new());
				self.transitions.get_mut(&trans).unwrap()
			}
        };
		tos.insert(to);
    }
}

fn regex_to_nfae_inner(regex: &RegularExpression, nfae: &mut NFAe, initial: u32, end: u32) {
    match regex {
        RegularExpression::Epsilon => {
            nfae.add_transition(initial, end, Symbol::Epsilon)
        },
        RegularExpression::Symbol(symbol) => {
            nfae.add_transition(initial, end, Symbol::Symbol(*symbol))
        },
        RegularExpression::Group(inner) => {
            regex_to_nfae_inner(&inner, nfae, initial, end);
        },
        RegularExpression::Union(a, b) => {
            let a_start = nfae.add_state(false);
            let a_end = nfae.add_state(false);
            let b_start = nfae.add_state(false);
            let b_end = nfae.add_state(false);
            nfae.add_transition(initial, a_start, Symbol::Epsilon);
            nfae.add_transition(initial, b_start, Symbol::Epsilon);
            nfae.add_transition(a_end, end, Symbol::Epsilon);
            nfae.add_transition(b_end, end, Symbol::Epsilon);
            regex_to_nfae_inner(&a, nfae, a_start, a_end);
            regex_to_nfae_inner(&b, nfae, b_start, b_end);
        },
        RegularExpression::Concat(a, b) => {
            let middle = nfae.add_state(false);
            regex_to_nfae_inner(&a, nfae, initial, middle);
            regex_to_nfae_inner(&b, nfae, middle, end);
        },
        RegularExpression::Closure(inner) => {
            let m_start = nfae.add_state(false);
            let m_end = nfae.add_state(false);
            nfae.add_transition(initial, m_start, Symbol::Epsilon);
            nfae.add_transition(end, m_end, Symbol::Epsilon);
            nfae.add_transition(initial, end, Symbol::Epsilon);
            nfae.add_transition(m_end, m_start, Symbol::Epsilon);
            regex_to_nfae_inner(&inner, nfae, m_start, m_end);
        },
    }
}

pub fn regex_to_nfae(regex: &RegularExpression) -> NFAe {
    let mut nfae = NFAe::new();
    let initial = nfae.initial;
    let end = nfae.add_state(true);
    regex_to_nfae_inner(regex, &mut nfae, initial, end);
    print!("{:?}", nfae.transitions);
    nfae
}
