use std::ops::{BitAnd, BitOr, Div, Index, Mul};
use std::simd::cmp::{SimdPartialEq, SimdPartialOrd};
use std::simd::num::SimdUint;
use std::simd::{Mask, Simd};
use crate::parse_ansi_text::raw_ansi_parse::parsers_only_text_and_graphics_manual_simd::simd_parsers::helpers::{AllOrNone, build_graphics_mode_result};
use crate::parse_ansi_text::raw_ansi_parse::parsers_only_text_and_graphics_manual_simd::simd_parsers::parse_predefined_colors::PARSE_GRAPHICS_MODE_PREDEFINED_COLOR_TYPE;

const LANES: usize = 32;

// Predefined colors here are between 100 and 107
pub const INVALID: u8 = 255;
// b'\x1b[100m' or other number instead of 100
pub const SIZE: u8 = 6;

const MASK: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([
    255, // b'\x1b',
    255, //b'[',
    255, // Everything
    255, // Everything
    255, // Everything
    255, //b'm',

    // Empty
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0,
]);

const MIN_MASK: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([
    b'\x1b',
    b'[',
    b'1', // Everything
    b'0', // Everything
    b'0', // Everything
    b'm',

    // Empty
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0,
]);

const MAX_MASK: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([
    b'\x1b',
    b'[',
    b'1', // Everything
    b'0', // Everything
    b'7', // Everything
    b'm',

    // Empty
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0,
]);


// INVALID_PREDEFINED_COLOR_3_BYTES for invalid, otherwise the number, only support bright colors above 99 to have fixed size
pub fn get_predefined_color_3_bytes(bytes: Simd<u8, LANES>) -> u8 {
    let only_relevant_part = bytes & MASK;

    let is_predefined_color = only_relevant_part.simd_ge(MIN_MASK).all() && only_relevant_part.simd_le(MAX_MASK).all();
    if !is_predefined_color {
        return INVALID;
    }

    // 2 as we want to get the number after b"\x1b[",
    let first_digit = only_relevant_part.index(2);
    let second_digit = only_relevant_part.index(3);
    let third_digit = only_relevant_part.index(4);

    // Get the number from the ascii
    return (first_digit - b'0') * 100 + (second_digit - b'0') * 10 + (third_digit - b'0');
}

pub fn get_predefined_color_3_bytes_simd(bytes: Simd<u8, LANES>) -> (Mask::<i8, 32>, Simd::<u8, 32>) {
    let only_relevant_part = bytes & MASK;

    // merge the two masks and check if all the lanes are true
    let valid_mask: Mask<i8, 32> = only_relevant_part.simd_ge(MIN_MASK)
        .bitand(only_relevant_part.simd_le(MAX_MASK))
        .all_or_none();

    // 2 as we want to get the number after b"\x1b[",
    let first_digit = only_relevant_part.index(2);
    let second_digit = only_relevant_part.index(3);
    let third_digit = only_relevant_part.index(4);

    // Get the number from the ascii (using saturating_<ops> to avoid overflow/underflow)
    // This is basically: (first_digit - b'0') * 100 + (second_digit - b'0') * 10 + (third_digit - b'0')
    let number = first_digit
        .saturating_sub(b'0')
        .saturating_mul(100)
        .saturating_add(
            second_digit
                .saturating_sub(b'0')
                .saturating_mul(10)
        )
        .saturating_add(
            third_digit
                .saturating_sub(b'0')
        );

    return (
        valid_mask,
        build_graphics_mode_result!(
            PARSE_GRAPHICS_MODE_PREDEFINED_COLOR_TYPE,
            SIZE,
            1, // one byte for the number
            number,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0
        )
    );
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::parse_ansi_text::ansi::colors::*;

    use super::*;

    fn create_simd_from_str(input: &str) -> Simd<u8, LANES> {
        Simd::<u8, LANES>::load_or_default(input.as_bytes())
    }


    #[test]
    fn get_predefined_color_3_bytes_should_support_all_predefined_colors() {
        assert_eq!(get_predefined_color_3_bytes(create_simd_from_str(BRIGHT_BLACK_BACKGROUND_CODE)), 100);
        assert_eq!(get_predefined_color_3_bytes(create_simd_from_str(BRIGHT_RED_BACKGROUND_CODE)), 101);
        assert_eq!(get_predefined_color_3_bytes(create_simd_from_str(BRIGHT_GREEN_BACKGROUND_CODE)), 102);
        assert_eq!(get_predefined_color_3_bytes(create_simd_from_str(BRIGHT_YELLOW_BACKGROUND_CODE)), 103);
        assert_eq!(get_predefined_color_3_bytes(create_simd_from_str(BRIGHT_BLUE_BACKGROUND_CODE)), 104);
        assert_eq!(get_predefined_color_3_bytes(create_simd_from_str(BRIGHT_MAGENTA_BACKGROUND_CODE)), 105);
        assert_eq!(get_predefined_color_3_bytes(create_simd_from_str(BRIGHT_CYAN_BACKGROUND_CODE)), 106);
        assert_eq!(get_predefined_color_3_bytes(create_simd_from_str(BRIGHT_WHITE_BACKGROUND_CODE)), 107);

    }

    #[test]
    fn get_predefined_color_3_bytes_should_support_all_predefined_colors_between_100_and_107() {
        for num in 100..=107 {
            let byte = b'0' + (num - 100);
            assert_eq!(
                get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(&[b'\x1b', b'[', b'1', b'0', byte, b'm'])),
                num
            );
        }
    }

    #[test]
    fn get_predefined_color_3_bytes_should_return_color_even_if_have_other_bytes_after_color() {
        for num in 100..=107 {
            let byte = b'0' + (num - 100);
            assert_eq!(
                get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(&[b'\x1b', b'[', b'1', b'0', byte, b'm', b'm', b'h', b'e', b'l', b'l', b'o'])),
                num
            );
        }
    }

    #[test]
    fn get_predefined_color_should_return_invalid_for_without_correct_structure() {
        assert_eq!(
            // should have been another 2 numbers between after 1
            get_predefined_color_3_bytes(create_simd_from_str("\x1b[1m")),
            INVALID
        );
        assert_eq!(
            // should have been another number between after 10
            get_predefined_color_3_bytes(create_simd_from_str("\x1b[10m")),
            INVALID
        );
        assert_eq!(
            // should have '[' and not ']'
            get_predefined_color_3_bytes(create_simd_from_str("\x1b]100m")),
            INVALID
        );
        assert_eq!(
            // should have '\x1b' and not 'a'
            get_predefined_color_3_bytes(create_simd_from_str("a[100m")),
            INVALID
        );
        assert_eq!(
            // should have m in the end
            get_predefined_color_3_bytes(create_simd_from_str("\x1b[100")),
            INVALID
        );
        assert_eq!(
            // must not be empty
            get_predefined_color_3_bytes(create_simd_from_str("")),
            INVALID
        );
        assert_eq!(
            // the escape code should be in the beginning
            get_predefined_color_3_bytes(create_simd_from_str("0\x1b[100m")),
            INVALID
        );
    }

    #[test]
    fn get_predefined_color_3_bytes_should_return_invalid_predefined_color_when_not_ascii_number_in_the_number_position() {
        for byte1 in 0..=255u8 {
            for byte2 in 0..=255u8 {
                for byte3 in 0..=255u8 {
                    // Ignore the ascii numbers between 100 and 107
                    if byte1 == b'1' && byte2 == b'0' && (byte3 >= b'0' && byte3 <= b'7') {
                        continue;
                    }

                    // Doing like this and not with load_or_default as it is much slower
                    let bytes = Simd::<u8, LANES>::from_slice(&[
                        b'\x1b', b'[', byte1, byte2, byte3, b'm',

                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0,
                    ]);

                    assert_eq!(
                        get_predefined_color_3_bytes(bytes),
                        INVALID
                    );
                }
            }
        }
    }

    #[test]
    fn get_predefined_color_3_bytes_should_return_invalid_predefined_color_for_bright_colors_below_100() {
        assert_eq!(get_predefined_color_3_bytes(create_simd_from_str(BLACK_FOREGROUND_CODE)), INVALID);
        assert_eq!(get_predefined_color_3_bytes(create_simd_from_str(BLACK_BACKGROUND_CODE)), INVALID);

        assert_eq!(get_predefined_color_3_bytes(create_simd_from_str(RED_FOREGROUND_CODE)), INVALID);
        assert_eq!(get_predefined_color_3_bytes(create_simd_from_str(RED_BACKGROUND_CODE)), INVALID);

        assert_eq!(get_predefined_color_3_bytes(create_simd_from_str(GREEN_FOREGROUND_CODE)), INVALID);
        assert_eq!(get_predefined_color_3_bytes(create_simd_from_str(GREEN_BACKGROUND_CODE)), INVALID);

        assert_eq!(get_predefined_color_3_bytes(create_simd_from_str(YELLOW_FOREGROUND_CODE)), INVALID);
        assert_eq!(get_predefined_color_3_bytes(create_simd_from_str(YELLOW_BACKGROUND_CODE)), INVALID);

        assert_eq!(get_predefined_color_3_bytes(create_simd_from_str(BLUE_FOREGROUND_CODE)), INVALID);
        assert_eq!(get_predefined_color_3_bytes(create_simd_from_str(BLUE_BACKGROUND_CODE)), INVALID);

        assert_eq!(get_predefined_color_3_bytes(create_simd_from_str(MAGENTA_FOREGROUND_CODE)), INVALID);
        assert_eq!(get_predefined_color_3_bytes(create_simd_from_str(MAGENTA_BACKGROUND_CODE)), INVALID);

        assert_eq!(get_predefined_color_3_bytes(create_simd_from_str(CYAN_FOREGROUND_CODE)), INVALID);
        assert_eq!(get_predefined_color_3_bytes(create_simd_from_str(CYAN_BACKGROUND_CODE)), INVALID);

        assert_eq!(get_predefined_color_3_bytes(create_simd_from_str(WHITE_FOREGROUND_CODE)), INVALID);
        assert_eq!(get_predefined_color_3_bytes(create_simd_from_str(WHITE_BACKGROUND_CODE)), INVALID);

        assert_eq!(get_predefined_color_3_bytes(create_simd_from_str(DEFAULT_FOREGROUND_CODE)), INVALID);
        assert_eq!(get_predefined_color_3_bytes(create_simd_from_str(DEFAULT_BACKGROUND_CODE)), INVALID);

        assert_eq!(get_predefined_color_3_bytes(create_simd_from_str(BRIGHT_BLACK_FOREGROUND_CODE)), INVALID);
        assert_eq!(get_predefined_color_3_bytes(create_simd_from_str(BRIGHT_RED_FOREGROUND_CODE)), INVALID);
        assert_eq!(get_predefined_color_3_bytes(create_simd_from_str(BRIGHT_GREEN_FOREGROUND_CODE)), INVALID);
        assert_eq!(get_predefined_color_3_bytes(create_simd_from_str(BRIGHT_YELLOW_FOREGROUND_CODE)), INVALID);
        assert_eq!(get_predefined_color_3_bytes(create_simd_from_str(BRIGHT_BLUE_FOREGROUND_CODE)), INVALID);
        assert_eq!(get_predefined_color_3_bytes(create_simd_from_str(BRIGHT_MAGENTA_FOREGROUND_CODE)), INVALID);
        assert_eq!(get_predefined_color_3_bytes(create_simd_from_str(BRIGHT_CYAN_FOREGROUND_CODE)), INVALID);
        assert_eq!(get_predefined_color_3_bytes(create_simd_from_str(BRIGHT_WHITE_FOREGROUND_CODE)), INVALID);
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
    fn get_predefined_color_3_bytes_simd_should_support_all_predefined_colors() {
        assert_eq!(get_predefined_color_3_bytes_simd(create_simd_from_str(BRIGHT_BLACK_BACKGROUND_CODE)), create_valid_result_for_value(100));
        assert_eq!(get_predefined_color_3_bytes_simd(create_simd_from_str(BRIGHT_RED_BACKGROUND_CODE)), create_valid_result_for_value(101));
        assert_eq!(get_predefined_color_3_bytes_simd(create_simd_from_str(BRIGHT_GREEN_BACKGROUND_CODE)), create_valid_result_for_value(102));
        assert_eq!(get_predefined_color_3_bytes_simd(create_simd_from_str(BRIGHT_YELLOW_BACKGROUND_CODE)), create_valid_result_for_value(103));
        assert_eq!(get_predefined_color_3_bytes_simd(create_simd_from_str(BRIGHT_BLUE_BACKGROUND_CODE)), create_valid_result_for_value(104));
        assert_eq!(get_predefined_color_3_bytes_simd(create_simd_from_str(BRIGHT_MAGENTA_BACKGROUND_CODE)), create_valid_result_for_value(105));
        assert_eq!(get_predefined_color_3_bytes_simd(create_simd_from_str(BRIGHT_CYAN_BACKGROUND_CODE)), create_valid_result_for_value(106));
        assert_eq!(get_predefined_color_3_bytes_simd(create_simd_from_str(BRIGHT_WHITE_BACKGROUND_CODE)), create_valid_result_for_value(107));

    }

    #[test]
    fn get_predefined_color_3_bytes_simd_should_support_all_predefined_colors_between_100_and_107() {
        for num in 100..=107 {
            let byte = b'0' + (num - 100);
            assert_eq!(
                get_predefined_color_3_bytes_simd(Simd::<u8, LANES>::load_or_default(&[b'\x1b', b'[', b'1', b'0', byte, b'm'])),
                create_valid_result_for_value(num)
            );
        }
    }

    #[test]
    fn get_predefined_color_3_bytes_simd_should_return_color_even_if_have_other_bytes_after_color() {
        for num in 100..=107 {
            let byte = b'0' + (num - 100);
            assert_eq!(
                get_predefined_color_3_bytes_simd(Simd::<u8, LANES>::load_or_default(&[b'\x1b', b'[', b'1', b'0', byte, b'm', b'm', b'h', b'e', b'l', b'l', b'o'])),
                create_valid_result_for_value(num)
            );
        }
    }

    #[test]
    fn get_predefined_color_simd_should_return_invalid_for_without_correct_structure() {
        let invalid_result = Mask::from_array([false; 32]);
        assert_eq!(
            // should have been another 2 numbers between after 1
            get_predefined_color_3_bytes_simd(create_simd_from_str("\x1b[1m")).0,
            invalid_result
        );
        assert_eq!(
            // should have been another number between after 10
            get_predefined_color_3_bytes_simd(create_simd_from_str("\x1b[10m")).0,
            invalid_result
        );
        assert_eq!(
            // should have '[' and not ']'
            get_predefined_color_3_bytes_simd(create_simd_from_str("\x1b]100m")).0,
            invalid_result
        );
        assert_eq!(
            // should have '\x1b' and not 'a'
            get_predefined_color_3_bytes_simd(create_simd_from_str("a[100m")).0,
            invalid_result
        );
        assert_eq!(
            // should have m in the end
            get_predefined_color_3_bytes_simd(create_simd_from_str("\x1b[100")).0,
            invalid_result
        );
        assert_eq!(
            // must not be empty
            get_predefined_color_3_bytes_simd(create_simd_from_str("")).0,
            invalid_result
        );
        assert_eq!(
            // the escape code should be in the beginning
            get_predefined_color_3_bytes_simd(create_simd_from_str("0\x1b[100m")).0,
            invalid_result
        );
    }

    #[test]
    fn get_predefined_color_3_bytes_simd_should_return_invalid_predefined_color_when_not_ascii_number_in_the_number_position() {
        let invalid_result = Mask::from_array([false; 32]);

        for byte1 in 0..=255u8 {
            for byte2 in 0..=255u8 {
                for byte3 in 0..=255u8 {
                    // Ignore the ascii numbers between 100 and 107
                    if byte1 == b'1' && byte2 == b'0' && (byte3 >= b'0' && byte3 <= b'7') {
                        continue;
                    }

                    // Doing like this and not with load_or_default as it is much slower
                    let bytes = Simd::<u8, LANES>::from_slice(&[
                        b'\x1b', b'[', byte1, byte2, byte3, b'm',

                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0,
                    ]);

                    assert_eq!(
                        get_predefined_color_3_bytes_simd(bytes).0,
                        invalid_result
                    );
                }
            }
        }
    }

    #[test]
    fn get_predefined_color_3_bytes_simd_should_return_invalid_predefined_color_for_bright_colors_below_100() {
        let invalid_result = Mask::from_array([false; 32]);

        assert_eq!(get_predefined_color_3_bytes_simd(create_simd_from_str(BLACK_FOREGROUND_CODE)).0, invalid_result);
        assert_eq!(get_predefined_color_3_bytes_simd(create_simd_from_str(BLACK_BACKGROUND_CODE)).0, invalid_result);

        assert_eq!(get_predefined_color_3_bytes_simd(create_simd_from_str(RED_FOREGROUND_CODE)).0, invalid_result);
        assert_eq!(get_predefined_color_3_bytes_simd(create_simd_from_str(RED_BACKGROUND_CODE)).0, invalid_result);

        assert_eq!(get_predefined_color_3_bytes_simd(create_simd_from_str(GREEN_FOREGROUND_CODE)).0, invalid_result);
        assert_eq!(get_predefined_color_3_bytes_simd(create_simd_from_str(GREEN_BACKGROUND_CODE)).0, invalid_result);

        assert_eq!(get_predefined_color_3_bytes_simd(create_simd_from_str(YELLOW_FOREGROUND_CODE)).0, invalid_result);
        assert_eq!(get_predefined_color_3_bytes_simd(create_simd_from_str(YELLOW_BACKGROUND_CODE)).0, invalid_result);

        assert_eq!(get_predefined_color_3_bytes_simd(create_simd_from_str(BLUE_FOREGROUND_CODE)).0, invalid_result);
        assert_eq!(get_predefined_color_3_bytes_simd(create_simd_from_str(BLUE_BACKGROUND_CODE)).0, invalid_result);

        assert_eq!(get_predefined_color_3_bytes_simd(create_simd_from_str(MAGENTA_FOREGROUND_CODE)).0, invalid_result);
        assert_eq!(get_predefined_color_3_bytes_simd(create_simd_from_str(MAGENTA_BACKGROUND_CODE)).0, invalid_result);

        assert_eq!(get_predefined_color_3_bytes_simd(create_simd_from_str(CYAN_FOREGROUND_CODE)).0, invalid_result);
        assert_eq!(get_predefined_color_3_bytes_simd(create_simd_from_str(CYAN_BACKGROUND_CODE)).0, invalid_result);

        assert_eq!(get_predefined_color_3_bytes_simd(create_simd_from_str(WHITE_FOREGROUND_CODE)).0, invalid_result);
        assert_eq!(get_predefined_color_3_bytes_simd(create_simd_from_str(WHITE_BACKGROUND_CODE)).0, invalid_result);

        assert_eq!(get_predefined_color_3_bytes_simd(create_simd_from_str(DEFAULT_FOREGROUND_CODE)).0, invalid_result);
        assert_eq!(get_predefined_color_3_bytes_simd(create_simd_from_str(DEFAULT_BACKGROUND_CODE)).0, invalid_result);

        assert_eq!(get_predefined_color_3_bytes_simd(create_simd_from_str(BRIGHT_BLACK_FOREGROUND_CODE)).0, invalid_result);
        assert_eq!(get_predefined_color_3_bytes_simd(create_simd_from_str(BRIGHT_RED_FOREGROUND_CODE)).0, invalid_result);
        assert_eq!(get_predefined_color_3_bytes_simd(create_simd_from_str(BRIGHT_GREEN_FOREGROUND_CODE)).0, invalid_result);
        assert_eq!(get_predefined_color_3_bytes_simd(create_simd_from_str(BRIGHT_YELLOW_FOREGROUND_CODE)).0, invalid_result);
        assert_eq!(get_predefined_color_3_bytes_simd(create_simd_from_str(BRIGHT_BLUE_FOREGROUND_CODE)).0, invalid_result);
        assert_eq!(get_predefined_color_3_bytes_simd(create_simd_from_str(BRIGHT_MAGENTA_FOREGROUND_CODE)).0, invalid_result);
        assert_eq!(get_predefined_color_3_bytes_simd(create_simd_from_str(BRIGHT_CYAN_FOREGROUND_CODE)).0, invalid_result);
        assert_eq!(get_predefined_color_3_bytes_simd(create_simd_from_str(BRIGHT_WHITE_FOREGROUND_CODE)).0, invalid_result);
    }
}
