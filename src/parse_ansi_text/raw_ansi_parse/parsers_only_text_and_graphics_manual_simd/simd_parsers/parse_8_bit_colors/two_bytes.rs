use std::ops::{BitAnd, BitOr, Div, Index, Mul};
use std::simd::cmp::{SimdPartialEq, SimdPartialOrd};
use std::simd::num::SimdUint;
use std::simd::Simd;

const LANES: usize = 32;

pub const INVALID: (u8, u8) = (255, 255);

// b'\x1b[38;5;00m' or other number instead of 00
pub const SIZE: usize = 10;

const MASK: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([
    255, // b'\x1b',
    255, //b'[',
    255, //b'3', or b'4'
    255, //b'8',
    255, //b';',
    255, //b'5',
    255, //b';',
    255, // Everything
    255, // Everything
    255, //b'm',

    // Empty
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0,
]);

const MIN_MASK: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([
    b'\x1b',
    b'[',
    b'3',
    b'8',
    b';',
    b'5',
    b';',
    b'0', // Everything
    b'0', // Everything
    b'm',

    // Empty
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0,
]);

const MAX_MASK: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([
    b'\x1b',
    b'[',
    b'4', // <--
    b'8',
    b';',
    b'5',
    b';',
    b'9', // Everything
    b'9', // Everything
    b'm',

    // Empty
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0,
]);


pub fn get_eight_bit_color_two_bytes(bytes: Simd<u8, LANES>) -> (u8, u8) {
    let only_relevant_part = bytes & MASK;

    let is_valid = only_relevant_part.simd_ge(MIN_MASK).all() && only_relevant_part.simd_le(MAX_MASK).all();
    if !is_valid {
        return INVALID;
    }

    // \x1B[38;5;Vm or \x1B[48;5;Vm (foreground or background)
    let color_type = only_relevant_part.index(2); // 2 is the position of the color type (3 or 4)
    let first_digit = only_relevant_part.index(7); // 7 is the position of the first digit
    let second_digit = only_relevant_part.index(8); // 8 is the position of the second digit

    // Get the number from the ascii
    // * 10 + 8 as the color is 38 or 48
    return ((color_type - b'0') * 10 + 8, (first_digit - b'0') * 10 + (second_digit - b'0'));
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::parse_ansi_text::ansi::colors::*;

    use super::*;

    #[test]
    fn should_support_all_eight_bit_colors_between_10_to_90() {
        for num in 10..=99 {
            assert_eq!(
                get_eight_bit_color_two_bytes(Simd::<u8, LANES>::load_or_default(EIGHT_BIT_FOREGROUND_CODE(num).as_bytes())),
                (38, num)
            );
            assert_eq!(
                get_eight_bit_color_two_bytes(Simd::<u8, LANES>::load_or_default(EIGHT_BIT_BACKGROUND_CODE(num).as_bytes())),
                (48, num)
            );
        }
    }


    #[test]
    fn should_return_color_even_if_have_other_bytes_after_color() {
        for num in 10..=99 {
            assert_eq!(
                get_eight_bit_color_two_bytes(Simd::<u8, LANES>::load_or_default((EIGHT_BIT_FOREGROUND_CODE(num) + "mhello").as_bytes())),
                (38, num)
            );
            assert_eq!(
                get_eight_bit_color_two_bytes(Simd::<u8, LANES>::load_or_default((EIGHT_BIT_BACKGROUND_CODE(num) + "mhello").as_bytes())),
                (48, num)
            );
        }
    }

    #[test]
    fn should_return_invalid_for_without_correct_structure() {
        // \x1B[38;5;Vm
        assert_eq!(
            // should be 5 instead of 2
            get_eight_bit_color_two_bytes(Simd::<u8, LANES>::load_or_default(b"\x1B[38;2;10m")),
            INVALID
        );
        assert_eq!(
            // should be 5 instead of 2
            get_eight_bit_color_two_bytes(Simd::<u8, LANES>::load_or_default(b"\x1B[38;2;10m")),
            INVALID
        );
        assert_eq!(
            // should be 38 or 48 instead of 00
            get_eight_bit_color_two_bytes(Simd::<u8, LANES>::load_or_default(b"\x1B[00;5;10m")),
            INVALID
        );
        assert_eq!(
            // should be 38 or 48 instead of 30
            get_eight_bit_color_two_bytes(Simd::<u8, LANES>::load_or_default(b"\x1B[30;5;10m")),
            INVALID
        );
        assert_eq!(
            // should be 38 or 48 instead of 30
            get_eight_bit_color_two_bytes(Simd::<u8, LANES>::load_or_default(b"\x1B[40;5;10m")),
            INVALID
        );
    }

    #[test]
    fn should_return_invalid_when_not_ascii_number_in_the_number_position() {
        for byte1 in 0..=255u8 {
            for byte2 in 0..=255u8 {
                // Ignore the ascii numbers between 00 and 99
                if byte1 >= b'0' && byte1 <= b'9' && byte2 >= b'0' && byte2 <= b'9' {
                    continue;
                }

                let bytes = Simd::<u8, LANES>::load_or_default(&[
                    b'\x1b',
                    b'[',
                    b'3', // <-- for foreground
                    b'8',
                    b';',
                    b'5',
                    b';',
                    byte1,
                    byte2,
                    b'm',
                ]);

                assert_eq!(
                    get_eight_bit_color_two_bytes(bytes),
                    INVALID
                );

                let bytes = Simd::<u8, LANES>::load_or_default(&[
                    b'\x1b',
                    b'[',
                    b'4', // <-- for background
                    b'8',
                    b';',
                    b'5',
                    b';',
                    byte1,
                    byte2,
                    b'm',
                ]);

                assert_eq!(
                    get_eight_bit_color_two_bytes(bytes),
                    INVALID
                );
            }

        }
    }

    #[test]
    fn should_return_invalid_for_eight_bit_color_greater_than_2_digits() {
        for num in 100..=255 {
            assert_eq!(
                get_eight_bit_color_two_bytes(Simd::<u8, LANES>::load_or_default(EIGHT_BIT_FOREGROUND_CODE(num).as_bytes())),
                INVALID
            );
            assert_eq!(
                get_eight_bit_color_two_bytes(Simd::<u8, LANES>::load_or_default(EIGHT_BIT_BACKGROUND_CODE(num).as_bytes())),
                INVALID
            );
        }
    }
    #[test]
    fn should_return_invalid_for_eight_bit_color_with_1_digit() {
        for num in 0..=9 {
            assert_eq!(
                get_eight_bit_color_two_bytes(Simd::<u8, LANES>::load_or_default(EIGHT_BIT_FOREGROUND_CODE(num).as_bytes())),
                INVALID
            );
            assert_eq!(
                get_eight_bit_color_two_bytes(Simd::<u8, LANES>::load_or_default(EIGHT_BIT_BACKGROUND_CODE(num).as_bytes())),
                INVALID
            );
        }
    }
}
