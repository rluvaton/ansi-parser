use std::ops::{Add, AddAssign, BitAnd, BitOr, Div, Index, Mul, Sub};
use std::simd::cmp::{SimdPartialEq, SimdPartialOrd};
use std::simd::num::SimdUint;
use std::simd::{Mask, Simd};
use crate::parse_ansi_text::raw_ansi_parse::parsers_only_text_and_graphics_manual_simd::simd_parsers::helpers::{AllOrNone, build_graphics_mode_result, simd_to_u64};
use crate::parse_ansi_text::raw_ansi_parse::parsers_only_text_and_graphics_manual_simd::simd_parsers::parse_predefined_colors::PARSE_GRAPHICS_MODE_PREDEFINED_COLOR_TYPE;

const LANES: usize = 32;

// Predefined colors here are not numbers above 99 and one of the specific colors
pub const INVALID: u8 = 255;
// b'\x1b[38m' or other number instead of 38
pub const SIZE: u8 = 5;

const MASK: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([
    255, // b'\x1b',
    255, //b'[',
    255, // Everything
    255, // Everything
    255, //b'm',

    // Empty
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0,
]);

const MASK_SMALL: Simd<u8, 8> = Simd::<u8, 8>::from_array([
    255, // b'\x1b',
    255, //b'[',
    255, // Everything
    255, // Everything
    255, //b'm',

    // Empty
    0, 0, 0,
]);

const MASK_SMALL_U64: u64 = 0xFF_FF_FF_FF_00_00_00_00;

// this is b'\x1b', b'[', -, b'm'
const EXPECTED_VALUE_WITHOUT_NUMBER_SMALL_U64: u64 = 0x1B_5B_00_00_6D_00_00_00;
const MASK_WITHOUT_NUMBER_SMALL_U64: u64 = 0xFF_FF_00_00_FF_00_00_00;

const MIN_REGULAR_COLOR_MASK: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([
    b'\x1b',
    b'[',
    b'3', // Everything
    b'0', // Everything
    b'm',

    // Empty
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0,
]);

const MIN_REGULAR_COLOR_MASK_SMALL: Simd<u8, 8> = Simd::<u8, 8>::from_array([
    b'\x1b',
    b'[',
    b'3', // Everything
    b'0', // Everything
    b'm',

    // Empty
    0, 0, 0,
]);

const MAX_REGULAR_COLOR_MASK: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([
    b'\x1b',
    b'[',
    b'4', // Everything
    b'9', // Everything
    b'm',

    // Empty
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0,
]);

const MAX_REGULAR_COLOR_MASK_SMALL: Simd<u8, 8> = Simd::<u8, 8>::from_array([
    b'\x1b',
    b'[',
    b'4', // Everything
    b'9', // Everything
    b'm',

    // Empty
    0, 0, 0,
]);

const MIN_BRIGHT_COLOR_MASK: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([
    b'\x1b',
    b'[',
    b'9', // Everything
    b'0', // Everything
    b'm',

    // Empty
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0,
]);

const MIN_BRIGHT_COLOR_MASK_SMALL: Simd<u8, 8> = Simd::<u8, 8>::from_array([
    b'\x1b',
    b'[',
    b'9', // Everything
    b'0', // Everything
    b'm',

    // Empty
    0, 0, 0,
]);

const MAX_BRIGHT_COLOR_MASK: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([
    b'\x1b',
    b'[',
    b'9', // Everything
    b'9', // Everything
    b'm',

    // Empty
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0,
]);

const MAX_BRIGHT_COLOR_MASK_SMALL: Simd<u8, 8> = Simd::<u8, 8>::from_array([
    b'\x1b',
    b'[',
    b'9', // Everything
    b'9', // Everything
    b'm',

    // Empty
    0, 0, 0,
]);

const SUBTRACT_NUM_TO_U8: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([
    0, // b'\x1b',
    0, // b'[',
    b'0', // b'9', // Everything
    b'0', // b'9', // Everything
    0, // b'm',

    // Empty
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0,
]);

const SUBTRACT_NUM_TO_U8_SMALL: Simd<u8, 8> = Simd::<u8, 8>::from_array([
    0, // b'\x1b',
    0, // b'[',
    b'0', // b'9', // Everything
    b'0', // b'9', // Everything
    0, // b'm',

    // Empty
    0, 0, 0,
]);

const SUBTRACT_NUM_TO_U8_SMALL_U64: u64 = 0x00_00_30_30_00_00_00_00;

const MULTIPLY_TO_U8: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([
    0, // b'\x1b',
    0, // b'[',
    10, // b'9', // Everything
    1, // b'9', // Everything
    0, // b'm',

    // Empty
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0,
]);

const MULTIPLY_TO_U8_SMALL: Simd<u8, 8> = Simd::<u8, 8>::from_array([
    0, // b'\x1b',
    0, // b'[',
    10, // b'9', // Everything
    1, // b'9', // Everything
    0, // b'm',

    // Empty
    0, 0, 0,
]);

const KEEP_VALUE_BYTE: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([
    0, // PARSE_GRAPHICS_MODE_PREDEFINED_COLOR_TYPE,
    0, // SIZE,
    0, // value size
    255, // the value

    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0
]);

const KEEP_VALUE_BYTE_SMALL: Simd<u8, 8> = Simd::<u8, 8>::from_array([
    0, // PARSE_GRAPHICS_MODE_PREDEFINED_COLOR_TYPE,
    0, // SIZE,
    0, // value size
    255, // the value

    0, 0, 0, 0,
]);

const GRAPHICS_MODE_RESULT: Simd<u8, LANES> = build_graphics_mode_result!(
    PARSE_GRAPHICS_MODE_PREDEFINED_COLOR_TYPE,
    SIZE,
    1, // one byte for the number
    0, // the value

    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0
);

const GRAPHICS_MODE_RESULT_SMALL: Simd<u8, 8> = Simd::<u8, 8>::from_array([
    PARSE_GRAPHICS_MODE_PREDEFINED_COLOR_TYPE,
    SIZE,
    1, // one byte for the number
    0, // the value

    0, 0, 0, 0,
]);

const GRAPHICS_MODE_RESULT_SMALL_U64: u64 = 0x02_05_01_00_00_00_00_00;

// INVALID_PREDEFINED_COLOR for invalid, otherwise the number, doesn't support bright colors above 99 to have fixed size
pub fn get_predefined_color_2_bytes(bytes: Simd<u8, LANES>) -> u8 {
    let only_relevant_part = bytes & MASK;

    let is_predefined_color = (
        only_relevant_part.simd_ge(MIN_REGULAR_COLOR_MASK).all() && only_relevant_part.simd_le(MAX_REGULAR_COLOR_MASK).all()
    ) || (
        only_relevant_part.simd_ge(MIN_BRIGHT_COLOR_MASK).all() && only_relevant_part.simd_le(MAX_BRIGHT_COLOR_MASK).all()
    );
    if !is_predefined_color {
        return INVALID;
    }

    // 2 as we want to get the number after b"\x1b[",
    let first_digit = only_relevant_part.index(2);
    let second_digit = only_relevant_part.index(3);

    // Get the number from the ascii
    return (first_digit - b'0') * 10 + (second_digit - b'0');
}

pub fn get_predefined_color_2_bytes_simd(bytes: Simd<u8, LANES>) -> (Mask::<i8, 32>, Simd::<u8, 32>) {
    let only_relevant_part = bytes & MASK;

    // merge the two masks and check if all the lanes are true
    let valid_mask_regular_color: Mask<i8, 32> = only_relevant_part.simd_ge(MIN_REGULAR_COLOR_MASK)
        .bitand(only_relevant_part.simd_le(MAX_REGULAR_COLOR_MASK))
        .all_or_none();

    let valid_mask_bright_color: Mask<i8, 32> = only_relevant_part.simd_ge(MIN_BRIGHT_COLOR_MASK)
        .bitand(only_relevant_part.simd_le(MAX_BRIGHT_COLOR_MASK))
        .all_or_none();

    // Because both masks are either all 1s or all 0s, we can just bitor them
    let valid_mask: Mask<i8, 32> = valid_mask_regular_color.bitor(valid_mask_bright_color);

    if !valid_mask.test(0) {
        return (
            valid_mask,
            GRAPHICS_MODE_RESULT
        );
    }

    let digits_as_nums = only_relevant_part
        // Getting the number from the ascii, not using saturating_sub as we already checked the range
        .sub(SUBTRACT_NUM_TO_U8)
        // This will make non-relevant numbers to 0
        .mul(MULTIPLY_TO_U8);


    let result = digits_as_nums
        .rotate_elements_right::<1>()
        .add(digits_as_nums)
        .bitand(KEEP_VALUE_BYTE)
        .add(GRAPHICS_MODE_RESULT);

    return (
        valid_mask,
        result
    );
}

pub fn get_predefined_color_2_bytes_simd_small(bytes: Simd<u8, 8>) -> (bool, Simd::<u8, 8>) {
    let only_relevant_part = bytes & MASK_SMALL;

    // merge the two masks and check if all the lanes are true
    let valid_mask_regular_color: Mask<i8, 8> = only_relevant_part.simd_ge(MIN_REGULAR_COLOR_MASK_SMALL)
        .bitand(only_relevant_part.simd_le(MAX_REGULAR_COLOR_MASK_SMALL))
        .all_or_none();

    let valid_mask_bright_color: Mask<i8, 8> = only_relevant_part.simd_ge(MIN_BRIGHT_COLOR_MASK_SMALL)
        .bitand(only_relevant_part.simd_le(MAX_BRIGHT_COLOR_MASK_SMALL))
        .all_or_none();

    // Because both masks are either all 1s or all 0s, we can just bitor them
    let valid = valid_mask_regular_color.bitor(valid_mask_bright_color).test(0);

    if !valid {
        return (
            false,
            GRAPHICS_MODE_RESULT_SMALL
        );
    }

    let digits_as_nums = only_relevant_part
        // Getting the number from the ascii, not using saturating_sub as we already checked the range
        .sub(SUBTRACT_NUM_TO_U8_SMALL)
        // This will make non-relevant numbers to 0
        .mul(MULTIPLY_TO_U8_SMALL);


    let result = digits_as_nums
        .rotate_elements_right::<1>()
        .add(digits_as_nums)
        .bitand(KEEP_VALUE_BYTE_SMALL)
        .add(GRAPHICS_MODE_RESULT_SMALL);

    return (
        true,
        result
    );
}

pub fn get_predefined_color_2_bytes_u64(bytes: u64) -> u64 {
    if bytes & MASK_WITHOUT_NUMBER_SMALL_U64 != EXPECTED_VALUE_WITHOUT_NUMBER_SMALL_U64 {
        return 0;
    }

    let first_digit = (bytes & 0x00_00_FF_00_00_00_00_00) >> 40;
    if (first_digit != b'3' as u64) && (first_digit != b'4' as u64) && (first_digit != b'9' as u64) {
        return 0;
    }

    let second_digit = (bytes & 0x00_00_00_FF_00_00_00_00) >> 32;
    if (second_digit < b'0' as u64) || (second_digit > b'9' as u64) {
        return 0;
    }

    let mut number = (first_digit - b'0' as u64) * 10 + (second_digit - b'0' as u64);

    // Move to the byte position
    number = number << 32;

    number = number | GRAPHICS_MODE_RESULT_SMALL_U64;

    return number;
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::parse_ansi_text::ansi::colors::*;
    use crate::parse_ansi_text::raw_ansi_parse::parsers_only_text_and_graphics_manual_simd::simd_parsers::helpers::{str_to_u64, u8_array_to_u64};

    use super::*;

    fn create_simd_from_str(input: &str) -> Simd<u8, LANES> {
        Simd::<u8, LANES>::load_or_default(input.as_bytes())
    }

    #[test]
    fn get_predefined_color_should_support_all_predefined_colors() {
        assert_eq!(get_predefined_color_2_bytes(create_simd_from_str(BLACK_FOREGROUND_CODE)), 30);
        assert_eq!(get_predefined_color_2_bytes(create_simd_from_str(BLACK_BACKGROUND_CODE)), 40);

        assert_eq!(get_predefined_color_2_bytes(create_simd_from_str(RED_FOREGROUND_CODE)), 31);
        assert_eq!(get_predefined_color_2_bytes(create_simd_from_str(RED_BACKGROUND_CODE)), 41);

        assert_eq!(get_predefined_color_2_bytes(create_simd_from_str(GREEN_FOREGROUND_CODE)), 32);
        assert_eq!(get_predefined_color_2_bytes(create_simd_from_str(GREEN_BACKGROUND_CODE)), 42);

        assert_eq!(get_predefined_color_2_bytes(create_simd_from_str(YELLOW_FOREGROUND_CODE)), 33);
        assert_eq!(get_predefined_color_2_bytes(create_simd_from_str(YELLOW_BACKGROUND_CODE)), 43);

        assert_eq!(get_predefined_color_2_bytes(create_simd_from_str(BLUE_FOREGROUND_CODE)), 34);
        assert_eq!(get_predefined_color_2_bytes(create_simd_from_str(BLUE_BACKGROUND_CODE)), 44);

        assert_eq!(get_predefined_color_2_bytes(create_simd_from_str(MAGENTA_FOREGROUND_CODE)), 35);
        assert_eq!(get_predefined_color_2_bytes(create_simd_from_str(MAGENTA_BACKGROUND_CODE)), 45);

        assert_eq!(get_predefined_color_2_bytes(create_simd_from_str(CYAN_FOREGROUND_CODE)), 36);
        assert_eq!(get_predefined_color_2_bytes(create_simd_from_str(CYAN_BACKGROUND_CODE)), 46);

        assert_eq!(get_predefined_color_2_bytes(create_simd_from_str(WHITE_FOREGROUND_CODE)), 37);
        assert_eq!(get_predefined_color_2_bytes(create_simd_from_str(WHITE_BACKGROUND_CODE)), 47);

        assert_eq!(get_predefined_color_2_bytes(create_simd_from_str(DEFAULT_FOREGROUND_CODE)), 39);
        assert_eq!(get_predefined_color_2_bytes(create_simd_from_str(DEFAULT_BACKGROUND_CODE)), 49);

        assert_eq!(get_predefined_color_2_bytes(create_simd_from_str(BRIGHT_BLACK_FOREGROUND_CODE)), 90);
        assert_eq!(get_predefined_color_2_bytes(create_simd_from_str(BRIGHT_RED_FOREGROUND_CODE)), 91);
        assert_eq!(get_predefined_color_2_bytes(create_simd_from_str(BRIGHT_GREEN_FOREGROUND_CODE)), 92);
        assert_eq!(get_predefined_color_2_bytes(create_simd_from_str(BRIGHT_YELLOW_FOREGROUND_CODE)), 93);
        assert_eq!(get_predefined_color_2_bytes(create_simd_from_str(BRIGHT_BLUE_FOREGROUND_CODE)), 94);
        assert_eq!(get_predefined_color_2_bytes(create_simd_from_str(BRIGHT_MAGENTA_FOREGROUND_CODE)), 95);
        assert_eq!(get_predefined_color_2_bytes(create_simd_from_str(BRIGHT_CYAN_FOREGROUND_CODE)), 96);
        assert_eq!(get_predefined_color_2_bytes(create_simd_from_str(BRIGHT_WHITE_FOREGROUND_CODE)), 97);
    }

    #[test]
    fn get_predefined_color_should_return_color_even_if_have_other_bytes_after_color() {
        for num in 30..=49 {
            let byte1 = b'0' + num / 10;
            let byte2 = b'0' + num % 10;

            assert_eq!(
                get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(&[b'\x1b', b'[', byte1, byte2, b'm', b'h', b'e', b'l', b'l', b'o'])),
                num
            );
        }
    }

    #[test]
    fn get_predefined_color_should_return_invalid_for_without_correct_structure() {
        assert_eq!(
            // should have been another number between after 3
            get_predefined_color_2_bytes(create_simd_from_str("\x1b[3m")),
            INVALID
        );
        assert_eq!(
            // should have '[' and not ']'
            get_predefined_color_2_bytes(create_simd_from_str("\x1b]39m")),
            INVALID
        );
        assert_eq!(
            // should have '\x1b' and not 'a'
            get_predefined_color_2_bytes(create_simd_from_str("a[39m")),
            INVALID
        );
        assert_eq!(
            // should have m in the end
            get_predefined_color_2_bytes(create_simd_from_str("\x1b[39")),
            INVALID
        );
        assert_eq!(
            // must not be empty
            get_predefined_color_2_bytes(create_simd_from_str("")),
            INVALID
        );
        assert_eq!(
            // the escape code should be in the beginning
            get_predefined_color_2_bytes(create_simd_from_str("0\x1b[39m")),
            INVALID
        );
    }

    #[test]
    fn get_predefined_color_should_return_invalid_predefined_color_when_not_ascii_number_in_the_number_position() {
        for byte1 in 0..=255u8 {
            for byte2 in 0..=255u8 {
                // Ignore the ascii numbers between 30 and 49 or 90 and 99
                if (byte1 == b'3' || byte1 == b'4' || byte1 == b'9') && (byte2 >= b'0' && byte2 <= b'9') {
                    continue;
                }

                assert_eq!(
                    get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(&[b'\x1b', b'[', byte1, byte2, b'm'])),
                    INVALID
                );
            }
        }
    }

    #[test]
    fn get_predefined_color_should_return_invalid_predefined_color_for_bright_colors_above_99() {
        assert_eq!(get_predefined_color_2_bytes(create_simd_from_str(BRIGHT_BLACK_BACKGROUND_CODE)), INVALID);
        assert_eq!(get_predefined_color_2_bytes(create_simd_from_str(BRIGHT_RED_BACKGROUND_CODE)), INVALID);
        assert_eq!(get_predefined_color_2_bytes(create_simd_from_str(BRIGHT_GREEN_BACKGROUND_CODE)), INVALID);
        assert_eq!(get_predefined_color_2_bytes(create_simd_from_str(BRIGHT_YELLOW_BACKGROUND_CODE)), INVALID);
        assert_eq!(get_predefined_color_2_bytes(create_simd_from_str(BRIGHT_BLUE_BACKGROUND_CODE)), INVALID);
        assert_eq!(get_predefined_color_2_bytes(create_simd_from_str(BRIGHT_MAGENTA_BACKGROUND_CODE)), INVALID);
        assert_eq!(get_predefined_color_2_bytes(create_simd_from_str(BRIGHT_CYAN_BACKGROUND_CODE)), INVALID);
        assert_eq!(get_predefined_color_2_bytes(create_simd_from_str(BRIGHT_WHITE_BACKGROUND_CODE)), INVALID);
    }


    // ----------------- SIMD -----------------

    fn create_valid_result_for_value(value: u8) -> (Mask<i8, 32>, Simd<u8, 32>) {
        (
            Mask::from_array([true; 32]),
            Simd::<u8, 32>::load_or_default(&[
                PARSE_GRAPHICS_MODE_PREDEFINED_COLOR_TYPE,
                SIZE,
                1,
                value,
            ])
        )
    }

    #[test]
    fn get_predefined_color_simd_should_support_all_predefined_colors() {
        // let input = create_simd_from_str(RED_FOREGROUND_CODE);
        // let actual = get_predefined_color_2_bytes_simd(input);
        // let expected = create_valid_result_for_value(31);
        //
        // println!("input: {:?}", input.to_array());
        // println!("actual: {:?}", actual.1.to_array());
        // println!("expected: {:?}", expected.1.to_array());
        // assert_eq!(actual, expected);
        assert_eq!(get_predefined_color_2_bytes_simd(create_simd_from_str(BLACK_FOREGROUND_CODE)), create_valid_result_for_value(30));
        assert_eq!(get_predefined_color_2_bytes_simd(create_simd_from_str(BLACK_BACKGROUND_CODE)), create_valid_result_for_value(40));

        assert_eq!(get_predefined_color_2_bytes_simd(create_simd_from_str(RED_FOREGROUND_CODE)), create_valid_result_for_value(31));
        assert_eq!(get_predefined_color_2_bytes_simd(create_simd_from_str(RED_BACKGROUND_CODE)), create_valid_result_for_value(41));

        assert_eq!(get_predefined_color_2_bytes_simd(create_simd_from_str(GREEN_FOREGROUND_CODE)), create_valid_result_for_value(32));
        assert_eq!(get_predefined_color_2_bytes_simd(create_simd_from_str(GREEN_BACKGROUND_CODE)), create_valid_result_for_value(42));

        assert_eq!(get_predefined_color_2_bytes_simd(create_simd_from_str(YELLOW_FOREGROUND_CODE)), create_valid_result_for_value(33));
        assert_eq!(get_predefined_color_2_bytes_simd(create_simd_from_str(YELLOW_BACKGROUND_CODE)), create_valid_result_for_value(43));

        assert_eq!(get_predefined_color_2_bytes_simd(create_simd_from_str(BLUE_FOREGROUND_CODE)), create_valid_result_for_value(34));
        assert_eq!(get_predefined_color_2_bytes_simd(create_simd_from_str(BLUE_BACKGROUND_CODE)), create_valid_result_for_value(44));

        assert_eq!(get_predefined_color_2_bytes_simd(create_simd_from_str(MAGENTA_FOREGROUND_CODE)), create_valid_result_for_value(35));
        assert_eq!(get_predefined_color_2_bytes_simd(create_simd_from_str(MAGENTA_BACKGROUND_CODE)), create_valid_result_for_value(45));

        assert_eq!(get_predefined_color_2_bytes_simd(create_simd_from_str(CYAN_FOREGROUND_CODE)), create_valid_result_for_value(36));
        assert_eq!(get_predefined_color_2_bytes_simd(create_simd_from_str(CYAN_BACKGROUND_CODE)), create_valid_result_for_value(46));

        assert_eq!(get_predefined_color_2_bytes_simd(create_simd_from_str(WHITE_FOREGROUND_CODE)), create_valid_result_for_value(37));
        assert_eq!(get_predefined_color_2_bytes_simd(create_simd_from_str(WHITE_BACKGROUND_CODE)), create_valid_result_for_value(47));

        assert_eq!(get_predefined_color_2_bytes_simd(create_simd_from_str(DEFAULT_FOREGROUND_CODE)), create_valid_result_for_value(39));
        assert_eq!(get_predefined_color_2_bytes_simd(create_simd_from_str(DEFAULT_BACKGROUND_CODE)), create_valid_result_for_value(49));

        assert_eq!(get_predefined_color_2_bytes_simd(create_simd_from_str(BRIGHT_BLACK_FOREGROUND_CODE)), create_valid_result_for_value(90));
        assert_eq!(get_predefined_color_2_bytes_simd(create_simd_from_str(BRIGHT_RED_FOREGROUND_CODE)), create_valid_result_for_value(91));
        assert_eq!(get_predefined_color_2_bytes_simd(create_simd_from_str(BRIGHT_GREEN_FOREGROUND_CODE)), create_valid_result_for_value(92));
        assert_eq!(get_predefined_color_2_bytes_simd(create_simd_from_str(BRIGHT_YELLOW_FOREGROUND_CODE)), create_valid_result_for_value(93));
        assert_eq!(get_predefined_color_2_bytes_simd(create_simd_from_str(BRIGHT_BLUE_FOREGROUND_CODE)), create_valid_result_for_value(94));
        assert_eq!(get_predefined_color_2_bytes_simd(create_simd_from_str(BRIGHT_MAGENTA_FOREGROUND_CODE)), create_valid_result_for_value(95));
        assert_eq!(get_predefined_color_2_bytes_simd(create_simd_from_str(BRIGHT_CYAN_FOREGROUND_CODE)), create_valid_result_for_value(96));
        assert_eq!(get_predefined_color_2_bytes_simd(create_simd_from_str(BRIGHT_WHITE_FOREGROUND_CODE)), create_valid_result_for_value(97));

    }

    #[test]
    fn get_predefined_color_simd_should_return_color_even_if_have_other_bytes_after_color() {
        for num in 30..=49 {
            let byte1 = b'0' + num / 10;
            let byte2 = b'0' + num % 10;

            assert_eq!(
                get_predefined_color_2_bytes_simd(Simd::<u8, LANES>::load_or_default(&[b'\x1b', b'[', byte1, byte2, b'm', b'h', b'e', b'l', b'l', b'o'])),
                create_valid_result_for_value(num)
            );
        }
    }

    #[test]
    fn get_predefined_color_simd_should_return_invalid_for_without_correct_structure() {
        assert_eq!(
            // should have been another number between after 3
            get_predefined_color_2_bytes_simd(create_simd_from_str("\x1b[3m")).0,
            Mask::from_array([false; 32])
        );
        assert_eq!(
            // should have '[' and not ']'
            get_predefined_color_2_bytes_simd(create_simd_from_str("\x1b]39m")).0,
            Mask::from_array([false; 32])
        );
        assert_eq!(
            // should have '\x1b' and not 'a'
            get_predefined_color_2_bytes_simd(create_simd_from_str("a[39m")).0,
            Mask::from_array([false; 32])
        );
        assert_eq!(
            // should have m in the end
            get_predefined_color_2_bytes_simd(create_simd_from_str("\x1b[39")).0,
            Mask::from_array([false; 32])
        );
        assert_eq!(
            // must not be empty
            get_predefined_color_2_bytes_simd(create_simd_from_str("")).0,
            Mask::from_array([false; 32])
        );
        assert_eq!(
            // the escape code should be in the beginning
            get_predefined_color_2_bytes_simd(create_simd_from_str("0\x1b[39m")).0,
            Mask::from_array([false; 32])
        );
    }

    #[test]
    fn get_predefined_color_simd_should_return_invalid_predefined_color_when_not_ascii_number_in_the_number_position() {
        for byte1 in 0..=255u8 {
            for byte2 in 0..=255u8 {
                // Ignore the ascii numbers between 30 and 49 or 90 and 99
                if (byte1 == b'3' || byte1 == b'4' || byte1 == b'9') && (byte2 >= b'0' && byte2 <= b'9') {
                    continue;
                }

                assert_eq!(
                    get_predefined_color_2_bytes_simd(Simd::<u8, LANES>::load_or_default(&[b'\x1b', b'[', byte1, byte2, b'm'])).0,
                    Mask::from_array([false; 32])
                );
            }
        }
    }

    #[test]
    fn get_predefined_color_simd_should_return_invalid_predefined_color_for_bright_colors_above_99() {

        let invalid_mask = Mask::from_array([false; 32]);

        assert_eq!(get_predefined_color_2_bytes_simd(create_simd_from_str(BRIGHT_BLACK_BACKGROUND_CODE)).0, invalid_mask);
        assert_eq!(get_predefined_color_2_bytes_simd(create_simd_from_str(BRIGHT_RED_BACKGROUND_CODE)).0, invalid_mask);
        assert_eq!(get_predefined_color_2_bytes_simd(create_simd_from_str(BRIGHT_GREEN_BACKGROUND_CODE)).0, invalid_mask);
        assert_eq!(get_predefined_color_2_bytes_simd(create_simd_from_str(BRIGHT_YELLOW_BACKGROUND_CODE)).0, invalid_mask);
        assert_eq!(get_predefined_color_2_bytes_simd(create_simd_from_str(BRIGHT_BLUE_BACKGROUND_CODE)).0, invalid_mask);
        assert_eq!(get_predefined_color_2_bytes_simd(create_simd_from_str(BRIGHT_MAGENTA_BACKGROUND_CODE)).0, invalid_mask);
        assert_eq!(get_predefined_color_2_bytes_simd(create_simd_from_str(BRIGHT_CYAN_BACKGROUND_CODE)).0, invalid_mask);
        assert_eq!(get_predefined_color_2_bytes_simd(create_simd_from_str(BRIGHT_WHITE_BACKGROUND_CODE)).0, invalid_mask);
    }

    // ----------------- u64 -----------------

    fn create_valid_result_for_value_u64(value: u8) -> u64 {
        let value_in_correct_position = (value as u64) << 32;

        return 0x02_05_01_00_00_00_00_00 | value_in_correct_position
    }

    #[test]
    fn get_predefined_color_u64_should_support_all_predefined_colors() {
        assert_eq!(get_predefined_color_2_bytes_u64(str_to_u64(BLACK_FOREGROUND_CODE)), create_valid_result_for_value_u64(30));
        assert_eq!(get_predefined_color_2_bytes_u64(str_to_u64(BLACK_BACKGROUND_CODE)), create_valid_result_for_value_u64(40));

        assert_eq!(get_predefined_color_2_bytes_u64(str_to_u64(RED_FOREGROUND_CODE)), create_valid_result_for_value_u64(31));
        assert_eq!(get_predefined_color_2_bytes_u64(str_to_u64(RED_BACKGROUND_CODE)), create_valid_result_for_value_u64(41));

        assert_eq!(get_predefined_color_2_bytes_u64(str_to_u64(GREEN_FOREGROUND_CODE)), create_valid_result_for_value_u64(32));
        assert_eq!(get_predefined_color_2_bytes_u64(str_to_u64(GREEN_BACKGROUND_CODE)), create_valid_result_for_value_u64(42));

        assert_eq!(get_predefined_color_2_bytes_u64(str_to_u64(YELLOW_FOREGROUND_CODE)), create_valid_result_for_value_u64(33));
        assert_eq!(get_predefined_color_2_bytes_u64(str_to_u64(YELLOW_BACKGROUND_CODE)), create_valid_result_for_value_u64(43));

        assert_eq!(get_predefined_color_2_bytes_u64(str_to_u64(BLUE_FOREGROUND_CODE)), create_valid_result_for_value_u64(34));
        assert_eq!(get_predefined_color_2_bytes_u64(str_to_u64(BLUE_BACKGROUND_CODE)), create_valid_result_for_value_u64(44));

        assert_eq!(get_predefined_color_2_bytes_u64(str_to_u64(MAGENTA_FOREGROUND_CODE)), create_valid_result_for_value_u64(35));
        assert_eq!(get_predefined_color_2_bytes_u64(str_to_u64(MAGENTA_BACKGROUND_CODE)), create_valid_result_for_value_u64(45));

        assert_eq!(get_predefined_color_2_bytes_u64(str_to_u64(CYAN_FOREGROUND_CODE)), create_valid_result_for_value_u64(36));
        assert_eq!(get_predefined_color_2_bytes_u64(str_to_u64(CYAN_BACKGROUND_CODE)), create_valid_result_for_value_u64(46));

        assert_eq!(get_predefined_color_2_bytes_u64(str_to_u64(WHITE_FOREGROUND_CODE)), create_valid_result_for_value_u64(37));
        assert_eq!(get_predefined_color_2_bytes_u64(str_to_u64(WHITE_BACKGROUND_CODE)), create_valid_result_for_value_u64(47));

        assert_eq!(get_predefined_color_2_bytes_u64(str_to_u64(DEFAULT_FOREGROUND_CODE)), create_valid_result_for_value_u64(39));
        assert_eq!(get_predefined_color_2_bytes_u64(str_to_u64(DEFAULT_BACKGROUND_CODE)), create_valid_result_for_value_u64(49));

        assert_eq!(get_predefined_color_2_bytes_u64(str_to_u64(BRIGHT_BLACK_FOREGROUND_CODE)), create_valid_result_for_value_u64(90));
        assert_eq!(get_predefined_color_2_bytes_u64(str_to_u64(BRIGHT_RED_FOREGROUND_CODE)), create_valid_result_for_value_u64(91));
        assert_eq!(get_predefined_color_2_bytes_u64(str_to_u64(BRIGHT_GREEN_FOREGROUND_CODE)), create_valid_result_for_value_u64(92));
        assert_eq!(get_predefined_color_2_bytes_u64(str_to_u64(BRIGHT_YELLOW_FOREGROUND_CODE)), create_valid_result_for_value_u64(93));
        assert_eq!(get_predefined_color_2_bytes_u64(str_to_u64(BRIGHT_BLUE_FOREGROUND_CODE)), create_valid_result_for_value_u64(94));
        assert_eq!(get_predefined_color_2_bytes_u64(str_to_u64(BRIGHT_MAGENTA_FOREGROUND_CODE)), create_valid_result_for_value_u64(95));
        assert_eq!(get_predefined_color_2_bytes_u64(str_to_u64(BRIGHT_CYAN_FOREGROUND_CODE)), create_valid_result_for_value_u64(96));
        assert_eq!(get_predefined_color_2_bytes_u64(str_to_u64(BRIGHT_WHITE_FOREGROUND_CODE)), create_valid_result_for_value_u64(97));
    }

    #[test]
    fn get_predefined_color_u64_should_return_color_even_if_have_other_bytes_after_color() {
        for num in 30..=49 {
            let byte1 = b'0' + num / 10;
            let byte2 = b'0' + num % 10;

            assert_eq!(
                get_predefined_color_2_bytes_u64(u8_array_to_u64([b'\x1b', b'[', byte1, byte2, b'm', b'h', b'e', b'l'])),
                create_valid_result_for_value_u64(num)
            );
        }
    }

    #[test]
    fn get_predefined_color_u64_should_return_invalid_for_without_correct_structure() {
        assert_eq!(
            // should have been another number between after 3
            get_predefined_color_2_bytes_u64(str_to_u64("\x1b[3m")),
            0
        );
        assert_eq!(
            // should have '[' and not ']'
            get_predefined_color_2_bytes_u64(str_to_u64("\x1b]39m")),
            0
        );
        assert_eq!(
            // should have '\x1b' and not 'a'
            get_predefined_color_2_bytes_u64(str_to_u64("a[39m")),
            0
        );
        assert_eq!(
            // should have m in the end
            get_predefined_color_2_bytes_u64(str_to_u64("\x1b[39")),
            0
        );
        assert_eq!(
            // must not be empty
            get_predefined_color_2_bytes_u64(str_to_u64("")),
            0
        );
        assert_eq!(
            // the escape code should be in the beginning
            get_predefined_color_2_bytes_u64(str_to_u64("0\x1b[39m")),
            0
        );
    }

    #[test]
    fn get_predefined_color_u64_should_return_invalid_predefined_color_when_not_ascii_number_in_the_number_position() {
        for byte1 in 0..=255u8 {
            for byte2 in 0..=255u8 {
                // Ignore the ascii numbers between 30 and 49 or 90 and 99
                if (byte1 == b'3' || byte1 == b'4' || byte1 == b'9') && (byte2 >= b'0' && byte2 <= b'9') {
                    continue;
                }

                assert_eq!(
                    get_predefined_color_2_bytes_u64(u8_array_to_u64([b'\x1b', b'[', byte1, byte2, b'm', 0, 0, 0])),
                    0
                );
            }
        }
    }

    #[test]
    fn get_predefined_color_u64_should_return_invalid_predefined_color_for_bright_colors_above_99() {
        assert_eq!(get_predefined_color_2_bytes_u64(str_to_u64(BRIGHT_BLACK_BACKGROUND_CODE)), 0);
        assert_eq!(get_predefined_color_2_bytes_u64(str_to_u64(BRIGHT_RED_BACKGROUND_CODE)), 0);
        assert_eq!(get_predefined_color_2_bytes_u64(str_to_u64(BRIGHT_GREEN_BACKGROUND_CODE)), 0);
        assert_eq!(get_predefined_color_2_bytes_u64(str_to_u64(BRIGHT_YELLOW_BACKGROUND_CODE)), 0);
        assert_eq!(get_predefined_color_2_bytes_u64(str_to_u64(BRIGHT_BLUE_BACKGROUND_CODE)), 0);
        assert_eq!(get_predefined_color_2_bytes_u64(str_to_u64(BRIGHT_MAGENTA_BACKGROUND_CODE)), 0);
        assert_eq!(get_predefined_color_2_bytes_u64(str_to_u64(BRIGHT_CYAN_BACKGROUND_CODE)), 0);
        assert_eq!(get_predefined_color_2_bytes_u64(str_to_u64(BRIGHT_WHITE_BACKGROUND_CODE)), 0);
    }


}
