use std::{collections::HashMap, io::Write};

use crate::nfae::NFAe;

struct StateData {
    id: usize,
    init: bool,
    fin: bool,
}

impl StateData {
    fn style(self: &Self) -> &str {
        match (self.init, self.fin) {
            (true, true) => "[color=blue][peripheries=2]",
            (true, false) => "[color=blue]",
            (false, true) => "[peripheries=2]",
            (false, false) => "",
        }
    }
}

fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}

pub fn nfae_dot<T: Write>(w: &mut T, nfae: &NFAe, hide_labels: bool) -> Result<(), std::io::Error> {
    let enumerated: HashMap<String, StateData> = nfae
        .get_states()
        .iter()
        .enumerate()
        .map(|x| {
            (
                x.1.to_owned().0,
                StateData {
                    id: x.0,
                    init: nfae.is_initial(x.1),
                    fin: nfae.is_final(x.1),
                },
            )
        })
        .into_iter()
        .collect();
    writeln!(w, "digraph nfae {{")?;
    for (k, v) in &enumerated {
        if hide_labels {
            writeln!(w, "    q{}{}", v.id, v.style())?;
        } else {
            writeln!(w, "    q{}[label=\"{}\"]{}", v.id, escape_default(k), v.style())?
        }
    }
    for t in nfae.get_transitions() {
        let from = enumerated.get(&t.from.0).unwrap();
        let to = enumerated.get(&t.to.0).unwrap();
        writeln!(
            w,
            "    q{} -> q{}[label=\"{}\"]",
            from.id,
            to.id,
            char::from(t.when)
        )?;
    }
    writeln!(w, "}}")
}

pub fn nfae_table<T: Write>(_w: &mut T, _nfae: &NFAe) -> Result<(), std::io::Error> {
    Ok(())
}