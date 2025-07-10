/*!
Find all 5×5 word squares under constraints.

See the crate README for more information.
*/

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

/// Split the runner out so that errors can be handled
/// reasonably in [main()].
fn run() -> Result<(usize, usize), Error> {
    let args = Args::parse();
    let mut app_state = AppState::new(&args);

    // Build the dictionary.
    let words = std::fs::read_to_string(args.dict)?;
    let words: Vec<&str> = words
        .lines()
        .collect();
    let dict = Dict::new(words.as_ref())?;

    // Run the search.
    let mut s = Square::default();
    let mut results = Vec::new();
    app_state.find_all(&mut s, &dict, &mut results);

    // Save the result.
    let save = File::create(args.output)?;
    serde_json::to_writer(save, &results)?;

    // Report success.
    Ok((results.len(), app_state.nodes))
}

/// Run the whole operation.
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
