use std::iter::Iterator;

pub struct SeqIterator<'a> {
    iter: Box<dyn Iterator<Item = u8> + 'a>,
    reverse: bool,
}

pub trait IntoSeqIterator<'a> {
    fn into_seq_iter(self) -> SeqIterator<'a>;
}

impl<'a, I: IntoIterator<Item = u8> + 'a> IntoSeqIterator<'a> for I {
    fn into_seq_iter(self) -> SeqIterator<'a> {
        SeqIterator {
            iter: Box::new(self.into_iter()),
            reverse: false,
        }
    }
}

pub enum C64Color {
    Black,
    White,
    Red,
    Cyan,
    Purple,
    Green,
    Blue,
    Yellow,
    Orange,
    Brown,
    Pink,
    DarkGray,
    MidGray,
    LightGreen,
    LightBlue,
    LightGray,
}

impl From<C64Color> for u8 {
    fn from(value: C64Color) -> Self {
        match value {
            C64Color::Black => 0,
            C64Color::White => 1,
            C64Color::Red => 2,
            C64Color::Cyan => 3,
            C64Color::Purple => 4,
            C64Color::Green => 5,
            C64Color::Blue => 6,
            C64Color::Yellow => 7,
            C64Color::Orange => 8,
            C64Color::Brown => 9,
            C64Color::Pink => 10,
            C64Color::DarkGray => 11,
            C64Color::MidGray => 12,
            C64Color::LightGreen => 13,
            C64Color::LightBlue => 14,
            C64Color::LightGray => 15,
        }
    }
}

pub enum SeqElement {
    ClearScreen,
    Color(C64Color),
    Reverse(bool),
    Character(u8),
}

impl<'a> Iterator for SeqIterator<'a> {
    type Item = SeqElement;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(0x93) => Some(SeqElement::ClearScreen),
            Some(0x90) => Some(SeqElement::Color(C64Color::Black)),
            Some(0x05) => Some(SeqElement::Color(C64Color::White)),
            Some(0x1c) => Some(SeqElement::Color(C64Color::Red)),
            Some(0x9f) => Some(SeqElement::Color(C64Color::Cyan)),
            Some(0x9c) => Some(SeqElement::Color(C64Color::Purple)),
            Some(0x1e) => Some(SeqElement::Color(C64Color::Green)),
            Some(0x1f) => Some(SeqElement::Color(C64Color::Blue)),
            Some(0x9e) => Some(SeqElement::Color(C64Color::Yellow)),
            Some(0x81) => Some(SeqElement::Color(C64Color::Orange)),
            Some(0x95) => Some(SeqElement::Color(C64Color::Brown)),
            Some(0x96) => Some(SeqElement::Color(C64Color::Pink)),
            Some(0x97) => Some(SeqElement::Color(C64Color::DarkGray)),
            Some(0x98) => Some(SeqElement::Color(C64Color::MidGray)),
            Some(0x99) => Some(SeqElement::Color(C64Color::LightGreen)),
            Some(0x9a) => Some(SeqElement::Color(C64Color::LightBlue)),
            Some(0x9b) => Some(SeqElement::Color(C64Color::LightGray)),
            Some(0x12) => {
                self.reverse = true;
                Some(SeqElement::Reverse(self.reverse))
            }
            Some(0x92) => {
                self.reverse = false;
                Some(SeqElement::Reverse(self.reverse))
            }
            Some(byte) => {
                let code = match byte {
                    0x40..=0x5f | 0x80..=0xbf => byte - 0x40,
                    0xc0..=0xfe => byte - 0x80,
                    0xff => 0x5e,
                    _ => byte,
                };
                if self.reverse {
                    Some(SeqElement::Character(code + 0x80))
                } else {
                    Some(SeqElement::Character(code))
                }
            }
            None => None,
        }
    }
}
