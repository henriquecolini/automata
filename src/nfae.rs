use crate::{
    regex::RegularExpression,
    state::{State, StateSet},
    symbol::Symbol,
    transition::Transition,
};
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fmt::Display,
};

pub struct NFAe {
    increment: u32,
    states: StateSet,
    finals: StateSet,
    initial: Option<State>,
    transitions: HashSet<Transition>,
}

impl NFAe {
    pub fn new() -> NFAe {
        NFAe {
            increment: 0,
            states: BTreeSet::new(),
            finals: BTreeSet::new(),
            initial: None,
            transitions: HashSet::new(),
        }
    }
    pub fn add_state(&mut self, id: State, end: bool) -> State {
        self.states.insert(id.to_owned());
        if end {
            self.finals.insert(id.to_owned());
        }
        id
    }
    pub fn add_state_auto(&mut self, end: bool) -> State {
        while let Some(_) = self.states.get(&State(format!("q{}", self.increment))) {
            self.increment += 1;
        }
        self.add_state(format!("q{}", self.increment).into(), end)
    }
    pub fn add_transition(&mut self, from: &State, to: &State, when: Symbol) {
        self.transitions.insert(Transition {
            from: from.to_owned(),
            to: to.to_owned(),
            when,
        });
    }
    fn closure_inner(&self, hs: &mut StateSet) {
        let prev = hs.clone();
        for state in prev.iter() {
            for t in &self.transitions {
                if t.from.0 != state.0 {
                    continue;
                }
                if let Symbol::Epsilon = t.when {
                    hs.insert(t.to.to_owned());
                }
            }
        }
        if hs.len() > prev.len() {
            self.closure_inner(hs)
        }
    }
    pub fn closure(&self, state: State) -> StateSet {
        if !self.states.contains(&state) {
            return BTreeSet::new();
        }
        let mut hs = BTreeSet::new();
        hs.insert(state);
        self.closure_inner(&mut hs);
        hs
    }
    pub fn alphabet(&self) -> HashSet<char> {
        let mut alphabet = HashSet::new();
        for tr in &self.transitions {
            if let Symbol::Symbol(c) = tr.when {
                alphabet.insert(c);
            }
        }
        alphabet
    }

    pub fn get_states(&self) -> StateSet {
        self.states.to_owned()
    }

    pub fn get_transitions(&self) -> HashSet<Transition> {
        self.transitions.to_owned()
    }

    pub fn is_initial(&self, x: &State) -> bool {
        match &self.initial {
            Some(st) => x.0 == st.0,
            None => false,
        }
    }

    pub fn is_final(&self, x: &State) -> bool {
        self.finals.contains(x)
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
        if let Some(initial) = &self.initial {
            writeln!(f, "q0 = {}", initial.0)?;
        } else {
            writeln!(f, "q0 = None")?;
        }
        write!(f, "F = {{")?;
        let mut first = true;
        for qi in &self.finals {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{}", qi.0)?;
            first = false;
        }
        write!(f, "}}")?;
        for t in &self.transitions {
            write!(f, "\nd({:?}, {}) = {:?}", t.from, t.when, t.to)?;
        }
        write!(f, "")
    }
}

fn regex_to_nfae_inner(regex: &RegularExpression, nfae: &mut NFAe, initial: &State, end: &State) {
    match regex {
        RegularExpression::Epsilon => nfae.add_transition(initial, end, Symbol::Epsilon),
        RegularExpression::Symbol(symbol) => {
            nfae.add_transition(initial, end, Symbol::Symbol(*symbol))
        }
        RegularExpression::Group(inner) => {
            regex_to_nfae_inner(&inner, nfae, initial, end);
        }
        RegularExpression::Union(a, b) => {
            let a_start = &nfae.add_state_auto(false);
            let a_end = &nfae.add_state_auto(false);
            let b_start = &nfae.add_state_auto(false);
            let b_end = &nfae.add_state_auto(false);
            nfae.add_transition(initial, a_start, Symbol::Epsilon);
            nfae.add_transition(initial, b_start, Symbol::Epsilon);
            nfae.add_transition(a_end, end, Symbol::Epsilon);
            nfae.add_transition(b_end, end, Symbol::Epsilon);
            regex_to_nfae_inner(&a, nfae, a_start, a_end);
            regex_to_nfae_inner(&b, nfae, b_start, b_end);
        }
        RegularExpression::Concat(a, b) => {
            let middle = &nfae.add_state_auto(false);
            regex_to_nfae_inner(&a, nfae, initial, middle);
            regex_to_nfae_inner(&b, nfae, middle, end);
        }
        RegularExpression::Closure(inner) => {
            let m_start = &nfae.add_state_auto(false);
            let m_end = &nfae.add_state_auto(false);
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
    let initial = nfae.add_state_auto(false);
    let end = nfae.add_state_auto(true);
    regex_to_nfae_inner(regex, &mut nfae, &initial, &end);
    nfae.initial = Some(initial);
    nfae
}

pub fn nfae_to_dfae(nfae: &NFAe) -> NFAe {
    let mut dfa = NFAe::new();
    let abc = nfae.alphabet();
    let mut added = BTreeSet::new();
    if let Some(state) = &nfae.initial {
        let initial = nfae.closure(state.to_owned());
        let initial_str = format!("{:?}", initial);
        dfa.add_state(
            initial_str.to_owned().into(),
            nfae.finals.intersection(&initial).count() > 0,
        );
        dfa.initial = Some(initial_str.into());
        added.insert(initial);
    }
    while added.len() > 0 {
        let prev_added = added.clone();
        // proceed::proceed();
        added.clear();
        for add in &prev_added {
            let mut next_states: HashMap<char, StateSet> = HashMap::new();
            for c in &abc {
                next_states.insert(*c, BTreeSet::new());
            }
            for single in add {
                for trans in &nfae.transitions {
                    if trans.from != *single {
                        continue;
                    }
                    if let Symbol::Symbol(c) = trans.when {
                        next_states
                            .get_mut(&c)
                            .unwrap()
                            .extend(nfae.closure(trans.to.to_owned()));
                    }
                }
            }
            let from_str = format!("{:?}", &add);
            for next in next_states {
                let to_str = format!("{:?}", &next.1);
                if !dfa.states.contains(&State(to_str.to_owned())) {
                    dfa.add_state(
                        to_str.to_owned().into(),
                        nfae.finals.intersection(&next.1).count() > 0,
                    );
                    added.insert(next.1);
                }
                dfa.add_transition(
                    &State(from_str.to_owned()),
                    &State(to_str),
                    Symbol::Symbol(next.0),
                );
            }
        }
    }
    dfa
}
