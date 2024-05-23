use std::iter::Iterator;

use async_stream::stream;
use futures_core::Stream;

use crate::parse_ansi_text::ansi::ansi_sequence_helpers::{
    AnsiSequenceType, get_type_from_ansi_sequence,
};
use crate::parse_ansi_text::ansi::colors::Color;
use crate::parse_ansi_text::ansi::types::Span;
use crate::parse_ansi_text::parse_options::ParseOptions;
use crate::parse_ansi_text::raw_ansi_parse::Output;

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub(crate) spans: Vec<Span>,
    pub(crate) location_in_file: usize,
}

pub async fn convert_ansi_output_to_lines_of_spans<'a, S: Stream<Item = Output>>(
    input: S,
    options: ParseOptions,
) -> impl Stream<Item = Line> {
    stream! {
        let mut line: Option<Vec<Span>> = None;
        let mut last_line_index: usize = 0;

        let mut current_span: Span = options
                .initial_span
                .clone()
                .replace_default_color_with_none();

        for await output in input {
            match output {
                Output::TextBlock(text) => {
                    current_span.text.push_str(text.text.as_str());
                    let mut from_index = text.location_in_text;

                    // If have new line than get
                    while current_span.text.contains("\n") {
                        let i = current_span.text.find("\n").unwrap();

                        // Create new span with the text until the newline
                        let new_span = current_span.clone().with_text(current_span.text[..i].to_string());
                    
                        let mut line_to_yield = line.clone().unwrap_or(vec![]);
                        if !new_span.text.is_empty() {
                            line_to_yield.push(new_span);
                        }
                        
                        // Use for te first time the last line index
                        let start_of_line = last_line_index;
                        
                        line = None;
                        current_span = current_span.clone().with_text(current_span.text[(i + 1)..].to_string());
                        last_line_index = from_index + i + 1;
                        from_index = last_line_index;
                        
                        yield Line {
                            spans: line_to_yield,
                            location_in_file: start_of_line,
                        };
                    }
                    
                    continue;
                }
                Output::Escape(seq) => {
                    let sequence_type = get_type_from_ansi_sequence(&seq);

                    match sequence_type {
                        AnsiSequenceType::Unsupported => {
                            continue;
                        }
                        AnsiSequenceType::Reset => {
                            // Ignore spans that are just empty text even if they have style as this won't be shown
                            if current_span.text.len() > 0 {
                                let span = current_span.clone();
                                current_span = Span::empty();

                                line.get_or_insert(vec![]).push(span);
                                continue;
                            }

                            current_span = Span::empty();
                        }
                        AnsiSequenceType::ForegroundColor(mut color) => {
                            // Default color is same as none
                            if matches!(color, Color::Default) {
                                color = Color::None;
                            }

                            if current_span.text.len() > 0 && current_span.color != color
                            {
                                let span = current_span.clone();
                                current_span = current_span
                                    .clone()
                                    .with_text("".to_string())
                                    // Apply the color
                                    .with_color(color);

                                line.get_or_insert(vec![]).push(span);
                                continue;
                            }

                            current_span.color = color;
                        }
                        AnsiSequenceType::BackgroundColor(mut color) => {
                            // Default color is same as none
                            if matches!(color, Color::Default) {
                                color = Color::None;
                            }

                            if current_span.text.len() > 0
                                && current_span.bg_color != color
                            {
                                let span = current_span.clone();
                                current_span = current_span
                                    .clone()
                                    .with_text("".to_string())
                                    // Apply the background color
                                    .with_bg_color(color);

                                line.get_or_insert(vec![]).push(span);
                                continue;
                            }
                            current_span.bg_color = color;
                        }
                        AnsiSequenceType::Brightness(brightness) => {
                            if current_span.text.len() > 0
                                && current_span.brightness != brightness
                            {
                                let span = current_span.clone();
                                current_span = current_span
                                    .clone()
                                    .with_text("".to_string())
                                    // Apply the background color
                                    .with_brightness(brightness);

                                line.get_or_insert(vec![]).push(span);
                                continue;
                            }
                            current_span.brightness = brightness;
                        }
                        AnsiSequenceType::TextStyle(style) => {
                            if current_span.text.len() > 0
                                && current_span.text_style != style
                            {
                                let span = current_span.clone();
                                current_span = current_span
                                    .clone()
                                    .with_text("".to_string())
                                    // Merge the style
                                    .with_text_style(current_span.text_style | style);

                                line.as_mut().unwrap().push(span);
                                continue;
                            }
                            // Merge the style
                            current_span.text_style = current_span.text_style | style;
                        }
                    }
                }
            }
        }
        
        if current_span != Span::empty() {
            let span = current_span.clone();
            
            line.get_or_insert(vec![]).push(span);
        }
        if line.is_some() {
            yield Line {
                spans: line.unwrap(),
                location_in_file: last_line_index,
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use futures::stream::StreamExt;
    use pretty_assertions::assert_eq;

    use crate::compose_async_steams;
    use crate::parse_ansi_text::ansi::colors::*;
    use crate::parse_ansi_text::ansi::constants::RESET_CODE;
    use crate::parse_ansi_text::ansi_text_to_output::stream_helpers::merge_text_output;
    use crate::parse_ansi_text::ansi_text_to_output::stream_parse::parse_ansi;
    use crate::parse_ansi_text::parse_options::ParseOptions;
    use crate::test_utils::{async_chars_stream, async_stream_from_vector};

    use super::*;

    #[tokio::test]
    async fn steams_split_to_lines_should_work_for_split_by_chars() {
        let input = vec![
            RED_FOREGROUND_CODE.to_string() + "abc" + RESET_CODE,
            YELLOW_FOREGROUND_CODE.to_string() + "d\nef\ng" + RESET_CODE,
            CYAN_FOREGROUND_CODE.to_string() + "hij" + RESET_CODE,
        ]
            .join("");

        let lines: Vec<Line> = compose_async_steams!(
            || async_chars_stream(input.clone()),
            parse_ansi,
            merge_text_output,
            |output| convert_ansi_output_to_lines_of_spans(output, ParseOptions::default())
        ).await.collect::<Vec<Line>>().await;

        let expected = vec![
            // Line 1:
            Line {
                spans: vec![
                    Span::empty()
                        .with_text("abc".to_string())
                        .with_color(Color::Red),
                    Span::empty()
                        .with_text("d".to_string())
                        .with_color(Color::Yellow),
                ],
                location_in_file: 0,
            },
            // Line 2:
            Line {
                spans: vec![Span::empty()
                    .with_text("ef".to_string())
                    .with_color(Color::Yellow)],
                location_in_file: input.find("ef").unwrap(),
            },
            // Line 3:
            Line {
                spans: vec![
                    Span::empty()
                        .with_text("g".to_string())
                        .with_color(Color::Yellow),
                    Span::empty()
                        .with_text("hij".to_string())
                        .with_color(Color::Cyan),
                ],
                location_in_file: input.find("g").unwrap(),
            },
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

        let lines: Vec<Line> = compose_async_steams!(
            || async_stream_from_vector(vec![chunks.clone()]),
            parse_ansi,
            merge_text_output,
            |output| convert_ansi_output_to_lines_of_spans(output, ParseOptions::default())
        ).await.collect::<Vec<Line>>().await;

        let expected = vec![
            // Line 1:
            Line {
                spans: vec![
                    Span::empty()
                        .with_text("abc".to_string())
                        .with_color(Color::Red),
                    Span::empty()
                        .with_text("d".to_string())
                        .with_color(Color::Yellow),
                ],
                location_in_file: 0,
            },
            // Line 2:
            Line {
                spans: vec![Span::empty()
                    .with_text("ef".to_string())
                    .with_color(Color::Yellow)],
                location_in_file: chunks.find("ef").unwrap(),
            },
            // Line 3:
            Line {
                spans: vec![
                    Span::empty()
                        .with_text("g".to_string())
                        .with_color(Color::Yellow),
                    Span::empty()
                        .with_text("hij".to_string())
                        .with_color(Color::Cyan),
                ],
                location_in_file: chunks.find("g").unwrap(),
            },
        ];

        assert_eq!(lines, expected);
    }
}
