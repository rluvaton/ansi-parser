use std::ops::{Add, AddAssign, BitAnd, BitOr, Div, Index, Mul, Sub};
use crate::parse_ansi_text::raw_ansi_parse::parsers_only_text_and_graphics_manual_simd::simd_parsers::parse_predefined_colors::PARSE_GRAPHICS_MODE_PREDEFINED_COLOR_TYPE;

const LANES: usize = 32;

// Predefined colors here are not numbers above 99 and one of the specific colors
pub const INVALID: u8 = 255;
// b'\x1b[38m' or other number instead of 38
pub const SIZE: u8 = 5;


// 255, // b'\x1b',
//     255, //b'[',
//     255, // Everything
//     255, // Everything
//     255, //b'm',
const MASK_SMALL_U64: u64 = 0xFF_FF_FF_FF_00_00_00_00;

// this is b'\x1b', b'[', -, b'm'
const EXPECTED_VALUE_WITHOUT_NUMBER_SMALL_U64: u64 = 0x1B_5B_00_00_6D_00_00_00;
const MASK_WITHOUT_NUMBER_SMALL_U64: u64 = 0xFF_FF_00_00_FF_00_00_00;

// 0, // b'\x1b',
//     0, // b'[',
//     b'0', // b'9', // Everything
//     b'0', // b'9', // Everything
//     0, // b'm',
const SUBTRACT_NUM_TO_U8_SMALL_U64: u64 = 0x00_00_30_30_00_00_00_00;



// 02 for PARSE_GRAPHICS_MODE_PREDEFINED_COLOR_TYPE
// 05 for SIZE
// 01 for value size
// 00 for the value
// rest is empty
const GRAPHICS_MODE_RESULT_SMALL_U64: u64 = 0x02_05_01_00_00_00_00_00;

#[inline(always)]
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
