use crate::words::Word;

#[derive(Debug)]
pub struct Square([u32; 10]);

impl Default for Square {
    fn default() -> Self {
        Square([0; 10])
    }
}

fn is_fit_word(target: u32, source: u32) -> bool {
    for i in 0..5 {
        let bi = 6 * (4 - i);
        let x = (target >> bi) & 0x3f;
        let y = (source >> bi) & 0x3f;
        if x & 0x20 > 0 && y & 0x20 > 0 && x != y {
            return false;
        }
    }

    true
}

impl Square {
    pub fn get_coord(&self, pos: usize, offset: usize) -> Option<u8> {
        assert!(pos < 10 && offset < 5);

        let v = self.0[pos] >> (6 * (4 - offset));
        if v & 0x20 > 0 {
            Some((v & 0x1f) as u8)
        } else {
            None
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

    pub fn is_fit(&self, pos: usize, word: Word) -> bool {
        is_fit_word(self.0[pos], word.0)
    }
}

#[test]
fn test_coord_pos() {
    let mut s = Square::default();
    let wx = Word::from_str("abcde").unwrap();
    s.set_pos(1, wx);
    assert_eq!(s.get_coord(1, 1).unwrap(), 1);
    assert!(s.get_coord(6, 0).is_none());
    assert_eq!(s.get_coord(8, 1).unwrap(), 3);

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
            assert!(s.get_coord(i, j).is_none(), "{} {}", i, j);
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
            assert!(s.get_coord(i, j).is_none(), "{} {}", i, j);
        }
    }
}
