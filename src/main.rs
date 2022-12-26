use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::time::Instant;

use crate::enfa::nfae_to_dfae;

mod transition;
mod symbol;
mod state;
mod enfa;
mod regex;

fn main() {
    let input = std::env::args()
        .nth(1)
        .expect("Missing input regular expression.");

    let start = Instant::now();
    let regex = regex::parse(&input);
    let duration = start.elapsed();
    println!("Parsing duration: {:?}", duration);

    let start = Instant::now();
    let nfae = enfa::regex_to_nfae(&regex);
    let duration = start.elapsed();
    println!("RegEx to eNFA duration: {:?}", duration);

    let nfae = nfae_to_dfae(&nfae);

    if let Some(dot_path) = std::env::args().nth(2) {
        let start = Instant::now();
        let mut dot_file = File::create(&dot_path).unwrap();
        dot::render(&nfae, &mut dot_file).unwrap();
        let duration = start.elapsed();
        println!("Rendering duration: {:?}", duration);

        if let Some(svg_path) = std::env::args().nth(3) {
            let svg = Command::new("dot")
                .arg("-Tsvg")
                .arg(&dot_path)
                .output()
                .expect("Failed to call dot program");
			let svg = String::from_utf8_lossy(&svg.stdout);
			let svg_file = File::create(&svg_path).unwrap();
			write!(&svg_file, "{}", svg).expect("Failed to write to svg file.");
        }
    } else {
        println!("{}", nfae);
    }
}
