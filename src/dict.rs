use crate::words::Word;

use std::cell::RefCell;
use std::collections::HashSet;

use anyhow::{Error, bail};
use caches::{Cache, lfu::WTinyLFUCache as Wtlfu};

pub struct Dict {
    word_list: Vec<Word>,
    word_set: HashSet<Word>,
    hit_cache: RefCell<HashSet<Word>>,
    count_cache: RefCell<Wtlfu<Word, usize>>,
}

impl Dict {
    fn init(word_list: Vec<Word>) -> Self {
        let word_set: HashSet<Word> = word_list.iter().copied().collect();
        let hit_cache = RefCell::new(HashSet::with_capacity(10));
        let count_cache = RefCell::new(Wtlfu::new(128, 4).unwrap());
        Self { word_list, word_set, hit_cache, count_cache }
    }

    pub fn new(words: &[&str]) -> Result<Self, Error>  {
        let word_list = words
            .iter()
            .map(|w| Word::from_str(w))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self::init(word_list))
    }

    pub fn from_words(words: &[Word]) -> Self {
        let mut word_list = words.to_vec();
        word_list.sort();
        Self::init(word_list)
    }

    pub fn iter(&self) -> impl Iterator<Item=&Word> {
        self.word_list.iter()
    }

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

    pub fn is_fit<T>(&self, targets: T) -> bool
    where
        T: Iterator<Item = Word>
    {
        let mut hit_cache = self.hit_cache.borrow_mut();
        let mut fits = HashSet::with_capacity(hit_cache.len());

        'search: for target in targets {
            if target.is_full() {
                if hit_cache.contains(&target) {
                    fits.insert(target);
                    continue;
                }

                if self.word_set.contains(&target) {
                    fits.insert(target);
                    continue;
                }
                
                return false;
            }

            for &h in &*hit_cache {
                if target.is_fit(h) {
                    fits.insert(h);
                    continue 'search;
                }
            }

            if let Some(h) = self.matches(target).next() {
                fits.insert(h);
            } else {
                return false;
            }
        }

        *hit_cache = fits;
        true
    }

    pub fn matches(&self, target: Word) -> impl Iterator<Item = Word> {
        self.word_list.iter().copied().filter(move |&w| target.is_fit(w))
    }

    pub fn match_count(&self, target: Word) -> usize {
        let mut count_cache = self.count_cache.borrow_mut();

        if let Some(&count) = count_cache.get(&target) {
            return count;
        }

        let count = self.matches(target).count();
        count_cache.put(target, count);
        count
    }
}

impl<'a> IntoIterator for &'a Dict {
    type Item = &'a Word;
    type IntoIter = std::slice::Iter<'a, Word>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.word_list.iter()
    }
}
