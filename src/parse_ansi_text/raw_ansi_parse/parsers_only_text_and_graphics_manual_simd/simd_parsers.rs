pub mod parse_u8;
pub mod parse_style;
pub mod parse_8_bit_colors;
pub mod parse_predefined_colors;
pub mod parse_rgb_colors;
pub mod helpers;

use std::ops::{BitAnd, BitOr, Div, Mul};
use std::simd::cmp::{SimdPartialEq, SimdPartialOrd};
use std::simd::num::SimdUint;
use std::simd::Simd;
use crate::parse_ansi_text::raw_ansi_parse::AnsiSequence;
use crate::parse_ansi_text::raw_ansi_parse::parsers_only_text_and_graphics_manual_simd::parse_escape;

pub const ESCAPE_AS_BYTES: &[u8] = b"\x1b";
pub const EMPTY_AS_BYTES: &[u8] = b"";

// constant Simd structs
pub const LANES: usize = 32;
pub const ZEROS: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([0; LANES]);
pub const ASCII_ZERO: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([b'0'; LANES]);
pub const ASCII_NINE: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([b'9'; LANES]);
pub const MULTIPLY_U8: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([100, 10, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
pub const DIVIDE_U8: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([100, 10, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);


// If between 0 and 255

//
// pub const GRAPHIC_MODE_1_SINGLE_BYTE: Simd<u8, LANES> = Simd::<u8, LANES>::load_or_default(&[
//     b'\x1b', b'[',
//     // for single byte string number
//     b'9',
//     b'm',
// ]);
// pub const GRAPHIC_MODE_1_TWO_BYTES: Simd<u8, LANES> = Simd::<u8, LANES>::load_or_default(&[
//     b'\x1b', b'[',
//     // for two byte string number
//     b'9',
//     b'9',
//     b'm',
// ]);
// pub const GRAPHIC_MODE_1_THREE_BYTES: Simd<u8, LANES> = Simd::<u8, LANES>::load_or_default(&[
//     b'\x1b', b'[',
//     // for three byte string number - 255 which is the maximum for u8
//     // 9 as it's the largest number
//     b'9',
//     b'9',
//     b'9',
//     b'm',
// ]);

pub const GRAPHICS: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([b'Z'; LANES]);


#[cfg(test)]
mod tests {
    use std::ops::{Mul, Shl};
    use std::simd::cmp::SimdPartialOrd;
    use std::simd::num::SimdUint;
    use pretty_assertions::assert_eq;

    use crate::parse_ansi_text::ansi::colors::*;
    use crate::parse_ansi_text::ansi::constants::RESET_CODE;
    use crate::parse_ansi_text::raw_ansi_parse::parsers_only_text_and_graphics_manual_simd::constants::*;
    use crate::parse_ansi_text::raw_ansi_parse::parsers_only_text_and_graphics_manual_simd::simd_parsers::parse_u8::parse_u8_simd;

    use super::*;

    #[test]
    fn test_value() {
        // Italic code
        let input = b"\x1B[3m";
        let bytes = Simd::<u8, LANES>::load_or_default(input);
        println!("bytes: {:?}", bytes.as_array());

        println!("ESCAPE_START: {:?}", ESCAPE_START.as_array());
        println!("GRAPHIC_MODE_1_SINGLE_BYTE_NUMBER_MIN: {:?}", GRAPHIC_MODE_1_SINGLE_BYTE_NUMBER_MIN.as_array());
        println!("GRAPHIC_MODE_1_SINGLE_BYTE_NUMBER_MAX: {:?}", GRAPHIC_MODE_1_SINGLE_BYTE_NUMBER_MAX.as_array());

        // TODO - if m is not there than it doesn't matter if other in range


        let is_graphic_1_single_byte = bytes.simd_ge(GRAPHIC_MODE_1_SINGLE_BYTE_NUMBER_MIN) & bytes.simd_le(GRAPHIC_MODE_1_SINGLE_BYTE_NUMBER_MAX);
        let is_graphic_1_two_bytes = bytes.simd_ge(GRAPHIC_MODE_1_TWO_BYTES_NUMBER_MIN) & bytes.simd_le(GRAPHIC_MODE_1_TWO_BYTES_NUMBER_MAX);
        let is_graphic_1_three_bytes = bytes.simd_ge(GRAPHIC_MODE_1_THREE_BYTES_NUMBER_MIN) & bytes.simd_le(GRAPHIC_MODE_1_THREE_BYTES_NUMBER_MAX);

        println!("is_graphic_1_single_byte: {:?}", is_graphic_1_single_byte.to_array());

        println!("single byte {:?}", is_graphic_1_single_byte.all());
        println!("two bytes {:?}", is_graphic_1_two_bytes.all());
        println!("three bytes {:?}", is_graphic_1_three_bytes.all());

        let result = is_graphic_mode_1(bytes);
        println!("is mode 1 {:?}", result);

        let value = parse_u8_simd(bytes.rotate_elements_left::<2>().resize(4));

        println!("value: {:?}", value);


        // let mask_with_escape = bytes.simd_eq(ESCAPE_START);
        //
        // println!("mask_with_escape: {:?}", mask_with_escape);
        //
        // let valid_start = mask_with_escape.test(0) && mask_with_escape.test(1);
        //
        // println!("valid_start: {:?}", valid_start);
    }

    #[test]
    fn str_num_to_u8() {
        let zero = b"0";
        let one = b"1";
        let two = b"2";
        let three = b"3";
        let four = b"4";
        let five = b"5";
        let six = b"6";
        let seven = b"7";
        let eight = b"8";
        let nine = b"9";

        let zero_u8 = zero[0] - b'0';
        println!("zero_u8: {:?}", zero_u8);

        let one_u8 = one[0] - b'0';
        println!("one_u8: {:?}", one_u8);

        let two_u8 = two[0] - b'0';
        println!("two_u8: {:?}", two_u8);

        let three_u8 = three[0] - b'0';
        println!("three_u8: {:?}", three_u8);

        let four_u8 = four[0] - b'0';
        println!("four_u8: {:?}", four_u8);

        let five_u8 = five[0] - b'0';
        println!("five_u8: {:?}", five_u8);

        let six_u8 = six[0] - b'0';
        println!("six_u8: {:?}", six_u8);

        let seven_u8 = seven[0] - b'0';
        println!("seven_u8: {:?}", seven_u8);

        let eight_u8 = eight[0] - b'0';
        println!("eight_u8: {:?}", eight_u8);

        let nine_u8 = nine[0] - b'0';
        println!("nine_u8: {:?}", nine_u8);


        let eleven = b"11";
        let eleven_u8 = (eleven[0] - b'0') * 10 + (eleven[1] - b'0');

        let u8_max = b"255";
        let u8_max_u8 = (u8_max[0] - b'0') * 100 + (u8_max[1] - b'0') * 10 + (u8_max[2] - b'0');
        println!("eleven_u8: {:?}", eleven_u8);
    }

    #[test]
    fn parse_num() {

        // let bytes = Simd::<u8, LANES>::load_or_default(b"\x1b[255m");
        let bytes = Simd::<u8, LANES>::load_or_default(b"999");
        let ascii_zero = Simd::<u8, LANES>::from_array([b'0'; LANES]);

        let string_to_u8 = bytes - ascii_zero;
        let mul_to_correct_num = Simd::<u8, LANES>::from_array([100, 10, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let current_nums = string_to_u8.mul(mul_to_correct_num);

        let num = current_nums.reduce_sum();

        println!("bytes: {:?}", bytes.as_array());
        println!("string_to_u8: {:?}", string_to_u8.as_array());
        println!("current_nums: {:?}", current_nums.as_array());

        println!("num: {:?}", num);
    }


    #[test]
    fn shift_only_if_needed() {
        // 0 in ascii
        let simd_num = Simd::<u8, LANES>::load_or_default(&[0, 48u8, b'm']);


        // Mask of only the numbers
        let only_numbers_mask = simd_num.simd_ge(ASCII_ZERO);
        let only_numbers_mask = only_numbers_mask & simd_num.simd_le(ASCII_NINE);
        let only_numbers = only_numbers_mask.select(simd_num, ZEROS);

        println!("simd_num: {:?}", simd_num.as_array());
        println!("only_numbers: {:?}", only_numbers.as_array());
    }

}