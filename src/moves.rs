use core::fmt;

#[derive(Debug)]
pub struct Square(pub u8);

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            (self.0 % 8 + 'a' as u8) as char,
            (self.0 / 8 + '1' as u8) as char,
        )
    }
}

pub type Move = u16;
