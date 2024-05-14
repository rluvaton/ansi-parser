
use crate::parse_ansi_text::colors::Color;
use crate::parse_ansi_text::style::{Brightness, TextStyle};


// See more here for ansi codes: https://tforgione.fr/posts/ansi-escape-codes/
#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    pub text: String,
    pub color: Color,
    pub bg_color: Color,

    pub brightness: Brightness,
    pub text_style: TextStyle,
}

impl Span {
    
    pub fn empty() -> Span {
        Span {
            text: "".to_string(),
            color: Color::None,
            bg_color: Color::None,
            text_style: TextStyle::None,
            brightness: Brightness::None,
        }
    }
    
    pub fn with_text(mut self, text: String) -> Span {
        self.text = text;
        self
    }
    
    pub fn with_color(mut self, color: Color) -> Span {
        self.color = color;
        self
    }
    
    pub fn with_bg_color(mut self, bg_color: Color) -> Span {
        self.bg_color = bg_color;
        self
    }
    
    pub fn with_brightness(mut self, brightness: Brightness) -> Span {
        self.brightness = brightness;
        self
    }
    
    pub fn with_text_style(mut self, text_style: TextStyle) -> Span {
        self.text_style = text_style;
        self
    }
    
    pub fn clone_without_text(span: &Span) -> Span {
        Span {
            text: "".to_string(),
            color: span.color,
            bg_color: span.bg_color,
            brightness: span.brightness,
            text_style: span.text_style,
        }
    }
}


#[cfg(test)]
mod tests {
    use pretty_assertions::{assert_eq};
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1 + 1, 2);
    }

    #[test]
    fn create_span_with_no_styling_have_no_styles_and_only_text() {
        let span = Span::empty().with_text("Hello, world!".to_string());
        assert_eq!(span, Span {
            text: "Hello, world!".to_string(),

            color: Color::None,
            bg_color: Color::None,
            text_style: TextStyle::None,
            brightness: Brightness::None,
        });
    }

    #[test]
    fn clone_span_without_text_should_only_copy_style() {
        let original_span = Span {
            text: "Hello, world!".to_string(),

            color: Color::Red,
            bg_color: Color::None,
            text_style: TextStyle::None,
            brightness: Brightness::None,
        };
        let span = original_span.clone().with_text("".to_string());
        assert_eq!(span, Span {
            text: "".to_string(),

            color: Color::Red,
            bg_color: Color::None,
            text_style: TextStyle::None,
            brightness: Brightness::None,
        });
    }

    #[test]
    fn clone_span_without_text_should_not_change_original_span() {
        let original_span = Span {
            text: "Hello, world!".to_string(),

            color: Color::Red,
            bg_color: Color::None,
            text_style: TextStyle::None,
            brightness: Brightness::None,
        };
        let span = original_span.clone().with_text("".to_string());
        assert_eq!(original_span, Span {
            text: "Hello, world!".to_string(),

            color: Color::Red,
            bg_color: Color::None,
            text_style: TextStyle::None,
            brightness: Brightness::None,
        });
    }
}
