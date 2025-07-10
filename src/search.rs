/*!
Theory of operation:

* A word square has five row and five column positions:

       56789
      0
      1
      2
      3
      4

* Start by placing a word in position 1.
* Continue by finding a most-constrained not-full
  position for the next placement.
* For every word that might be placed there, check that it
  will "fit" and not violate any word constraint.
* Recursively call to place another branch.
* If a square is ever filled, record it.

*/

use crate::appstate::*;
use crate::squares::*;
use crate::dict::*;

/// Report the number of most-constrained next-word position
/// matches, and the position. Returns [None] if no
/// placement is possible.
// XXX Used to have a gratuitous collect().
fn best_pos(s: &Square, dict: &Dict) -> Option<(usize, usize)> {
    (0..10)
        .map(|p| (p, s.get_pos(p)))
        .filter(|(_, target)| !target.is_empty() && !target.is_full())
        .map(|(p, target)| (dict.match_count(target), p))
        .min()
}

/// Verify that all cross-targets of position `pos` can be
/// legally filled with something, under the given constraints.
// XXX The order of tests here matters a bit.
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
    /// Accumulate all the word squares under the given
    /// constraints in `results`. Returns [false] if
    /// terminating early, [true] otherwise.
    pub fn find_all(
        &mut self,
        s: &mut Square,
        dict: &Dict,
        results: &mut Vec<Square>,
    ) -> bool {
        self.nodes += 1;

        // Initial case: place a word in the first row and recurse.
        if s.get_pos(0).is_empty() {
            for &w in dict {
                s.set_pos(0, w);
                if !self.find_all(s, dict, results) {
                    return false;
                }
            }
            return true;
        }

        // Base case: found a solution. Save and trace it.
        if s.is_full() {
            // Safety check.
            if !self.doubled {
                assert!(!s.has_double(), "{}", s.as_string());
            }

            // Save the solution.
            results.push(s.clone());

            // Show progress according to style.
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

            // If enough solutions have been found, bail.
            match self.limit {
                Some(limit) => return results.len() < limit,
                None => return true,
            }
        }

        // Recursive case: Try to place a word, then try to solve the rest.

        // Find the placement position.
        let p = if let Some((m, p)) = best_pos(s, dict) {
            // Safety checks.
            if m == 0 {
                panic!("internal error: best_pos 0:\n{}\n", s.as_string());
            }
            assert!(p > 0);

            p
        } else {
            panic!("internal error: best_pos None:\n{}\n", s.as_string());
        };

        // Try to solve the rest. Get possible next words
        // and see if they fit. If so, recurse.
        let target = s.get_pos(p);
        for w in dict.matches(target) {
            // Place the word.
            s.set_pos(p, w);

            // Check for fit.
            let fit = cross_fit(s, dict, p, self.doubled, self.transposed);

            #[allow(clippy::collapsible_if)]
            // I find this much more readable
            if fit {
                // Recurse.
                if !self.find_all(s, dict, results) {
                    return false;
                }
            }

            // Undo the placement.
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
