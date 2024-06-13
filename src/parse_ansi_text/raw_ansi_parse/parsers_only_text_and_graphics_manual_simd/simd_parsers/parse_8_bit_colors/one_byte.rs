// use std::ops::{BitAnd, BitOr, Div, Index, Mul};
// use std::simd::cmp::{SimdPartialEq, SimdPartialOrd};
// use std::simd::num::SimdUint;
// use std::simd::Simd;
//
// const LANES: usize = 32;
//
// pub const INVALID: (u8, u8) = (255, 255);
//
// // b'\x1b[38;5;0m' or other number instead of 0
// pub const SIZE: usize = 9;
//
// const MASK: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([
//     255, // b'\x1b',
//     255, //b'[',
//     255, //b'3', or b'4'
//     255, //b'8',
//     255, //b';',
//     255, //b'5',
//     255, //b';',
//     255, // Everything
//     255, //b'm',
//
//     // Empty
//     0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     0, 0, 0,
// ]);
//
// const MIN_MASK: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([
//     b'\x1b',
//     b'[',
//     b'3',
//     b'8',
//     b';',
//     b'5',
//     b';',
//     b'0', // Everything
//     b'm',
//
//     // Empty
//     0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     0, 0, 0,
// ]);
//
// const MAX_MASK: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([
//     b'\x1b',
//     b'[',
//     b'4', // <--
//     b'8',
//     b';',
//     b'5',
//     b';',
//     b'9', // Everything
//     b'm',
//
//     // Empty
//     0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     0, 0, 0,
// ]);
//
//
// pub fn get_eight_bit_color_one_byte(bytes: Simd<u8, LANES>) -> (u8, u8) {
//     let only_relevant_part = bytes & MASK;
//
//     let is_valid = only_relevant_part.simd_ge(MIN_MASK).all() && only_relevant_part.simd_le(MAX_MASK).all();
//     if !is_valid {
//         return INVALID;
//     }
//
//     // \x1B[38;5;Vm or \x1B[48;5;Vm (foreground or background)
//     let color_type = only_relevant_part.index(2); // 2 is the position of the color type (3 or 4)
//     let digit = only_relevant_part.index(7); // 7 is the position of the digit
//
//     // Get the number from the ascii
//     // * 10 + 8 as the color is 38 or 48
//     return ((color_type - b'0') * 10 + 8, digit - b'0');
// }
//
// #[cfg(test)]
// mod tests {
//     use pretty_assertions::assert_eq;
//
//     use crate::parse_ansi_text::ansi::colors::*;
//
//     use super::*;
//
//     #[test]
//     fn get_eight_bit_color_one_byte_should_support_all_eight_bit_colors_between_0_and_9() {
//         for num in 0..=9 {
//             assert_eq!(
//                 get_eight_bit_color_one_byte(Simd::<u8, LANES>::load_or_default(EIGHT_BIT_FOREGROUND_CODE(num).as_bytes())),
//                 (38, num)
//             );
//             assert_eq!(
//                 get_eight_bit_color_one_byte(Simd::<u8, LANES>::load_or_default(EIGHT_BIT_BACKGROUND_CODE(num).as_bytes())),
//                 (48, num)
//             );
//         }
//     }
//
//
//     #[test]
//     fn get_eight_bit_color_one_byte_should_return_color_even_if_have_other_bytes_after_color() {
//         for num in 0..=9 {
//             assert_eq!(
//                 get_eight_bit_color_one_byte(Simd::<u8, LANES>::load_or_default((EIGHT_BIT_FOREGROUND_CODE(num) + "mhello").as_bytes())),
//                 (38, num)
//             );
//             assert_eq!(
//                 get_eight_bit_color_one_byte(Simd::<u8, LANES>::load_or_default((EIGHT_BIT_BACKGROUND_CODE(num) + "mhello").as_bytes())),
//                 (48, num)
//             );
//         }
//     }
//
//     #[test]
//     fn get_color_should_return_invalid_for_without_correct_structure() {
//         // \x1B[38;5;Vm
//         assert_eq!(
//             // should be 5 instead of 2
//             get_eight_bit_color_one_byte(Simd::<u8, LANES>::load_or_default(b"\x1B[38;2;1m")),
//             INVALID
//         );
//         assert_eq!(
//             // should be 5 instead of 2
//             get_eight_bit_color_one_byte(Simd::<u8, LANES>::load_or_default(b"\x1B[38;2;1m")),
//             INVALID
//         );
//         assert_eq!(
//             // should be 38 or 48 instead of 00
//             get_eight_bit_color_one_byte(Simd::<u8, LANES>::load_or_default(b"\x1B[00;5;1m")),
//             INVALID
//         );
//         assert_eq!(
//             // should be 38 or 48 instead of 30
//             get_eight_bit_color_one_byte(Simd::<u8, LANES>::load_or_default(b"\x1B[30;5;1m")),
//             INVALID
//         );
//         assert_eq!(
//             // should be 38 or 48 instead of 30
//             get_eight_bit_color_one_byte(Simd::<u8, LANES>::load_or_default(b"\x1B[40;5;1m")),
//             INVALID
//         );
//     }
//
//     #[test]
//     fn get_eight_bit_color_one_byte_should_return_invalid_when_not_ascii_number_in_the_number_position() {
//         for byte in 0..=255u8 {
//             // Ignore the ascii numbers between 0 and 9
//             if byte >= b'0' && byte <= b'9' {
//                 continue;
//             }
//
//             let bytes = Simd::<u8, LANES>::load_or_default(&[
//                 b'\x1b',
//                 b'[',
//                 b'3', // <-- for foreground
//                 b'8',
//                 b';',
//                 b'5',
//                 b';',
//                 byte,
//                 b'm',
//             ]);
//
//             assert_eq!(
//                 get_eight_bit_color_one_byte(bytes),
//                 INVALID
//             );
//
//             let bytes = Simd::<u8, LANES>::load_or_default(&[
//                 b'\x1b',
//                 b'[',
//                 b'4', // <-- for background
//                 b'8',
//                 b';',
//                 b'5',
//                 b';',
//                 byte,
//                 b'm',
//             ]);
//
//             assert_eq!(
//                 get_eight_bit_color_one_byte(bytes),
//                 INVALID
//             );
//         }
//     }
//
//     #[test]
//     fn should_return_invalid_for_eight_bit_color_greater_than_1_digit() {
//         for num in 10..=255 {
//             assert_eq!(
//                 get_eight_bit_color_one_byte(Simd::<u8, LANES>::load_or_default(EIGHT_BIT_FOREGROUND_CODE(num).as_bytes())),
//                 INVALID
//             );
//             assert_eq!(
//                 get_eight_bit_color_one_byte(Simd::<u8, LANES>::load_or_default(EIGHT_BIT_BACKGROUND_CODE(num).as_bytes())),
//                 INVALID
//             );
//         }
//     }
// }
