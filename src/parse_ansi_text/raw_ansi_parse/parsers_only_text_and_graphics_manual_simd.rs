use std::ops::Index;

use heapless::Vec;
use memchr::memchr;


use crate::parse_ansi_text::raw_ansi_parse::enums::AnsiSequence;
use crate::parse_ansi_text::raw_ansi_parse::parsers_only_text_and_graphics_manual_simd::simd_parsers::helpers::u8_slice_to_u64;
use crate::parse_ansi_text::raw_ansi_parse::parsers_only_text_and_graphics_manual_simd::simd_parsers::parse_predefined_colors::{get_predefined_color_2_bytes_u64, get_predefined_color_3_bytes_u64};
use crate::parse_ansi_text::raw_ansi_parse::parsers_only_text_and_graphics_manual_simd::simd_parsers::parse_style::get_style_u64;
use crate::parse_ansi_text::raw_ansi_parse::parsers_only_text_and_graphics_manual_simd::simd_parsers::parse_style::STYLE_SIZE;

mod simd_parsers;


const INCOMPLETE_RESULT_SMALL_U64: u64 = 0;

// all false to take the next result
// const INITIAL_MASK: Mask<i8, 8> = Mask::from_array([false;8]);

enum ParseGraphicsMode<'a> {
    Parsed(&'a [u8], AnsiSequence<'a>),
    Invalid,
    Incomplete,
}

#[inline(always)]
fn parse_graphics_mode_simd_ifs(input: &[u8]) -> u64 {
    let bytes = u8_slice_to_u64(input);

    return get_style_u64(bytes) | get_predefined_color_2_bytes_u64(bytes) | get_predefined_color_3_bytes_u64(bytes);
    //
    // let possible_result = get_style_u64(bytes);
    //
    // if possible_result > 0 {
    //     return possible_result;
    // }
    //
    // let possible_result = get_predefined_color_2_bytes_u64(bytes);
    //
    // if possible_result > 0 {
    //     return possible_result;
    // }
    //
    // let possible_result = get_predefined_color_3_bytes_u64(bytes);
    //
    // if possible_result > 0 {
    //     return possible_result;
    // }

    // TODO - implement 8 bit colors and 255

    // return INCOMPLETE_RESULT_SMALL_U64;
}

#[inline(always)]
pub fn parse_escape(input: &[u8], complete_string: bool) -> Option<(&[u8], AnsiSequence)> {
    if input.is_empty() {
        // Return the empty string, TODO - should not reach here
        return None;
    }

    // If not starting with the escape code then the matching string shouldn't be empty, I think
    if !input.starts_with( b"\x1b") {
        let pos = memchr(b'\x1b', input);

        return Some(match pos {
            Some(i) => {
                (&input[i..], AnsiSequence::Text(&input[..i]))
            }
            None => {
                (&[], AnsiSequence::Text(input))
            }
        });
    }

    let res_simd = parse_graphics_mode_simd_ifs(input);

    // If the leftmost byte is not set then it is an invalid result
    if res_simd == 0 {
        return None;
    }

    let mut size = res_simd & 0x00_FF_00_00_00_00_00_00;
    size = size >> 48;

    return Some((&input[size as usize..], AnsiSequence::SetGraphicsModeU64(res_simd)));
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::parse_ansi_text::ansi::colors::*;
    use crate::parse_ansi_text::ansi::constants::RESET_CODE;

    use super::*;

    #[test]
    fn test_value() {
        assert_eq!(
            parse_escape(RED_BACKGROUND_CODE.as_bytes(), true),
            Some((
                &[] as &[u8],
                AnsiSequence::SetGraphicsMode(Vec::from_slice(&[41]).unwrap())
            ))
        );
        assert_eq!(
            parse_escape(RESET_CODE.as_bytes(), true),
            Some((
                &[] as &[u8],

                AnsiSequence::SetGraphicsMode(Vec::from_slice(&[0]).unwrap())
            ))
        );
    }

    #[test]
    fn small_file() {
        let file_path: &str = "/Users/rluvaton/dev/personal/ansi-viewer/examples/fixtures/tiny.ans";
        let file_content = std::fs::read(file_path).expect("Failed to read file");
        let content = file_content.as_slice();

        let mut res = parse_escape(content, true);

        loop {
            if res.is_none() {
                break;
            }

            res = parse_escape(res.unwrap().0, true);
        }
    }
}
