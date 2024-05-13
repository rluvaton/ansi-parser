use heapless::{Vec, consts::U5};

pub enum ColorType {
    None,
    Foreground(Color),
    Background(Color),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub enum Color {
    None,

    // Foreground color: \x1B[30m
    // Background color: \x1B[40m
    Black,
    
    // Foreground color: \x1B[31m
    // Background color: \x1B[41m
    Red,
    
    // Foreground color: \x1B[32m
    // Background color: \x1B[42m
    Green,
    
    // Foreground color: \x1B[33m
    // Background color: \x1B[43m
    Yellow,
    
    // Foreground color: \x1B[34m
    // Background color: \x1B[44m
    Blue,
    
    // Foreground color: \x1B[35m
    // Background color: \x1B[45m
    Magenta,
    
    // Foreground color: \x1B[36m
    // Background color: \x1B[46m
    Cyan,
    
    // Foreground color: \x1B[37m
    // Background color: \x1B[47m
    White,
    
    // TODO - have a value to store the rgb value
    // Foreground color: \x1B[38;2;R;G;Bm
    // Background color: \x1B[48;2;R;G;Bm
    Rgb(u8, u8, u8),
    
    // There is also any palette color, but I'm not sure what it is
}


pub fn get_color_type(vec: &Vec<u8, U5>) -> ColorType {
    if vec.len() == 0 {
        return ColorType::None;
    }

    let code = vec[0];

    if code < 30 || code > 48 || (code > 38 && code < 40) {
        return ColorType::None;
    }

    let code_color_digit = code % 10;
    let type_color_digit = (code / 10) % 10;

    let color = match code_color_digit {
        0 => Color::Black,
        1 => Color::Red,
        2 => Color::Green,
        3 => Color::Yellow,
        4 => Color::Blue,
        5 => Color::Magenta,
        6 => Color::Cyan,
        7 => Color::White,
        8 => {
            if vec.len() < 5 {
                println!("Invalid RGB color code: {:?}", vec);
                return ColorType::None;
            }

            // vec[1] is always 2

            Color::Rgb(vec[2], vec[3], vec[4])
        },
        _ => panic!("Invalid color code: {:?}", vec),
    };

    if type_color_digit == 3 {
        return ColorType::Foreground(color);
    }

    if type_color_digit == 4 {
        return ColorType::Background(color);
    }

    panic!("Invalid color code: {:?}", vec);

}

#[allow(dead_code)]
pub const BLACK_FOREGROUND_CODE: &str = "\x1B[30m";
#[allow(dead_code)]
pub const BLACK_BACKGROUND_CODE: &str = "\x1B[40m";

#[allow(dead_code)]
pub const RED_FOREGROUND_CODE: &str = "\x1B[31m";
#[allow(dead_code)]
pub const RED_BACKGROUND_CODE: &str = "\x1B[41m";

#[allow(dead_code)]
pub const GREEN_FOREGROUND_CODE: &str = "\x1B[32m";
#[allow(dead_code)]
pub const GREEN_BACKGROUND_CODE: &str = "\x1B[42m";

#[allow(dead_code)]
pub const YELLOW_FOREGROUND_CODE: &str = "\x1B[33m";
#[allow(dead_code)]
pub const YELLOW_BACKGROUND_CODE: &str = "\x1B[43m";

#[allow(dead_code)]
pub const BLUE_FOREGROUND_CODE: &str = "\x1B[34m";
#[allow(dead_code)]
pub const BLUE_BACKGROUND_CODE: &str = "\x1B[44m";

#[allow(dead_code)]
pub const MAGENTA_FOREGROUND_CODE: &str = "\x1B[35m";
#[allow(dead_code)]
pub const MAGENTA_BACKGROUND_CODE: &str = "\x1B[45m";

#[allow(dead_code)]
pub const CYAN_FOREGROUND_CODE: &str = "\x1B[36m";
#[allow(dead_code)]
pub const CYAN_BACKGROUND_CODE: &str = "\x1B[46m";

#[allow(dead_code)]
pub const WHITE_FOREGROUND_CODE: &str = "\x1B[37m";
#[allow(dead_code)]
pub const WHITE_BACKGROUND_CODE: &str = "\x1B[47m";

#[allow(dead_code)]
pub fn RGB_FOREGROUND_CODE(r: u8, g: u8, b: u8) -> String {
    // \x1B[38;2;R;G;Bm	
    return format!("\x1B[38;2;{};{};{}m", r, g, b)
}

#[allow(dead_code)]
pub fn RGB_BACKGROUND_CODE(r: u8, g: u8, b: u8) -> String {
    return format!("\x1B[48;2;{};{};{}m", r, g, b)
}

