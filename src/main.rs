#![allow(clippy::uninlined_format_args)]

mod squares;
mod words;

use squares::*;
use words::*;

use std::collections::HashSet;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::process::exit;

use anyhow::Error;
extern crate serde_json;

type WordSet = HashSet<Word>;

// XXX Limited; only checks columns, assumes rows are ok.
// XXX Buggy; allows fitting the same word to multiple columns.
fn cols_ok(s: &Square, words: &[Word], used: &WordSet) -> bool {
    let cross_fit = move |target: Word| {
        for &w in words {
            if used.contains(&w) {
                continue;
            }
            if target.is_fit(w) {
                return true;
            }
        }
        false
    };

    for p in 5..10 {
        let target = s.get_pos(p);
        if !cross_fit(target) {
            return false;
        }
    }
    true
}

fn find_all(s: &mut Square, words: &[Word], used: &mut WordSet, results: &mut Vec<Square>) {
    let mut p = 0;
    while p < 5 {
        if s.get_char(p, 0) == '.' {
            break;
        }
        p += 1;
    }
    if p == 5 {
        eprintln!("{}", s.as_string());
        eprintln!();
        results.push(s.clone());
        return;
    }
    for &w in words {
        if used.contains(&w) {
            continue;
        }
        s.set_pos(p, w);
        used.insert(w);
        if cols_ok(s, words, used) {
            find_all(s, words, used, results);
        }
        used.remove(&w);
        s.clear_pos(p);
    }
}

fn run() -> Result<usize, Error> {
    let words = File::open("usa_5.txt")?;
    let words = BufReader::new(words)
        .lines()
        .map(|w| Word::from_str(&w?))
        .collect::<Result<Vec<_>, Error>>()?;

    let mut s = Square::default();
    let mut used = HashSet::with_capacity(25);
    let mut results = Vec::new();
    find_all(&mut s, &words, &mut used, &mut results);
    
    let save = File::create("squares.json")?;
    serde_json::to_writer(save, &results)?;
    Ok(results.len())
}

fn main() {
    match run() {
        Err(e) => {
            eprintln!("dms: {}", e);
            exit(1);
        }
        Ok(nsquares) => {
            println!("{} squares", nsquares);
            
        }
    }
}
