use std::ops::{BitAnd, BitOr, Div, Mul};
use std::simd::cmp::{SimdPartialEq, SimdPartialOrd};
use std::simd::num::SimdUint;
use std::simd::{Mask, Simd};

// constant Simd structs
const LANES: usize = 4; // we need 3 but simd is always in powers of 2
const ZEROS: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([0; LANES]);
const ASCII_ZERO: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([b'0'; LANES]);
const ASCII_NINE: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([b'9'; LANES]);
const MULTIPLY_U8: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([100, 10, 1, 0]);
const DIVIDE_U8: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([100, 10, 1, 1]);

const ZERO_IN_SECOND_BYTE: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([0, 255, 0, 0]);
const ZERO_IN_THIRD_BYTE: Simd<u8, LANES> = Simd::<u8, LANES>::from_array([0, 0, 255, 0]);

// If between 0 and 255
pub fn parse_u8_simd(bytes: Simd::<u8, LANES>) -> (bool, u8) {
    let ascii_nums = get_only_ascii_numbers(bytes);
    let one_shift = ascii_nums.rotate_elements_right::<1>();
    let two_shift = one_shift.rotate_elements_right::<1>();
    // if there is 0 in the second byte than use two_shift
    // if there is 0 in the third byte than use one_shift
    // otherwise use ascii_nums

    let is_zero_in_second_byte = (ascii_nums & ZERO_IN_SECOND_BYTE).simd_eq(ZEROS);
    let is_zero_in_second_byte = is_zero_in_second_byte.all();
    let ascii_nums = Mask::splat(is_zero_in_second_byte).select(two_shift, ascii_nums);

    let is_zero_in_third_byte = (ascii_nums & ZERO_IN_THIRD_BYTE).simd_eq(ZEROS);
    let is_zero_in_third_byte = is_zero_in_third_byte.all();
    let ascii_nums = Mask::splat(is_zero_in_third_byte).select(one_shift, ascii_nums);

    // saturating_sub meaning if it's less than 0 it will be 0
    let string_to_u8 = ascii_nums.saturating_sub(ASCII_ZERO);
    let current_nums = string_to_u8.mul(MULTIPLY_U8);
    let before_mul = current_nums.div(DIVIDE_U8);

    let num = current_nums.reduce_sum();
    return (before_mul == before_mul , num)
}

fn get_only_ascii_numbers(ascii_num_simd: Simd<u8, LANES>) -> Simd<u8, LANES> {
    // Mask of only the numbers
    let only_numbers_mask = ascii_num_simd.simd_ge(ASCII_ZERO);
    let only_numbers_mask = only_numbers_mask & ascii_num_simd.simd_le(ASCII_NINE);
    let only_numbers = only_numbers_mask.select(ascii_num_simd, ZEROS);

    return only_numbers;
}


#[cfg(test)]
mod tests {
    use std::ops::Mul;
    use std::simd::cmp::SimdPartialOrd;
    use std::simd::num::SimdUint;

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn parse_num() {

        // let bytes = Simd::<u8, LANES>::load_or_default(b"\x1b[255m");
        let bytes = Simd::<u8, LANES>::load_or_default(b"999");
        let ascii_zero = Simd::<u8, LANES>::from_array([b'0'; LANES]);

        let string_to_u8 = bytes - ascii_zero;
        let mul_to_correct_num = Simd::<u8, LANES>::from_array([100, 10, 1, 0]);
        let current_nums = string_to_u8.mul(mul_to_correct_num);

        let num = current_nums.reduce_sum();

        println!("bytes: {:?}", bytes.as_array());
        println!("string_to_u8: {:?}", string_to_u8.as_array());
        println!("current_nums: {:?}", current_nums.as_array());

        println!("num: {:?}", num);
    }

    #[test]
    fn is_valid_u8() {
        for expected_num in 0..=255 {
            let simd_num = Simd::<u8, LANES>::load_or_default(expected_num.to_string().as_bytes());
            let (is_valid, actual_num) = parse_u8_simd(simd_num);
            assert_eq!(is_valid, true);
            assert_eq!(actual_num, expected_num);
        }
    }

    #[test]
    fn u8_parse() {
        let expected_num = 11;
        let simd_num = Simd::<u8, LANES>::load_or_default(expected_num.to_string().as_bytes());
        let (is_valid, actual_num) = parse_u8_simd(simd_num);
        println!("expected_num: {:?}", expected_num);
        println!("simd_num: {:?}", simd_num.as_array());
        println!("is_valid: {:?}", is_valid);
        println!("actual_num: {:?}", actual_num);
        assert_eq!(is_valid, true);
        assert_eq!(actual_num, expected_num);
    }

    #[test]
    fn get_only_ascii_numbers_test() {
        // 0 in ascii
        let simd_num = Simd::<u8, LANES>::load_or_default(&[0, b'0', b'm']);
        let expected_simd = Simd::<u8, LANES>::load_or_default(&[0, b'0', 0]);
        let ascii_nums = get_only_ascii_numbers(simd_num);

        assert_eq!(ascii_nums.as_array().to_vec(), expected_simd.as_array().to_vec());

        // 9 in ascii
        let simd_num = Simd::<u8, LANES>::load_or_default(&[0, b'9', b'm']);
        let expected_simd = Simd::<u8, LANES>::load_or_default(&[0, b'9', 0]);
        let ascii_nums = get_only_ascii_numbers(simd_num);

        assert_eq!(ascii_nums.as_array().to_vec(), expected_simd.as_array().to_vec());
    }
}
