use crate::words::Word;

use anyhow::{Error, bail};

pub struct Dict {
    word_list: Vec<Word>,
}

impl Dict {
    pub fn new(words: &[&str]) -> Result<Self, Error>  {
        let word_list = words
            .iter()
            .map(|w| Word::from_str(w))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { word_list })
    }

    pub fn from_words(words: &[Word]) -> Self {
        let mut word_list = words.to_vec();
        word_list.sort();
        Self { word_list }
    }

    pub fn iter(&self) -> impl Iterator<Item=&Word> {
        self.word_list.iter()
    }

    pub fn add_str(&mut self, word: &str) -> Result<(), Error> {
        let word = Word::from_str(word)?;
        if !word.is_full() {
            bail!("incomplete word");
        }
        self.word_list.push(word);
        self.word_list.sort();
        Ok(())
    }

    pub fn has_match(&self, target: Word) -> bool {
        self.matches(target).nth(0).is_some()
    }

    pub fn matches(&self, target: Word) -> impl Iterator<Item = Word> {
        self.word_list
            .iter()
            .copied()
            .filter(move |&w| target.is_fit(w))
    }
}

impl<'a> IntoIterator for &'a Dict {
    type Item = &'a Word;
    type IntoIter = std::slice::Iter<'a, Word>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.word_list.iter()
    }
}
