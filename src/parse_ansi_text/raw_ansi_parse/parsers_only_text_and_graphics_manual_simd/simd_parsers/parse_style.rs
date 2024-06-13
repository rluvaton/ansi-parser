use std::ops::{Add, BitAnd, BitAndAssign, BitOr, BitXor, Div, Index, Mul, Sub};


const LANES: usize = 32;

pub const PARSE_GRAPHICS_MODE_STYLE_TYPE: u8 = 1;

pub const INVALID_STYLE: u8 = 255;
// b'\x1b[0m' or other number instead of 0
pub const STYLE_SIZE: u8 = 4;

// 255, // b'\x1b',
//     255, //b'[',
//     255, // Everything
//     255, //b'm',
const MASK_SMALL_U64: u64 = 0xFF_FF_FF_FF_00_00_00_00;

// this is b'\x1b', b'[', -, b'm'
const EXPECTED_VALUE_WITHOUT_NUMBER_SMALL_U64: u64 = 0x1B_5B_00_6D_00_00_00_00;
const MASK_WITHOUT_NUMBER_SMALL_U64: u64 = 0xFF_FF_00_FF_00_00_00_00;

// 0, // b'\x1b',
//     0, // b'[',
//     b'0', // b'9', // Everything
//     0, // b'm',

// 30 is the ascii value of '0'
const SUBTRACT_NUM_TO_U8_SMALL_U64: u64 = 0x00_00_30_00_00_00_00_00;


// 0, // PARSE_GRAPHICS_MODE_STYLE_TYPE,
//     0, // SIZE,
//     0, // value size
//     255, // the value
const KEEP_VALUE_BYTE_SMALL_U64: u64 = 0x00_00_00_FF_00_00_00_00;


// 01 for PARSE_GRAPHICS_MODE_STYLE_TYPE
// 04 for SIZE
// 01 for value size
// 00 for the value
// rest is empty
const GRAPHICS_MODE_RESULT_SMALL_U64: u64 = 0x01_04_01_00_00_00_00_00;


#[inline(always)]
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
