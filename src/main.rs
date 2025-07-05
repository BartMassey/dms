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

type WordSet = HashSet<Word>;

fn find_first(s: &mut Square, words: &[Word], used: &mut WordSet) -> Option<Square> {
    if s.is_full() {
        return Some(s.clone());
    }

    for &w in words {
        if used.contains(&w) {
            continue;
        }
        let range = if s.get_char(0, 0) == '.' {
            0..1
        } else {
            1..10
        };
        for p in range {
            if s.is_fit(p, w) {
                let undo = s.get_pos(p);
                s.set_pos(p, w);
                used.insert(w);
                if let Some(s) = find_first(s, words, used) {
                    return Some(s);
                }
                used.remove(&w);
                s.set_pos(p, undo);
            }
        }
    }

    None
}

fn run() -> Result<Option<Square>, Error> {
    let words = File::open("usa_5.txt")?;
    let words = BufReader::new(words)
        .lines()
        .map(|w| Word::from_str(&w?))
        .collect::<Result<Vec<_>, Error>>()?;

    let mut s = Square::default();
    let mut used = HashSet::with_capacity(25);
    Ok(find_first(&mut s, &words, &mut used))
}

fn main() {
    match run() {
        Err(e) => {
            eprintln!("dms: {}", e);
            exit(1);
        }
        Ok(Some(s)) => println!("{}", s.as_string()),
        Ok(None) => println!("no squares"),
    }
}
