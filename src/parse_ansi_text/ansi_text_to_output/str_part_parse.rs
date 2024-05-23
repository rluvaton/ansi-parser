use std::fmt::Display;
use std::sync::Arc;

use crate::parse_ansi_text::raw_ansi_parse::{AnsiSequence, Output, parse_escape, Text};

#[derive(Debug, PartialEq, Clone)]
pub struct ParseSingleAnsiResult<'a> {
    pub(crate) output: Vec<Output<'a>>,
    pub(crate) current_location_until_pending_string: usize,
    pub(crate) pending_string: Vec<u8>,
}

pub fn parse_single_ansi(value: &[u8], mut current_location_until_pending_string: usize) -> ParseSingleAnsiResult {
    let mut output: Vec<Output> = Vec::new();
    let mut buf = value;
    loop {
        let pending_text_size_before = buf.len();

        match parse_escape(buf, true) {
            Ok((pending, seq)) => {
                buf = pending;
                let text_location = current_location_until_pending_string;

                current_location_until_pending_string += pending_text_size_before - buf.len();

                match seq {
                    AnsiSequence::Text(str) => {
                        output.push(
                            Output::TextBlock(Text {
                                text: str,
                                location_in_text: text_location,
                            })
                        );
                    },
                    _ => {
                        output.push(
                            Output::Escape(seq)
                        );

                    },
                }
            }
            Err(_) => {
                break;
            },
        }
    }

    return ParseSingleAnsiResult {
        output,
        current_location_until_pending_string,
        pending_string: buf.to_vec(),
    }
}

// pub fn parse_single_ansi_from_box_str<'a>(value: &'a String, current_location_until_pending_string: usize) -> ParseSingleAnsiResult<'a> {
//     return parse_single_ansi(value.as_str(), current_location_until_pending_string);
// }

#[cfg(test)]
mod tests {
    use heapless::Vec as HeaplessVec;
    use pretty_assertions::assert_eq;

    use crate::parse_ansi_text::ansi::colors::*;
    use crate::parse_ansi_text::raw_ansi_parse::{AnsiSequence, Output, Text};

    use super::*;

    #[test]
    fn should_get_the_pending_string_to_next_slice_after_finish_parsing_existing_escape_codes_when_stopping_in_middle_of_escape() {
        let input = RED_FOREGROUND_CODE.to_string() + "abc\x1B";

        let result = parse_single_ansi(input.as_str(), 0);

        let output = vec![
            Output::Escape(AnsiSequence::SetGraphicsMode(HeaplessVec::from_slice(&[31]).unwrap())),
            Output::TextBlock(Text {
                text: "abc",
                location_in_text: input.find("abc").unwrap(),
            }),
        ];

        let expected = ParseSingleAnsiResult {
            output,
            pending_string: "\x1B".to_string(),
            current_location_until_pending_string: input.find("abc").unwrap() + 3,
        };

        assert_eq!(result, expected);
    }
    #[test]
    fn should_not_get_pending_state_when_not_ending_with_any_starting_of_possible_escape_code() {
        let input = RED_FOREGROUND_CODE.to_string() + "abc";

        let result = parse_single_ansi(input.as_str(), 0);

        let output = vec![
            Output::Escape(AnsiSequence::SetGraphicsMode(HeaplessVec::from_slice(&[31]).unwrap())),
            Output::TextBlock(Text {
                text: "abc",
                location_in_text: input.find("abc").unwrap(),
            }),
        ];

        let expected = ParseSingleAnsiResult {
            output,
            pending_string: "".to_string(),
            current_location_until_pending_string: input.find("abc").unwrap() + 3,
        };

        assert_eq!(result, expected);
    }

}
