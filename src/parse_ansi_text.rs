pub mod span;
pub mod types;
pub mod colors;
pub mod constants;
pub mod style;
mod ansi_sequence_helpers;

use ansi_parser::{Output, AnsiParser};

use types::Span;
use crate::parse_ansi_text::ansi_sequence_helpers::*;
use crate::parse_ansi_text::span::*;


pub fn parse_ansi_text(str: &str) -> Vec<Span> {


    //Parse the first two blocks in the list
    //By parsing it this way, it allows you to iterate over the
    //elements returned.
    //
    //The parser only every holds a reference to the data,
    //so there is no allocation.
    let parsed: Vec<Output> = str
        .ansi_parse()
        // .take(2)
        .collect();

    let spans: Vec<Span> = create_span_from_output(parsed);
    
    return spans;
}



// TODO - replace argument to be iterator and return type to be iterator for best performance
fn create_span_from_output(parsed: Vec<Output>) -> Vec<Span> {
    if parsed.len() == 0 {
        return vec![];
    }

    let mut all_spans: Vec<Span> = vec![];

    let mut span: Span = create_unstyled_span("".to_string());

    for output in parsed {
        println!("Output: {:?}", output);

        match output {
            Output::TextBlock(text) => {
                println!("Text block: {}", text);
                span.text.push_str(text);
            },
            Output::Escape(seq) => {
                let sequence_type = get_type_from_ansi_sequence(&seq);
                
                match sequence_type {
                    AnsiSequenceType::Unsupported => {
                        continue;
                    },
                    AnsiSequenceType::Reset => {
                        // Ignore spans that are just empty text even if they have style as this won't be shown
                        if span.text.len() > 0 {
                            all_spans.push(span);
                        }
                        
                        span = create_unstyled_span("".to_string());
                    },
                    AnsiSequenceType::ForegroundColor(color) => {
                        span.color = color;
                    },
                    AnsiSequenceType::BackgroundColor(color) => {
                        span.bg_color = color;
                    },
                    AnsiSequenceType::Brightness(brightness) => {
                        span.brightness = brightness;
                    },
                    AnsiSequenceType::Style(style) => {
                        // Merge the style
                        span.style = span.style | style;
                    },
                }
            },
            
        }
    }

    // Add last span if it has text
    if span.text.len() > 0 {
        all_spans.push(span);
    }

    return all_spans
}


#[cfg(test)]
mod tests {
    use crate::parse_ansi_text::colors::*;
    use crate::parse_ansi_text::constants::RESET_CODE;
    use crate::parse_ansi_text::span::create_unstyled_span;
    use crate::parse_ansi_text::style::*;
    use super::*;

    #[test]
    fn empty_text_should_return_empty_array() {
        let input = "";
        let expected = vec![];
        assert_eq!(parse_ansi_text(input), expected);
    }
    
    // -------------
    // No ANSI codes
    // -------------

    #[test]
    fn single_text_without_ansi_codes_should_return_array_with_one_unstyled_span() {
        let input = "Hello, world!";
        let expected = vec![create_unstyled_span("Hello, world!".to_string())];
        assert_eq!(parse_ansi_text(input), expected);
    }

    #[test]
    fn multiline_text_without_ansi_codes_should_return_array_with_one_unstyled_span() {
        let input = "Hello, world!\nhow are you";
        let expected = vec![create_unstyled_span("Hello, world!\nhow are you".to_string())];
        assert_eq!(parse_ansi_text(input), expected);
    }

    // -------------
    // Single color
    // -------------

    // TODO - add test-case crate for each color
    #[test]
    fn red_foreground_text_single_line_with_escapse_in_the_end() {
        let input = [
            RED_FOREGROUND_CODE,
            "Hello, world!",
            RESET_CODE,
        ].join("");
        let expected = vec![Span {
            text: "Hello, world!".to_string(),
            color: Color::Red,
            bg_color: Color::None,
            brightness: Brightness::None,
            style: Style::empty(),
        }];
        assert_eq!(parse_ansi_text(&input), expected);
    }
    
    // TODO - add test that color after color should be the last color and if have text in the middle than should create a new span


    // TODO - add tests for every combination
    #[test]
    fn style_combination() {
        let input = [
            &*RGB_BACKGROUND_CODE(188, 29, 68),
            &*RGB_FOREGROUND_CODE(255, 19, 94),
            ITALIC_CODE,
            UNDERLINE_CODE,
            BOLD_CODE,
            "Hello, world!",
            RESET_CODE,
        ].join("");
        let expected = vec![Span {
            text: "Hello, world!".to_string(),
            color: Color::Rgb(255, 19, 94),
            bg_color: Color::Rgb(188, 29, 68),
            brightness: Brightness::Bold,
            style: Style::Italic | Style::Underline,
        }];
        assert_eq!(parse_ansi_text(&input), expected);
    }
    
}
