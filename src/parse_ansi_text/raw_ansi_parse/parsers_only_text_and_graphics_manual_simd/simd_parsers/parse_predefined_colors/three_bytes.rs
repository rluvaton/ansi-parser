use std::ops::{BitAnd, BitOr, Div, Index, Mul};
use std::simd::cmp::{SimdPartialEq, SimdPartialOrd};
use std::simd::num::SimdUint;
use std::simd::Simd;

const LANES: usize = 32;

// Predefined colors here are between 100 and 107
pub const INVALID: u8 = 255;
// b'\x1b[100m' or other number instead of 100
pub const SIZE: usize = 6;

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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::parse_ansi_text::ansi::colors::*;

    use super::*;

    #[test]
    fn get_predefined_color_3_bytes_should_support_all_predefined_colors() {
        assert_eq!(get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(BRIGHT_BLACK_BACKGROUND_CODE.as_bytes())), 100);
        assert_eq!(get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(BRIGHT_RED_BACKGROUND_CODE.as_bytes())), 101);
        assert_eq!(get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(BRIGHT_GREEN_BACKGROUND_CODE.as_bytes())), 102);
        assert_eq!(get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(BRIGHT_YELLOW_BACKGROUND_CODE.as_bytes())), 103);
        assert_eq!(get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(BRIGHT_BLUE_BACKGROUND_CODE.as_bytes())), 104);
        assert_eq!(get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(BRIGHT_MAGENTA_BACKGROUND_CODE.as_bytes())), 105);
        assert_eq!(get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(BRIGHT_CYAN_BACKGROUND_CODE.as_bytes())), 106);
        assert_eq!(get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(BRIGHT_WHITE_BACKGROUND_CODE.as_bytes())), 107);

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
            get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(b"\x1b[1m")),
            INVALID
        );
        assert_eq!(
            // should have been another number between after 10
            get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(b"\x1b[10m")),
            INVALID
        );
        assert_eq!(
            // should have '[' and not ']'
            get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(b"\x1b]100m")),
            INVALID
        );
        assert_eq!(
            // should have '\x1b' and not 'a'
            get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(b"a[100m")),
            INVALID
        );
        assert_eq!(
            // should have m in the end
            get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(b"\x1b[100")),
            INVALID
        );
        assert_eq!(
            // must not be empty
            get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(b"")),
            INVALID
        );
        assert_eq!(
            // the escape code should be in the beginning
            get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(b"0\x1b[100m")),
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
        assert_eq!(get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(BLACK_FOREGROUND_CODE.as_bytes())), INVALID);
        assert_eq!(get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(BLACK_BACKGROUND_CODE.as_bytes())), INVALID);

        assert_eq!(get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(RED_FOREGROUND_CODE.as_bytes())), INVALID);
        assert_eq!(get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(RED_BACKGROUND_CODE.as_bytes())), INVALID);

        assert_eq!(get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(GREEN_FOREGROUND_CODE.as_bytes())), INVALID);
        assert_eq!(get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(GREEN_BACKGROUND_CODE.as_bytes())), INVALID);

        assert_eq!(get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(YELLOW_FOREGROUND_CODE.as_bytes())), INVALID);
        assert_eq!(get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(YELLOW_BACKGROUND_CODE.as_bytes())), INVALID);

        assert_eq!(get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(BLUE_FOREGROUND_CODE.as_bytes())), INVALID);
        assert_eq!(get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(BLUE_BACKGROUND_CODE.as_bytes())), INVALID);

        assert_eq!(get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(MAGENTA_FOREGROUND_CODE.as_bytes())), INVALID);
        assert_eq!(get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(MAGENTA_BACKGROUND_CODE.as_bytes())), INVALID);

        assert_eq!(get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(CYAN_FOREGROUND_CODE.as_bytes())), INVALID);
        assert_eq!(get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(CYAN_BACKGROUND_CODE.as_bytes())), INVALID);

        assert_eq!(get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(WHITE_FOREGROUND_CODE.as_bytes())), INVALID);
        assert_eq!(get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(WHITE_BACKGROUND_CODE.as_bytes())), INVALID);

        assert_eq!(get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(DEFAULT_FOREGROUND_CODE.as_bytes())), INVALID);
        assert_eq!(get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(DEFAULT_BACKGROUND_CODE.as_bytes())), INVALID);

        assert_eq!(get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(BRIGHT_BLACK_FOREGROUND_CODE.as_bytes())), INVALID);
        assert_eq!(get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(BRIGHT_RED_FOREGROUND_CODE.as_bytes())), INVALID);
        assert_eq!(get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(BRIGHT_GREEN_FOREGROUND_CODE.as_bytes())), INVALID);
        assert_eq!(get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(BRIGHT_YELLOW_FOREGROUND_CODE.as_bytes())), INVALID);
        assert_eq!(get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(BRIGHT_BLUE_FOREGROUND_CODE.as_bytes())), INVALID);
        assert_eq!(get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(BRIGHT_MAGENTA_FOREGROUND_CODE.as_bytes())), INVALID);
        assert_eq!(get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(BRIGHT_CYAN_FOREGROUND_CODE.as_bytes())), INVALID);
        assert_eq!(get_predefined_color_3_bytes(Simd::<u8, LANES>::load_or_default(BRIGHT_WHITE_FOREGROUND_CODE.as_bytes())), INVALID);
    }
}
