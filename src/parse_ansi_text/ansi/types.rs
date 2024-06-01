use crate::parse_ansi_text::ansi::colors::ColorType::{Background, Foreground};
use crate::parse_ansi_text::ansi::colors::{
    convert_color_type_to_ansi_code, get_rgb_values_from_8_bit, Color,
};
use crate::parse_ansi_text::ansi::style::{
    Brightness, TextStyle, BOLD_CODE, DIM_CODE, INVERSE_CODE, ITALIC_CODE, STRIKETHROUGH_CODE,
    UNDERLINE_CODE,
};
use nom::AsBytes;
use std::ops::Deref;
use std::{fmt, str};

#[derive(PartialEq, Clone)]
pub struct Span {
    pub text: Vec<u8>,
    pub color: Color,
    pub bg_color: Color,

    pub brightness: Brightness,
    pub text_style: TextStyle,
}

impl fmt::Debug for Span {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Span")
            .field("text", &str::from_utf8(self.text.deref()).unwrap())
            .field("color", &self.color)
            .field("bg_color", &self.bg_color)
            .field("brightness", &self.brightness)
            .field("text_style", &self.text_style)
            .finish()
    }
}

impl Span {
    pub fn empty() -> Span {
        Span {
            text: vec![],
            color: Color::None,
            bg_color: Color::None,
            text_style: TextStyle::None,
            brightness: Brightness::None,
        }
    }

    pub fn with_text(mut self, text: Vec<u8>) -> Span {
        self.text = text;
        self
    }

    pub fn with_color(mut self, color: Color) -> Span {
        // Set default color as none
        if matches!(color, Color::Default) {
            self.color = Color::None;
        } else {
            self.color = color;
        }
        self
    }

    pub fn with_bg_color(mut self, bg_color: Color) -> Span {
        // Default color is None
        if matches!(bg_color, Color::Default) {
            self.bg_color = Color::None;
        } else {
            self.bg_color = bg_color;
        }
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
            text: vec![],
            color: span.color,
            bg_color: span.bg_color,
            brightness: span.brightness,
            text_style: span.text_style,
        }
    }

    // TODO - add tests
    pub fn create_css_string(&self) -> String {
        let mut css = "".to_string();

        // Brightness
        if matches!(self.brightness, Brightness::Bold) {
            css = format!("{}font-weight: bold;", css);
        } else if matches!(self.brightness, Brightness::Dim) {
            css = format!("{}font-weight: lighter;", css);
        }

        // Text style
        // TODO - support inverse
        if self.text_style & TextStyle::Italic != TextStyle::empty() {
            css = format!("{}font-style: italic;", css);
        }
        if self.text_style & (TextStyle::Underline | TextStyle::Strikethrough)
            == TextStyle::Underline | TextStyle::Strikethrough
        {
            css = format!("{}text-decoration: line-through underline;", css);
        } else if self.text_style & TextStyle::Underline != TextStyle::empty() {
            css = format!("{}text-decoration: underline;", css);
        } else if self.text_style & TextStyle::Strikethrough != TextStyle::empty() {
            css = format!("{}text-decoration: line-through;", css);
        }

        // Color
        if !matches!(self.color, Color::None) {
            css = format!(
                "{}color: {};",
                css,
                Self::get_color_str_from_color(self.color).unwrap()
            );
        }

        if !matches!(self.bg_color, Color::None) {
            css = format!(
                "{}background-color: {};",
                css,
                Self::get_color_str_from_color(self.bg_color).unwrap()
            );
        }

        return css;
    }

    pub fn serialize_to_ansi_string(self) -> Vec<u8> {
        let mut ansi_string = vec![];

        // Brightness
        if matches!(self.brightness, Brightness::Bold) {
            ansi_string = [ansi_string, BOLD_CODE.as_bytes().to_vec()].concat();
        } else if matches!(self.brightness, Brightness::Dim) {
            ansi_string = [ansi_string, DIM_CODE.as_bytes().to_vec()].concat();
        }

        // Text style
        if self.text_style & TextStyle::Inverse != TextStyle::empty() {
            ansi_string = [ansi_string, INVERSE_CODE.as_bytes().to_vec()].concat();
        }
        if self.text_style & TextStyle::Italic != TextStyle::empty() {
            ansi_string = [ansi_string, ITALIC_CODE.as_bytes().to_vec()].concat();
        }
        if self.text_style & TextStyle::Underline != TextStyle::empty() {
            ansi_string = [ansi_string, UNDERLINE_CODE.as_bytes().to_vec()].concat();
        }
        if self.text_style & TextStyle::Strikethrough != TextStyle::empty() {
            ansi_string = [ansi_string, STRIKETHROUGH_CODE.as_bytes().to_vec()].concat();
        }

        // Color
        ansi_string = [
            ansi_string,
            convert_color_type_to_ansi_code(Foreground(self.color))
                .as_bytes()
                .to_vec(),
        ]
        .concat();
        ansi_string = [
            ansi_string,
            convert_color_type_to_ansi_code(Background(self.bg_color))
                .as_bytes()
                .to_vec(),
        ]
        .concat();

        // Text
        ansi_string = [ansi_string, self.text].concat();

        return ansi_string;
    }

    pub fn replace_default_color_with_none(mut self) -> Span {
        if matches!(self.color, Color::Default) {
            self.color = Color::None;
        }

        if matches!(self.bg_color, Color::Default) {
            self.bg_color = Color::None;
        }

        self
    }

    fn get_color_str_from_color(color: Color) -> Option<String> {
        match color {
            Color::Default => None,
            Color::None => None,
            Color::Black => Some("black".to_string()),
            Color::Red => Some("red".to_string()),
            Color::Green => Some("green".to_string()),
            Color::Yellow => Some("yellow".to_string()),
            Color::Blue => Some("blue".to_string()),
            Color::Magenta => Some("magenta".to_string()),
            Color::Cyan => Some("cyan".to_string()),
            Color::White => Some("white".to_string()),

            // TODO - maybe make the bright color return RGB instead of the name?
            Color::BrightBlack => Some("brightBlack".to_string()),
            Color::BrightRed => Some("brightRed".to_string()),
            Color::BrightGreen => Some("brightGreen".to_string()),
            Color::BrightYellow => Some("brightYellow".to_string()),
            Color::BrightBlue => Some("brightBlue".to_string()),
            Color::BrightMagenta => Some("brightMagenta".to_string()),
            Color::BrightCyan => Some("brightCyan".to_string()),
            Color::BrightWhite => Some("brightWhite".to_string()),

            Color::EightBit(eight_bit) => {
                let (r, g, b) = get_rgb_values_from_8_bit(eight_bit);

                Some(format!("rgb({}, {}, {})", r, g, b))
            }
            Color::Rgb(r, g, b) => Some(format!("rgb({}, {}, {})", r, g, b)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn create_span_with_no_styling_have_no_styles_and_only_text() {
        let span = Span::empty().with_text(b"Hello, world!".to_vec());
        assert_eq!(
            span,
            Span {
                text: b"Hello, world!".to_vec(),

                color: Color::None,
                bg_color: Color::None,
                text_style: TextStyle::None,
                brightness: Brightness::None,
            }
        );
    }

    #[test]
    fn clone_span_without_text_should_only_copy_style() {
        let original_span = Span {
            text: "Hello, world!".to_string().as_bytes().to_vec(),

            color: Color::Red,
            bg_color: Color::None,
            text_style: TextStyle::None,
            brightness: Brightness::None,
        };
        let span = original_span.clone().with_text(vec![]);
        assert_eq!(
            span,
            Span {
                text: vec![],

                color: Color::Red,
                bg_color: Color::None,
                text_style: TextStyle::None,
                brightness: Brightness::None,
            }
        );
    }

    #[test]
    fn clone_span_without_text_should_not_change_original_span() {
        let original_span = Span {
            text: "Hello, world!".to_string().as_bytes().to_vec(),

            color: Color::Red,
            bg_color: Color::None,
            text_style: TextStyle::None,
            brightness: Brightness::None,
        };
        assert_eq!(
            original_span,
            Span {
                text: "Hello, world!".to_string().as_bytes().to_vec(),

                color: Color::Red,
                bg_color: Color::None,
                text_style: TextStyle::None,
                brightness: Brightness::None,
            }
        );
    }

    // TODO - add tests for create_css_string
}
