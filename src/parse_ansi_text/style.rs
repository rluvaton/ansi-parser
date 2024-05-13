bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Style: u8 {
        
        const None = 0b00000000;
        
        // \x1B[3m
        const Italic = 0b00000001;
    
        // \x1B[4m
        const Underline = 0b00000010;
    
        // \x1B[7m code
        const Inverse = 0b00000100;
        
        // \x1B[9m
        const Strikethrough = 0b00001000;
    }
}

// This is not part of the style flags because bold cannot be combined with dim
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Brightness {

    None,

    // \x1B[1m
    // Also known as bright (https://github.com/xpl/ansicolor/blob/6f2b837075c8e819a667c65c11f9c934731f323a/ansicolor.js#L142)
    Bold,

    // \x1B[2m
    // also known as faint
    Dim,

}


pub fn get_brightness_type(code: u8) -> Brightness {
    return match code { 
        1 => Brightness::Bold,
        2 => Brightness::Dim,
        _ => Brightness::None,
    }
}


pub fn get_style_type(code: u8) -> Style { 
    return match code {
        3 => Style::Italic,
        4 => Style::Underline,
        7 => Style::Inverse,
        9 => Style::Strikethrough,
        _ => Style::None,
    }
}

#[allow(dead_code)]
pub const ITALIC_CODE: &str = "\x1B[3m";
#[allow(dead_code)]
pub const UNDERLINE_CODE: &str = "\x1B[4m";
#[allow(dead_code)]
pub const INVERSE_CODE: &str = "\x1B[7m";
#[allow(dead_code)]
pub const STRIKETHROUGH_CODE: &str = "\x1B[9m";

#[allow(dead_code)]
pub const BOLD_CODE: &str = "\x1B[1m";
#[allow(dead_code)]
pub const DIM_CODE: &str = "\x1B[2m";