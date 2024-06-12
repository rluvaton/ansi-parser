use std::ops::{BitAnd, BitOr, Div, Index, Mul};
use std::simd::cmp::{SimdPartialEq, SimdPartialOrd};
use std::simd::num::SimdUint;
use std::simd::Simd;

const LANES: usize = 32;

// Predefined colors here are not numbers above 99 and one of the specific colors
pub const INVALID: u8 = 255;
// b'\x1b[38m' or other number instead of 38
pub const SIZE: usize = 5;

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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::parse_ansi_text::ansi::colors::*;

    use super::*;

    #[test]
    fn get_predefined_color_should_support_all_predefined_colors() {
        assert_eq!(get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(BLACK_FOREGROUND_CODE.as_bytes())), 30);
        assert_eq!(get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(BLACK_BACKGROUND_CODE.as_bytes())), 40);

        assert_eq!(get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(RED_FOREGROUND_CODE.as_bytes())), 31);
        assert_eq!(get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(RED_BACKGROUND_CODE.as_bytes())), 41);

        assert_eq!(get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(GREEN_FOREGROUND_CODE.as_bytes())), 32);
        assert_eq!(get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(GREEN_BACKGROUND_CODE.as_bytes())), 42);

        assert_eq!(get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(YELLOW_FOREGROUND_CODE.as_bytes())), 33);
        assert_eq!(get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(YELLOW_BACKGROUND_CODE.as_bytes())), 43);

        assert_eq!(get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(BLUE_FOREGROUND_CODE.as_bytes())), 34);
        assert_eq!(get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(BLUE_BACKGROUND_CODE.as_bytes())), 44);

        assert_eq!(get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(MAGENTA_FOREGROUND_CODE.as_bytes())), 35);
        assert_eq!(get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(MAGENTA_BACKGROUND_CODE.as_bytes())), 45);

        assert_eq!(get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(CYAN_FOREGROUND_CODE.as_bytes())), 36);
        assert_eq!(get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(CYAN_BACKGROUND_CODE.as_bytes())), 46);

        assert_eq!(get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(WHITE_FOREGROUND_CODE.as_bytes())), 37);
        assert_eq!(get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(WHITE_BACKGROUND_CODE.as_bytes())), 47);

        assert_eq!(get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(DEFAULT_FOREGROUND_CODE.as_bytes())), 39);
        assert_eq!(get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(DEFAULT_BACKGROUND_CODE.as_bytes())), 49);

        assert_eq!(get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(BRIGHT_BLACK_FOREGROUND_CODE.as_bytes())), 90);
        assert_eq!(get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(BRIGHT_RED_FOREGROUND_CODE.as_bytes())), 91);
        assert_eq!(get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(BRIGHT_GREEN_FOREGROUND_CODE.as_bytes())), 92);
        assert_eq!(get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(BRIGHT_YELLOW_FOREGROUND_CODE.as_bytes())), 93);
        assert_eq!(get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(BRIGHT_BLUE_FOREGROUND_CODE.as_bytes())), 94);
        assert_eq!(get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(BRIGHT_MAGENTA_FOREGROUND_CODE.as_bytes())), 95);
        assert_eq!(get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(BRIGHT_CYAN_FOREGROUND_CODE.as_bytes())), 96);
        assert_eq!(get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(BRIGHT_WHITE_FOREGROUND_CODE.as_bytes())), 97);
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
            get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(b"\x1b[3m")),
            INVALID
        );
        assert_eq!(
            // should have '[' and not ']'
            get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(b"\x1b]39m")),
            INVALID
        );
        assert_eq!(
            // should have '\x1b' and not 'a'
            get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(b"a[39m")),
            INVALID
        );
        assert_eq!(
            // should have m in the end
            get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(b"\x1b[39")),
            INVALID
        );
        assert_eq!(
            // must not be empty
            get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(b"")),
            INVALID
        );
        assert_eq!(
            // the escape code should be in the beginning
            get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(b"0\x1b[39m")),
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
        assert_eq!(get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(BRIGHT_BLACK_BACKGROUND_CODE.as_bytes())), INVALID);
        assert_eq!(get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(BRIGHT_RED_BACKGROUND_CODE.as_bytes())), INVALID);
        assert_eq!(get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(BRIGHT_GREEN_BACKGROUND_CODE.as_bytes())), INVALID);
        assert_eq!(get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(BRIGHT_YELLOW_BACKGROUND_CODE.as_bytes())), INVALID);
        assert_eq!(get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(BRIGHT_BLUE_BACKGROUND_CODE.as_bytes())), INVALID);
        assert_eq!(get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(BRIGHT_MAGENTA_BACKGROUND_CODE.as_bytes())), INVALID);
        assert_eq!(get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(BRIGHT_CYAN_BACKGROUND_CODE.as_bytes())), INVALID);
        assert_eq!(get_predefined_color_2_bytes(Simd::<u8, LANES>::load_or_default(BRIGHT_WHITE_BACKGROUND_CODE.as_bytes())), INVALID);
    }
}
