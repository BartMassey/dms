/*!
The dictionary is stored as [Word]s. It is duplicated in multiple
forms for various optimizations. It contains caches that are maintained
using interior mutability for borrow-checker sanity.
*/

use crate::words::*;

use std::cell::RefCell;
use std::collections::HashSet;

use anyhow::Error;
#[cfg(test)]
use anyhow::bail;
use caches::{Cache, lfu::WTinyLFUCache as Wtlfu};

/// This contains 5 copies of the dictionary.
/// The i-th copy contains a split of the dictionary
/// as sorted by position i. The copy is split
/// into a list of words that contain the unique letters
/// 0..25 (`a`..`z`) in order.
pub type WordIndex = [[Vec<Word>; 26]; 5];

/// The dictionary.
pub struct Dict {
    /// Raw word list in order.
    word_list: Vec<Word>,
    /// Word set for quick containment checks.
    word_set: HashSet<Word>,
    /// Split copies of the word list, for quick matching checks.
    word_index: WordIndex,
    /// Cached hits for [Dict::is_fit()].
    hit_cache: RefCell<Wtlfu<Word, bool>>,
    /// Cached match counts for [Dict::matches()] and friends.
    count_cache: RefCell<Wtlfu<Word, usize>>,
}

impl Dict {
    /// Complete the dictionary initialization.
    fn init(word_list: Vec<Word>) -> Self {
        let word_set: HashSet<Word> = word_list.iter().copied().collect();
        let word_index = Word::build_word_index(&word_list);

        // XXX these parameters were mildly hand-tuned. May be close.
        let hit_cache = RefCell::new(Wtlfu::new(40_000, 2000).unwrap());
        let count_cache = RefCell::new(Wtlfu::new(40_000, 2000).unwrap());

        Self { word_list, word_set, word_index, hit_cache, count_cache }
    }

    /// Make a new dictionary from some strings.
    pub fn new(words: &[&str]) -> Result<Self, Error>  {
        let mut word_list = words
            .iter()
            .map(|w| Word::from_str(w))
            .collect::<Result<Vec<_>, _>>()?;
        word_list.sort();
        Ok(Self::init(word_list))
    }

    /// Make a new dictionary from some words.
    pub fn from_words(words: &[Word]) -> Self {
        let mut word_list = words.to_vec();
        word_list.sort();
        Self::init(word_list)
    }

    /// Given an iterator over targets, verify that
    /// they all match in the dictionary.
    pub fn is_fit<T>(&self, targets: T) -> bool
    where
        T: Iterator<Item = Word>
    {
        // Faster to remember whether things are there than
        // to dig for them.
        let mut hit_cache = self.hit_cache.borrow_mut();

        for target in targets {
            // Do we already know this one?
            if let Some(&status) = hit_cache.get(&target) {
                if status {
                    continue;
                } else {
                    return false;
                }
            }

            // If the word is complete, just check for it.
            if target.is_full() {
                let status = self.word_set.contains(&target);
                hit_cache.put(target, status);
                if status {
                    continue;
                } else {
                    return false;
                }
            }

            // Dig for matches in the index.
            let status = self.matches(target).next().is_some();

            // Remember what we found.
            hit_cache.put(target, status);

            // Nope.
            if !status {
                return false;
            }
        }

        true
    }

    /// Return an iterator producing matches of the target from
    /// the dictionary.
    pub fn matches(&self, target: Word) -> impl Iterator<Item = Word> {
        // Find the index with the fewest entries that is
        // consistent with the target. Panic if it doesn't fit.
        let smallest = (0..5)
            .map(|i| (i, target.get_bits(i)))
            .filter(|&(_, b)| b & 0x20 > 0)
            .map(|(i, b)| &self.word_index[i][(b & 0x1f) as usize])
            .min_by_key(|wi| wi.len())
            .unwrap();
        
        // Return the words from the index that fit the target.
        smallest
            .iter()
            .copied()
            .filter(move |&w| target.is_fit(w))
    }

    /// Count the number of matching words. This count does
    /// not have to be exact for its use in the search: a
    /// heuristic would suffice. But for now it's exact.
    pub fn match_count(&self, target: Word) -> usize {
        // Faster to remember the count than compute it.
        let mut count_cache = self.count_cache.borrow_mut();

        // We know this one.
        if let Some(&count) = count_cache.get(&target) {
            return count;
        }

        // Just stupidly iterate the matches.
        let count = self.matches(target).count();

        // Remember what we counted.
        count_cache.put(target, count);

        count
    }
}

// It is convenient to iterate directly over a dictionary
// with a `for` loop.
impl<'a> IntoIterator for &'a Dict {
    type Item = &'a Word;
    type IntoIter = std::slice::Iter<'a, Word>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.word_list.iter()
    }
}

#[cfg(test)]
impl Dict {
    /// Add a string to the dictionary for tests. Very slow
    /// on large dictionaries, as it has to rebuild
    /// everything.
    pub fn add_str(&mut self, word: &str) -> Result<(), Error> {
        let word = Word::from_str(word)?;
        if !word.is_full() {
            bail!("incomplete word");
        }
        let empty_list = Vec::new();
        let mut word_list = std::mem::replace(&mut self.word_list, empty_list);
        word_list.push(word);
        word_list.sort();
        let d = Self::init(word_list);
        *self = d;
        Ok(())
    }
}
