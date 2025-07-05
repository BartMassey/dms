#![allow(clippy::uninlined_format_args)]

mod squares;
mod words;

use squares::*;
use words::*;

use std::io::{BufReader, BufRead};
use std::fs::File;
use std::process::exit;

use anyhow::Error;

fn run() -> Result<(), Error> {
    let words = File::open("usa_5.txt")?;
    let words = BufReader::new(words)
        .lines()
        .map(|w| Word::from_str(&w?))
        .collect::<Result<Vec<_>, Error>>()?;
    // println!("{:?}", words);
    for w in words {
        println!("{}", w.as_string());
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("dms: {}", e);
        exit(1);
    }
}
