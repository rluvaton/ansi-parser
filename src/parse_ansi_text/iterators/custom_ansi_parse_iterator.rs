use std::fmt::{Display, Formatter, Result as DisplayResult};
use std::path::PathBuf;

use ansi_parser::{AnsiSequence, parse_escape};
use crate::iterators::file_iterator_helpers::create_file_iterator;


pub struct AnsiParseIterator<'a> {
    pending_string: &'a str,
    pub(crate) iterator: Box<dyn Iterator<Item = String>>,
}
impl<'a> Iterator for AnsiParseIterator<'a> {
    type Item = Output<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO - if string contain the escape code than split to before the escape code and return it and then the escape code
        //        if the string contain the start of the escape code but not the end of it, then split to before the escape code and return it
        //        and wait for the next string to get the rest of the escape code if it is there, if it is not there then return the rest of the string

        if self.pending_string.is_empty() {
            let next = self.iterator.next();

            if next.is_none() {
                return None;
            }

            // TODO - should use different way to ensure the lifetime of the string
            let next: &'a mut str = next.unwrap().leak();
            self.pending_string = next;
        }

        let pos = self.pending_string.find('\u{1b}');
        if let Some(loc) = pos {
            if loc == 0 { // If the beginning of the string is the key for escape code
                let res = parse_escape(&self.pending_string[loc..]);

                if let Ok(ret) = res { // If there is escape code after the escape key
                    self.pending_string = ret.0;
                    return Some(Output::Escape(ret.1))
                } else { // If no escape code after the escape key
                    let pos = self.pending_string[(loc + 1)..].find('\u{1b}');
                    if let Some(loc) = pos { // If there is escape key also exists in the middle of the string then split to before the escape code and from it to the end of the string
                        //Added to because it's based one character ahead
                        let loc = loc + 1;

                        let temp = &self.pending_string[..loc];
                        self.pending_string = &self.pending_string[loc..];

                        return Some(Output::TextBlock(temp))
                    }
                    
                    // If no other escape key exists in the string, do nothing as the next string might will
                }
            } else { // If in the middle than split to before the escape code and after and keep the after for the next iteration
                let temp = &self.pending_string[..loc];
                self.pending_string = &self.pending_string[loc..];

                return Some(Output::TextBlock(temp))
            }
        } else {
            let temp = self.pending_string;
            self.pending_string = "";
            return Some(Output::TextBlock(temp))
        }

        let next = self.iterator.next();

        if next.is_none() {
            let temp = self.pending_string;
            self.pending_string = "";
            return Some(Output::TextBlock(temp))
        }

        let mut tmp = self.pending_string.to_string();
        tmp.push_str(next.unwrap().as_str());
        // TODO - should use different way to ensure the lifetime of the string
        let tmp: &'a mut str = tmp.leak();
        self.pending_string = tmp;

        // Return empty
        Some(Output::IgnoreMe)
    }

}

impl AnsiParseIterator<'_> {

    pub fn create<'a>(str_iterator: Box<dyn Iterator<Item=String>>) -> AnsiParseIterator<'a> {
        AnsiParseIterator {
            iterator: str_iterator,
            pending_string: "",
        }
    }

    pub fn create_from_str<'a>(str: String) -> AnsiParseIterator<'a> {
        AnsiParseIterator {
            iterator: Box::new(vec![str].into_iter()),
            pending_string: "",
        }
    }


    pub fn create_from_file_path<'a>(input_file_path: PathBuf) -> AnsiParseIterator<'a> {
        AnsiParseIterator {
            iterator: create_file_iterator(input_file_path),
            pending_string: "",
        }
    }
}


///This is what is outputted by the parsing iterator.
///Each block contains either straight-up text, or simply
///an ANSI escape sequence.
#[derive(Debug, Clone, PartialEq)]
pub enum Output<'a> {
    IgnoreMe,
    TextBlock(&'a str),
    Escape(AnsiSequence),
}

impl<'a> Display for Output<'a> {
    fn fmt(&self, formatter: &mut Formatter) -> DisplayResult {
        use Output::*;
        match self {
            IgnoreMe => write!(formatter, "IgnoreMe"),
            Output::TextBlock(txt) => write!(formatter, "{}", txt),
            Output::Escape(seq) => write!(formatter, "{}", seq),
        }
    }
}
