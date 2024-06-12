use std::ops::{BitAnd, BitOr, Div, Index, Mul};
use std::simd::cmp::{SimdPartialEq, SimdPartialOrd};
use std::simd::num::SimdUint;
use std::simd::Simd;

const LANES: usize = 64;

pub const INVALID_STYLE: u8 = 255;
// b'\x1b[0m' or other number instead of 0
pub const STYLE_SIZE: usize = 4;

const MASK: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([
    255, // b'\x1b',
    255, //b'[',
    255, // Everything
    255, //b'm',

    // Empty
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
]);

const MIN_MASK: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([
    b'\x1b',
    b'[',
    b'0', // Everything
    b'm',

    // Empty
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
]);

const MAX_MASK: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([
    b'\x1b',
    b'[',
    b'9', // Everything
    b'm',

    // Empty
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
]);


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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::parse_ansi_text::ansi::constants::RESET_CODE;
    use crate::parse_ansi_text::ansi::style::*;

    use super::*;

    #[test]
    fn get_style_should_return_0_for_reset() {
        let bytes = Simd::<u8, LANES>::load_or_default(RESET_CODE.as_bytes());

        assert_eq!(get_style(bytes), 0);
    }

    #[test]
    fn get_style_should_return_1_for_bold() {
        let bytes = Simd::<u8, LANES>::load_or_default(BOLD_CODE.as_bytes());

        assert_eq!(get_style(bytes), 1);
    }

    #[test]
    fn get_style_should_return_2_for_dim() {
        let bytes = Simd::<u8, LANES>::load_or_default(DIM_CODE.as_bytes());

        assert_eq!(get_style(bytes), 2);
    }

    #[test]
    fn get_style_should_return_3_for_italic() {
        let bytes = Simd::<u8, LANES>::load_or_default(ITALIC_CODE.as_bytes());

        assert_eq!(get_style(bytes), 3);
    }

    #[test]
    fn get_style_should_return_4_for_underline() {
        let bytes = Simd::<u8, LANES>::load_or_default(UNDERLINE_CODE.as_bytes());

        assert_eq!(get_style(bytes), 4);
    }

    #[test]
    fn get_style_should_return_7_for_inverse() {
        let bytes = Simd::<u8, LANES>::load_or_default(INVERSE_CODE.as_bytes());

        assert_eq!(get_style(bytes), 7);
    }

    #[test]
    fn get_style_should_return_9_for_strikethrough() {
        let bytes = Simd::<u8, LANES>::load_or_default(STRIKETHROUGH_CODE.as_bytes());

        assert_eq!(get_style(bytes), 9);
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
            get_style(Simd::<u8, LANES>::load_or_default(b"\x1b[m")),
            INVALID_STYLE
        );
        assert_eq!(
            // should have '[' and not ']'
            get_style(Simd::<u8, LANES>::load_or_default(b"\x1b]1m")),
            INVALID_STYLE
        );
        assert_eq!(
            // should have '\x1b' and not 'a'
            get_style(Simd::<u8, LANES>::load_or_default(b"a[1m")),
            INVALID_STYLE
        );
        assert_eq!(
            // should have m in the end
            get_style(Simd::<u8, LANES>::load_or_default(b"\x1b[1")),
            INVALID_STYLE
        );
        assert_eq!(
            // must not be empty
            get_style(Simd::<u8, LANES>::load_or_default(b"")),
            INVALID_STYLE
        );
        assert_eq!(
            // the escape code should be in the beginning
            get_style(Simd::<u8, LANES>::load_or_default(b"0\x1b[1m")),
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
}
