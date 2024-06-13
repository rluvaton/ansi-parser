use std::ops::{Add, BitAnd, BitAndAssign, BitOr, BitXor, Div, Index, Mul, Sub};
use std::simd::cmp::{SimdPartialEq, SimdPartialOrd};
use std::simd::num::SimdUint;
use std::simd::{Mask, Simd};

use crate::parse_ansi_text::raw_ansi_parse::parsers_only_text_and_graphics_manual_simd::simd_parsers::helpers::{AllOrNone, build_graphics_mode_result, simd_to_u64};

const LANES: usize = 32;

pub const PARSE_GRAPHICS_MODE_STYLE_TYPE: u8 = 1;

pub const INVALID_STYLE: u8 = 255;
// b'\x1b[0m' or other number instead of 0
pub const STYLE_SIZE: u8 = 4;

const MASK: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([
    255, // b'\x1b',
    255, //b'[',
    255, // Everything
    255, //b'm',

    // Empty
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
]);

const MASK_SMALL: Simd<u8, 8> = Simd::<u8, 8>::from_array([
    255, // b'\x1b',
    255, //b'[',
    255, // Everything
    255, //b'm',

    // Empty
    0, 0, 0, 0,
]);
const MASK_SMALL_U64: u64 = 0xFF_FF_FF_FF_00_00_00_00;

// this is b'\x1b', b'[', -, b'm'
const EXPECTED_VALUE_WITHOUT_NUMBER_SMALL_U64: u64 = 0x1B_5B_00_6D_00_00_00_00;
const MASK_WITHOUT_NUMBER_SMALL_U64: u64 = 0xFF_FF_00_FF_00_00_00_00;


const MIN_MASK: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([
    b'\x1b',
    b'[',
    b'0', // Everything
    b'm',

    // Empty
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
]);

const MIN_MASK_SMALL: Simd<u8, 8> = Simd::<u8, 8>::from_array([
    b'\x1b',
    b'[',
    b'0', // Everything
    b'm',

    // Empty
    0, 0, 0, 0,
]);

const MAX_MASK: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([
    b'\x1b',
    b'[',
    b'9', // Everything
    b'm',

    // Empty
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
]);

const MAX_MASK_SMALL: Simd<u8, 8> = Simd::<u8, 8>::from_array([
    b'\x1b',
    b'[',
    b'9', // Everything
    b'm',

    // Empty
    0, 0, 0, 0,
]);


const SUBTRACT_NUM_TO_U8: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([
    0, // b'\x1b',
    0, // b'[',
    b'0', // b'9', // Everything
    0, // b'm',

    // Empty
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0
]);

const SUBTRACT_NUM_TO_U8_SMALL: Simd<u8, 8> = Simd::<u8, 8>::from_array([
    0, // b'\x1b',
    0, // b'[',
    b'0', // b'9', // Everything
    0, // b'm',

    // Empty
    0, 0, 0, 0,
]);

// 30 is the ascii value of '0'
const SUBTRACT_NUM_TO_U8_SMALL_U64: u64 = 0x00_00_30_00_00_00_00_00;


const KEEP_VALUE_BYTE: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([
    0, // PARSE_GRAPHICS_MODE_STYLE_TYPE,
    0, // SIZE,
    0, // value size
    255, // the value

    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0
]);

const KEEP_VALUE_BYTE_SMALL: Simd<u8, 8> = Simd::<u8, 8>::from_array([
    0, // PARSE_GRAPHICS_MODE_STYLE_TYPE,
    0, // SIZE,
    0, // value size
    255, // the value

    // Empty
    0, 0, 0, 0,
]);

const KEEP_VALUE_BYTE_SMALL_U64: u64 = 0x00_00_00_FF_00_00_00_00;

const GRAPHICS_MODE_RESULT: Simd<u8, LANES> = build_graphics_mode_result!(
    PARSE_GRAPHICS_MODE_STYLE_TYPE,
    STYLE_SIZE,
    1, // one byte for the number
    0, // the value

    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0
);


const GRAPHICS_MODE_RESULT_SMALL: Simd<u8, 8> = Simd::<u8, 8>::from_array([
    PARSE_GRAPHICS_MODE_STYLE_TYPE,
    STYLE_SIZE,
    1, // one byte for the number
    0, // the value

    0, 0, 0, 0,
]);

const GRAPHICS_MODE_RESULT_SMALL_U64: u64 = 0x01_04_01_00_00_00_00_00;

// INVALID_STYLE for invalid, otherwise the number
pub fn get_style(bytes: Simd<u8, LANES>) -> u8 {
    let only_relevant_part = bytes & MASK;

    let is_text_style = only_relevant_part.simd_ge(MIN_MASK).all() && only_relevant_part.simd_le(MAX_MASK).all();
    if !is_text_style {
        return INVALID_STYLE;
    }

    // 2 as we want to get the number after b"\x1b[",
    let num = only_relevant_part.index(2);
    // Get the number from the ascii
    return num - b'0';
}

pub fn get_style_simd(bytes: Simd<u8, LANES>) -> (Mask::<i8, 32>, Simd::<u8, 32>) {
    let only_relevant_part = bytes & MASK;

    // merge the two masks and check if all the lanes are true
    let valid_mask: Mask<i8, 32> = only_relevant_part.simd_ge(MIN_MASK)
        .bitand(only_relevant_part.simd_le(MAX_MASK))
        .all_or_none();

    if !valid_mask.test(0) {
        return (
            valid_mask,
            GRAPHICS_MODE_RESULT
        );
    }

    let result = only_relevant_part
        // Getting the number from the ascii not using saturating_sub as we know it's valid range
        .sub(SUBTRACT_NUM_TO_U8)
        // Move to the correct position
        .rotate_elements_right::<1>()
        // Keep only the value byte
        .bitand(KEEP_VALUE_BYTE)

        .add(GRAPHICS_MODE_RESULT);


    return (
        valid_mask,
        result
    );
}

pub fn get_style_simd_small(bytes: Simd<u8, 8>) -> (bool, Simd::<u8, 8>) {
    let only_relevant_part = bytes & MASK_SMALL;

    // merge the two masks and check if all the lanes are true
    let valid_mask = only_relevant_part.simd_ge(MIN_MASK_SMALL)
        .bitand(only_relevant_part.simd_le(MAX_MASK_SMALL))
        .all();

    if !valid_mask {
        return (
            false,
            GRAPHICS_MODE_RESULT_SMALL
        );
    }

    let result = only_relevant_part
        // Getting the number from the ascii not using saturating_sub as we know it's valid range
        .sub(SUBTRACT_NUM_TO_U8_SMALL)
        // Move to the correct position
        .rotate_elements_right::<1>()
        // Keep only the value byte
        .bitand(KEEP_VALUE_BYTE_SMALL)

        .add(GRAPHICS_MODE_RESULT_SMALL);


    return (
        true,
        result
    );
}

pub fn get_style_u64(mut bytes: u64) -> u64 {
    bytes &= MASK_SMALL_U64;

    if bytes & MASK_WITHOUT_NUMBER_SMALL_U64 != EXPECTED_VALUE_WITHOUT_NUMBER_SMALL_U64 {
        return 0;
    }

    let number = (bytes & 0x00_00_FF_00_00_00_00_00) >> 40;

    if number < b'0' as u64 || number > b'9' as u64 {
        return 0;
    }

    let mut number = number - b'0' as u64;

    number = number << 32;

    number |= GRAPHICS_MODE_RESULT_SMALL_U64;

    return number;
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::parse_ansi_text::ansi::constants::RESET_CODE;
    use crate::parse_ansi_text::ansi::style::*;
    use crate::parse_ansi_text::raw_ansi_parse::parsers_only_text_and_graphics_manual_simd::simd_parsers::helpers::{str_to_u64, u8_array_to_u64};

    use super::*;


    fn create_simd_from_str(input: &str) -> Simd<u8, LANES> {
        Simd::<u8, LANES>::load_or_default(input.as_bytes())
    }

    #[test]
    fn get_style_should_return_matching_value() {
        assert_eq!(get_style(create_simd_from_str(RESET_CODE)), 0);
        assert_eq!(get_style(create_simd_from_str(BOLD_CODE)), 1);
        assert_eq!(get_style(create_simd_from_str(DIM_CODE)), 2);
        assert_eq!(get_style(create_simd_from_str(ITALIC_CODE)), 3);
        assert_eq!(get_style(create_simd_from_str(UNDERLINE_CODE)), 4);
        assert_eq!(get_style(create_simd_from_str(INVERSE_CODE)), 7);
        assert_eq!(get_style(create_simd_from_str(STRIKETHROUGH_CODE)), 9);
    }


    #[test]
    fn get_style_should_return_style_even_if_have_other_bytes_after_style() {
        for num in 0..=9 {
            let byte = b'0' + num;

            assert_eq!(
                get_style(Simd::<u8, LANES>::load_or_default(&[b'\x1b', b'[', byte, b'm', b'h', b'e', b'l', b'l', b'o'])),
                num
            );
        }
    }

    #[test]
    fn get_style_should_return_invalid_for_without_correct_structure() {
        assert_eq!(
            // should have been a number between '[' and 'm'
            get_style(create_simd_from_str("\x1b[m")),
            INVALID_STYLE
        );
        assert_eq!(
            // should have '[' and not ']'
            get_style(create_simd_from_str("\x1b]1m")),
            INVALID_STYLE
        );
        assert_eq!(
            // should have '\x1b' and not 'a'
            get_style(create_simd_from_str("a[1m")),
            INVALID_STYLE
        );
        assert_eq!(
            // should have m in the end
            get_style(create_simd_from_str("\x1b[1")),
            INVALID_STYLE
        );
        assert_eq!(
            // must not be empty
            get_style(create_simd_from_str("")),
            INVALID_STYLE
        );
        assert_eq!(
            // the escape code should be in the beginning
            get_style(create_simd_from_str("0\x1b[1m")),
            INVALID_STYLE
        );
    }

    #[test]
    fn get_style_should_return_invalid_style_when_not_ascii_number_in_the_number_position() {
        for byte in 0..=255u8 {
            // Ignore the ascii numbers
            if byte >= b'0' && byte <= b'9' {
                continue;
            }

            assert_eq!(
                get_style(Simd::<u8, LANES>::load_or_default(&[b'\x1b', b'[', byte, b'm'])),
                INVALID_STYLE
            );
        }
    }


    // ----------------- SIMD -----------------

    fn create_valid_result_for_value(value: u8) -> (Mask<i8, 32>, Simd<u8, 32>) {
        (
            Mask::from_array([true; 32]),
            Simd::<u8, 32>::load_or_default(&[
                PARSE_GRAPHICS_MODE_STYLE_TYPE,
                STYLE_SIZE,
                1,
                value,
            ])
        )
    }

    #[test]
    fn get_style_simd_should_return_matching_value_for() {
        assert_eq!(
            get_style_simd(create_simd_from_str(RESET_CODE)),
            create_valid_result_for_value(0)
        );
        assert_eq!(
            get_style_simd(create_simd_from_str(BOLD_CODE)),
            create_valid_result_for_value(1)
        );
        assert_eq!(
            get_style_simd(create_simd_from_str(DIM_CODE)),
            create_valid_result_for_value(2)
        );
        assert_eq!(
            get_style_simd(create_simd_from_str(ITALIC_CODE)),
            create_valid_result_for_value(3)
        );
        assert_eq!(
            get_style_simd(create_simd_from_str(UNDERLINE_CODE)),
            create_valid_result_for_value(4)
        );
        assert_eq!(
            get_style_simd(create_simd_from_str(INVERSE_CODE)),
            create_valid_result_for_value(7)
        );
        assert_eq!(
            get_style_simd(create_simd_from_str(STRIKETHROUGH_CODE)),
            create_valid_result_for_value(9)
        );
    }


    #[test]
    fn get_style_simd_should_return_style_even_if_have_other_bytes_after_style() {
        for num in 0..=9 {
            let byte = b'0' + num;

            assert_eq!(
                get_style_simd(Simd::<u8, LANES>::load_or_default(&[b'\x1b', b'[', byte, b'm', b'h', b'e', b'l', b'l', b'o'])),
                create_valid_result_for_value(num)
            );
        }
    }

    #[test]
    fn get_style_simd_should_return_invalid_for_without_correct_structure() {
        let all_false = Mask::from_array([false; 32]);

        assert_eq!(
            // should have been a number between '[' and 'm'
            get_style_simd(create_simd_from_str("\x1b[m")).0,
            all_false
        );
        assert_eq!(
            // should have '[' and not ']'
            get_style_simd(create_simd_from_str("\x1b]1m")).0,
            all_false
        );
        assert_eq!(
            // should have '\x1b' and not 'a'
            get_style_simd(create_simd_from_str("a[1m")).0,
            all_false
        );
        assert_eq!(
            // should have m in the end
            get_style_simd(create_simd_from_str("\x1b[1")).0,
            all_false
        );
        assert_eq!(
            // must not be empty
            get_style_simd(create_simd_from_str("")).0,
            all_false
        );
        assert_eq!(
            // the escape code should be in the beginning
            get_style_simd(create_simd_from_str("0\x1b[1m")).0,
            all_false
        );
    }

    #[test]
    fn get_style_simd_should_return_invalid_style_when_not_ascii_number_in_the_number_position() {
        let all_false = Mask::from_array([false; 32]);

        for byte in 0..=255u8 {
            // Ignore the ascii numbers
            if byte >= b'0' && byte <= b'9' {
                continue;
            }

            assert_eq!(
                get_style_simd(Simd::<u8, LANES>::load_or_default(&[b'\x1b', b'[', byte, b'm'])).0,
                all_false
            );
        }
    }

    // ----------------- u64 -----------------

    fn create_valid_result_for_value_u64(value: u8) -> u64 {
        let value_in_correct_position = (value as u64) << 32;

        return 0x01_04_01_00_00_00_00_00 | value_in_correct_position
    }


    #[test]
    fn get_style_u64_should_return_matching_value() {
        assert_eq!(get_style_u64(str_to_u64(RESET_CODE)), create_valid_result_for_value_u64(0));
        assert_eq!(get_style_u64(str_to_u64(BOLD_CODE)), create_valid_result_for_value_u64(1));
        assert_eq!(get_style_u64(str_to_u64(DIM_CODE)), create_valid_result_for_value_u64(2));
        assert_eq!(get_style_u64(str_to_u64(ITALIC_CODE)), create_valid_result_for_value_u64(3));
        assert_eq!(get_style_u64(str_to_u64(UNDERLINE_CODE)), create_valid_result_for_value_u64(4));
        assert_eq!(get_style_u64(str_to_u64(INVERSE_CODE)), create_valid_result_for_value_u64(7));
        assert_eq!(get_style_u64(str_to_u64(STRIKETHROUGH_CODE)), create_valid_result_for_value_u64(9));
    }


    #[test]
    fn get_style_u64_should_return_style_even_if_have_other_bytes_after_style() {
        for num in 0..=9 {
            let byte = b'0' + num;

            assert_eq!(
                get_style_u64(u8_array_to_u64([b'\x1b', b'[', byte, b'm', b'h', b'e', b'l', b'l'])),
                create_valid_result_for_value_u64(num)
            );
        }
    }

    #[test]
    fn get_style_u64_should_return_invalid_for_without_correct_structure() {
        assert_eq!(
            // should have been a number between '[' and 'm'
            get_style_u64(str_to_u64("\x1b[m")),
            0
        );
        assert_eq!(
            // should have '[' and not ']'
            get_style_u64(str_to_u64("\x1b]1m")),
            0
        );
        assert_eq!(
            // should have '\x1b' and not 'a'
            get_style_u64(str_to_u64("a[1m")),
            0
        );
        assert_eq!(
            // should have m in the end
            get_style_u64(str_to_u64("\x1b[1")),
            0
        );
        assert_eq!(
            // must not be empty
            get_style_u64(str_to_u64("")),
            0
        );
        assert_eq!(
            // the escape code should be in the beginning
            get_style_u64(str_to_u64("0\x1b[1m")),
            0
        );
    }

    #[test]
    fn get_style_u64_should_return_invalid_style_when_not_ascii_number_in_the_number_position() {
        for byte in 0..=255u8 {
            // Ignore the ascii numbers
            if byte >= b'0' && byte <= b'9' {
                continue;
            }

            assert_eq!(
                get_style_u64(u8_array_to_u64([b'\x1b', b'[', byte, b'm', 0, 0, 0, 0])),
                0
            );
        }
    }

}
