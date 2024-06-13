use std::ops::Index;
use std::simd::Simd;


// structure
// byte 0 is for the type - style or type of color
// byte 1 is for the size - the size that should be skipped
// byte 2 is for the size of the bytes in the value
// the rest are for the value
// for style only 1 byte is needed
// for color there is minimum 2 - 1 for foreground or background and 1 for the color
// for 255 bit colors with need 4 bytes - 1 for foreground or background, 3 for rgb values
// so the maximum consumed size will be 7 bytes

// 32 as we need to align with the input size for the mask

macro_rules! build_graphics_mode_result {
    ($result_type:expr, $total_size:expr, $value_size:literal, $value: expr, $($rest:expr),*) => {
        Simd::<u8, 32>::from_array([$result_type,$total_size,$value_size, $value, $($rest,)*]);
    };
}

#[inline(always)]
pub fn is_valid(result: Simd::<u8, 8>) -> bool {
    return get_type(result) != 0;
}

#[inline(always)]
pub fn get_type(result: Simd::<u8, 8>) -> u8 {
    *result.index(0)
}

#[inline(always)]
pub fn get_size(result: Simd::<u8, 8>) -> u8 {
    *result.index(1)
}

#[inline(always)]
pub fn get_value_size(result: Simd::<u8, 8>) -> u8 {
    *result.index(2)
}

pub(crate) use build_graphics_mode_result;
