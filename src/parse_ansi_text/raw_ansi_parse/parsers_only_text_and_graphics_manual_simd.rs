use core::simd::prelude::*;

use atoi::atoi;
use heapless::Vec;
use memchr::memchr;
use nom::branch::alt;
use nom::bytes::streaming::tag;
use nom::character::streaming::digit1;
use nom::combinator::{map, map_res, value};
use nom::error::ErrorKind;
use nom::IResult;
use nom::sequence::tuple;

use constants::*;

use crate::parse_ansi_text::raw_ansi_parse::enums::AnsiSequence;
use crate::parse_ansi_text::raw_ansi_parse::parsers_only_text_and_graphics_manual_simd::simd_parsers::{
    parse_style::{get_style, INVALID_STYLE, STYLE_SIZE},
    parse_predefined_colors::{
        get_predefined_color_2_bytes, INVALID_PREDEFINED_COLOR_2_BYTES, PREDEFINED_COLOR_SIZE_2_BYTES,
        get_predefined_color_3_bytes, INVALID_PREDEFINED_COLOR_3_BYTES, PREDEFINED_COLOR_SIZE_3_BYTES,
    },
    parse_8_bit_colors::{
        get_eight_bit_color_one_byte, INVALID_EIGHT_BIT_COLOR_1_BYTE, EIGHT_BIT_COLOR_SIZE_1_BYTE,
    }
};
use crate::parse_ansi_text::raw_ansi_parse::parsers_only_text_and_graphics_manual_simd::simd_parsers::parse_u8::parse_u8_simd;

mod constants;
mod simd_parsers;

fn parse_u8(input: &[u8]) -> IResult<&[u8], u8> {
    map_res(digit1, |s: &[u8]| {
        return atoi::<u8>(s).ok_or(nom::Err::Error(nom::error::Error::new(
            input,
            ErrorKind::Digit,
        )));
    })(input)
}

fn graphics_mode1(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    let bytes = Simd::<u8, 32>::load_or_default(input);
    // if is_graphic_mode_1(bytes) {
    // parse_u8


    let value = parse_u8_simd(bytes.rotate_elements_left::<2>().resize(4));
    if !value.0 {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            ErrorKind::Digit,
        )));
    }

    let m = if value.1 % 100 > 0 {
        3
    } else if value.1 % 10 > 0 {
        2
    } else {
        1
    };


    return Ok((&input[m..], AnsiSequence::SetGraphicsMode(heapless::Vec::from_slice(&[value.1]).unwrap())));

    // return Ok((value.0, AnsiSequence::SetGraphicsMode(heapless::Vec::from_slice(&[value.1]).unwrap())))
    // }

    // return Err(nom::Err::Error(nom::error::Error::new(
    //     input,
    //     ErrorKind::Digit,
    // )));

    // map(delimited(tag(b"\x1b["), parse_u8, tag(b"m")), |val| {
    //     let mode =
    //         Vec::from_slice(&[val]).expect("Vec::from_slice should allocate sufficient size");
    //     AnsiSequence::SetGraphicsMode(mode)
    // })(input)
}

fn graphics_mode2(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    map(
        tuple((tag(b"\x1b["), parse_u8, tag(b";"), parse_u8, tag(b"m"))),
        |(_, val1, _, val2, _)| {
            let mode = Vec::from_slice(&[val1, val2])
                .expect("Vec::from_slice should allocate sufficient size");
            AnsiSequence::SetGraphicsMode(mode)
        },
    )(input)
}

fn graphics_mode3(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    map(
        tuple((
            tag(b"\x1b["),
            parse_u8,
            tag(b";"),
            parse_u8,
            tag(b";"),
            parse_u8,
            tag(b"m"),
        )),
        |(_, val1, _, val2, _, val3, _)| {
            let mode = Vec::from_slice(&[val1, val2, val3])
                .expect("Vec::from_slice should allocate sufficient size");
            AnsiSequence::SetGraphicsMode(mode)
        },
    )(input)
}

fn graphics_mode4(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    value(AnsiSequence::SetGraphicsMode(Vec::new()), tag(b"\x1b[m"))(input)
}

fn graphics_mode5(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    map(
        tuple((
            tag(b"\x1b["),
            parse_u8,
            tag(b";"),
            parse_u8,
            tag(b";"),
            parse_u8,
            tag(b";"),
            parse_u8,
            tag(b";"),
            parse_u8,
            tag(b"m"),
        )),
        |(_, val1, _, val2, _, val3, _, val4, _, val5, _)| {
            let mode = Vec::from_slice(&[val1, val2, val3, val4, val5])
                .expect("Vec::from_slice should allocate sufficient size");
            AnsiSequence::SetGraphicsMode(mode)
        },
    )(input)
}

fn graphics_mode(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    alt((
        graphics_mode1,
        graphics_mode2,
        graphics_mode3,
        graphics_mode4,
        graphics_mode5,
    ))(input)
}

fn get_graphic_mode(input: &[u8]) -> u8 {

    // 0 for normal text (no attributes)
    if input.len() > 2 && input[0] == b'\x1b' && input[1] == b'[' {
        // it should start with \x1b[
        return 0;
    }

    // in graphic mode 1, the ending - m - is at the 4th byte
    // TODO - check exists so won't overflow
    if input.len() >= 4 && input[3] == b'm' {
        return 1;
    }

    // in graphic mode 2, the ending - m - is at the 6th byte
    // TODO - check exists so won't overflow
    if input.len() >= 6 && input[5] == b'm' {
        return 2;
    }

    // in graphic mode 3, the ending - m - is at the 8th byte
    // TODO - check exists so won't overflow
    if input.len() >= 8 && input[7] == b'm' {
        return 3;
    }

    // in graphic mode 4, the ending - m - is at the 3rd byte
    // TODO - check exists so won't overflow
    if input.len() >= 3 && input[2] == b'm' {
        return 4;
    }

    // in graphic mode 5, the ending - m - is at the 12th byte
    // TODO - check exists so won't overflow
    if input.len() >= 12 && input[11] == b'm' {
        return 5;
    }

    // if none of the above, then it's not a graphic mode
    return 0;
}


fn combined(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    // graphics_mode1:
    // map(delimited(tag(b"\x1b["), parse_u8, tag(b"m")), |val| {
    //         let mode =
    //             Vec::from_slice(&[val]).expect("Vec::from_slice should allocate sufficient size");
    //         AnsiSequence::SetGraphicsMode(mode)
    //     })(input)
    // Sizes:
    // ------
    // 1 byte for the escape code, 1 byte for the bracket, 1 byte for the mode, 1 byte for the m
    // so 4 bytes for the graphics mode 1

    // graphics_mode2:
    // map(
    //         tuple((tag(b"\x1b["), parse_u8, tag(b";"), parse_u8, tag(b"m"))),
    //         |(_, val1, _, val2, _)| {
    //             let mode = Vec::from_slice(&[val1, val2])
    //                 .expect("Vec::from_slice should allocate sufficient size");
    //             AnsiSequence::SetGraphicsMode(mode)
    //         },
    //     )(input)
    //
    // Sizes:
    // ------
    // 1 byte for the escape code, 1 byte for the bracket, 1 byte for the mode, 1 byte for the semicolon, 1 byte for the mode, 1 byte for the m
    // so 6 bytes for the graphics mode 2

    // graphics_mode3:
    // map(
    //         tuple((
    //             tag(b"\x1b["),
    //             parse_u8,
    //             tag(b";"),
    //             parse_u8,
    //             tag(b";"),
    //             parse_u8,
    //             tag(b"m"),
    //         )),
    //         |(_, val1, _, val2, _, val3, _)| {
    //             let mode = Vec::from_slice(&[val1, val2, val3])
    //                 .expect("Vec::from_slice should allocate sufficient size");
    //             AnsiSequence::SetGraphicsMode(mode)
    //         },
    //     )(input)
    //
    // Sizes:
    // ------
    // 1 byte for the escape code, 1 byte for the bracket, 1 byte for the mode, 1 byte for the semicolon, 1 byte for the mode, 1 byte for the semicolon, 1 byte for the mode, 1 byte for the m
    // so 8 bytes for the graphics mode 3

    // graphics_mode4:
    // value(AnsiSequence::SetGraphicsMode(Vec::new()), tag(b"\x1b[m"))(input)
    // not sure what about this

    // graphics_mode5:
    // map(
    //         tuple((
    //             tag(b"\x1b["),
    //             parse_u8,
    //             tag(b";"),
    //             parse_u8,
    //             tag(b";"),
    //             parse_u8,
    //             tag(b";"),
    //             parse_u8,
    //             tag(b";"),
    //             parse_u8,
    //             tag(b"m"),
    //         )),
    //         |(_, val1, _, val2, _, val3, _, val4, _, val5, _)| {
    //             let mode = Vec::from_slice(&[val1, val2, val3, val4, val5])
    //                 .expect("Vec::from_slice should allocate sufficient size");
    //             AnsiSequence::SetGraphicsMode(mode)
    //         },
    //     )(input)
    //
    // Sizes:
    // ------
    // 1 byte for the escape code, 1 byte for the bracket, 1 byte for the mode, 1 byte for the semicolon, 1 byte for the mode, 1 byte for the semicolon, 1 byte for the mode, 1 byte for the semicolon, 1 byte for the mode, 1 byte for the semicolon, 1 byte for the mode and 1 byte for the m
    // so 12 bytes for the graphics mode 5
    let bytes = Simd::<u8, 64>::load_or_default(input);

    let style = get_style(bytes);

    if style != INVALID_STYLE {
        return Ok((&input[STYLE_SIZE..], AnsiSequence::SetGraphicsMode1Byte(style)));
    }

    let predefined_color = get_predefined_color_2_bytes(bytes);

    if predefined_color != INVALID_PREDEFINED_COLOR_2_BYTES {
        return Ok((&input[PREDEFINED_COLOR_SIZE_2_BYTES..], AnsiSequence::SetGraphicsModePredefinedColor(predefined_color)));
    }

    let predefined_color = get_predefined_color_3_bytes(bytes);

    if predefined_color != INVALID_PREDEFINED_COLOR_3_BYTES {
        return Ok((&input[PREDEFINED_COLOR_SIZE_3_BYTES..], AnsiSequence::SetGraphicsModePredefinedColor(predefined_color)));
    }

    let eight_bit = get_eight_bit_color_one_byte(bytes);

    if eight_bit != INVALID_EIGHT_BIT_COLOR_1_BYTE {
        return Ok((&input[EIGHT_BIT_COLOR_SIZE_1_BYTE..], AnsiSequence::SetGraphicsModeEightBitColor(eight_bit.0, eight_bit.1)));
    }

    let bytes = Simd::<u8, 32>::load_or_default(input);

    let graphic_mode = get_graphic_mode(input);

    return match graphic_mode {
        2 => graphics_mode2(input),
        3 => graphics_mode3(input),
        4 => graphics_mode4(input),
        5 => graphics_mode5(input),

        // TODO - should not reach here
        _ => panic!("Should not reach here")
    };
}

fn escape_codes(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    // removed the preceding tag so we can match it in the value
    combined(input)
}


pub fn parse_escape(input: &[u8], complete_string: bool) -> Option<(&[u8], AnsiSequence)> {
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

    let res = escape_codes(input);
    match res {
        Ok(res) => {
            return Some(res);
        }
        Err(e) => {
            match e {
                nom::Err::Error(sub_error) => {
                    // If fail to match than we have escape code in the first char
                    // we check in fail to match and not incomplete as we might get more text that might be escape code
                    if matches!(sub_error.code, ErrorKind::Tag) {
                        let next_escape_pos = memchr(b'\x1b', &input[1..]);

                        return Some(match next_escape_pos {
                            Some(mut i) => {
                                // i + 1 as we are starting from 1 to skip the first escape code
                                i += 1;
                                (&input[i..], AnsiSequence::Text(&input[..i]))
                            }
                            None => {
                                (EMPTY_AS_BYTES, AnsiSequence::Text(input))
                            }
                        });
                    }
                    panic!("Should not reach here");
                    // return Err(nom::Err::Error(sub_error));
                }
                _ => {
                    // panic!("Should not reach here");
                    return None;
                    // return Err(e);
                }
            }
        }
    }
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
