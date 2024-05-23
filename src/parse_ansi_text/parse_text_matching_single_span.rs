use std::iter::Iterator;

use ansi_parser::{AnsiParser, Output};

use crate::parse_ansi_text::ansi::ansi_sequence_helpers::{get_type_from_ansi_sequence, AnsiSequenceType, old_ansi_sequence_to_new};
use crate::parse_ansi_text::ansi::colors::Color;
use crate::parse_ansi_text::ansi::types::Span;

// The text here MUST correspond to a single span.
// Span with empty text is valid here
pub fn parse_text_matching_single_span(text: &str) -> Span {
    let mut span = Span::empty();
    let mut raw_ansi_parse = text.ansi_parse();

    while let Some(output) = raw_ansi_parse.next() {
        match output {
            Output::TextBlock(text) => {
                span.text = [span.text, text.as_bytes().to_vec()].concat();
            }
            Output::Escape(seq) => {
                let sequence_type = get_type_from_ansi_sequence(&old_ansi_sequence_to_new(seq));

                match sequence_type {
                    AnsiSequenceType::Unsupported => {
                        continue;
                    }
                    AnsiSequenceType::Reset => {
                        // Can't be here as this is guaranteed to not have reset
                        // But if we getting it anyway, then reset the styles
                        span = Span::empty();
                    }
                    AnsiSequenceType::ForegroundColor(mut color) => {
                        // Default color is same as none
                        if matches!(color, Color::Default) {
                            color = Color::None;
                        }

                        span.color = color;
                    }
                    AnsiSequenceType::BackgroundColor(mut color) => {
                        // Default color is same as none
                        if matches!(color, Color::Default) {
                            color = Color::None;
                        }
                        span.bg_color = color;
                    }
                    AnsiSequenceType::Brightness(brightness) => {
                        span.brightness = brightness;
                    }
                    AnsiSequenceType::TextStyle(style) => {
                        // Merge the style
                        span.text_style = span.text_style | style;
                    }
                }
            }
        }
    }

    return span;
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::parse_ansi_text::ansi::colors::*;
    use crate::parse_ansi_text::parse_text_matching_single_span::*;
    use crate::parse_ansi_text::ansi::style::*;
    use crate::parse_ansi_text::ansi::types::*;

    #[test]
    fn should_return_empty_span_for_empty_input() {
        let input_str = "";

        let output: Span = parse_text_matching_single_span(input_str);

        let expected = Span::empty();
        assert_eq!(output, expected);
    }

    #[test]
    fn should_return_unstyled_span_for_text_without_styling() {
        let input_str = "Hello world";

        let output: Span = parse_text_matching_single_span(input_str);

        let expected = Span::empty().with_text("Hello world".to_string().as_bytes().to_vec());
        assert_eq!(output, expected);
    }

    #[test]
    fn should_return_styled_span_for_text_with_style() {
        let input_str = RED_BACKGROUND_CODE.to_string() + "Hello world";

        let output: Span = parse_text_matching_single_span(input_str.as_str());

        let expected = Span::empty().with_bg_color(Color::Red).with_text("Hello world".to_string().as_bytes().to_vec());
        assert_eq!(output, expected);
    }

    #[test]
    fn should_return_the_styles_even_when_the_actual_text_is_empty() {
        let input_str = [RED_BACKGROUND_CODE].join("");

        let output: Span = parse_text_matching_single_span(input_str.as_str());

        let expected = Span::empty().with_bg_color(Color::Red);
        assert_eq!(output, expected);
    }

    #[test]
    fn should_return_styled_span_for_input_that_have_possible_styles() {
        let input_str = "".to_string() +
            // Colors
            BLUE_BACKGROUND_CODE +
            CYAN_FOREGROUND_CODE +
            
            // Brightness
            BOLD_CODE +
            
            // Text style
            ITALIC_CODE +
            STRIKETHROUGH_CODE +
            INVERSE_CODE +
            UNDERLINE_CODE +
            
            // Text
            "Hello world";

        let output: Span = parse_text_matching_single_span(input_str.as_str());

        let expected = Span::empty()
            .with_bg_color(Color::Blue)
            .with_color(Color::Cyan)
            .with_brightness(Brightness::Bold)
            .with_text_style(TextStyle::Italic | TextStyle::Strikethrough | TextStyle::Inverse | TextStyle::Underline)
            .with_text("Hello world".to_string().as_bytes().to_vec());
        assert_eq!(output, expected);
    }
}
