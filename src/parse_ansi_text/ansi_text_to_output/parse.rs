use std::fmt::{Debug, Display};
use std::fs::File;

use genawaiter::{rc::gen, yield_};
use get_chunk::iterator::FileIter;
use nom::ExtendInto;

use crate::parse_ansi_text::ansi_text_to_output::str_part_parse::parse_single_ansi;
use crate::parse_ansi_text::raw_ansi_parse::{AnsiSequence, Output, Text};

#[allow(unused_variables, unused_mut)]
pub fn parse_ansi<'a, I: Iterator<Item = Vec<u8>> + 'a>(
    mut input: I,
) -> impl Iterator<Item = Output<'a>> + 'a {
    #[allow(clippy::needless_return)]
    return gen!({
        let mut current_location_until_pending_string: usize = 0;
        let mut pending_string: Vec<u8> = vec![];

        for value in input {
            pending_string = [pending_string, value].concat();

            // TODO - avoid leak
            let result =
                parse_single_ansi(pending_string.leak(), current_location_until_pending_string);

            current_location_until_pending_string = result.current_location_until_pending_string;
            pending_string = result.pending_string;

            for item in result.output {
                yield_!(item);
            }
        }
        if !pending_string.is_empty() {
            yield_!(Output::TextBlock(Text {
                text: pending_string.leak(),
                location_in_text: current_location_until_pending_string,
            }));
        }
    })
    .into_iter();
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::iterators::compose::ComposeByIterator;
    use crate::parse_ansi_text::ansi::colors::*;
    use crate::parse_ansi_text::ansi::constants::RESET_CODE;
    use crate::parse_ansi_text::ansi_text_to_output::helpers::merge_text_output;
    use crate::parse_ansi_text::raw_ansi_parse::{AnsiSequence, Output, Text};
    use crate::test_utils::chars_iterator;

    use super::*;

    fn create_text_block(text: &str, location_in_text: usize) -> Output {
        Output::TextBlock(Text {
            text: text.as_bytes(),
            location_in_text,
        })
    }

    #[test]
    fn streams_split_to_lines_should_work_for_split_by_chars() {
        let input = vec![
            RED_FOREGROUND_CODE.to_string() + "abc" + RESET_CODE,
            YELLOW_FOREGROUND_CODE.to_string() + "d\nef\ng" + RESET_CODE,
            CYAN_FOREGROUND_CODE.to_string() + "hij" + RESET_CODE,
        ]
        .join("");

        let lines: Vec<Output> = chars_iterator(input.clone())
            .compose(parse_ansi)
            .compose(merge_text_output)
            .filter(|item| match item {
                Output::TextBlock(_) => true,
                _ => false,
            })
            .collect();

        let expected = vec![
            create_text_block("abc", input.find("abc").unwrap()),
            create_text_block("d\nef\ng", input.find("d\nef\ng").unwrap()),
            create_text_block("hij", input.find("hij").unwrap()),
        ];

        assert_eq!(lines, expected);
    }

    #[test]
    fn streams_split_to_lines_should_work_for_single_chunk() {
        let input = vec![
            RED_FOREGROUND_CODE.to_string() + "abc" + RESET_CODE,
            YELLOW_FOREGROUND_CODE.to_string() + "d\nef\ng" + RESET_CODE,
            CYAN_FOREGROUND_CODE.to_string() + "hij" + RESET_CODE,
        ]
        .join("")
        .to_string();

        let lines: Vec<Output> = vec![input.clone().into_bytes()]
            .into_iter()
            .compose(parse_ansi)
            .compose(merge_text_output)
            .filter(|item| match item {
                Output::TextBlock(_) => true,
                _ => false,
            })
            .collect();

        let expected = vec![
            create_text_block("abc", input.find("abc").unwrap()),
            create_text_block("d\nef\ng", input.find("d\nef\ng").unwrap()),
            create_text_block("hij", input.find("hij").unwrap()),
        ];

        assert_eq!(lines, expected);
    }

    #[test]
    fn streams_split_to_lines_should_work_for_split_by_chars_when_text_have_escape_code_used_without_data(
    ) {
        let input = vec![
            // Adding \x1B which is the escape code to make sure treated as text
            RED_FOREGROUND_CODE.to_string() + "a\x1Bbc" + RESET_CODE,
            // Added \x1B before escape code to make sure treated as text
            YELLOW_FOREGROUND_CODE.to_string() + "d\nef\ng\x1B" + RESET_CODE,
            CYAN_FOREGROUND_CODE.to_string() + "hij" + RESET_CODE,
        ]
        .join("");

        let lines: Vec<Output> = chars_iterator(input.clone())
            .compose(parse_ansi)
            .compose(merge_text_output)
            .filter(|item| match item {
                Output::TextBlock(_) => true,
                _ => false,
            })
            .collect();

        let expected = vec![
            create_text_block("a\x1Bbc", input.find("a\x1Bbc").unwrap()),
            create_text_block("d\nef\ng\x1B", input.find("d\nef\ng\x1B").unwrap()),
            create_text_block("hij", input.find("hij").unwrap()),
        ];

        assert_eq!(lines, expected);
    }
}
