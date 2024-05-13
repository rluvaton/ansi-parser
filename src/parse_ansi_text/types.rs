
use crate::parse_ansi_text::colors::Color;
use crate::parse_ansi_text::style::{Brightness, Style};


// See more here for ansi codes: https://tforgione.fr/posts/ansi-escape-codes/
#[derive(Debug, PartialEq)]
pub struct Span {
    pub text: String,
    pub color: Color,
    pub bg_color: Color,

    pub brightness: Brightness,
    pub style: Style,
}


// the reset code is \x1B[0m