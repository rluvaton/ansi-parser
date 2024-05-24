use std::ops::Deref;
use nom::AsBytes;
use sonic_rs::{Serialize, Deserialize, Error};
use std::str;
use serde::{Deserializer, Serializer};
use crate::parse_ansi_text::ansi::colors::{Color, convert_color_type_to_ansi_code, get_rgb_values_from_8_bit};
use crate::parse_ansi_text::ansi::colors::ColorType::{Background, Foreground};
use crate::parse_ansi_text::ansi::style::{BOLD_CODE, Brightness, DIM_CODE, INVERSE_CODE, ITALIC_CODE, STRIKETHROUGH_CODE, TextStyle, UNDERLINE_CODE};
use crate::traits::ToJson;


#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    pub text: Vec<u8>,
    pub color: Color,
    pub bg_color: Color,

    pub brightness: Brightness,
    pub text_style: TextStyle,
}

// TODO - find a better way to create a new struct for json
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SpanJson<'a> {
    // Always serialize
    pub text: &'a str,

    // Colors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bg_color: Option<String>,
    
    // Brightness
    
    #[serde(skip_serializing_if = "std::ops::Not::not", default = "bool::default")]
    pub bold: bool,
    
    #[serde(skip_serializing_if = "std::ops::Not::not", default = "bool::default")]
    pub dim: bool,
    
    // Text Style
    
    #[serde(skip_serializing_if = "std::ops::Not::not", default = "bool::default")]
    pub italic: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not", default = "bool::default")]
    pub underline: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not", default = "bool::default")]
    pub inverse: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not", default = "bool::default")]
    pub strikethrough: bool,
}

impl Serialize for Span {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let span_json = SpanJson::create_from_span(self);

        return span_json.serialize(serializer);
    }
}



impl<'de> Deserialize<'de> for Span {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>
    {
        let span_json = SpanJson::deserialize(deserializer)?;

        return Ok(Span::from_span_json(span_json));
    }
}


impl Span {
    
    pub fn empty() -> Span {
        Span {
            text: "".as_bytes().to_vec(),
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
            text: "".as_bytes().to_vec(),
            color: span.color,
            bg_color: span.bg_color,
            brightness: span.brightness,
            text_style: span.text_style,
        }
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
        ansi_string = [ansi_string, convert_color_type_to_ansi_code(Foreground(self.color)).as_bytes().to_vec()].concat();
        ansi_string = [ansi_string, convert_color_type_to_ansi_code(Background(self.bg_color)).as_bytes().to_vec()].concat();
        
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
            },
            Color::Rgb(r, g, b) => Some(format!("rgb({}, {}, {})", r, g, b)),
        }
    }

    pub fn from_json(text: &str) -> Result<Span, Error> {
        let json = sonic_rs::from_str::<SpanJson>(text)?;

        return Ok(Span::from_span_json(json));
    }
    pub fn from_json_array(text: &str) -> Result<Vec<Span>, Error> {
        let json = sonic_rs::from_str::<Vec<SpanJson>>(text)?;

        return Ok(json.iter().map(|item| Span::from_span_json(item.clone())).collect());
    }

    pub fn from_span_json(json: SpanJson) -> Span {
        return Span::empty()
            .with_text(json.text.as_bytes().to_vec())
            .with_text_style(
                if json.italic {
                    TextStyle::Italic
                } else {
                    TextStyle::None
                }
                    |
                    if json.underline {
                        TextStyle::Underline
                    } else {
                        TextStyle::None
                    }
                    |
                    if json.inverse {
                        TextStyle::Inverse
                    } else {
                        TextStyle::None
                    }
                    |
                    if json.strikethrough {
                        TextStyle::Strikethrough
                    } else {
                        TextStyle::None
                    }
            )
            .with_bg_color(
                // TODO - move this to a function
                json.bg_color.map_or(Color::None, |color| {
                    match color.as_str() {
                        "black" => Color::Black,
                        "red" => Color::Red,
                        "green" => Color::Green,
                        "yellow" => Color::Yellow,
                        "blue" => Color::Blue,
                        "magenta" => Color::Magenta,
                        "cyan" => Color::Cyan,
                        "white" => Color::White,
                        "brightBlack" => Color::BrightBlack,
                        "brightRed" => Color::BrightRed,
                        "brightGreen" => Color::BrightGreen,
                        "brightYellow" => Color::BrightYellow,
                        "brightBlue" => Color::BrightBlue,
                        "brightMagenta" => Color::BrightMagenta,
                        "brightCyan" => Color::BrightCyan,
                        "brightWhite" => Color::BrightWhite,
                        _ => Color::None,
                    }
                })
            )
            .with_color(
                // TODO - move this to a function
                json.color.map_or(Color::None, |color| {
                    match color.as_str() {
                        "black" => Color::Black,
                        "red" => Color::Red,
                        "green" => Color::Green,
                        "yellow" => Color::Yellow,
                        "blue" => Color::Blue,
                        "magenta" => Color::Magenta,
                        "cyan" => Color::Cyan,
                        "white" => Color::White,
                        "brightBlack" => Color::BrightBlack,
                        "brightRed" => Color::BrightRed,
                        "brightGreen" => Color::BrightGreen,
                        "brightYellow" => Color::BrightYellow,
                        "brightBlue" => Color::BrightBlue,
                        "brightMagenta" => Color::BrightMagenta,
                        "brightCyan" => Color::BrightCyan,
                        "brightWhite" => Color::BrightWhite,
                        _ => Color::None,
                    }
                })
            )
            .with_brightness(
                if json.bold {
                    Brightness::Bold
                } else if json.dim {
                    Brightness::Dim
                } else {
                    Brightness::None
                }
            );
    }
}

impl SpanJson<'_> {
    pub fn create_from_span(span: &Span) -> SpanJson<'_> {
        SpanJson {
            text: str::from_utf8(span.text.deref()).unwrap(),
            
            // Colors
            color: Self::get_color_str_from_color(span.color),
            bg_color: Self::get_color_str_from_color(span.bg_color),
            
            // Brightness
            bold: span.brightness == Brightness::Bold,
            dim: span.brightness == Brightness::Dim,
            
            // Text style
            italic: span.text_style & TextStyle::Italic != TextStyle::empty(),
            underline: span.text_style & TextStyle::Underline != TextStyle::empty(),
            inverse: span.text_style & TextStyle::Inverse != TextStyle::empty(),
            strikethrough: span.text_style & TextStyle::Strikethrough != TextStyle::empty(),
        }
    }

    pub fn get_color_str_from_color(color: Color) -> Option<String> {
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
            },
            Color::Rgb(r, g, b) => Some(format!("rgb({}, {}, {})", r, g, b)),
        }
    }
}

impl ToJson for Span {
    fn to_json(&self) -> String {
        let span_json = SpanJson {
            text: str::from_utf8(self.text.deref()).unwrap(),

            // Colors
            color: Self::get_color_str_from_color(self.color),
            bg_color: Self::get_color_str_from_color(self.bg_color),

            // Brightness
            bold: self.brightness == Brightness::Bold,
            dim: self.brightness == Brightness::Dim,

            // Text style
            italic: self.text_style & TextStyle::Italic != TextStyle::empty(),
            underline: self.text_style & TextStyle::Underline != TextStyle::empty(),
            inverse: self.text_style & TextStyle::Inverse != TextStyle::empty(),
            strikethrough: self.text_style & TextStyle::Strikethrough != TextStyle::empty(),
        };

        return sonic_rs::to_string(&span_json).unwrap();
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
        let span = Span::empty().with_text("Hello, world!".to_string().as_bytes().to_vec());
        assert_eq!(span, Span {
            text: "Hello, world!".to_string().as_bytes().to_vec(),

            color: Color::None,
            bg_color: Color::None,
            text_style: TextStyle::None,
            brightness: Brightness::None,
        });
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
        let span = original_span.clone().with_text("".to_string().as_bytes().to_vec());
        assert_eq!(span, Span {
            text: "".to_string().as_bytes().to_vec(),

            color: Color::Red,
            bg_color: Color::None,
            text_style: TextStyle::None,
            brightness: Brightness::None,
        });
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
        assert_eq!(original_span, Span {
            text: "Hello, world!".to_string().as_bytes().to_vec(),

            color: Color::Red,
            bg_color: Color::None,
            text_style: TextStyle::None,
            brightness: Brightness::None,
        });
    }

    // TODO - add test for from json and to json with combinations and missing values
}
