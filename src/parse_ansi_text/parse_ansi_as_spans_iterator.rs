use std::iter::Iterator;

use crate::parse_ansi_text::ansi_sequence_helpers::{AnsiSequenceType, get_type_from_ansi_sequence};
use crate::parse_ansi_text::colors::Color;
use crate::parse_ansi_text::custom_ansi_parse_iterator::{AnsiParseIterator, Output};
use crate::parse_ansi_text::parse_ansi_split_by_lines_as_spans_iterator::ParseAnsiAsSpansByLinesIterator;
use crate::parse_ansi_text::parse_options::ParseOptions;
use crate::parse_ansi_text::types::Span;

pub struct ParseAnsiAsSpansIterator<'a> {
    pub(crate) iter: AnsiParseIterator<'a>,
    pub(crate) current_span: Span,
}

impl<'a> Iterator for ParseAnsiAsSpansIterator<'a> {
    type Item = Span;

    // https://users.rust-lang.org/t/how-to-write-iterator-adapter/8835/2
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(output) = self.iter.next() {
            match output {
                Output::IgnoreMe => {
                    
                },
                Output::TextBlock(text) => {
                    // println!("Text block: {}", text);
                    self.current_span.text.push_str(text.as_str());
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
                        AnsiSequenceType::ForegroundColor(mut color) => {
                            // Default color is same as none
                            if matches!(color, Color::Default) {
                                color = Color::None;
                            }

                            // TODO - add here that if current color is default or None and new color is default or none don't treat as different
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
                        AnsiSequenceType::BackgroundColor(mut color) => {
                            // Default color is same as none
                            if matches!(color, Color::Default) {
                                color = Color::None;
                            }

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


impl ParseAnsiAsSpansIterator<'_> {
    pub fn create<'a>(parse_iterator: AnsiParseIterator<'a>, options: ParseOptions) -> ParseAnsiAsSpansIterator<'a> {
        ParseAnsiAsSpansIterator { iter: parse_iterator, current_span: options.initial_span.clone().replace_default_color_with_none() }
    }

    pub fn create_from_string_iterator<'a>(str_iterator: Box<dyn Iterator<Item=String>>, options: ParseOptions) -> ParseAnsiAsSpansIterator<'a> {
        ParseAnsiAsSpansIterator { iter: AnsiParseIterator::create(str_iterator), current_span: options.initial_span.clone().replace_default_color_with_none() }
    }

    pub fn create_from_str<'a>(str: String, options: ParseOptions) -> ParseAnsiAsSpansIterator<'a> {
        ParseAnsiAsSpansIterator { iter: AnsiParseIterator::create_from_str(str), current_span: options.initial_span.clone().replace_default_color_with_none() }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::{assert_eq};
    use crate::parse_ansi_text::colors::*;
    use crate::parse_ansi_text::constants::*;
    use crate::parse_ansi_text::custom_ansi_parse_iterator::*;
    use crate::parse_ansi_text::playground_iterator::CharsIterator;
    use crate::parse_ansi_text::types::*;
    use super::*;

    #[test]
    fn should_parse_chars_iterator_correctly() {
        let input_str = vec![
            RED_BACKGROUND_CODE.to_string(),
            "Hello, World!".to_string(),
            RESET_CODE.to_string(),
        ].join("");
        
        let chars = CharsIterator {
            index: 0,
            str: input_str,
        };

        let parse_ansi_from_text_iterator = AnsiParseIterator::create(Box::new(chars));
        
        let parse_ansi_as_spans_iterator = ParseAnsiAsSpansIterator {
            iter: parse_ansi_from_text_iterator,
            current_span: Span::empty(),
        };
        let output: Vec<Span> = parse_ansi_as_spans_iterator.collect::<Vec<Span>>();
        
        let expected = vec![Span::empty().with_text("Hello, World!".to_string()).with_bg_color(Color::Red)];
        assert_eq!(output, expected);
    }
    
    #[test]
    fn should_be_available_as_iterator() {
        let input_str = [
            RED_BACKGROUND_CODE,
            "Hello, World!",
            RESET_CODE,
        ].join("");


        let parse_ansi_from_text_iterator = AnsiParseIterator::create_from_str(input_str);

        let parse_ansi_as_spans_iterator = ParseAnsiAsSpansIterator {
            iter: parse_ansi_from_text_iterator,
            current_span: Span::empty(),
        };
        let output: Vec<Span> = parse_ansi_as_spans_iterator.collect::<Vec<Span>>();

        let expected = vec![Span::empty().with_text("Hello, World!".to_string()).with_bg_color(Color::Red)];
        assert_eq!(output, expected);
    }
}
