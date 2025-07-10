/*!
Words are represented internally by 30 bits stored in a [u32].
The representation is 5 six-bit fields: the first character
is in the most-significant bits. Each field is of the form

    sccccc

where `s` is 1 if the field is filled, in which case the
remaining five `ccccc` bits are an encoding of the letter:
`'a' == 0`, `'b' == 1`, etc. If `s` is 0, the field is
empty, and the remaining `ccccc` bits must be 0.
*/

// XXX The bit arithmetic done here should mostly be factored: it's
// error-prone and hard to read.

use crate::dict::*;

use anyhow::{Error, bail};

use std::array::from_fn as array_fn;
use std::cmp::Ordering;

/// The word representation, with most of the properties of
/// its underlying [u32].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Word(pub u32);

impl Word {
    /// Turn the given string into a [Word], failing
    /// if it does not meet necessary conditions.
    pub fn from_str(word: &str) -> Result<Self, Error> {
        // Must be right length.
        if word.len() != 5 {
            bail!("word length error");
        }

        // Must be an ASCII lowercase letter, or `.`.  If
        // so, fill the bitfields.
        let mut result = 0u32;
        for (i, c) in word.chars().enumerate() {
            if c == '.' {
                continue;
            }
            if !c.is_ascii_lowercase() {
                bail!("invalid char error");
            }
            let c = c as u8 - b'a';
            result |= (0x20 | c as u32) << (6 * (4 - i));
        }

        Ok(Self(result))
    }

    /// Iterator over the bitfields of the word.
    pub fn bits(self) -> impl Iterator<Item = u8> {
        let mut i = 0;
        std::iter::from_fn(move || {
            if i < 5 {
                let bits = (self.0 >> (6 * (4 - i))) & 0x3f;
                i += 1;
                Some(bits as u8)
            } else {
                None
            }
        })
    }

    /// Get a particular bitfield from the word.
    pub fn get_bits(self, i: usize) -> u8 {
        ((self.0 >> (6 * (4 - i))) & 0x3f) as u8
    }

    /// Word index construction for [Dict].
    // XXX Why is this here instead of there?
    pub fn build_word_index(words: &[Word]) -> WordIndex {
        let mut words = words.to_vec();
        // `std::array::from_fn()` is new and useful.
        // `std::slice::partition_point()` is old and useful.
        array_fn(|i| {
            words.sort_by_key(|&w| (w.get_bits(i), w));
            array_fn(|j| {
                let start = words.partition_point(|w| {
                    w.get_bits(i) < 0x20 | j as u8
                });
                let end = words.partition_point(|w| {
                    w.get_bits(i) <= 0x20 | j as u8
                });
                words[start..end].to_vec()
            })
        })
    }

    /// Iterator over the chars of a word.
    pub fn chars(self) -> impl Iterator<Item = char> {
        self.bits().map(|bits| {
            if bits & 0x20 > 0 {
                ((bits & 0x1f) + b'a') as char
            } else if bits == 0 {
                '.'
            } else {
                panic!("internal error: bad bits value");
            }
        })
    }

    /// Return the word to string form.
    pub fn as_string(self) -> String {
        self.chars().collect()
    }

    /// Check whether this word and the target word
    /// conflict.  Empty fields are allowed in both.
    // XXX Could be rewritten in functional style.
    pub fn is_fit(self, word: Self) -> bool {
        for (target, source) in self.bits().zip(word.bits()) {
            if target & 0x20 > 0 && source & 0x20 > 0 && target != source {
                return false;
            }
        }
        true
    }

    /// True if the word contains all blanks.
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// True if the word contains no blanks.
    pub fn is_full(self) -> bool {
        let mask = 0b00_100000_100000_100000_100000_100000;
        self.0 & mask == mask
    }

    /// Partial order on words containing blanks.  This
    /// returns [true] when the "across" self word is
    /// guaranteed to be lexically greater than the `down`
    /// word.
    pub fn is_transposed(self, down: Word) -> bool {
        for (a, d) in self.bits().zip(down.bits()) {
            if d & 0x20 > 0 {
                match a.cmp(&d) {
                    Ordering::Less => return false,
                    Ordering::Greater => return true,
                    _ => (),
                }
            } else {
                break;
            }
        }
        false
    }
}

#[test]
fn test_is_transposed() {
    let across = Word::from_str("defgh").unwrap();
    let downs = [
        ("dfghi", false),
        ("dezab", false),
        ("d.abc", false),
        ("def.b", false),
        ("da.bc", true),
    ];
    let downs = downs.map(|(w, r)| (Word::from_str(w).unwrap(), r));
    
    for (down, result) in downs {
        assert_eq!(across.is_transposed(down), result, "{}", down.as_string());
    }
}

#[test]
fn test_word() {
    let w = "abcde";
    let word = Word::from_str(w).unwrap();
    assert_eq!(word.0, 0b00_100000_100001_100010_100011_100100);
    let s = word.as_string();
    assert_eq!(w, &s);

    let w = "abc.e";
    let word = Word::from_str(w).unwrap();
    assert_eq!(word.0, 0b00_100000_100001_100010_000000_100100);
    let s = word.as_string();
    assert_eq!(w, &s);
}

#[test]
fn test_is_fit() {
    let target = Word::from_str("ab.d.").unwrap();
    let word = Word::from_str("abcde").unwrap();
    assert!(target.is_fit(word));
    let word = Word::from_str(".bc..").unwrap();
    assert!(target.is_fit(word));
    let word = Word::from_str(".cc..").unwrap();
    assert!(!target.is_fit(word));
}

impl std::fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}
