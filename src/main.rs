use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::time::Instant;

use crate::export::{nfae_dot, nfae_table};

mod export;
mod nfae;
mod regex;
mod state;
mod symbol;
mod transition;

#[derive(Parser)]
#[command(name = "Finite Automata")]
#[command(author = "Henrique Colini")]
#[command(version = "0.1")]
#[command(about = "Parses Formal Regular Expressions and outputs graphs.", long_about = None)]
struct Args {
    expression: String,
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    dfa: bool,
    #[command(subcommand)]
    export: Option<ExportSettings>,
}

#[derive(Subcommand)]
enum ExportSettings {
    DOT {
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        no_labels: bool,
        output: String,
    },
    SVG {
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        no_labels: bool,
        output: String,
    },
    Table {
        output: String,
    },
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    // let input = std::env::args()
    //     .nth(1)
    //     .expect("Missing input regular expression.");

    let start = Instant::now();
    let regex = regex::parse(&args.expression);
    let duration = start.elapsed();
    println!("Parsing duration: {:?}", duration);

    let start = Instant::now();
    let mut nfae = nfae::regex_to_nfae(&regex);
    let duration = start.elapsed();
    println!("RegEx to NFA-ε duration: {:?}", duration);

    if args.dfa {
        let start = Instant::now();
        nfae = nfae::nfae_to_dfae(&nfae);
        let duration = start.elapsed();
        println!("NFA-ε to DFA duration: {:?}", duration);
    }

    if let Some(export) = args.export {
        match export {
            ExportSettings::DOT { no_labels, output } => {
                let start = Instant::now();
                let mut dot_file = File::create(output).unwrap();
                nfae_dot(&mut dot_file, &nfae, no_labels).unwrap();
                let duration = start.elapsed();
                println!("Exporting duration: {:?}", duration);
            }
            ExportSettings::SVG { no_labels, output } => {
                let start = Instant::now();
                let mut child = Command::new("dot")
                    .arg("-Tsvg")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("Failed to call dot program");
                if let Some(mut stdin) = child.stdin.take() {
                    nfae_dot(&mut stdin, &nfae, no_labels)?;
                }
                let generated = child.wait_with_output()?;
                let mut svg_file = File::create(output).unwrap();
                write!(&mut svg_file, "{}", String::from_utf8_lossy(&generated.stdout))?;
                let duration = start.elapsed();
                println!("Exporting duration: {:?}", duration);
            }
            ExportSettings::Table { output } => {
                let start = Instant::now();
                let mut table_file = File::create(output).unwrap();
                nfae_table(&mut table_file, &nfae).unwrap();
                let duration = start.elapsed();
                println!("Exporting duration: {:?}", duration);
            },
        }
    }
    Ok(())
}
