use std::ops::{BitAnd, BitOr, Div, Index, Mul};
use std::simd::cmp::{SimdPartialEq, SimdPartialOrd};
use std::simd::num::SimdUint;
use std::simd::Simd;

const LANES: usize = 32;

// first is the color type (38 or 48), the rest are RGB values
pub const INVALID: (u8, u8, u8, u8) = (255, 255, 255, 255);

// b'\x1B[38;2;R;G;Bm'
pub const SIZE: usize = 13;

const MASK: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([
    255, // b'\x1b',
    255, //b'[',
    255, //b'3', or b'4'
    255, //b'8',
    255, //b';',
    255, //b'2',
    255, //b';',
    255, // R
    255, //b';',
    255, // G
    255, //b';',
    255, // B
    255, //b'm',

    // Empty
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0,
]);

const MIN_MASK: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([
    b'\x1b',
    b'[',
    b'3',
    b'8',
    b';',
    b'2',
    b';',
    b'0', // R
    b';',
    b'0', // G
    b';',
    b'0', // B
    b'm',

    // Empty
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0,
]);

const MAX_MASK: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([
    b'\x1b',
    b'[',
    b'4', // <--
    b'8',
    b';',
    b'2',
    b';',
    b'9', // R
    b';',
    b'9', // G
    b';',
    b'9', // B
    b'm',

    // Empty
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0,
]);


pub fn get_rgb_color_one_byte(bytes: Simd<u8, LANES>) -> (u8, u8, u8, u8) {
    let only_relevant_part = bytes & MASK;

    let is_valid = only_relevant_part.simd_ge(MIN_MASK).all() && only_relevant_part.simd_le(MAX_MASK).all();
    if !is_valid {
        return INVALID;
    }

    // \x1B[38;2;R;G;Bm or \x1B[48;2;R;G;Bm (foreground or background)
    let color_type = only_relevant_part.index(2); // 2 is the position of the color type (3 or 4)
    let r_digit = only_relevant_part.index(7);
    let g_digit = only_relevant_part.index(9);
    let b_digit = only_relevant_part.index(11);

    return (
        // * 10 + 8 as the color is 38 or 48
        (color_type - b'0') * 10 + 8,
        r_digit - b'0',
        g_digit - b'0',
        b_digit - b'0',
    );
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::parse_ansi_text::ansi::colors::*;

    use super::*;

    #[test]
    fn should_support_all_rgb_colors_between_0_and_9_in_each_value() {
        for r in 0..=9 {
            for g in 0..=9 {
                for b in 0..=9 {
                    assert_eq!(
                        get_rgb_color_one_byte(Simd::<u8, LANES>::load_or_default(RGB_FOREGROUND_CODE(r, g, b).as_bytes())),
                        (38, r, g, b)
                    );
                    assert_eq!(
                        get_rgb_color_one_byte(Simd::<u8, LANES>::load_or_default(RGB_BACKGROUND_CODE(r, g, b).as_bytes())),
                        (48, r, g, b)
                    );
                }
            }
        }
    }


    #[test]
    fn should_support_all_rgb_colors_between_0_and_9_in_each_value_even_if_have_other_bytes_after_color() {
        for r in 0..=9 {
            for g in 0..=9 {
                for b in 0..=9 {
                    assert_eq!(
                        get_rgb_color_one_byte(Simd::<u8, LANES>::load_or_default((RGB_FOREGROUND_CODE(r, g, b) + "mhello").as_bytes())),
                        (38, r, g, b)
                    );
                    assert_eq!(
                        get_rgb_color_one_byte(Simd::<u8, LANES>::load_or_default((RGB_BACKGROUND_CODE(r, g, b) + "mhello").as_bytes())),
                        (48, r, g, b)
                    );
                }
            }
        }
    }

    #[test]
    fn get_color_should_return_invalid_for_without_correct_structure() {
        assert_eq!(
            // should be 2 instead of 5
            get_rgb_color_one_byte(Simd::<u8, LANES>::load_or_default(b"\x1B[38;5;1;1;1m")),
            INVALID
        );
    }

    // TODO

    // #[test]
    // fn should_return_invalid_when_not_ascii_number_in_the_number_position() {
    //     // TODO
    //     for byte in 0..=255u8 {
    //         // Ignore the ascii numbers between 0 and 9
    //         if byte >= b'0' && byte <= b'9' {
    //             continue;
    //         }
    //
    //         let bytes = Simd::<u8, LANES>::load_or_default(&[
    //             b'\x1b',
    //             b'[',
    //             b'3', // <-- for foreground
    //             b'8',
    //             b';',
    //             b'5',
    //             b';',
    //             byte,
    //             b'm',
    //         ]);
    //
    //         assert_eq!(
    //             get_eight_bit_color_one_byte(bytes),
    //             INVALID
    //         );
    //
    //         let bytes = Simd::<u8, LANES>::load_or_default(&[
    //             b'\x1b',
    //             b'[',
    //             b'4', // <-- for background
    //             b'8',
    //             b';',
    //             b'5',
    //             b';',
    //             byte,
    //             b'm',
    //         ]);
    //
    //         assert_eq!(
    //             get_eight_bit_color_one_byte(bytes),
    //             INVALID
    //         );
    //     }
    // }
    //
    // #[test]
    // fn should_return_invalid_for_eight_bit_color_greater_than_1_digit() {
    //     for num in 10..=255 {
    //         assert_eq!(
    //             get_eight_bit_color_one_byte(Simd::<u8, LANES>::load_or_default(EIGHT_BIT_FOREGROUND_CODE(num).as_bytes())),
    //             INVALID
    //         );
    //         assert_eq!(
    //             get_eight_bit_color_one_byte(Simd::<u8, LANES>::load_or_default(EIGHT_BIT_BACKGROUND_CODE(num).as_bytes())),
    //             INVALID
    //         );
    //     }
    // }
}
