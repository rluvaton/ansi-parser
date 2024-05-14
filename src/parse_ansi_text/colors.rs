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
    
    // Foreground color: \x1B[38;5;Vm
    // Background color: \x1B[48;5;Vm
    // Also known as Palate color
    EightBit(u8),
    
    // TODO - have a value to store the rgb value
    // Foreground color: \x1B[38;2;R;G;Bm
    // Background color: \x1B[48;2;R;G;Bm
    Rgb(u8, u8, u8),
    
    Default,
}


pub fn get_color_type(vec: &Vec<u8, U5>) -> ColorType {
    if vec.len() == 0 {
        return ColorType::None;
    }

    let code = vec[0];

    if code < 30 || code > 49 {
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
            if(vec.len() < 2) {
                panic!("Invalid Color code {:?}", vec);
            }
            
            let color_type = vec[1];
            
            match color_type {
                // RGB
                2 => {
                    let mut color: Color;
                    if vec.len() < 5 {
                        println!("Invalid RGB color code: {:?}", vec);
                        color = Color::None;
                    } else {
                        color = Color::Rgb(vec[2], vec[3], vec[4]);
                    }
                    
                    color
                },
                // 8-bit color
                5 => {
                    let mut color: Color;
                    
                    if vec.len() < 3 {
                        println!("Invalid 8bit color code: {:?}", vec);
                        color = Color::None;
                    } else {
                        let (r, g, b) = get_rgb_values_from_8_bit(vec[2]);
                        color = Color::Rgb(r, g, b);
                    }
                    
                    color
                },
                _ => panic!("Unknown color code {}, it should be either 2 for 8bit color or 5 for RGB color. vec is: {:?}", color_type, vec)
            }
        },
        9 => Color::Default,
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

pub fn get_rgb_values_from_8_bit(eight_bit_color: u8) -> (u8, u8, u8) {
    // Reference: https://sweworld.net/cheatsheets/terminal_escape_code/#256-color-escape-codes
    return match eight_bit_color {
        // Black (SYSTEM)
        0 => (0, 0, 0),
        // Maroon (SYSTEM)
        1 => (128, 0, 0),
        // Green (SYSTEM)
        2 => (0, 128, 0),
        // Olive (SYSTEM)
        3 => (128, 128, 0),
        // Navy (SYSTEM)
        4 => (0, 0, 128),
        // Purple (SYSTEM)
        5 => (128, 0, 128),
        // Teal (SYSTEM)
        6 => (0, 128, 128),
        // Silver (SYSTEM)
        7 => (192, 192, 192),
        // Grey (SYSTEM)
        8 => (128, 128, 128),
        // Red (SYSTEM)
        9 => (255, 0, 0),
        // Lime (SYSTEM)
        10 => (0, 255, 0),
        // Yellow (SYSTEM)
        11 => (255, 255, 0),
        // Blue (SYSTEM)
        12 => (0, 0, 255),
        // Fuchsia (SYSTEM)
        13 => (255, 0, 255),
        // Aqua (SYSTEM)
        14 => (0, 255, 255),
        // White (SYSTEM)
        15 => (255, 255, 255),
        // Grey0
        16 => (0, 0, 0),
        // NavyBlue
        17 => (0, 0, 95),
        // DarkBlue
        18 => (0, 0, 135),
        // Blue3
        19 => (0, 0, 175),
        // Blue3
        20 => (0, 0, 215),
        // Blue1
        21 => (0, 0, 255),
        // DarkGreen
        22 => (0, 95, 0),
        // DeepSkyBlue4
        23 => (0, 95, 95),
        // DeepSkyBlue4
        24 => (0, 95, 135),
        // DeepSkyBlue4
        25 => (0, 95, 175),
        // DodgerBlue3
        26 => (0, 95, 215),
        // DodgerBlue2
        27 => (0, 95, 255),
        // Green4
        28 => (0, 135, 0),
        // SpringGreen4
        29 => (0, 135, 95),
        // Turquoise4
        30 => (0, 135, 135),
        // DeepSkyBlue3
        31 => (0, 135, 175),
        // DeepSkyBlue3
        32 => (0, 135, 215),
        // DodgerBlue1
        33 => (0, 135, 255),
        // Green3
        34 => (0, 175, 0),
        // SpringGreen3
        35 => (0, 175, 95),
        // DarkCyan
        36 => (0, 175, 135),
        // LightSeaGreen
        37 => (0, 175, 175),
        // DeepSkyBlue2
        38 => (0, 175, 215),
        // DeepSkyBlue1
        39 => (0, 175, 255),
        // Green3
        40 => (0, 215, 0),
        // SpringGreen3
        41 => (0, 215, 95),
        // SpringGreen2
        42 => (0, 215, 135),
        // Cyan3
        43 => (0, 215, 175),
        // DarkTurquoise
        44 => (0, 215, 215),
        // Turquoise2
        45 => (0, 215, 255),
        // Green1
        46 => (0, 255, 0),
        // SpringGreen2
        47 => (0, 255, 95),
        // SpringGreen1
        48 => (0, 255, 135),
        // MediumSpringGreen
        49 => (0, 255, 175),
        // Cyan2
        50 => (0, 255, 215),
        // Cyan1
        51 => (0, 255, 255),
        // DarkRed
        52 => (95, 0, 0),
        // DeepPink4
        53 => (95, 0, 95),
        // Purple4
        54 => (95, 0, 135),
        // Purple4
        55 => (95, 0, 175),
        // Purple3
        56 => (95, 0, 215),
        // BlueViolet
        57 => (95, 0, 255),
        // Orange4
        58 => (95, 95, 0),
        // Grey37
        59 => (95, 95, 95),
        // MediumPurple4
        60 => (95, 95, 135),
        // SlateBlue3
        61 => (95, 95, 175),
        // SlateBlue3
        62 => (95, 95, 215),
        // RoyalBlue1
        63 => (95, 95, 255),
        // Chartreuse4
        64 => (95, 135, 0),
        // DarkSeaGreen4
        65 => (95, 135, 95),
        // PaleTurquoise4
        66 => (95, 135, 135),
        // SteelBlue
        67 => (95, 135, 175),
        // SteelBlue3
        68 => (95, 135, 215),
        // CornflowerBlue
        69 => (95, 135, 255),
        // Chartreuse3
        70 => (95, 175, 0),
        // DarkSeaGreen4
        71 => (95, 175, 95),
        // CadetBlue
        72 => (95, 175, 135),
        // CadetBlue
        73 => (95, 175, 175),
        // SkyBlue3
        74 => (95, 175, 215),
        // SteelBlue1
        75 => (95, 175, 255),
        // Chartreuse3
        76 => (95, 215, 0),
        // PaleGreen3
        77 => (95, 215, 95),
        // SeaGreen3
        78 => (95, 215, 135),
        // Aquamarine3
        79 => (95, 215, 175),
        // MediumTurquoise
        80 => (95, 215, 215),
        // SteelBlue1
        81 => (95, 215, 255),
        // Chartreuse2
        82 => (95, 255, 0),
        // SeaGreen2
        83 => (95, 255, 95),
        // SeaGreen1
        84 => (95, 255, 135),
        // SeaGreen1
        85 => (95, 255, 175),
        // Aquamarine1
        86 => (95, 255, 215),
        // DarkSlateGray2
        87 => (95, 255, 255),
        // DarkRed
        88 => (135, 0, 0),
        // DeepPink4
        89 => (135, 0, 95),
        // DarkMagenta
        90 => (135, 0, 135),
        // DarkMagenta
        91 => (135, 0, 175),
        // DarkViolet
        92 => (135, 0, 215),
        // Purple
        93 => (135, 0, 255),
        // Orange4
        94 => (135, 95, 0),
        // LightPink4
        95 => (135, 95, 95),
        // Plum4
        96 => (135, 95, 135),
        // MediumPurple3
        97 => (135, 95, 175),
        // MediumPurple3
        98 => (135, 95, 215),
        // SlateBlue1
        99 => (135, 95, 255),
        // Yellow4
        100 => (135, 135, 0),
        // Wheat4
        101 => (135, 135, 95),
        // Grey53
        102 => (135, 135, 135),
        // LightSlateGrey
        103 => (135, 135, 175),
        // MediumPurple
        104 => (135, 135, 215),
        // LightSlateBlue
        105 => (135, 135, 255),
        // Yellow4
        106 => (135, 175, 0),
        // DarkOliveGreen3
        107 => (135, 175, 95),
        // DarkSeaGreen
        108 => (135, 175, 135),
        // LightSkyBlue3
        109 => (135, 175, 175),
        // LightSkyBlue3
        110 => (135, 175, 215),
        // SkyBlue2
        111 => (135, 175, 255),
        // Chartreuse2
        112 => (135, 215, 0),
        // DarkOliveGreen3
        113 => (135, 215, 95),
        // PaleGreen3
        114 => (135, 215, 135),
        // DarkSeaGreen3
        115 => (135, 215, 175),
        // DarkSlateGray3
        116 => (135, 215, 215),
        // SkyBlue1
        117 => (135, 215, 255),
        // Chartreuse1
        118 => (135, 255, 0),
        // LightGreen
        119 => (135, 255, 95),
        // LightGreen
        120 => (135, 255, 135),
        // PaleGreen1
        121 => (135, 255, 175),
        // Aquamarine1
        122 => (135, 255, 215),
        // DarkSlateGray1
        123 => (135, 255, 255),
        // Red3
        124 => (175, 0, 0),
        // DeepPink4
        125 => (175, 0, 95),
        // MediumVioletRed
        126 => (175, 0, 135),
        // Magenta3
        127 => (175, 0, 175),
        // DarkViolet
        128 => (175, 0, 215),
        // Purple
        129 => (175, 0, 255),
        // DarkOrange3
        130 => (175, 95, 0),
        // IndianRed
        131 => (175, 95, 95),
        // HotPink3
        132 => (175, 95, 135),
        // MediumOrchid3
        133 => (175, 95, 175),
        // MediumOrchid
        134 => (175, 95, 215),
        // MediumPurple2
        135 => (175, 95, 255),
        // DarkGoldenrod
        136 => (175, 135, 0),
        // LightSalmon3
        137 => (175, 135, 95),
        // RosyBrown
        138 => (175, 135, 135),
        // Grey63
        139 => (175, 135, 175),
        // MediumPurple2
        140 => (175, 135, 215),
        // MediumPurple1
        141 => (175, 135, 255),
        // Gold3
        142 => (175, 175, 0),
        // DarkKhaki
        143 => (175, 175, 95),
        // NavajoWhite3
        144 => (175, 175, 135),
        // Grey69
        145 => (175, 175, 175),
        // LightSteelBlue3
        146 => (175, 175, 215),
        // LightSteelBlue
        147 => (175, 175, 255),
        // Yellow3
        148 => (175, 215, 0),
        // DarkOliveGreen3
        149 => (175, 215, 95),
        // DarkSeaGreen3
        150 => (175, 215, 135),
        // DarkSeaGreen2
        151 => (175, 215, 175),
        // LightCyan3
        152 => (175, 215, 215),
        // LightSkyBlue1
        153 => (175, 215, 255),
        // GreenYellow
        154 => (175, 255, 0),
        // DarkOliveGreen2
        155 => (175, 255, 95),
        // PaleGreen1
        156 => (175, 255, 135),
        // DarkSeaGreen2
        157 => (175, 255, 175),
        // DarkSeaGreen1
        158 => (175, 255, 215),
        // PaleTurquoise1
        159 => (175, 255, 255),
        // Red3
        160 => (215, 0, 0),
        // DeepPink3
        161 => (215, 0, 95),
        // DeepPink3
        162 => (215, 0, 135),
        // Magenta3
        163 => (215, 0, 175),
        // Magenta3
        164 => (215, 0, 215),
        // Magenta2
        165 => (215, 0, 255),
        // DarkOrange3
        166 => (215, 95, 0),
        // IndianRed
        167 => (215, 95, 95),
        // HotPink3
        168 => (215, 95, 135),
        // HotPink2
        169 => (215, 95, 175),
        // Orchid
        170 => (215, 95, 215),
        // MediumOrchid1
        171 => (215, 95, 255),
        // Orange3
        172 => (215, 135, 0),
        // LightSalmon3
        173 => (215, 135, 95),
        // LightPink3
        174 => (215, 135, 135),
        // Pink3
        175 => (215, 135, 175),
        // Plum3
        176 => (215, 135, 215),
        // Violet
        177 => (215, 135, 255),
        // Gold3
        178 => (215, 175, 0),
        // LightGoldenrod3
        179 => (215, 175, 95),
        // Tan
        180 => (215, 175, 135),
        // MistyRose3
        181 => (215, 175, 175),
        // Thistle3
        182 => (215, 175, 215),
        // Plum2
        183 => (215, 175, 255),
        // Yellow3
        184 => (215, 215, 0),
        // Khaki3
        185 => (215, 215, 95),
        // LightGoldenrod2
        186 => (215, 215, 135),
        // LightYellow3
        187 => (215, 215, 175),
        // Grey84
        188 => (215, 215, 215),
        // LightSteelBlue1
        189 => (215, 215, 255),
        // Yellow2
        190 => (215, 255, 0),
        // DarkOliveGreen1
        191 => (215, 255, 95),
        // DarkOliveGreen1
        192 => (215, 255, 135),
        // DarkSeaGreen1
        193 => (215, 255, 175),
        // Honeydew2
        194 => (215, 255, 215),
        // LightCyan1
        195 => (215, 255, 255),
        // Red1
        196 => (255, 0, 0),
        // DeepPink2
        197 => (255, 0, 95),
        // DeepPink1
        198 => (255, 0, 135),
        // DeepPink1
        199 => (255, 0, 175),
        // Magenta2
        200 => (255, 0, 215),
        // Magenta1
        201 => (255, 0, 255),
        // OrangeRed1
        202 => (255, 95, 0),
        // IndianRed1
        203 => (255, 95, 95),
        // IndianRed1
        204 => (255, 95, 135),
        // HotPink
        205 => (255, 95, 175),
        // HotPink
        206 => (255, 95, 215),
        // MediumOrchid1
        207 => (255, 95, 255),
        // DarkOrange
        208 => (255, 135, 0),
        // Salmon1
        209 => (255, 135, 95),
        // LightCoral
        210 => (255, 135, 135),
        // PaleVioletRed1
        211 => (255, 135, 175),
        // Orchid2
        212 => (255, 135, 215),
        // Orchid1
        213 => (255, 135, 255),
        // Orange1
        214 => (255, 175, 0),
        // SandyBrown
        215 => (255, 175, 95),
        // LightSalmon1
        216 => (255, 175, 135),
        // LightPink1
        217 => (255, 175, 175),
        // Pink1
        218 => (255, 175, 215),
        // Plum1
        219 => (255, 175, 255),
        // Gold1
        220 => (255, 215, 0),
        // LightGoldenrod2
        221 => (255, 215, 95),
        // LightGoldenrod2
        222 => (255, 215, 135),
        // NavajoWhite1
        223 => (255, 215, 175),
        // MistyRose1
        224 => (255, 215, 215),
        // Thistle1
        225 => (255, 215, 255),
        // Yellow1
        226 => (255, 255, 0),
        // LightGoldenrod1
        227 => (255, 255, 95),
        // Khaki1
        228 => (255, 255, 135),
        // Wheat1
        229 => (255, 255, 175),
        // Cornsilk1
        230 => (255, 255, 215),
        // Grey100
        231 => (255, 255, 255),
        // Grey3
        232 => (8, 8, 8),
        // Grey7
        233 => (18, 18, 18),
        // Grey11
        234 => (28, 28, 28),
        // Grey15
        235 => (38, 38, 38),
        // Grey19
        236 => (48, 48, 48),
        // Grey23
        237 => (58, 58, 58),
        // Grey27
        238 => (68, 68, 68),
        // Grey30
        239 => (78, 78, 78),
        // Grey35
        240 => (88, 88, 88),
        // Grey39
        241 => (98, 98, 98),
        // Grey42
        242 => (108, 108, 108),
        // Grey46
        243 => (118, 118, 118),
        // Grey50
        244 => (128, 128, 128),
        // Grey54
        245 => (138, 138, 138),
        // Grey58
        246 => (148, 148, 148),
        // Grey62
        247 => (158, 158, 158),
        // Grey66
        248 => (168, 168, 168),
        // Grey70
        249 => (178, 178, 178),
        // Grey74
        250 => (188, 188, 188),
        // Grey78
        251 => (198, 198, 198),
        // Grey82
        252 => (208, 208, 208),
        // Grey85
        253 => (218, 218, 218),
        // Grey89
        254 => (228, 228, 228),
        // Grey93
        255 => (238, 238, 238),
        _ => panic!("Unknown 8-bit color {}", eight_bit_color)
    }
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
pub fn EIGHT_BIT_FOREGROUND_CODE(byte: u8) -> String {
    // \x1B[38;2;R;G;Bm	
    return format!("\x1B[38;5;{}m", byte)
}

#[allow(dead_code)]
pub fn EIGHT_BIT_BACKGROUND_CODE(byte: u8) -> String {
    return format!("\x1B[48;5;{}m", byte)
}

#[allow(dead_code)]
pub fn RGB_FOREGROUND_CODE(r: u8, g: u8, b: u8) -> String {
    // \x1B[38;2;R;G;Bm	
    return format!("\x1B[38;2;{};{};{}m", r, g, b)
}

#[allow(dead_code)]
pub fn RGB_BACKGROUND_CODE(r: u8, g: u8, b: u8) -> String {
    return format!("\x1B[48;2;{};{};{}m", r, g, b)
}

#[allow(dead_code)]
pub const DEFAULT_FOREGROUND_CODE: &str = "\x1B[39m";
#[allow(dead_code)]
pub const DEFAULT_BACKGROUND_CODE: &str = "\x1B[49m";