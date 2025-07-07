mod squares;
mod words;

use squares::*;
use words::*;

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::process::exit;

use anyhow::Error;
extern crate serde_json;

fn best_pos(s: &Square, words: &[Word]) -> Option<(usize, usize)> {
    let mut scores = Vec::with_capacity(9);
    for p in 1..10 {
        let target = s.get_pos(p);
        if target.is_empty() || target.is_full() {
            continue;
        }
        let nmatches = words
            .iter()
            .filter(|&&w| target.is_fit(w))
            .count();
        scores.push((nmatches, p));
    }

    scores.into_iter().min()
}

fn cross_fit(s: &Square, words: &[Word], pos: usize) -> bool {
    let cross_fit_word = move |target: Word| {
        for &w in words {
            if target.is_fit(w) {
                return true;
            }
        }
        false
    };

    let range = if pos < 5 {
        5..10
    } else {
        0..5
    };

    for p in range {
        let target = s.get_pos(p);
        if !cross_fit_word(target) {
            return false;
        }
    }
    true
}

#[test]
fn test_fitting() {
    let mut s = Square::from_rows([
        "abcde",
        "fghij",
        "klmno",
        "pqrst",
        "uvwxy",
    ]);

    let mut dict: Vec<Word> = (0..10)
        .map(|i| s.get_pos(i))
        .collect();
    dict.push(Word::from_str("agmsy").unwrap());

    s.set_coord(1, 1, '.');
    s.set_coord(1, 2, '.');

    let (_, p) = best_pos(&s, &dict).unwrap();
    assert!(p == 1, "{}", p);

    let word = dict[6];
    assert!(s.is_fit(6, word));
    s.set_pos(6, word);
    assert!(cross_fit(&s, &dict, 6), "{}", s.as_string());

    let word = dict[7];
    assert!(s.is_fit(7, word));
    s.set_pos(7, word);
    assert!(cross_fit(&s, &dict, 7), "{}", s.as_string());
}

fn find_all(s: &mut Square, words: &[Word], results: &mut Vec<Square>) {
    if s.is_empty() {
        for &w in words {
            s.set_pos(0, w);
            find_all(s, words, results);
        }
        return;
    }

    if s.is_full() {
        eprintln!("{}", s.as_string());
        eprintln!();
        results.push(s.clone());
        return;
    }

    let p = if let Some((m, p)) = best_pos(s, words) {
        if m == 0 {
            panic!("internal error: best_pos 0:\n{}\n", s.as_string());
        }
        assert!(p > 0);
        p
    } else {
        panic!("internal error: best_pos None:\n{}\n", s.as_string());
    };

    let target = s.get_pos(p);
    for &w in words.iter().filter(|&&w| target.is_fit(w)) {
        s.set_pos(p, w);
        if cross_fit(s, words, p) {
            find_all(s, words, results);
        }
        s.set_pos(p, target);
    }
}

fn run() -> Result<usize, Error> {
    let words = File::open("usa_5.txt")?;
    let words = BufReader::new(words)
        .lines()
        .map(|w| Word::from_str(&w?))
        .collect::<Result<Vec<_>, Error>>()?;

    let mut s = Square::default();
    let mut results = Vec::new();
    find_all(&mut s, &words, &mut results);

    let save = File::create("squares.json")?;
    serde_json::to_writer(save, &results)?;
    Ok(results.len())
}

fn main() {
    match run() {
        Err(e) => {
            eprintln!("dms: {e}");
            exit(1);
        }
        Ok(nsquares) => {
            println!("{nsquares} squares");
            
        }
    }
}
