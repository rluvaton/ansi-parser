use core::simd::prelude::*;
use std::ops::Index;

use heapless::Vec;
use memchr::memchr;

use constants::*;

use crate::parse_ansi_text::raw_ansi_parse::enums::AnsiSequence;
use crate::parse_ansi_text::raw_ansi_parse::parsers_only_text_and_graphics_manual_simd::simd_parsers::{
    parse_8_bit_colors::{
        EIGHT_BIT_COLOR_SIZE_1_BYTE, EIGHT_BIT_COLOR_SIZE_2_BYTES, EIGHT_BIT_COLOR_SIZE_3_BYTES,
        get_eight_bit_color_one_byte, get_eight_bit_color_three_bytes, get_eight_bit_color_two_bytes,
        INVALID_EIGHT_BIT_COLOR_1_BYTE, INVALID_EIGHT_BIT_COLOR_2_BYTES, INVALID_EIGHT_BIT_COLOR_3_BYTES,
    },
    parse_predefined_colors::{
        get_predefined_color_2_bytes, get_predefined_color_3_bytes, INVALID_PREDEFINED_COLOR_2_BYTES,
        INVALID_PREDEFINED_COLOR_3_BYTES, PREDEFINED_COLOR_SIZE_2_BYTES, PREDEFINED_COLOR_SIZE_3_BYTES,
    },
    parse_style::{get_style, get_style_simd, INVALID_STYLE, STYLE_SIZE},
};
use crate::parse_ansi_text::raw_ansi_parse::parsers_only_text_and_graphics_manual_simd::simd_parsers::parse_predefined_colors::{get_predefined_color_2_bytes_simd, get_predefined_color_3_bytes_simd};
use crate::parse_ansi_text::raw_ansi_parse::parsers_only_text_and_graphics_manual_simd::simd_parsers::helpers::graphics_mode_result;

mod constants;
mod simd_parsers;


const INCOMPLETE_RESULT: Simd::<u8, 32> = Simd::<u8, 32>::from_array([0; 32]);

// all false to take the next result
// const INITIAL_MASK: Mask<i8, 8> = Mask::from_array([false;8]);

enum ParseGraphicsMode<'a> {
    Parsed(&'a [u8], AnsiSequence<'a>),
    Invalid,
    Incomplete,
}

fn parse_graphics_mode(input: &[u8]) -> ParseGraphicsMode<'_> {
    // The minimum size of the input should be STYLE_SIZE
    if input.len() < STYLE_SIZE as usize {
        return ParseGraphicsMode::Incomplete;
    }

    // TODO - change to load_or_default to avoid panic if the length is less than 32
    let bytes = Simd::<u8, 32>::load_or_default(input);

    let style = get_style(bytes);

    if style != INVALID_STYLE {
        return ParseGraphicsMode::Parsed(&input[(STYLE_SIZE as usize)..], AnsiSequence::SetGraphicsMode1Byte(style));
    }

    let predefined_color = get_predefined_color_2_bytes(bytes);

    if predefined_color != INVALID_PREDEFINED_COLOR_2_BYTES {
        return ParseGraphicsMode::Parsed(&input[(PREDEFINED_COLOR_SIZE_2_BYTES as usize)..], AnsiSequence::SetGraphicsModePredefinedColor(predefined_color));
    }

    let predefined_color = get_predefined_color_3_bytes(bytes);

    if predefined_color != INVALID_PREDEFINED_COLOR_3_BYTES {
        return ParseGraphicsMode::Parsed(&input[(PREDEFINED_COLOR_SIZE_3_BYTES as usize)..], AnsiSequence::SetGraphicsModePredefinedColor(predefined_color));
    }

    let eight_bit = get_eight_bit_color_one_byte(bytes);

    if eight_bit != INVALID_EIGHT_BIT_COLOR_1_BYTE {
        return ParseGraphicsMode::Parsed(&input[EIGHT_BIT_COLOR_SIZE_1_BYTE..], AnsiSequence::SetGraphicsModeEightBitColor(eight_bit.0, eight_bit.1));
    }

    let eight_bit = get_eight_bit_color_two_bytes(bytes);

    if eight_bit != INVALID_EIGHT_BIT_COLOR_2_BYTES {
        return ParseGraphicsMode::Parsed(&input[EIGHT_BIT_COLOR_SIZE_2_BYTES..], AnsiSequence::SetGraphicsModeEightBitColor(eight_bit.0, eight_bit.1));
    }

    let eight_bit = get_eight_bit_color_three_bytes(bytes);

    if eight_bit != INVALID_EIGHT_BIT_COLOR_3_BYTES {
        return ParseGraphicsMode::Parsed(&input[EIGHT_BIT_COLOR_SIZE_3_BYTES..], AnsiSequence::SetGraphicsModeEightBitColor(eight_bit.0, eight_bit.1));
    }


    // TODO - implement 255 colors

    // TODO - implement invalid result by checking if the length is enough to parse the escape code

    return ParseGraphicsMode::Incomplete;
}

fn parse_graphics_mode_simd_ifs(input: &[u8]) -> Simd::<u8, 32> {
    // The minimum size of the input should be STYLE_SIZE
    if input.len() < STYLE_SIZE as usize {
        return INCOMPLETE_RESULT
    }
    if input.len() < 32 {
        return INCOMPLETE_RESULT
    }

    // TODO - change to load_or_default to avoid panic if the length is less than 32
    let bytes = Simd::<u8, 32>::from_slice(&input[..32]);

    // invalid result
    // let current_result = INCOMPLETE_RESULT;

    let (mask, possible_result) = get_style_simd(bytes);

    if mask.test(0) {
        return possible_result;
    }

    // let current_result = mask.select(possible_result, current_result);


    let (mask, possible_result) = get_predefined_color_2_bytes_simd(bytes);

    if mask.test(0) {
        return possible_result;
    }

    // let current_result = mask.select(possible_result, current_result);

    let (mask, possible_result) = get_predefined_color_3_bytes_simd(bytes);

    if mask.test(0) {
        return possible_result;
    }

    // let current_result = mask.select(possible_result, current_result);

    // TODO - implement 8 bit colors and 255

    return INCOMPLETE_RESULT;
}

fn parse_graphics_mode_simd_bits_only(input: &[u8]) -> Simd::<u8, 32> {
    // The minimum size of the input should be STYLE_SIZE
    if input.len() < STYLE_SIZE as usize {
        return INCOMPLETE_RESULT
    }

    if input.len() < 32 {
        return INCOMPLETE_RESULT
    }

    // TODO - change to load_or_default to avoid panic if the length is less than 32
    let bytes = Simd::<u8, 32>::from_slice(&input[..32]);

    // invalid result
    let current_result = INCOMPLETE_RESULT;

    let (mask, possible_result) = get_style_simd(bytes);

    let current_result = mask.select(possible_result, current_result);

    let (mask, possible_result) = get_predefined_color_2_bytes_simd(bytes);

    let current_result = mask.select(possible_result, current_result);

    let (mask, possible_result) = get_predefined_color_3_bytes_simd(bytes);

    let current_result = mask.select(possible_result, current_result);

    // TODO - implement 8 bit colors and 255

    return current_result;
}


pub fn parse_escape_with_ifs(input: &[u8], complete_string: bool) -> Option<(&[u8], AnsiSequence)> {
    if input.is_empty() {
        // Return the empty string, TODO - should not reach here
        return None;
    }

    // If not starting with the escape code then the matching string shouldn't be empty, I think
    if !input.starts_with(ESCAPE_AS_BYTES) {
        let pos = memchr(b'\x1b', input);

        return Some(match pos {
            Some(i) => {
                (&input[i..], AnsiSequence::Text(&input[..i]))
            }
            None => {
                (EMPTY_AS_BYTES, AnsiSequence::Text(input))
            }
        });
    }

    let res = parse_graphics_mode(input);

    return match res {
        ParseGraphicsMode::Parsed(input, res) => {
            Some((input, res))
        }
        ParseGraphicsMode::Invalid => {
            // If fail to match than we have escape code in the first char
            // we check in fail to match and not incomplete as we might get more text that might be escape code
            let next_escape_pos = memchr(b'\x1b', &input[1..]);

            Some(match next_escape_pos {
                Some(mut i) => {
                    // i + 1 as we are starting from 1 to skip the first escape code
                    i += 1;
                    (&input[i..], AnsiSequence::Text(&input[..i]))
                }
                None => {
                    (EMPTY_AS_BYTES, AnsiSequence::Text(input))
                }
            })
        }
        ParseGraphicsMode::Incomplete => {
            None
        }
    };
}

pub fn parse_escape(input: &[u8], complete_string: bool) -> Option<(&[u8], AnsiSequence)> {
    if input.is_empty() {
        // Return the empty string, TODO - should not reach here
        return None;
    }

    // If not starting with the escape code then the matching string shouldn't be empty, I think
    if input[0] != b'\x1b' {
        let pos = memchr(b'\x1b', input);

        return Some(match pos {
            Some(i) => {
                (&input[i..], AnsiSequence::Text(&input[..i]))
            }
            None => {
                (EMPTY_AS_BYTES, AnsiSequence::Text(input))
            }
        });
    }

    let res_simd = parse_graphics_mode_simd_ifs(input);

    if !graphics_mode_result::is_valid(res_simd) {
        return None;
    }

    let size = graphics_mode_result::get_size(res_simd) as usize;

    return Some((&input[size..], AnsiSequence::SetGraphicsModeSimd(res_simd)));
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
                EMPTY_AS_BYTES,
                AnsiSequence::SetGraphicsMode(Vec::from_slice(&[41]).unwrap())
            ))
        );
        assert_eq!(
            parse_escape(RESET_CODE.as_bytes(), true),
            Some((
                EMPTY_AS_BYTES,
                AnsiSequence::SetGraphicsMode(Vec::from_slice(&[0]).unwrap())
            ))
        );
    }
}
