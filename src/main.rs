mod squares;
mod words;
mod dict;

use squares::*;
use dict::*;

use std::fs::File;
use std::process::exit;

use anyhow::Error;
extern crate serde_json;

fn best_pos(s: &Square, dict: &Dict) -> Option<(usize, usize)> {
    let mut scores = Vec::with_capacity(9);
    for p in 1..10 {
        let target = s.get_pos(p);
        if target.is_empty() || target.is_full() {
            continue;
        }
        let nmatches = dict.matches(target).count();
        scores.push((nmatches, p));
    }

    scores.into_iter().min()
}

fn cross_fit(s: &Square, dict: &Dict, pos: usize) -> bool {
    let range = if pos < 5 {
        5..10
    } else {
        0..5
    };

    for p in range {
        let target = s.get_pos(p);
        if !dict.has_match(target) {
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

    let words: Vec<Word> = (0..10)
        .map(|i| s.get_pos(i))
        .collect();
    let mut dict = Dict::from_words(words.as_ref());
    dict.add_str("agmsy").unwrap();

    s.set_coord(1, 1, '.');
    s.set_coord(1, 2, '.');

    let (_, p) = best_pos(&s, &dict).unwrap();
    assert!(p == 1, "{}", p);

    let word = words[6];
    assert!(s.is_fit(6, word));
    s.set_pos(6, word);
    assert!(cross_fit(&s, &dict, 6), "{}", s.as_string());

    let word = words[7];
    assert!(s.is_fit(7, word));
    s.set_pos(7, word);
    assert!(cross_fit(&s, &dict, 7), "{}", s.as_string());
}

fn find_all(s: &mut Square, dict: &Dict, results: &mut Vec<Square>) {
    if s.is_empty() {
        for &w in dict {
            s.set_pos(0, w);
            find_all(s, dict, results);
        }
        return;
    }

    if s.is_full() {
        eprintln!("{}", s.as_string());
        eprintln!();
        results.push(s.clone());
        return;
    }

    let p = if let Some((m, p)) = best_pos(s, dict) {
        if m == 0 {
            panic!("internal error: best_pos 0:\n{}\n", s.as_string());
        }
        assert!(p > 0);
        p
    } else {
        panic!("internal error: best_pos None:\n{}\n", s.as_string());
    };

    let target = s.get_pos(p);
    for &w in dict.iter().filter(|&&w| target.is_fit(w)) {
        s.set_pos(p, w);
        if cross_fit(s, dict, p) {
            find_all(s, dict, results);
        }
        s.set_pos(p, target);
    }
}

fn run() -> Result<usize, Error> {
    let words = std::fs::read_to_string("usa_5.txt")?;
    let words: Vec<&str> = words
        .lines()
        .collect();
    let dict = Dict::new(words.as_ref())?;

    let mut s = Square::default();
    let mut results = Vec::new();
    find_all(&mut s, &dict, &mut results);

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
