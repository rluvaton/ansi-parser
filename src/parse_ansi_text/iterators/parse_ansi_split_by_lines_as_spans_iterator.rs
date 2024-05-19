use async_stream::stream;
use futures_core::Stream;
use std::iter::Iterator;
use std::path::PathBuf;

use crate::parse_ansi_text::ansi::ansi_sequence_helpers::{
    get_type_from_ansi_sequence, AnsiSequenceType,
};
use crate::parse_ansi_text::ansi::colors::Color;
use crate::parse_ansi_text::ansi::types::Span;
use crate::parse_ansi_text::iterators::custom_ansi_parse_iterator::{AnsiParseIterator, Output};
use crate::parse_ansi_text::parse_options::ParseOptions;

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub(crate) spans: Vec<Span>,
    pub(crate) location_in_file: usize,
}

pub struct ParseAnsiAsSpansByLinesIterator<'a> {
    iter: AnsiParseIterator<'a>,
    current_span: Span,
    line: Option<Vec<Span>>,
    pending_span: Option<Span>,
    last_line_index: usize,
}

impl<'a> Iterator for ParseAnsiAsSpansByLinesIterator<'a> {
    type Item = Line;

    // https://users.rust-lang.org/t/how-to-write-iterator-adapter/8835/2
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // Have some span from previous iteration that was cut off
        if self.pending_span.is_some() {
            let pending_span = self.pending_span.clone().unwrap();

            // If this span still contain text, then extract the 2 spans, one with the text until the new line and the other with the rest of the text
            if pending_span.text.contains("\n") {
                return Some(self.on_span_with_new_line(pending_span, self.last_line_index));
            }

            self.current_span = self.pending_span.clone().unwrap();

            self.pending_span = None;
        }

        while let Some(output) = self.iter.next() {
            match output {
                Output::IgnoreMe => {}
                Output::TextBlock(text) => {
                    self.current_span.text.push_str(text.text);

                    // If have new line than get
                    if self.current_span.text.contains("\n") {
                        return Some(self.on_span_with_new_line(
                            self.current_span.clone(),
                            text.location_in_text,
                        ));
                    }
                }
                Output::Escape(seq) => {
                    let sequence_type = get_type_from_ansi_sequence(&seq);

                    match sequence_type {
                        AnsiSequenceType::Unsupported => {
                            continue;
                        }
                        AnsiSequenceType::Reset => {
                            // Ignore spans that are just empty text even if they have style as this won't be shown
                            if self.current_span.text.len() > 0 {
                                let span = self.current_span.clone();
                                self.current_span = Span::empty();

                                self.line.get_or_insert(vec![]).push(span);
                                continue;
                            }

                            self.current_span = Span::empty();
                        }
                        AnsiSequenceType::ForegroundColor(mut color) => {
                            // Default color is same as none
                            if matches!(color, Color::Default) {
                                color = Color::None;
                            }

                            if self.current_span.text.len() > 0 && self.current_span.color != color
                            {
                                let span = self.current_span.clone();
                                self.current_span = self
                                    .current_span
                                    .clone()
                                    .with_text("".to_string())
                                    // Apply the color
                                    .with_color(color);

                                self.line.get_or_insert(vec![]).push(span);
                                continue;
                            }

                            self.current_span.color = color;
                        }
                        AnsiSequenceType::BackgroundColor(mut color) => {
                            // Default color is same as none
                            if matches!(color, Color::Default) {
                                color = Color::None;
                            }

                            if self.current_span.text.len() > 0
                                && self.current_span.bg_color != color
                            {
                                let span = self.current_span.clone();
                                self.current_span = self
                                    .current_span
                                    .clone()
                                    .with_text("".to_string())
                                    // Apply the background color
                                    .with_bg_color(color);

                                self.line.get_or_insert(vec![]).push(span);
                                continue;
                            }
                            self.current_span.bg_color = color;
                        }
                        AnsiSequenceType::Brightness(brightness) => {
                            if self.current_span.text.len() > 0
                                && self.current_span.brightness != brightness
                            {
                                let span = self.current_span.clone();
                                self.current_span = self
                                    .current_span
                                    .clone()
                                    .with_text("".to_string())
                                    // Apply the background color
                                    .with_brightness(brightness);

                                self.line.get_or_insert(vec![]).push(span);
                                continue;
                            }
                            self.current_span.brightness = brightness;
                        }
                        AnsiSequenceType::TextStyle(style) => {
                            if self.current_span.text.len() > 0
                                && self.current_span.text_style != style
                            {
                                let span = self.current_span.clone();
                                self.current_span = self
                                    .current_span
                                    .clone()
                                    .with_text("".to_string())
                                    // Merge the style
                                    .with_text_style(self.current_span.text_style | style);

                                self.line.as_mut().unwrap().push(span);
                                continue;
                            }
                            // Merge the style
                            self.current_span.text_style = self.current_span.text_style | style;
                        }
                    }
                }
            }
        }

        if self.current_span.text.len() > 0 {
            let span = self.current_span.clone();
            self.current_span = Span::empty();

            self.line.as_mut().unwrap().push(span);

            let line = self.line.clone().unwrap();

            self.line = None;

            return Some(Line {
                spans: line,
                location_in_file: self.last_line_index,
            });
        }

        // If no text is current span
        if self.line.is_some() {
            let line = self.line.clone().unwrap();
            self.line = None;

            return Some(Line {
                spans: line,
                location_in_file: self.last_line_index,
            });
        }

        return None;
    }
}

impl<'a> ParseAnsiAsSpansByLinesIterator<'a> {
    fn on_span_with_new_line(&mut self, span: Span, location_in_text: usize) -> Line {
        let i = span.text.find("\n").unwrap();

        // Create new span with the text until the newline
        let new_span = span.clone().with_text(span.text[..i].to_string());

        let mut line = self.line.clone().unwrap();
        if !new_span.text.is_empty() {
            line.push(new_span);
        }

        let start_of_line = self.last_line_index;

        let location_in_file = location_in_text + i + 1;

        self.line = Some(vec![]);

        // Remove the string from it
        self.pending_span = Some(span.clone().with_text(span.text[(i + 1)..].to_string()));

        self.last_line_index = location_in_file;

        Line {
            spans: line,
            location_in_file: start_of_line,
        }
    }
}

impl<'a> ParseAnsiAsSpansByLinesIterator<'a> {
    pub fn create(
        parse_iterator: AnsiParseIterator,
        options: ParseOptions,
    ) -> ParseAnsiAsSpansByLinesIterator {
        ParseAnsiAsSpansByLinesIterator {
            last_line_index: 0,
            iter: parse_iterator,
            line: Some(vec![]),
            current_span: options
                .initial_span
                .clone()
                .replace_default_color_with_none(),
            pending_span: Some(
                options
                    .clone()
                    .initial_span
                    .clone()
                    .replace_default_color_with_none(),
            ),
        }
    }

    pub fn create_from_string_iterator(
        str_iterator: Box<dyn Iterator<Item = String>>,
        options: ParseOptions,
    ) -> ParseAnsiAsSpansByLinesIterator<'a> {
        ParseAnsiAsSpansByLinesIterator {
            last_line_index: 0,
            iter: AnsiParseIterator::create(str_iterator),
            line: Some(vec![]),
            current_span: options
                .initial_span
                .clone()
                .replace_default_color_with_none(),
            pending_span: Some(
                options
                    .clone()
                    .initial_span
                    .clone()
                    .replace_default_color_with_none(),
            ),
        }
    }

    pub fn create_from_str(
        str: String,
        options: ParseOptions,
    ) -> ParseAnsiAsSpansByLinesIterator<'a> {
        ParseAnsiAsSpansByLinesIterator {
            last_line_index: 0,
            iter: AnsiParseIterator::create_from_str(str),
            line: Some(vec![]),
            current_span: options
                .initial_span
                .clone()
                .replace_default_color_with_none(),
            pending_span: Some(
                options
                    .clone()
                    .initial_span
                    .clone()
                    .replace_default_color_with_none(),
            ),
        }
    }

    pub fn create_from_file_path(
        input_file_path: PathBuf,
        options: ParseOptions,
    ) -> ParseAnsiAsSpansByLinesIterator<'a> {
        ParseAnsiAsSpansByLinesIterator {
            last_line_index: 0,
            iter: AnsiParseIterator::create_from_file_path(input_file_path),
            line: Some(vec![]),
            current_span: options
                .initial_span
                .clone()
                .replace_default_color_with_none(),
            pending_span: Some(
                options
                    .clone()
                    .initial_span
                    .clone()
                    .replace_default_color_with_none(),
            ),
        }
    }
}

pub async fn convert_ansi_output_to_lines_of_spans<'a, S: Stream<Item = Output<'a>>>(
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
                Output::IgnoreMe => {}
                Output::TextBlock(text) => {
                    current_span.text.push_str(text.text);
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

            // if current_span.text.len() > 0 {
            //     let span = current_span.clone();
            //     current_span = Span::empty();
            // 
            //     line.as_mut().unwrap().push(span);
            // 
            //     let new_line = line.clone().unwrap();
            // 
            //     line = Some(vec![]);
            // 
            //     yield Line {
            //         spans: new_line,
            //         location_in_file: last_line_index,
            //     };
            // }
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
    use crate::parse_ansi_text::iterators::custom_ansi_parse_iterator::parse_ansi;
    use crate::parse_ansi_text::iterators::playground_iterator::CharsIterator;
    use crate::parse_ansi_text::parse_options::ParseOptions;
    use crate::test_utils::{async_chars_stream, async_stream_from_vector};

    use super::*;

    #[test]
    fn iterators_split_to_lines_should_work_for_split_by_chars() {
        let input = "";

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

        let lines: Vec<Line> = ParseAnsiAsSpansByLinesIterator::create_from_string_iterator(
            Box::new(chars),
            ParseOptions::default(),
        )
        .collect();

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

    #[test]
    fn iterators_split_to_lines_should_work_for_single_chunk() {
        let chunks = vec![
            RED_FOREGROUND_CODE.to_string() + "abc" + RESET_CODE,
            YELLOW_FOREGROUND_CODE.to_string() + "d\nef\ng" + RESET_CODE,
            CYAN_FOREGROUND_CODE.to_string() + "hij" + RESET_CODE,
        ]
        .join("")
        .to_string();

        let lines: Vec<Line> = ParseAnsiAsSpansByLinesIterator::create_from_str(
            chunks.clone(),
            ParseOptions::default(),
        )
        .collect();

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
    
    #[tokio::test]
    async fn steams_split_to_lines_should_work_for_split_by_chars() {
        let input = "";

        let input = vec![
            RED_FOREGROUND_CODE.to_string() + "abc" + RESET_CODE,
            YELLOW_FOREGROUND_CODE.to_string() + "d\nef\ng" + RESET_CODE,
            CYAN_FOREGROUND_CODE.to_string() + "hij" + RESET_CODE,
        ]
            .join("");

        let lines: Vec<Line> = compose_async_steams!(
            || async_chars_stream(input.clone()),
            parse_ansi,
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
