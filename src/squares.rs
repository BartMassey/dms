use crate::words::Word;

#[derive(Debug, Clone, Default)]
pub struct Square([u32; 10]);

impl Square {
    fn get_coord(&self, pos: usize, offset: usize) -> Option<u8> {
        assert!(pos < 10 && offset < 5);

        let v = self.0[pos] >> (6 * (4 - offset));
        if v & 0x20 > 0 {
            Some((v & 0x1f) as u8)
        } else {
            None
        }
    }

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
            self.0[i + yoffset] |= x;
        }
    }

    pub fn clear_pos(&mut self, pos: usize) {
        self.0[pos] = 0;
        
        let (xoffset, yoffset) = if pos < 5 {
            (0, 5)
        } else {
            (5, 0)
        };
        let tpos = 6 * (4 - (pos - xoffset));
        for i in 0..5 {
            self.0[i + yoffset] &= !(0x3f << tpos);
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
            assert_eq!(s.get_char(i, j), '.', "{} {}", i, j);
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
            assert_eq!(s.get_char(i, j), '.', "{} {}", i, j);
        }
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
