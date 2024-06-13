use std::ops::{Add, BitAnd, BitOr, Div, Index, IndexMut, Mul, Sub};

const LANES: usize = 32;

// Predefined colors here are between 100 and 107
pub const INVALID: u8 = 255;
// b'\x1b[100m' or other number instead of 100
pub const SIZE: u8 = 6;

// FF, // b'\x1b',
// FF, //b'[',
// FF, // Everything
// FF, // Everything
// FF, // Everything
// FF, //b'm',
const MASK_SMALL_U64: u64 = 0xFF_FF_FF_FF_FF_00_00_00;

// this is b'\x1b', b'[', b'1', b'0', -, b'm'
const EXPECTED_VALUE_WITHOUT_NUMBER_SMALL_U64: u64 = 0x1B_5B_31_30_00_6D_00_00;
const MASK_WITHOUT_NUMBER_SMALL_U64: u64 = 0xFF_FF_FF_FF_00_FF_00_00;



// 0, // PARSE_GRAPHICS_MODE_PREDEFINED_COLOR_TYPE,
//     0, // SIZE,
//     0, // value size
//     255, // the value
const KEEP_VALUE_BYTE_SMALL_U64: u64 = 0x00_00_00_FF_00_00_00_00;


// 02 for PARSE_GRAPHICS_MODE_PREDEFINED_COLOR_TYPE
// 06 for SIZE
// 01 for value size
// 00 for the value
// rest is empty
const GRAPHICS_MODE_RESULT_SMALL_U64: u64 = 0x02_06_01_00_00_00_00_00;


#[inline(always)]
pub fn get_predefined_color_3_bytes_u64(bytes: u64) -> u64 {
    if bytes & MASK_WITHOUT_NUMBER_SMALL_U64 != EXPECTED_VALUE_WITHOUT_NUMBER_SMALL_U64 {
        return 0;
    }

    let first_digit = (bytes & 0x00_00_00_00_FF_00_00_00) >> 24;
    if (first_digit < b'0' as u64) || (first_digit > b'7' as u64) {
        return 0;
    }

    let mut number = first_digit - b'0' as u64 + 100;

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

    fn create_valid_result_for_value_u64(value: u8) -> u64 {
        let value_in_correct_position = (value as u64) << 32;

        return 0x02_06_01_00_00_00_00_00 | value_in_correct_position
    }


    #[test]
    fn get_predefined_color_3_bytes_u64_should_support_all_predefined_colors() {
        assert_eq!(get_predefined_color_3_bytes_u64(str_to_u64(BRIGHT_BLACK_BACKGROUND_CODE)), create_valid_result_for_value_u64(100));
        assert_eq!(get_predefined_color_3_bytes_u64(str_to_u64(BRIGHT_RED_BACKGROUND_CODE)), create_valid_result_for_value_u64(101));
        assert_eq!(get_predefined_color_3_bytes_u64(str_to_u64(BRIGHT_GREEN_BACKGROUND_CODE)), create_valid_result_for_value_u64(102));
        assert_eq!(get_predefined_color_3_bytes_u64(str_to_u64(BRIGHT_YELLOW_BACKGROUND_CODE)), create_valid_result_for_value_u64(103));
        assert_eq!(get_predefined_color_3_bytes_u64(str_to_u64(BRIGHT_BLUE_BACKGROUND_CODE)), create_valid_result_for_value_u64(104));
        assert_eq!(get_predefined_color_3_bytes_u64(str_to_u64(BRIGHT_MAGENTA_BACKGROUND_CODE)), create_valid_result_for_value_u64(105));
        assert_eq!(get_predefined_color_3_bytes_u64(str_to_u64(BRIGHT_CYAN_BACKGROUND_CODE)), create_valid_result_for_value_u64(106));
        assert_eq!(get_predefined_color_3_bytes_u64(str_to_u64(BRIGHT_WHITE_BACKGROUND_CODE)), create_valid_result_for_value_u64(107));

    }

    #[test]
    fn get_predefined_color_3_bytes_u64_should_support_all_predefined_colors_between_100_and_107() {
        for num in 100..=107 {
            let byte = b'0' + (num - 100);
            assert_eq!(
                get_predefined_color_3_bytes_u64(u8_array_to_u64([b'\x1b', b'[', b'1', b'0', byte, b'm', 0, 0])),
                create_valid_result_for_value_u64(num)
            );
        }
    }

    #[test]
    fn get_predefined_color_3_bytes_u64_should_return_color_even_if_have_other_bytes_after_color() {
        for num in 100..=107 {
            let byte = b'0' + (num - 100);
            let actual = get_predefined_color_3_bytes_u64(u8_array_to_u64([b'\x1b', b'[', b'1', b'0', byte, b'm', b'm', b'h']));
            let expected = create_valid_result_for_value_u64(num);

            assert_eq!(
                actual,
                expected
            );
        }
    }

    #[test]
    fn get_predefined_color_u64_should_return_invalid_for_without_correct_structure() {
        assert_eq!(
            // should have been another 2 numbers between after 1
            get_predefined_color_3_bytes_u64(str_to_u64("\x1b[1m")),
            0
        );
        assert_eq!(
            // should have been another number between after 10
            get_predefined_color_3_bytes_u64(str_to_u64("\x1b[10m")),
            0
        );
        assert_eq!(
            // should have '[' and not ']'
            get_predefined_color_3_bytes_u64(str_to_u64("\x1b]100m")),
            0
        );
        assert_eq!(
            // should have '\x1b' and not 'a'
            get_predefined_color_3_bytes_u64(str_to_u64("a[100m")),
            0
        );
        assert_eq!(
            // should have m in the end
            get_predefined_color_3_bytes_u64(str_to_u64("\x1b[100")),
            0
        );
        assert_eq!(
            // must not be empty
            get_predefined_color_3_bytes_u64(str_to_u64("")),
            0
        );
        assert_eq!(
            // the escape code should be in the beginning
            get_predefined_color_3_bytes_u64(str_to_u64("0\x1b[100m")),
            0
        );
    }

    #[test]
    fn get_predefined_color_3_bytes_u64_should_return_invalid_predefined_color_when_not_ascii_number_in_the_number_position() {
        for byte1 in 0..=255u8 {
            for byte2 in 0..=255u8 {
                for byte3 in 0..=255u8 {
                    // Ignore the ascii numbers between 100 and 107
                    if byte1 == b'1' && byte2 == b'0' && (byte3 >= b'0' && byte3 <= b'7') {
                        continue;
                    }

                    // Doing like this and not with load_or_default as it is much slower
                    let bytes = u8_array_to_u64([
                        b'\x1b', b'[', byte1, byte2, byte3, b'm',

                        0, 0,
                    ]);

                    assert_eq!(
                        get_predefined_color_3_bytes_u64(bytes),
                        0
                    );
                }
            }
        }
    }

    #[test]
    fn get_predefined_color_3_bytes_u64_should_return_invalid_predefined_color_for_bright_colors_below_100() {
        assert_eq!(get_predefined_color_3_bytes_u64(str_to_u64(BLACK_FOREGROUND_CODE)), 0);
        assert_eq!(get_predefined_color_3_bytes_u64(str_to_u64(BLACK_BACKGROUND_CODE)), 0);

        assert_eq!(get_predefined_color_3_bytes_u64(str_to_u64(RED_FOREGROUND_CODE)), 0);
        assert_eq!(get_predefined_color_3_bytes_u64(str_to_u64(RED_BACKGROUND_CODE)), 0);

        assert_eq!(get_predefined_color_3_bytes_u64(str_to_u64(GREEN_FOREGROUND_CODE)), 0);
        assert_eq!(get_predefined_color_3_bytes_u64(str_to_u64(GREEN_BACKGROUND_CODE)), 0);

        assert_eq!(get_predefined_color_3_bytes_u64(str_to_u64(YELLOW_FOREGROUND_CODE)), 0);
        assert_eq!(get_predefined_color_3_bytes_u64(str_to_u64(YELLOW_BACKGROUND_CODE)), 0);

        assert_eq!(get_predefined_color_3_bytes_u64(str_to_u64(BLUE_FOREGROUND_CODE)), 0);
        assert_eq!(get_predefined_color_3_bytes_u64(str_to_u64(BLUE_BACKGROUND_CODE)), 0);

        assert_eq!(get_predefined_color_3_bytes_u64(str_to_u64(MAGENTA_FOREGROUND_CODE)), 0);
        assert_eq!(get_predefined_color_3_bytes_u64(str_to_u64(MAGENTA_BACKGROUND_CODE)), 0);

        assert_eq!(get_predefined_color_3_bytes_u64(str_to_u64(CYAN_FOREGROUND_CODE)), 0);
        assert_eq!(get_predefined_color_3_bytes_u64(str_to_u64(CYAN_BACKGROUND_CODE)), 0);

        assert_eq!(get_predefined_color_3_bytes_u64(str_to_u64(WHITE_FOREGROUND_CODE)), 0);
        assert_eq!(get_predefined_color_3_bytes_u64(str_to_u64(WHITE_BACKGROUND_CODE)), 0);

        assert_eq!(get_predefined_color_3_bytes_u64(str_to_u64(DEFAULT_FOREGROUND_CODE)), 0);
        assert_eq!(get_predefined_color_3_bytes_u64(str_to_u64(DEFAULT_BACKGROUND_CODE)), 0);

        assert_eq!(get_predefined_color_3_bytes_u64(str_to_u64(BRIGHT_BLACK_FOREGROUND_CODE)), 0);
        assert_eq!(get_predefined_color_3_bytes_u64(str_to_u64(BRIGHT_RED_FOREGROUND_CODE)), 0);
        assert_eq!(get_predefined_color_3_bytes_u64(str_to_u64(BRIGHT_GREEN_FOREGROUND_CODE)), 0);
        assert_eq!(get_predefined_color_3_bytes_u64(str_to_u64(BRIGHT_YELLOW_FOREGROUND_CODE)), 0);
        assert_eq!(get_predefined_color_3_bytes_u64(str_to_u64(BRIGHT_BLUE_FOREGROUND_CODE)), 0);
        assert_eq!(get_predefined_color_3_bytes_u64(str_to_u64(BRIGHT_MAGENTA_FOREGROUND_CODE)), 0);
        assert_eq!(get_predefined_color_3_bytes_u64(str_to_u64(BRIGHT_CYAN_FOREGROUND_CODE)), 0);
        assert_eq!(get_predefined_color_3_bytes_u64(str_to_u64(BRIGHT_WHITE_FOREGROUND_CODE)), 0);
    }

}
