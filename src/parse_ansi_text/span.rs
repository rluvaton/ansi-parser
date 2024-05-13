use crate::parse_ansi_text::colors::Color;
use crate::parse_ansi_text::style::{Brightness, Style};
use crate::parse_ansi_text::types::*;

pub fn create_unstyled_span(text: String) -> Span {
    Span {
        text: text,
        color: Color::None,
        bg_color: Color::None,
        style: Style::empty(),
        brightness: Brightness::None,
    }
}

// TODO - make sure to take this into account:
// brightness codes does not overlap
// https://github.com/xpl/ansicolor/blob/6f2b837075c8e819a667c65c11f9c934731f323a/ansicolor.js#L155C1-L159C4


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1 + 1, 2);
    }

    #[test]
    fn create_span_with_no_styling_have_no_styles_and_only_text() {
        let span = create_unstyled_span("Hello, world!".to_string());
        assert_eq!(span, Span {
            text: "Hello, world!".to_string(),

            color: Color::None,
            bg_color: Color::None,
            style: Style::empty(),
            brightness: Brightness::None,
        });
    }
}
