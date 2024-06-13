use std::ops::{BitAnd, BitAndAssign, BitOr, BitXor, Div, Index, Mul};
use std::simd::cmp::{SimdPartialEq, SimdPartialOrd};
use std::simd::Mask;
use std::simd::num::SimdUint;

pub trait AllOrNone {
    fn all_or_none(&self) -> Self;
    fn all_or_none_mut(&mut self) -> Self;
}

impl AllOrNone for Mask::<i8, 32> {
    fn all_or_none(&self) -> Self {
        return Mask::<i8, 32>::from_array([self.all(); 32]);
        // let bitmask = self.to_bitmask();
        //
        // let lower_bits = bitmask & 0xFFFF_FFFF;
        // let all_bits_set = ((lower_bits) + 1) >> 32; // This will be 1 if all bits are set, 0 otherwise
        // let result = lower_bits * all_bits_set;
        //
        // return Mask::<i8, 32>::from_bitmask(result);
    }

    fn all_or_none_mut(&mut self) -> Self {
        let bitmask = self.to_bitmask();

        let lower_bits = bitmask & 0xFFFF_FFFF;
        let all_bits_set = ((lower_bits) + 1) >> 32; // This will be 1 if all bits are set, 0 otherwise
        let result = lower_bits * all_bits_set;

        *self = Mask::<i8, 32>::from_bitmask(result);

        return *self;
    }
}

impl AllOrNone for Mask::<i8, 16> {
    fn all_or_none(&self) -> Self {
        let bitmask = self.to_bitmask();

        let lower_bits = bitmask & 0xFFFF;
        let all_bits_set = ((lower_bits) + 1) >> 16; // This will be 1 if all bits are set, 0 otherwise
        let result = lower_bits * all_bits_set;

        return Mask::<i8, 16>::from_bitmask(result);
    }

    fn all_or_none_mut(&mut self) -> Self {
        let bitmask = self.to_bitmask();

        let lower_bits = bitmask & 0xFFFF;
        let all_bits_set = ((lower_bits) + 1) >> 16; // This will be 1 if all bits are set, 0 otherwise
        let result = lower_bits * all_bits_set;

        *self = Mask::<i8, 16>::from_bitmask(result);

        return *self;
    }
}

impl AllOrNone for Mask::<i8, 8> {
    fn all_or_none(&self) -> Self {
        let bitmask = self.to_bitmask();

        let lower_bits = bitmask & 0xFF;
        let all_bits_set = ((lower_bits) + 1) >> 8; // This will be 1 if all bits are set, 0 otherwise
        let result = lower_bits * all_bits_set;

        return Mask::<i8, 8>::from_bitmask(result);
    }

    fn all_or_none_mut(&mut self) -> Self {
        let bitmask = self.to_bitmask();

        let lower_bits = bitmask & 0xFF;
        let all_bits_set = ((lower_bits) + 1) >> 8; // This will be 1 if all bits are set, 0 otherwise
        let result = lower_bits * all_bits_set;

        *self = Mask::<i8, 8>::from_bitmask(result);

        return *self;
    }
}

impl AllOrNone for Mask::<i8, 4> {
    fn all_or_none(&self) -> Self {
        let bitmask = self.to_bitmask();

        let lower_bits = bitmask & 0xF;
        let all_bits_set = ((lower_bits) + 1) >> 4; // This will be 1 if all bits are set, 0 otherwise
        let result = lower_bits * all_bits_set;

        return Mask::<i8, 4>::from_bitmask(result);
    }

    fn all_or_none_mut(&mut self) -> Self {
        let bitmask = self.to_bitmask();

        let lower_bits = bitmask & 0xF;
        let all_bits_set = ((lower_bits) + 1) >> 4; // This will be 1 if all bits are set, 0 otherwise
        let result = lower_bits * all_bits_set;

        *self = Mask::<i8, 4>::from_bitmask(result);

        return *self;
    }
}



#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    // ---------------------
    //  Mask<i8, 32>
    // ---------------------
    #[test]
    fn should_return_all_true_for_all_true_mask_i8_32() {
        let input_mask = Mask::from_array([true; 32]);
        let actual_mask = input_mask.all_or_none();
        let expected_mask = Mask::from_array([true; 32]);

        assert_eq!(actual_mask, expected_mask);
    }

    #[test]
    fn should_return_all_false_for_all_false_mask_i8_32() {
        let input_mask = Mask::from_array([false; 32]);
        let actual_mask = input_mask.all_or_none();

        let expected_mask = Mask::from_array([false; 32]);

        assert_eq!(actual_mask, expected_mask);
    }
    #[test]
    fn should_return_all_false_for_single_false_in_all_true_mask_i8_32() {
        let input_mask = Mask::from_array([
            true, true, false, true,

            true, true, true, true,
            true, true, true, true,
            true, true, true, true,
            true, true, true, true,
            true, true, true, true,
            true, true, true, true,
            true, true, true, true
        ]);
        let actual_mask = input_mask.all_or_none();

        let expected_mask = Mask::from_array([false; 32]);

        assert_eq!(actual_mask, expected_mask);
    }
    #[test]
    fn should_not_ignore_values_at_last_byte_for_mask_i8_32() {
        let input_mask = Mask::from_array([
            true, true, true, true,
            true, true, true, true,
            true, true, true, true,
            true, true, true, true,
            true, true, true, true,
            true, true, true, true,
            true, true, true, true,
            true, true, true, false,
        ]);
        let actual_mask = input_mask.all_or_none();

        let expected_mask = Mask::from_array([false; 32]);

        assert_eq!(actual_mask, expected_mask);
    }


    // ---------------------
    //  Mask<i8, 16>
    // ---------------------

    #[test]
    fn should_return_all_true_for_all_true_mask_i8_16() {
        let input_mask = Mask::from_array([true; 16]);
        let actual_mask = input_mask.all_or_none();
        let expected_mask = Mask::from_array([true; 16]);

        assert_eq!(actual_mask, expected_mask);
    }

    #[test]
    fn should_return_all_false_for_all_false_mask_i8_16() {
        let input_mask = Mask::from_array([false; 16]);
        let actual_mask = input_mask.all_or_none();

        let expected_mask = Mask::from_array([false; 16]);

        assert_eq!(actual_mask, expected_mask);
    }
    #[test]
    fn should_return_all_false_for_single_false_in_all_true_mask_i8_16() {
        let input_mask = Mask::from_array([
            true, true, false, true,

            true, true, true, true,
            true, true, true, true,
            true, true, true, true,
        ]);
        let actual_mask = input_mask.all_or_none();

        let expected_mask = Mask::from_array([false; 16]);

        assert_eq!(actual_mask, expected_mask);
    }
    #[test]
    fn should_not_ignore_values_at_last_byte_for_mask_i8_16() {
        let input_mask = Mask::from_array([
            true, true, true, true,
            true, true, true, true,
            true, true, true, true,
            true, true, true, false,
        ]);
        let actual_mask = input_mask.all_or_none();

        let expected_mask = Mask::from_array([false; 16]);

        assert_eq!(actual_mask, expected_mask);
    }



    // ---------------------
    //  Mask<i8, 8>
    // ---------------------

    #[test]
    fn should_return_all_true_for_all_true_mask_i8_8() {
        let input_mask = Mask::from_array([true; 8]);
        let actual_mask = input_mask.all_or_none();
        let expected_mask = Mask::from_array([true; 8]);

        assert_eq!(actual_mask, expected_mask);
    }

    #[test]
    fn should_return_all_false_for_all_false_mask_i8_8() {
        let input_mask = Mask::from_array([false; 8]);
        let actual_mask = input_mask.all_or_none();

        let expected_mask = Mask::from_array([false; 8]);

        assert_eq!(actual_mask, expected_mask);
    }
    #[test]
    fn should_return_all_false_for_single_false_in_all_true_mask_i8_8() {
        let input_mask = Mask::from_array([
            true, true, false, true,
            true, true, true, true,
        ]);
        let actual_mask = input_mask.all_or_none();

        let expected_mask = Mask::from_array([false; 8]);

        assert_eq!(actual_mask, expected_mask);
    }
    #[test]
    fn should_not_ignore_values_at_last_byte_for_mask_i8_8() {
        let input_mask = Mask::from_array([
            true, true, true, true,
            true, true, true, false,
        ]);
        let actual_mask = input_mask.all_or_none();

        let expected_mask = Mask::from_array([false; 8]);

        assert_eq!(actual_mask, expected_mask);
    }


    // ---------------------
    //  Mask<i8, 4>
    // ---------------------

    #[test]
    fn should_return_all_true_for_all_true_mask_i8_4() {
        let input_mask = Mask::from_array([true; 4]);
        let actual_mask = input_mask.all_or_none();
        let expected_mask = Mask::from_array([true; 4]);

        assert_eq!(actual_mask, expected_mask);
    }

    #[test]
    fn should_return_all_false_for_all_false_mask_i8_4() {
        let input_mask = Mask::from_array([false; 4]);
        let actual_mask = input_mask.all_or_none();

        let expected_mask = Mask::from_array([false; 4]);

        assert_eq!(actual_mask, expected_mask);
    }
    #[test]
    fn should_return_all_false_for_single_false_in_all_true_mask_i8_4() {
        let input_mask = Mask::from_array([
            true, true, false, true,
        ]);
        let actual_mask = input_mask.all_or_none();

        let expected_mask = Mask::from_array([false; 4]);

        assert_eq!(actual_mask, expected_mask);
    }
    #[test]
    fn should_not_ignore_values_at_last_byte_for_mask_i8_4() {
        let input_mask = Mask::from_array([
            true, true, true, false,
        ]);
        let actual_mask = input_mask.all_or_none();

        let expected_mask = Mask::from_array([false; 4]);

        assert_eq!(actual_mask, expected_mask);
    }
}
