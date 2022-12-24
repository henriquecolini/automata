use crate::regex::RegularExpression;
use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    fmt::Display,
};

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Symbol {
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
        Self {
            from: self.from.clone(),
            when: self.when.clone(),
        }
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

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Epsilon => write!(f, "ε"),
            Symbol::Symbol(c) => write!(f, "{}", c),
        }
    }
}

impl Display for NFAe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[epsilon-NFA]\nq = {{")?;
        let mut first = true;
        for qi in &self.states {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{}", qi.0)?;
            first = false;
        }
        writeln!(f, "}}")?;
        writeln!(f, "q0 = {}", self.initial)?;
        write!(f, "F = {{")?;
        let mut first = true;
        for qi in &self.states {
            if !qi.1 {
                continue;
            }
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{}", qi.0)?;
            first = false;
        }
        write!(f, "}}")?;
        for t in &self.transitions {
            write!(f, "\nd({}, {}) = {:?}", t.0.from, t.0.when, t.1)?;
        }
        write!(f, "")
    }
}

type Nd<'a> = u32;
type Ed<'a> = (Nd<'a>, Nd<'a>, Symbol);

impl<'a> dot::GraphWalk<'a, Nd<'a>, Ed<'a>> for NFAe {
    fn nodes(&'a self) -> dot::Nodes<'a, Nd<'a>> {
        let mut nodes = Vec::with_capacity(self.states.len());
        for a in &self.states {
            nodes.push(*a.0);
        }
        nodes.sort();
        Cow::Owned(nodes)
    }

    fn edges(&'a self) -> dot::Edges<'a, Ed<'a>> {
        let mut edges = Vec::with_capacity(self.transitions.len());
        for trans in &self.transitions {
            for dst in trans.1 {
                edges.push((trans.0.from, *dst, trans.0.when));
            }
        }
        Cow::Owned(edges)
    }

    fn source(&'a self, edge: &Ed<'a>) -> Nd<'a> {
        edge.0
    }

    fn target(&'a self, edge: &Ed<'a>) -> Nd<'a> {
        edge.1
    }
}

impl<'a> dot::Labeller<'a, Nd<'a>, Ed<'a>> for NFAe {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new("epsilonnfa").unwrap()
    }

    fn node_id(&'a self, n: &Nd) -> dot::Id<'a> {
        let end = self.states[n];
        dot::Id::new(match end {
            true => format!("fq{}", n),
            false => format!("q{}", n),
        }).unwrap()
    }

    fn edge_label<'b>(&'b self, e: &Ed) -> dot::LabelText<'b> {
        dot::LabelText::LabelStr(
            match e.2 {
                Symbol::Epsilon => "eps".into(),
                Symbol::Symbol(c) => format!("{}", c),
            }
            .into(),
        )
    }
}

fn regex_to_nfae_inner(regex: &RegularExpression, nfae: &mut NFAe, initial: u32, end: u32) {
    match regex {
        RegularExpression::Epsilon => nfae.add_transition(initial, end, Symbol::Epsilon),
        RegularExpression::Symbol(symbol) => {
            nfae.add_transition(initial, end, Symbol::Symbol(*symbol))
        }
        RegularExpression::Group(inner) => {
            regex_to_nfae_inner(&inner, nfae, initial, end);
        }
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
        }
        RegularExpression::Concat(a, b) => {
            let middle = nfae.add_state(false);
            regex_to_nfae_inner(&a, nfae, initial, middle);
            regex_to_nfae_inner(&b, nfae, middle, end);
        }
        RegularExpression::Closure(inner) => {
            let m_start = nfae.add_state(false);
            let m_end = nfae.add_state(false);
            nfae.add_transition(initial, m_start, Symbol::Epsilon);
            nfae.add_transition(m_end, end, Symbol::Epsilon);
            nfae.add_transition(initial, end, Symbol::Epsilon);
            nfae.add_transition(m_end, m_start, Symbol::Epsilon);
            regex_to_nfae_inner(&inner, nfae, m_start, m_end);
        }
    }
}

pub fn regex_to_nfae(regex: &RegularExpression) -> NFAe {
    let mut nfae = NFAe::new();
    let initial = nfae.initial;
    let end = nfae.add_state(true);
    regex_to_nfae_inner(regex, &mut nfae, initial, end);
    nfae
}
