mod one_byte;
mod two_bytes;
mod three_bytes;

pub use one_byte::{
    INVALID as INVALID_EIGHT_BIT_COLOR_1_BYTE,
    get_eight_bit_color_one_byte,
    SIZE as EIGHT_BIT_COLOR_SIZE_1_BYTE
};
pub use two_bytes::{
    INVALID as INVALID_EIGHT_BIT_COLOR_2_BYTES,
    get_eight_bit_color_two_bytes,
    SIZE as EIGHT_BIT_COLOR_SIZE_2_BYTES
};
pub use three_bytes::{
    INVALID as INVALID_EIGHT_BIT_COLOR_3_BYTES,
    get_eight_bit_color_three_bytes,
    SIZE as EIGHT_BIT_COLOR_SIZE_3_BYTES
};
