use anyhow::{bail, Error};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Word(pub u32);

impl Default for Word {
    fn default() -> Self {
        Self(0)
    }
}

impl Word {
    pub fn from_str(word: &str) -> Result<Self, Error> {
        if word.len() != 5 {
            bail!("word length error");
        }

        let mut result = 0u32;
        for (i, c) in word.chars().enumerate() {
            if !c.is_ascii_lowercase() {
                bail!("invalid char error");
            }
            let c = c as u8 - b'a';
            result |= (0x20 | c as u32) << (6 * (4 - i));
        }
        Ok(Self(result))
    }

    pub fn chars(self) -> impl Iterator<Item = char> {
        let mut i = 0;
        std::iter::from_fn(move || {
            if i < 5 {
                let c = (self.0 >> (6 * (4 - i))) & 0x1f;
                i += 1;
                Some((c as u8 + b'a') as char)
            } else {
                None
            }
        })
    }

    pub fn as_string(self) -> String {
        self.chars().collect()
    }
}

#[test]
fn test_word() {
    let w = "abcde";
    let word = Word::from_str(w).unwrap();
    assert_eq!(word.0, 0b00_100000_100001_100010_100011_100100);

    let s = word.as_string();
    assert_eq!(w, &s);
}

impl std::fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}
