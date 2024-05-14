use std::iter::Iterator;

use ansi_parser::{AnsiParseIterator, AnsiParser, Output};

use crate::parse_ansi_text::ansi_sequence_helpers::{AnsiSequenceType, get_type_from_ansi_sequence};
use crate::parse_ansi_text::parse_options::ParseOptions;
use crate::parse_ansi_text::types::Span;

pub trait ParseAnsiAsSpans {
    fn parse_ansi_as_spans(&self, options: ParseOptions) -> ParseAnsiAsSpansIterator<'_>;
}

impl ParseAnsiAsSpans for str {
    fn parse_ansi_as_spans(&self, options: ParseOptions) -> ParseAnsiAsSpansIterator<'_> {
        ParseAnsiAsSpansIterator { iter: self.ansi_parse(), current_span: options.initial_span }
    }
}


#[cfg(any(feature = "std", test))]
impl ParseAnsiAsSpans for String {
    fn parse_ansi_as_spans(&self, options: ParseOptions) -> ParseAnsiAsSpansIterator<'_> {
        ParseAnsiAsSpansIterator { iter: self.ansi_parse(), current_span: options.initial_span }
    }
}


#[derive(Debug)]
pub struct ParseAnsiAsSpansIterator<'a> {
    iter: AnsiParseIterator<'a>,
    current_span: Span,
}

impl<'a> Iterator for ParseAnsiAsSpansIterator<'a> {
    type Item = Span;

    // https://users.rust-lang.org/t/how-to-write-iterator-adapter/8835/2
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(output) = self.iter.next() {
            match output {
                Output::TextBlock(text) => {
                    // println!("Text block: {}", text);
                    self.current_span.text.push_str(text);
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
                                return Some(span);
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

                                return Some(span);
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

                                return Some(span);
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

                                return Some(span);
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

                                return Some(span);
                            }
                            // Merge the style
                            self.current_span.text_style = self.current_span.text_style | style;
                        },
                    }
                },

            }
        }

        // Add last span if it has text
        if self.current_span.text.len() > 0 {
            let span = self.current_span.clone();
            self.current_span = Span::empty();

            return Some(span);
        }

        return None
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::{assert_eq};
    use crate::parse_ansi_text::parse_ansi_as_spans_iterator::ParseAnsiAsSpans;
    use crate::parse_ansi_text::colors::*;
    use crate::parse_ansi_text::constants::*;
    use crate::parse_ansi_text::parse_options::ParseOptions;
    use crate::parse_ansi_text::types::*;

    #[test]
    fn should_be_available_as_iterator() {
        let input_str = [
            RED_BACKGROUND_CODE,
            "Hello, World!",
            RESET_CODE,
        ].join("");

        let output: Vec<Span> = input_str.parse_ansi_as_spans(ParseOptions::default()).collect();

        let expected = vec![Span::empty().with_text("Hello, World!".to_string()).with_bg_color(Color::Red)];
        assert_eq!(output, expected);
    }
}
