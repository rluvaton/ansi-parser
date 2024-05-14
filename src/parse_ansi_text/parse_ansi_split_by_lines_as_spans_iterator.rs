use std::iter::Iterator;

use ansi_parser::{AnsiParseIterator, AnsiParser, Output};

use crate::parse_ansi_text::ansi_sequence_helpers::{AnsiSequenceType, get_type_from_ansi_sequence};
use crate::parse_ansi_text::parse_options::ParseOptions;
use crate::parse_ansi_text::types::Span;

pub trait ParseAnsiAsSpansByLines {
    fn parse_ansi_as_spans_by_lines(&self, options: ParseOptions) -> ParseAnsiAsSpansByLinesIterator<'_>;
}

impl ParseAnsiAsSpansByLines for str {
    fn parse_ansi_as_spans_by_lines(&self, options: ParseOptions) -> ParseAnsiAsSpansByLinesIterator<'_> {
        ParseAnsiAsSpansByLinesIterator { iter: self.ansi_parse(), line: Some(vec![]), current_span: options.initial_span.clone(), pending_span: Some(options.clone().initial_span.clone()) }
    }
}


#[cfg(any(feature = "std", test))]
impl ParseAnsiAsSpansByLines for String {
    fn parse_ansi_as_spans_by_lines(&self, options: ParseOptions) -> ParseAnsiAsSpansByLinesIterator<'_> {
        ParseAnsiAsSpansByLinesIterator { iter: self.ansi_parse(), line: Some(vec![]), current_span: options.initial_span.clone(), pending_span: Some(options.clone().initial_span.clone()) }
    }
}


#[derive(Debug)]
pub struct ParseAnsiAsSpansByLinesIterator<'a> {
    iter: AnsiParseIterator<'a>,
    current_span: Span,
    line: Option<Vec<Span>>,
    pending_span: Option<Span>,
}

impl<'a> Iterator for ParseAnsiAsSpansByLinesIterator<'a> {
    type Item = Vec<Span>;

    // https://users.rust-lang.org/t/how-to-write-iterator-adapter/8835/2
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // Have some span from previous iteration that was cut off
        if self.pending_span.is_some() {
            let pending_span = self.pending_span.clone().unwrap();

            // If this span still contain text, than extract the 2 spans, one with the text until the new line and the other with the rest of the text
            if pending_span.text.contains("\n") {
                return Some(self.on_span_with_new_line(pending_span));
            }

            self.current_span = self.pending_span.clone().unwrap();

            self.pending_span = None;
        }

        while let Some(output) = self.iter.next() {
            match output {
                Output::TextBlock(text) => {
                    self.current_span.text.push_str(text);

                    // If have new line than get 
                    if self.current_span.text.contains("\n") {
                        return Some(self.on_span_with_new_line(self.current_span.clone()));
                    }
                },
                Output::Escape(seq) => {
                    let sequence_type = get_type_from_ansi_sequence(&seq);

                    match sequence_type {
                        AnsiSequenceType::Unsupported => {
                            continue;
                        },
                        AnsiSequenceType::Reset => {
                            // Ignore spans that are just empty text even if they have style as this won't be shown
                            if self.current_span.text.len() > 0 {
                                let span = self.current_span.clone();
                                self.current_span = Span::empty();

                                self.line.get_or_insert(vec![]).push(span);
                                continue;
                            }

                            self.current_span = Span::empty();
                        },
                        AnsiSequenceType::ForegroundColor(color) => {
                            if self.current_span.text.len() > 0 && self.current_span.color != color {
                                let span = self.current_span.clone();
                                self.current_span = self.current_span.clone()
                                    .with_text("".to_string())
                                    // Apply the color
                                    .with_color(color);

                                self.line.get_or_insert(vec![]).push(span);
                                continue;
                            }

                            self.current_span.color = color;
                        },
                        AnsiSequenceType::BackgroundColor(color) => {
                            if self.current_span.text.len() > 0 && self.current_span.bg_color != color {
                                let span = self.current_span.clone();
                                self.current_span = self.current_span.clone()
                                    .with_text("".to_string())
                                    // Apply the background color
                                    .with_bg_color(color);

                                self.line.get_or_insert(vec![]).push(span);
                                continue;
                            }
                            self.current_span.bg_color = color;
                        },
                        AnsiSequenceType::Brightness(brightness) => {
                            if self.current_span.text.len() > 0 && self.current_span.brightness != brightness {
                                let span = self.current_span.clone();
                                self.current_span = self.current_span.clone()
                                    .with_text("".to_string())
                                    // Apply the background color
                                    .with_brightness(brightness);

                                self.line.get_or_insert(vec![]).push(span);
                                continue;
                            }
                            self.current_span.brightness = brightness;
                        },
                        AnsiSequenceType::TextStyle(style) => {
                            if self.current_span.text.len() > 0 && self.current_span.text_style != style {
                                let span = self.current_span.clone();
                                self.current_span = self.current_span.clone()
                                    .with_text("".to_string())

                                    // Merge the style
                                    .with_text_style(self.current_span.text_style | style);

                                self.line.as_mut().unwrap().push(span);
                                continue;
                            }
                            // Merge the style
                            self.current_span.text_style = self.current_span.text_style | style;
                        },
                    }
                },
            }
        }

        if self.current_span.text.len() > 0 {
            let span = self.current_span.clone();
            self.current_span = Span::empty();

            self.line.as_mut().unwrap().push(span);

            let line = self.line.clone().unwrap();
            
            self.line = None;

            return Some(line);
        }

        // If no text is current span
        if self.line.is_some() {
            let line = self.line.clone().unwrap();
            self.line = None;

            return Some(line);
        }

        return None;
    }
}

impl<'a> ParseAnsiAsSpansByLinesIterator<'a> {
    fn on_span_with_new_line(&mut self, span: Span) -> Vec<Span> {
        let i = span.text.find("\n").unwrap();

        // Create new span with the text until the newline
        let new_span = span.clone().with_text(span.text[..i].to_string());

        let mut line = self.line.clone().unwrap();
        if !new_span.text.is_empty() {
            line.push(new_span);
        }

        self.line = Some(vec![]);

        // Remove the string from it
        self.pending_span = Some(span.clone().with_text(span.text[(i + 1)..].to_string()));

        return line;
    }
}


#[cfg(test)]
mod tests {
    use pretty_assertions::{assert_eq};
    use crate::parse_ansi_text::colors::*;
    use crate::parse_ansi_text::constants::RESET_CODE;
    use crate::parse_ansi_text::parse_options::ParseOptions;
    use super::*;

    #[test]
    fn split_to_lines_should_work() {
        let input = "";

        let chunks = vec![
            RED_FOREGROUND_CODE.to_string() + "abc" + RESET_CODE,
            YELLOW_FOREGROUND_CODE.to_string() + "d\nef\ng" + RESET_CODE,
            CYAN_FOREGROUND_CODE.to_string() + "hij" + RESET_CODE
        ]
            .join("")
            .to_string();

        let lines: Vec<Vec<Span>> = chunks.parse_ansi_as_spans_by_lines(ParseOptions::default()).collect();

        let expected = vec![
            // Line 1:
            vec![
                Span::empty().with_text("abc".to_string()).with_color(Color::Red),
                Span::empty().with_text("d".to_string()).with_color(Color::Yellow)
            ],

            // Line 2:
            vec![
                Span::empty().with_text("ef".to_string()).with_color(Color::Yellow)
            ],

            // Line 3:
            vec![
                Span::empty().with_text("g".to_string()).with_color(Color::Yellow),
                Span::empty().with_text("hij".to_string()).with_color(Color::Cyan)
            ],
        ];

        assert_eq!(lines, expected);
    }
}
