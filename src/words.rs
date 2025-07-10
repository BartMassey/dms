use anyhow::{Error, bail};

use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Word(pub u32);

impl Word {
    pub fn from_str(word: &str) -> Result<Self, Error> {
        if word.len() != 5 {
            bail!("word length error");
        }

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

    pub fn as_string(self) -> String {
        self.chars().collect()
    }

    pub fn is_fit(self, word: Self) -> bool {
        for (target, source) in self.bits().zip(word.bits()) {
            if target & 0x20 > 0 && source & 0x20 > 0 && target != source {
                return false;
            }
        }
        true
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn is_full(self) -> bool {
        let mask = 0b00_100000_100000_100000_100000_100000;
        self.0 & mask == mask
    }

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
