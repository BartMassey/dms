use crate::words::Word;

use serde::{self, ser::SerializeSeq};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Square([u32; 10]);

impl Square {
    #[allow(unused)]
    pub fn from_rows(rows: [&str; 5]) -> Self {
        let posns: [[u8; 5]; 5] = rows.map(|r| {
            let word = Word::from_str(r).unwrap();
            let mut row = [0u8; 5];
            for (c, b) in row.iter_mut().zip(word.bits()) {
                *c = b;
            }
            row
        });

        let mut s = Self::default();

        #[allow(clippy::needless_range_loop)]
        for i in 0..5 {
            for j in 0..5 {
                s.0[i] |= (posns[i][j] as u32) << (6 * (4 - j));
            }
        }

        #[allow(clippy::needless_range_loop)]
        for i in 0..5 {
            for j in 0..5 {
                s.0[i + 5] |= (posns[j][i] as u32) << (6 * (4 - j));
            }
        }

        s
    }

    fn get_coord(&self, pos: usize, offset: usize) -> Option<u8> {
        assert!(pos < 10 && offset < 5);

        let v = self.0[pos] >> (6 * (4 - offset));
        if v & 0x20 > 0 {
            Some((v & 0x1f) as u8)
        } else {
            None
        }
    }

    #[allow(unused)]
    pub fn set_coord(&mut self, pos: usize, offset: usize, value: char) {
        assert!(pos < 10 && offset < 5);

        let bit_v = if value.is_ascii_alphabetic() {
            0x20 | (value as u8 - b'a') as u32
        } else if value == '.' {
            0
        } else {
            panic!("set_coord: bad value");
        };

        let mask = !(0x3f << (6 * (4 - offset)));
        let v = bit_v << (6 * (4 - offset));
        let target = &mut self.0[pos];
        *target = (*target & mask) | v;

        let (pos, offset) = if pos < 5 {
            (offset + 5, pos)
        } else {
            (offset, pos - 5)
        };

        let mask = !(0x3f << (6 * (4 - offset)));
        let v = bit_v << (6 * (4 - offset));
        let target = &mut self.0[pos];
        *target = (*target & mask) | v;
    }

    #[allow(unused)]
    pub fn get_char(&self, pos: usize, offset: usize) -> char {
        match self.get_coord(pos, offset) {
            Some(v) => (v + b'a') as char,
            None => '.',
        }
    }

    pub fn get_pos(&self, pos: usize) -> Word {
        Word(self.0[pos])
    }

    pub fn set_pos(&mut self, pos: usize, w: Word) {
        self.0[pos] = w.0;
        
        let (xoffset, yoffset) = if pos < 5 {
            (0, 5)
        } else {
            (5, 0)
        };
        let tpos = 6 * (4 - (pos - xoffset));
        for i in 0..5 {
            let x = ((w.0 >> (6 * (4 - i))) & 0x3f) << tpos;
            let mask = !(0x3f << tpos);
            let target = &mut self.0[i + yoffset];
            *target = (*target & mask) | x;
        }
    }

    #[allow(unused)]
    pub fn is_fit(&self, pos: usize, word: Word) -> bool {
        Word(self.0[pos]).is_fit(word)
    }

    pub fn as_string(&self) -> String {
        let mut result = String::with_capacity(35);

        for i in 0..5 {
            let row = self.get_pos(i);
            result += &row.as_string();
            if i < 4 {
                result += "\n";
            }
        }

        result
    }

    #[allow(unused)]
    pub fn is_full(&self) -> bool {
        for pos in 0..5 {
            for offset in 0..5 {
                if self.get_coord(pos, offset).is_none() {
                    return false;
                }
            }
        }

        true
    }

    pub fn is_empty(&self) -> bool {
        (0..5).all(|i| self.get_pos(i).is_empty())
    }

    #[allow(unused)]
    fn fsck_square(&self) {
        for p in 0..10 {
            let word: Vec<char> = self.get_pos(p).chars().collect();
            let (i, range) = if p < 5 {
                (p, 5..10)
            } else {
                (p - 5, 0..5)
            };
            for (j, cross) in range.clone().enumerate() {
                let cross_word = self.get_pos(cross);
                let cross_letter = cross_word.chars().nth(i).unwrap();
                let c = word[j];
                assert_eq!(
                    c, cross_letter,
                    "{} {} {} \n{}", i, j, cross_word,
                    self.as_string(),
                );
            }
        }
    }
}

#[test]
fn test_coord_pos() {
    let mut s = Square::default();
    let wx = Word::from_str("abcde").unwrap();
    s.set_pos(1, wx);
    assert_eq!(s.get_char(1, 1), 'b');
    assert_eq!(s.get_char(6, 0), '.');
    assert_eq!(s.get_char(8, 1), 'd');

    let wy = Word::from_str("udwxy").unwrap();
    assert!(s.is_fit(8, wy));
    assert!(!s.is_fit(9, wy));
    s.set_pos(8, wy);
    assert_eq!(s.get_pos(1), wx);
    assert_eq!(s.get_pos(8), wy);

    for i in 0..5 {
        if i == 1 {
            continue;
        }
        for j in 0..5 {
            if j == 3 {
                continue;
            }
            assert_eq!(s.get_char(i, j), '.', "{i} {j}");
        }
    }
    for i in 5..10 {
        if i == 8 {
            continue;
        }
        for j in 0..5 {
            if j == 1 {
                continue;
            }
            assert_eq!(s.get_char(i, j), '.', "{i} {j}");
        }
    }
}

impl serde::Serialize for Square {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        let mut seq = serializer.serialize_seq(Some(5))?;
        for pos in 0..5 {
            let row = self.get_pos(pos).as_string();
            seq.serialize_element(&row)?;
        }
        seq.end()
    }
}

#[test]
fn test_as_string() {
    let mut s = Square::default();
    let wx = Word::from_str("abcde").unwrap();
    s.set_pos(1, wx);
    let wy = Word::from_str("udwxy").unwrap();
    s.set_pos(8, wy);

    let r = s.as_string();
    let r0 = "...u.\nabcde\n...w.\n...x.\n...y.";
    assert_eq!(&r, r0);
}

#[test]
fn test_set_pos() {
    let mut s = Square::default();

    let set_word_pos = |s: &mut Square, p, w| {
        let word = Word::from_str(w).unwrap();
        s.set_pos(p, word);
        s.fsck_square();
    };

    set_word_pos(&mut s, 0, "abcde");
    set_word_pos(&mut s, 6, "bfghi");
    set_word_pos(&mut s, 7, "cjklm");
    set_word_pos(&mut s, 9, "nhol.");

    for p in [0, 6, 7] {
        let word = s.get_pos(p);
        assert!(word.is_full());
    }

    let t = Square::from_rows([
        "abcdn",
        ".fj.h",
        ".gk.o",
        ".hl.l",
        ".im..",
    ]);
    assert_eq!(s, t, "\n{}\n\n{}", s.as_string(), t.as_string());
}
