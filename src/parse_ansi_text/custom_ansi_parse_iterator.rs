use std::fmt::{Display, Formatter, Result as DisplayResult};
use ansi_parser::{AnsiSequence, parse_escape};
use crate::parse_ansi_text::parse_ansi_split_by_lines_as_spans_iterator::ParseAnsiAsSpansByLinesIterator;
use crate::parse_ansi_text::parse_options::ParseOptions;

pub struct AnsiParseIterator<'a> {
    pending_string: String,
    pub(crate) iterator: Box<dyn Iterator<Item = String> + 'a>,
}

impl<'a> Iterator for AnsiParseIterator<'a> {
    type Item = Output;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO - if string contain the escape code than split to before the escape code and return it and then the escape code
        //        if the string contain the start of the escape code but not the end of it, then split to before the escape code and return it
        //        and wait for the next string to get the rest of the escape code if it is there, if it is not there then return the rest of the string

        if self.pending_string.is_empty() {
            let next = self.iterator.next();

            if next.is_none() {
                return None;
            }

            self.pending_string = next.unwrap().to_string();
        }

        let pos = self.pending_string.find('\u{1b}');
        if let Some(loc) = pos {
            if loc == 0 { // If the beginning of the string is the key for escape code
                let from_loc = self.pending_string[loc..].to_string();
                let res = parse_escape(from_loc.as_str());

                if let Ok(ret) = res { // If there is escape code after the escape key
                    self.pending_string = ret.0.to_string();
                    return Some(Output::Escape(ret.1))
                } else { // If no escape code after the escape key

                    let from_loc = self.pending_string[(loc + 1)..].to_string().clone();
                    let pos = from_loc.find('\u{1b}');
                    if let Some(loc) = pos { // If there is escape key also exists in the middle of the string then split to before the escape code and from it to the end of the string
                        //Added to because it's based one character ahead
                        let loc = loc + 1;

                        let temp = self.pending_string[..loc].to_string();
                        self.pending_string = self.pending_string[loc..].to_string();

                        return Some(Output::TextBlock(temp))
                    } else { // If no other escape key exists in the string, do nothing as the next string might will
                        // let temp = self.pending_string.clone();
                        // self.pending_string = "".to_string();
                        // 
                        // 
                        // return Some(Output::TextBlock(temp))
                    }
                }


            } else { // If in the middle than split to before the escape code and after and keep the after for the next iteration
                let temp = self.pending_string[..loc].to_string();
                self.pending_string = self.pending_string[loc..].to_string();

                return Some(Output::TextBlock(temp))
            }
        } else {
            let temp = self.pending_string.clone();
            self.pending_string = "".to_string();
            return Some(Output::TextBlock(temp))
        }

        let next = self.iterator.next();

        if next.is_none() {
            let temp = self.pending_string.clone();
            self.pending_string = "".to_string();
            return Some(Output::TextBlock(temp))
        }

        self.pending_string = self.pending_string.clone() + next.unwrap().as_str();

        // Return empty
        Some(Output::IgnoreMe)
    }

}

impl AnsiParseIterator<'_> {

    pub fn create<'a>(str_iterator: Box<dyn Iterator<Item=String>>) -> AnsiParseIterator<'a> {
        AnsiParseIterator {
            iterator: str_iterator,
            pending_string: "".to_string(),
        }
    }

    pub fn create_from_str<'a>(str: String) -> AnsiParseIterator<'a> {
        AnsiParseIterator {
            iterator: Box::new(vec![str].into_iter()),
            pending_string: "".to_string(),
        }
    }
}


///This is what is outputted by the parsing iterator.
///Each block contains either straight-up text, or simply
///an ANSI escape sequence.
#[derive(Debug, Clone, PartialEq)]
pub enum Output {
    IgnoreMe,
    TextBlock(String),
    Escape(AnsiSequence),
}

impl Display for Output {
    fn fmt(&self, formatter: &mut Formatter) -> DisplayResult {
        use Output::*;
        match self {
            IgnoreMe => write!(formatter, "IgnoreMe"),
            Output::TextBlock(txt) => write!(formatter, "{}", txt),
            Output::Escape(seq) => write!(formatter, "{}", seq),
        }
    }
}