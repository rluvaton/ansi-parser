use std::fmt::Display;

use async_stream::stream;
use tokio_stream::{Stream, StreamExt};

use crate::parse_ansi_text::ansi_text_to_output::str_part_parse::parse_single_ansi;
use crate::parse_ansi_text::raw_ansi_parse::{Output, Text};

pub async fn parse_ansi<'a, S: Stream<Item = String>>(input: S) -> impl Stream<Item = Output> {
    stream! {
        let mut current_location_until_pending_string: usize = 0;
        let mut pending_string: String = "".to_string();
        
        for await value in input {
            pending_string.push_str(value.as_str());
            
            let result = parse_single_ansi(pending_string.as_str(), current_location_until_pending_string);
            current_location_until_pending_string = result.current_location_until_pending_string;
            pending_string = result.pending_string;
            
            for item in result.output {
                yield item;
            }
        }
        if !pending_string.is_empty() {
            yield Output::TextBlock(Text {
                text: pending_string,
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
    use crate::parse_ansi_text::ansi_text_to_output::stream_helpers::merge_text_output;
    use crate::parse_ansi_text::raw_ansi_parse::{Output, Text};
    use crate::streams_helpers::vector_to_async_stream;
    use crate::test_utils::{async_chars_stream, chars_stream};

    use super::*;

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
            parse_ansi,
            merge_text_output
        )
            .await
            .filter(|item| match item {
                Output::TextBlock(_) => true,
                _ => false,
            })
            .collect()
            .await;

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

    #[tokio::test]
    async fn streams_split_to_lines_should_work_for_single_chunk() {
        let chunks = vec![
            RED_FOREGROUND_CODE.to_string() + "abc" + RESET_CODE,
            YELLOW_FOREGROUND_CODE.to_string() + "d\nef\ng" + RESET_CODE,
            CYAN_FOREGROUND_CODE.to_string() + "hij" + RESET_CODE,
        ]
            .join("")
            .to_string();

        let lines: Vec<Output> =
            compose_streams!(|| stream::iter(vec![chunks.clone()]), parse_ansi)
                .await
                .filter(|item| match item {
                    Output::TextBlock(_) => true,
                    _ => false,
                })
                .collect()
                .await;

        let lines: Vec<Output> = compose_async_steams!(
            || vector_to_async_stream(vec![chunks.clone()]),
            parse_ansi,
            merge_text_output
        )
            .await
            .filter(|item| match item {
                Output::TextBlock(_) => true,
                _ => false,
            })
            .collect()
            .await;

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
            parse_ansi,
            merge_text_output
        )
            .await
            .filter(|item| match item {
                Output::TextBlock(_) => true,
                _ => false,
            })
            .collect()
            .await;

        let expected = vec![
            Output::TextBlock(Text {
                text: "a\x1Bbc".to_string(),
                location_in_text: input.find("a\x1Bbc").unwrap(),
            }),
            Output::TextBlock(Text {
                text: "d\nef\ng\x1B".to_string(),
                location_in_text: input.find("d\nef\ng\x1B").unwrap(),
            }),
            Output::TextBlock(Text {
                text: "hij".to_string(),
                location_in_text: input.find("hij").unwrap(),
            }),
        ];

        assert_eq!(lines, expected);
    }
}
