use std::fmt::{Display, Formatter, Result as DisplayResult};

use crate::parse_ansi_text::raw_ansi_parse::{AnsiSequence};

#[derive(Debug, Clone, PartialEq)]
pub struct Text {
    pub(crate) text: String,
    pub(crate) location_in_text: usize,
}

///This is what is outputted by the parsing iterator.
///Each block contains either straight-up text, or simply
///an ANSI escape sequence.
#[derive(Debug, Clone, PartialEq)]
pub enum Output {
    TextBlock(Text),
    Escape(AnsiSequence),
}

impl<'a> Display for Output {
    fn fmt(&self, formatter: &mut Formatter) -> DisplayResult {
        use Output::*;
        match self {
            TextBlock(txt) => write!(formatter, "{}", txt.text),
            Escape(seq) => write!(formatter, "{}", seq),
        }
    }
}

