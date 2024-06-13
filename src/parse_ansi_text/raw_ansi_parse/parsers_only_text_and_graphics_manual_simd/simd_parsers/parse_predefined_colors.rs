mod two_bytes;
mod three_bytes;

pub const PARSE_GRAPHICS_MODE_PREDEFINED_COLOR_TYPE: u8 = 2;

pub use two_bytes::{
    INVALID as INVALID_PREDEFINED_COLOR_2_BYTES,
    get_predefined_color_2_bytes,
    get_predefined_color_2_bytes_simd,
    SIZE as PREDEFINED_COLOR_SIZE_2_BYTES,

};

pub use three_bytes::{
    INVALID as INVALID_PREDEFINED_COLOR_3_BYTES,
    get_predefined_color_3_bytes,
    get_predefined_color_3_bytes_simd,
    SIZE as PREDEFINED_COLOR_SIZE_3_BYTES
};
