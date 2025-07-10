use crate::appstate::*;
use crate::squares::*;
use crate::dict::*;

fn best_pos(s: &Square, dict: &Dict) -> Option<(usize, usize)> {
    let mut scores = Vec::with_capacity(9);
    for p in 1..10 {
        let target = s.get_pos(p);
        if target.is_empty() || target.is_full() {
            continue;
        }
        let nmatches = dict.match_count(target);
        scores.push((nmatches, p));
    }

    scores.into_iter().min()
}

fn cross_fit(
    s: &Square,
    dict: &Dict,
    pos: usize,
    doubled: bool,
    transposed: bool,
) -> bool {
    if !transposed && s.is_transposed() {
        return false;
    }

    let range = if pos < 5 {
        5..10
    } else {
        0..5
    };
    if !dict.is_fit(range.map(|p| s.get_pos(p))) {
        return false;
    }

    if !doubled && s.has_double() {
        return false;
    }

    true
}

impl AppState {
    pub fn find_all(&mut self, s: &mut Square, dict: &Dict, results: &mut Vec<Square>) -> bool {
        self.nodes += 1;

        if s.is_empty() {
            for &w in dict {
                s.set_pos(0, w);
                if !self.find_all(s, dict, results) {
                    return false;
                }
            }
            return true;
        }

        if s.is_full() {
            if self.doubled {
                assert!(!s.has_double(), "{}", s.as_string());
            }

            results.push(s.clone());

            match self.trace {
                TraceStyle::None => (),
                TraceStyle::Short => {
                    if results.len() % 100 == 0 {
                        let tr: String = s
                            .get_pos(0)
                            .as_string()
                            .chars()
                            .take(2)
                            .collect();
                        eprintln!("{tr}");
                    }
                },
                TraceStyle::Full => eprintln!("{}\n", s.as_string()),
            }

            match self.limit {
                Some(limit) => return results.len() < limit,
                None => return true,
            }
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
        for w in dict.matches(target) {
            s.set_pos(p, w);
            let fit = cross_fit(s, dict, p, self.doubled, self.transposed);
            #[allow(clippy::collapsible_if)]
            if fit {
                if !self.find_all(s, dict, results) {
                    return false;
                }
            }
            s.set_pos(p, target);
        }

        true
    }
}

#[test]
fn test_fitting() {
    use crate::words::Word;

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
    dict.add_str("fzzij").unwrap();

    let word = Word::from_str("...ij").unwrap();
    s.set_pos(1, word);
    let word = Word::from_str("k.mno").unwrap();
    s.set_pos(2, word);

    let (_, p) = best_pos(&s, &dict).unwrap();
    assert!(p == 2, "{}", p);
    assert!(!s.is_transposed());

    s.set_coord(1, 0, 'a');
    assert!(s.is_transposed());
    s.set_coord(1, 0, 'f');
    let (_, p) = best_pos(&s, &dict).unwrap();
    assert!(p == 2, "{}", p);
    assert!(!s.is_transposed());
    assert!(cross_fit(&s, &dict, 5, false, false), "{}", s.as_string());

    let word = words[6];
    assert!(s.is_fit(6, word));
    s.set_pos(6, word);
    assert!(cross_fit(&s, &dict, 6, false, false), "{}", s.as_string());

    let word = words[7];
    assert!(s.is_fit(7, word));
    s.set_pos(7, word);
    assert!(cross_fit(&s, &dict, 7, false, false), "{}", s.as_string());

    let word = Word::from_str("aakpu").unwrap();
    s.set_pos(5, word);
    assert!(!cross_fit(&s, &dict, 7, true, false), "{}", s.as_string());
}
