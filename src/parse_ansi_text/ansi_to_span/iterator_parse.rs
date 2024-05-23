use std::fmt::Display;
use std::path::PathBuf;

use tokio_stream::{Stream, StreamExt};

use crate::files::iterators::create_file_iterator;
use crate::parse_ansi_text::raw_ansi_parse::{Output, parse_escape, Text};

pub struct AnsiParseIterator<'a> {
    pending_string: &'a str,
    pub(crate) iterator: Box<dyn Iterator<Item = String>>,
    current_location_until_pending_string: usize,
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

            // TODO - should use different way to ensure the lifetime of the string
            let next: &'a mut str = next.unwrap().leak();
            self.pending_string = next;
        }

        let pos = self.pending_string.find('\u{1b}');
        if let Some(loc) = pos {
            if loc == 0 {
                // If the beginning of the string is the key for escape code
                let pending_text_size_before = self.pending_string.len();
                let res = parse_escape(&self.pending_string[loc..], false);

                if let Ok(ret) = res {
                    // If there is escape code after the escape key
                    self.pending_string = ret.0;
                    self.current_location_until_pending_string +=
                        pending_text_size_before - self.pending_string.len();
                    return Some(Output::Escape(ret.1));
                } else {
                    // If no escape code after the escape key
                    let old_loc = loc;
                    let pos = self.pending_string[(loc + 1)..].find('\u{1b}');
                    if let Some(loc) = pos {
                        // If there is escape key also exists in the middle of the string then split to before the escape code and from it to the end of the string
                        //Added to because it's based one character ahead
                        let loc = loc + 1;
                        let text_location = self.current_location_until_pending_string;

                        let temp = &self.pending_string[..loc];
                        self.current_location_until_pending_string += old_loc + loc;
                        self.pending_string = &self.pending_string[loc..];

                        return Some(Output::TextBlock(Text {
                            text: temp.to_string(),
                            location_in_text: text_location,
                        }));
                    }

                    // If no other escape key exists in the string, do nothing as the next string might will
                }
            } else {
                // If in the middle than split to before the escape code and after and keep the after for the next iteration
                let temp = &self.pending_string[..loc];
                let text_location = self.current_location_until_pending_string;

                self.current_location_until_pending_string += loc;
                self.pending_string = &self.pending_string[loc..];

                return Some(Output::TextBlock(Text {
                    text: temp.to_string(),
                    location_in_text: text_location,
                }));
            }
        }

        let next = self.iterator.next();

        if next.is_none() {
            let text_location = self.current_location_until_pending_string;
            let temp = self.pending_string;
            self.current_location_until_pending_string += temp.len();
            self.pending_string = "";
            return Some(Output::TextBlock(Text {
                text: temp.to_string(),
                location_in_text: text_location,
            }));
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
    pub fn create<'a>(str_iterator: Box<dyn Iterator<Item = String>>) -> AnsiParseIterator<'a> {
        AnsiParseIterator {
            current_location_until_pending_string: 0,
            iterator: str_iterator,
            pending_string: "",
        }
    }

    pub fn create_from_str<'a>(str: String) -> AnsiParseIterator<'a> {
        AnsiParseIterator {
            current_location_until_pending_string: 0,
            iterator: Box::new(vec![str].into_iter()),
            pending_string: "",
        }
    }

    pub fn create_from_file_path<'a>(input_file_path: PathBuf) -> AnsiParseIterator<'a> {
        AnsiParseIterator {
            current_location_until_pending_string: 0,
            iterator: create_file_iterator(input_file_path),
            pending_string: "",
        }
    }
}


#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::parse_ansi_text::ansi::colors::*;
    use crate::parse_ansi_text::ansi::constants::RESET_CODE;
    use crate::parse_ansi_text::iterators::playground_iterator::CharsIterator;

    use super::*;

    #[test]
    fn iterator_split_to_lines_should_work_for_split_by_chars() {
        let input = vec![
            RED_FOREGROUND_CODE.to_string() + "abc" + RESET_CODE,
            YELLOW_FOREGROUND_CODE.to_string() + "d\nef\ng" + RESET_CODE,
            CYAN_FOREGROUND_CODE.to_string() + "hij" + RESET_CODE,
        ]
            .join("");

        let chars = CharsIterator {
            index: 0,
            str: input.clone(),
        };

        let lines: Vec<Output> = AnsiParseIterator::create(Box::new(chars))
            .filter(|item| match item {
                Output::TextBlock(_) => true,
                _ => false,
            })
            .collect();

        let expected = vec![
            Output::TextBlock(Text {
                text: "abc".to_string(),
                location_in_text: input.find("abc").unwrap(),
            }),
            Output::TextBlock(Text {
                text: "d\nef\ng".to_string(),
                location_in_text: input.find("d\nef\ng").unwrap(),
            }),
            Output::TextBlock(Text {
                text: "hij".to_string(),
                location_in_text: input.find("hij").unwrap(),
            }),
        ];

        assert_eq!(lines, expected);
    }

    #[test]
    fn iterator_split_to_lines_should_work_for_single_chunk() {
        let chunks = vec![
            RED_FOREGROUND_CODE.to_string() + "abc" + RESET_CODE,
            YELLOW_FOREGROUND_CODE.to_string() + "d\nef\ng" + RESET_CODE,
            CYAN_FOREGROUND_CODE.to_string() + "hij" + RESET_CODE,
        ]
            .join("")
            .to_string();

        let lines: Vec<Output> = AnsiParseIterator::create_from_str(chunks.clone())
            .filter(|item| match item {
                Output::TextBlock(_) => true,
                _ => false,
            })
            .collect();

        let expected = vec![
            Output::TextBlock(Text {
                text: "abc".to_string(),
                location_in_text: chunks.find("abc").unwrap(),
            }),
            Output::TextBlock(Text {
                text: "d\nef\ng".to_string(),
                location_in_text: chunks.find("d\nef\ng").unwrap(),
            }),
            Output::TextBlock(Text {
                text: "hij".to_string(),
                location_in_text: chunks.find("hij").unwrap(),
            }),
        ];

        assert_eq!(lines, expected);
    }

}
