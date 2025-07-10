mod appstate;
mod dict;
mod words;
mod search;
mod squares;

use appstate::*;
use squares::*;
use dict::*;

use std::fs::File;
use std::process::exit;

use anyhow::Error;
use clap::Parser;
extern crate serde_json;

fn run() -> Result<(usize, usize), Error> {
    let args = Args::parse();
    let mut app_state = AppState::new(&args);

    let words = std::fs::read_to_string(args.dict)?;
    let words: Vec<&str> = words
        .lines()
        .collect();
    let dict = Dict::new(words.as_ref())?;

    let mut s = Square::default();
    let mut results = Vec::new();
    app_state.find_all(&mut s, &dict, &mut results);

    let save = File::create(args.output)?;
    serde_json::to_writer(save, &results)?;
    Ok((results.len(), app_state.nodes))
}

fn main() {
    match run() {
        Err(e) => {
            eprintln!("dms: {e}");
            exit(1);
        }
        Ok((nsquares, nnodes)) => {
            println!("{nsquares} squares ({nnodes} nodes)");
        }
    }
}
