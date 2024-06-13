use std::cmp::min;


pub fn u8_array_to_u64(arr: [u8; 8]) -> u64 {
    return u64::from_be_bytes(arr);
}

pub fn str_to_u64(str: &str) -> u64 {
    let bytes = str.as_bytes();
    let mut arr: [u8; 8] = [0; 8];

    for i in 0..min(8, bytes.len()) {
        arr[i] = bytes[i];
    }

    u8_array_to_u64(arr)
}


pub unsafe fn u8_slice_to_u64_unchecked(slice: &[u8]) -> u64 {
    let num = unsafe {
        let ptr = slice.as_ptr() as *const u64;
        ptr.read_unaligned()
    };

    num.to_be()
}

#[inline(always)]
pub fn u8_slice_to_u64(slice: &[u8]) -> u64 {
    if slice.len() < 8 {
        let mut arr: [u8; 8] = [0; 8];

        // Align to have the required length
        arr[..slice.len()].copy_from_slice(slice);

        return u8_array_to_u64(arr);
    }

    let num = unsafe {
        let ptr = slice.as_ptr() as *const u64;
        ptr.read_unaligned()
    };

    num.to_be()
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn should_convert_u8_array_to_u64() {
        let arr: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

        let actual = u8_array_to_u64(arr);

        let expected = 0x01_02_03_04_05_06_07_08;

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_convert_u8_slice_unchecked_to_u64() {
        let slice: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8];

        let actual = unsafe { u8_slice_to_u64_unchecked(slice) };

        let expected = 0x01_02_03_04_05_06_07_08;

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_convert_u8_slice_to_u64() {
        let slice: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8];

        let actual = u8_slice_to_u64(slice);

        let expected = 0x01_02_03_04_05_06_07_08;

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_convert_empty_u8_slice_to_u64() {
        let slice: &[u8] = &[];

        let actual = u8_slice_to_u64(slice);

        let expected = 0x00_00_00_00_00_00_00_00;

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_convert_smaller_u8_slice_to_u64() {
        let slice: &[u8] = &[1, 2, 3, 4];

        let actual = u8_slice_to_u64(slice);

        let expected = 0x01_02_03_04_00_00_00_00;

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_convert_larger_u8_slice_to_u64() {
        let slice: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

        let actual = u8_slice_to_u64(slice);

        let expected = 0x01_02_03_04_05_06_07_08;

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_convert_str_to_u64() {
        let str = "\x01\x02\x03\x04\x05\x06\x07\x08";

        let actual = str_to_u64(str);

        let expected = 0x01_02_03_04_05_06_07_08;

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_convert_smaller_str_to_u64() {
        let str = "\x01\x02\x03\x04";

        let actual = str_to_u64(str);

        let expected = 0x01_02_03_04_00_00_00_00;

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_convert_longer_str_to_u64() {
        let str = "\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A";

        let actual = str_to_u64(str);

        let expected = 0x01_02_03_04_05_06_07_08;

        assert_eq!(actual, expected);
    }
}
