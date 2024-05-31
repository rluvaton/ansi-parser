use std::fmt::{Display, Formatter, Result as DisplayResult};

use crate::parse_ansi_text::raw_ansi_parse::AnsiSequence;

#[derive(Debug, Clone, PartialEq)]
pub struct Text<'a> {
    pub text: &'a [u8],
    pub location_in_text: usize,
}

///This is what is outputted by the parsing iterator.
///Each block contains either straight-up text, or simply
///an ANSI escape sequence.
#[derive(Debug, Clone, PartialEq)]
pub enum Output<'a> {
    TextBlock(Text<'a>),
    Escape(AnsiSequence<'a>),
}

impl<'a> Display for Output<'a> {
    fn fmt(&self, formatter: &mut Formatter) -> DisplayResult {
        use Output::*;
        match self {
            TextBlock(txt) => write!(
                formatter,
                "{}",
                String::from_utf8(txt.text.to_vec()).unwrap()
            ),
            Escape(seq) => write!(formatter, "{}", seq),
        }
    }
}
