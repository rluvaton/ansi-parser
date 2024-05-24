use std::fmt::Display;

use async_stream::stream;
use tokio_stream::{Stream, StreamExt};

use crate::parse_ansi_text::ansi_text_to_output::str_part_parse::{parse_single_ansi};
use crate::parse_ansi_text::raw_ansi_parse::{Output, Text};

pub async fn parse_ansi<'a, S: Stream<Item = Vec<u8>> + 'a>(input: S) -> impl Stream<Item = Output<'a>> + 'a {
    stream! {
        let mut current_location_until_pending_string: usize = 0;
        let mut pending_string: Vec<u8> = vec![];
        
        for await value in input {
            pending_string = [pending_string, value].concat();
            let new_pending = pending_string.clone();
            
                // TODO - avoid leak
            // let result = parse_single_ansi(new_pending, current_location_until_pending_string);
            let result = parse_single_ansi("".as_bytes(), current_location_until_pending_string);
            current_location_until_pending_string = result.current_location_until_pending_string;
            pending_string = result.pending_string;
            
            for item in result.output {
                yield item;
            }
        }
        if !pending_string.is_empty() {
            yield Output::TextBlock(Text {
                // TODO - avoid leak
                text: pending_string.leak(),
                location_in_text: current_location_until_pending_string,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use futures_util::stream;
    use pretty_assertions::assert_eq;

    use crate::{compose_async_steams, compose_streams};
    use crate::parse_ansi_text::ansi::colors::*;
    use crate::parse_ansi_text::ansi::constants::RESET_CODE;
    use crate::parse_ansi_text::ansi_text_to_output::helpers::merge_text_output;
    use crate::parse_ansi_text::raw_ansi_parse::{Output, Text};
    use crate::streams_helpers::vector_to_async_stream;
    use crate::test_utils::async_chars_stream;

    use super::*;

    fn create_text_block(text: &str, location_in_text: usize) -> Output {
        Output::TextBlock(Text {
            text: text.as_bytes(),
            location_in_text,
        })
    }

    #[tokio::test]
    async fn streams_split_to_lines_should_work_for_split_by_chars() {
        let input = "";

        let input = vec![
            RED_FOREGROUND_CODE.to_string() + "abc" + RESET_CODE,
            YELLOW_FOREGROUND_CODE.to_string() + "d\nef\ng" + RESET_CODE,
            CYAN_FOREGROUND_CODE.to_string() + "hij" + RESET_CODE,
        ]
            .join("");

        let lines: Vec<Output> = compose_async_steams!(
            || async_chars_stream(input.clone()),
            parse_ansi
            // TODO
            // merge_text_output,
        )
            .await
            .filter(|item| match item {
                Output::TextBlock(_) => true,
                _ => false,
            })
            .collect()
            .await;

        let expected = vec![
            create_text_block("abc", input.find("abc").unwrap()),
            create_text_block("d\nef\ng", input.find("d\nef\ng").unwrap()),
            create_text_block("hij", input.find("hij").unwrap()),
        ];

        assert_eq!(lines, expected);
    }

    #[tokio::test]
    async fn streams_split_to_lines_should_work_for_single_chunk() {
        let input = vec![
            RED_FOREGROUND_CODE.to_string() + "abc" + RESET_CODE,
            YELLOW_FOREGROUND_CODE.to_string() + "d\nef\ng" + RESET_CODE,
            CYAN_FOREGROUND_CODE.to_string() + "hij" + RESET_CODE,
        ]
            .join("")
            .to_string();

        let lines: Vec<Output> =
            compose_streams!(|| stream::iter(vec![input.clone().as_bytes().to_vec()]), parse_ansi)
                .await
                .filter(|item| match item {
                    Output::TextBlock(_) => true,
                    _ => false,
                })
                .collect()
                .await;

        let lines: Vec<Output> = compose_async_steams!(
            || vector_to_async_stream(vec![input.clone().as_bytes().to_vec()]),
            parse_ansi
            // TODO
            // merge_text_output,
        )
            .await
            .filter(|item| match item {
                Output::TextBlock(_) => true,
                _ => false,
            })
            .collect()
            .await;

        let expected = vec![
            create_text_block("abc", input.find("abc").unwrap()),
            create_text_block("d\nef\ng", input.find("d\nef\ng").unwrap()),
            create_text_block("hij", input.find("hij").unwrap()),
        ];

        assert_eq!(lines, expected);
    }

    #[tokio::test]
    async fn streams_split_to_lines_should_work_for_split_by_chars_when_text_have_escape_code_used_without_data(
    ) {
        let input = "";

        let input = vec![
            // Adding \x1B which is the escape code to make sure treated as text
            RED_FOREGROUND_CODE.to_string() + "a\x1Bbc" + RESET_CODE,
            // Added \x1B before escape code to make sure treated as text
            YELLOW_FOREGROUND_CODE.to_string() + "d\nef\ng\x1B" + RESET_CODE,
            CYAN_FOREGROUND_CODE.to_string() + "hij" + RESET_CODE,
        ]
            .join("");


        let lines: Vec<Output> = compose_async_steams!(
            || async_chars_stream(input.clone()),
            parse_ansi
            // TODO
            // merge_text_output,
        )
            .await
            .filter(|item| match item {
                Output::TextBlock(_) => true,
                _ => false,
            })
            .collect()
            .await;

        let expected = vec![
            create_text_block("a\x1Bbc", input.find("a\x1Bbc").unwrap()),
            create_text_block("d\nef\ng\x1B", input.find("d\nef\ng\x1B").unwrap()),
            create_text_block("hij", input.find("hij").unwrap()),
        ];

        assert_eq!(lines, expected);
    }
}
