pub mod two_bytes;
pub mod three_bytes;

pub use two_bytes::{
    INVALID as INVALID_PREDEFINED_COLOR_2_BYTES,
    get_predefined_color_2_bytes,
    SIZE as PREDEFINED_COLOR_SIZE_2_BYTES
};

pub use three_bytes::{
    INVALID as INVALID_PREDEFINED_COLOR_3_BYTES,
    get_predefined_color_3_bytes,
    SIZE as PREDEFINED_COLOR_SIZE_3_BYTES
};
