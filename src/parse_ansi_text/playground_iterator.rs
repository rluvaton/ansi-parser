use std::fmt::{Display, Formatter, Result as DisplayResult};
use crate::parse_ansi_text::colors::{BLACK_BACKGROUND_CODE, RED_BACKGROUND_CODE};
use crate::parse_ansi_text::constants::RESET_CODE;
use std::iter::Iterator;
use ansi_parser::{AnsiSequence, parse_escape};
use ansi_parser::Output::{Escape, TextBlock};

fn run() {
    // TODO - create string iterator that can be swaopped to file iterator or stdin or whatever
    // TODO - try the https://crates.io/crates/get_chunk crate

    let black_background_code = BLACK_BACKGROUND_CODE.to_string();

    let mut first_part_for_black_background = black_background_code.clone();
    let second_part_for_black_background =
        first_part_for_black_background.split_off(black_background_code.len() / 2);

    let input_chunks: Vec<String> = vec![
        RED_BACKGROUND_CODE.to_string(),
        "Hello, World!".to_string(),
        RESET_CODE.to_string(),
        " ".to_string(),
        // Split same style to two parts to make sure it works
        first_part_for_black_background,
        second_part_for_black_background,
        "Goodbye".to_string(),
        " world!".to_string(),
    ];
    let iterator = RandomStringsIterator { vec: input_chunks, index: 0 };

    let ansi_parse_iterator = AnsiParseIterator {
        pending_string: "".to_string(),
        iterator: Box::new(iterator),
    };
    // AnsiParseIterator {
    //     dat: iterator.into_iter()
    // }
    ansi_parse_iterator.for_each(|item| {
        println!("{:#?}", item);
    });

    // TODO - find a better way to create iterator from input, just a function that get the 
    // "".random_strings(input_chunks)
    // TODO - ansi parse on the iterator
}

pub trait RandomStrings {
    fn random_strings(&self, vec: Vec<String>) -> RandomStringsIterator;
}

impl RandomStrings for str {
    fn random_strings(&self, vec: Vec<String>) -> RandomStringsIterator {
        RandomStringsIterator { vec, index: 0 }
    }
}

#[derive(Debug)]
pub struct RandomStringsIterator {
    vec: Vec<String>,
    index: usize,
}

impl Iterator for RandomStringsIterator {
    type Item = String;

    // https://users.rust-lang.org/t/how-to-write-iterator-adapter/8835/2
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len() {
            let item = self.vec[self.index].clone();
            self.index += 1;
            return Some(item);
        }

        return None;
    }
}


pub struct AnsiParseIterator<'a> {
    pending_string: String,
    iterator: Box<dyn Iterator<Item = String> + 'a>,
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
            
            self.pending_string = next.unwrap();
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
            TextBlock(txt) => write!(formatter, "{}", txt),
            Escape(seq) => write!(formatter, "{}", seq),
        }
    }
}


#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn should_be_available_as_iterator() {
        let input_str = [RED_BACKGROUND_CODE, "Hello, World!", RESET_CODE].join("");

        run();
        
        // TODO - add test that no matter how the escape code is split it will be parsed correctly
        // assert_eq!(output, expected);
    }
}
