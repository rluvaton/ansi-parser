use atoi::atoi;
use heapless::Vec;
use memchr::memchr;
use nom::branch::alt;
use nom::bytes::streaming::{tag, take};
use nom::character::streaming::digit1;
use nom::combinator::{map, map_res, value};
use nom::error::ErrorKind;
use nom::IResult;
use nom::sequence::{delimited, tuple};

use crate::parse_ansi_text::raw_ansi_parse::enums::AnsiSequence;

pub const ESCAPE_AS_BYTES: &[u8] = b"\x1b";
const EMPTY_AS_BYTES: &[u8] = b"";


fn parse_u8(input: &[u8]) -> IResult<&[u8], u8> {
    map_res(digit1, |s: &[u8]| {
        return atoi::<u8>(s).ok_or(nom::Err::Error(nom::error::Error::new(
            input,
            ErrorKind::Digit,
        )));
    })(input)
}

fn graphics_mode1(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    map(delimited(tag(b"\x1b["), parse_u8, tag(b"m")), |val| {
        let mode =
            Vec::from_slice(&[val]).expect("Vec::from_slice should allocate sufficient size");
        AnsiSequence::SetGraphicsMode(mode)
    })(input)
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

    // TODO - these are not true as the u8 numbers are as string and not 1 char

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

    let graphic_mode = get_graphic_mode(input);

    return match graphic_mode {
        1 => graphics_mode1(input),
        2 => graphics_mode2(input),
        3 => graphics_mode3(input),
        4 => graphics_mode4(input),
        5 => graphics_mode5(input),

        // TODO - should not reach here
        _ => graphics_mode(input)
    };
    //
    //
    // let a = u8x16::from([1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4]);
    // let b = u8x16::from([2_u8; 16]);
    // let expected = u8x16::from([
    //     0,
    //     u8::MAX,
    //     0,
    //     0,
    //     0,
    //     u8::MAX,
    //     0,
    //     0,
    //     0,
    //     u8::MAX,
    //     0,
    //     0,
    //     0,
    //     u8::MAX,
    //     0,
    //     0,
    // ]);
    // let actual = a.cmp_eq(b);
    // assert_eq!(expected, actual);
    // return graphics_mode(input);
    // `alt` only supports up to 21 parsers, and nom doesn't seem to
    // have an alternative with higher variability.
    // So we simply nest them.
    // alt((
    //     alt((
    //         cursor_pos,
    //         cursor_up,
    //         cursor_down,
    //         cursor_forward,
    //         cursor_backward,
    //         cursor_save,
    //         cursor_restore,
    //         erase_display,
    //         erase_line,
    //         graphics_mode,
    //         set_mode,
    //         reset_mode,
    //         hide_cursor,
    //         show_cursor,
    //         cursor_to_app,
    //         set_new_line_mode,
    //         set_col_132,
    //         set_smooth_scroll,
    //         set_reverse_video,
    //         set_origin_rel,
    //     )),
    //     alt((
    //         set_auto_wrap,
    //         set_auto_repeat,
    //         set_interlacing,
    //         set_linefeed,
    //         set_cursorkey,
    //         set_vt52,
    //         set_col80,
    //         set_jump_scroll,
    //         set_normal_video,
    //         set_origin_abs,
    //         reset_auto_wrap,
    //         reset_auto_repeat,
    //         reset_interlacing,
    //         set_top_and_bottom,
    //         set_alternate_keypad,
    //         set_numeric_keypad,
    //         set_uk_g0,
    //         set_uk_g1,
    //         set_us_g0,
    //         set_us_g1,
    //         set_g0_special,
    //     )),
    //     set_g1_special,
    //     set_g0_alternate,
    //     set_g1_alternate,
    //     set_g0_graph,
    //     set_g1_graph,
    //     set_single_shift2,
    //     set_single_shift3,
    // ))(input)
}

fn escape_codes(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    // removed the preceding tag so we can match it in the value
    combined(input)
}

fn take_single(s: &[u8]) -> IResult<&[u8], &[u8]> {
    take(1usize)(s)
}

fn until_escape(s: &[u8]) -> IResult<&[u8], &[u8]> {
    let a = memchr(b'\x1b', s);

    return match a {
        Some(i) => Ok((&s[i..], &s[..i])),
        None => Err(nom::Err::Incomplete(nom::Needed::Unknown)),
    };
}

pub fn parse_escape(input: &[u8], complete_string: bool) -> IResult<&[u8], AnsiSequence> {
    if input.is_empty() {
        return Err(nom::Err::Incomplete(nom::Needed::Unknown));
    }

    // If not starting with the escape code then the matching string shouldn't be empty, I think
    if !input.starts_with(ESCAPE_AS_BYTES) {
        let res = until_escape(input);

        match res {
            Ok(res) => {
                let (str, matched_string) = res;
                if !matched_string.is_empty() {
                    return Ok((str, AnsiSequence::Text(matched_string)));
                }
            }
            Err(err) => {
                if complete_string && matches!(err, nom::Err::Incomplete(_)) {
                    return Ok((EMPTY_AS_BYTES, AnsiSequence::Text(input)));
                }
            }
        }
    }

    let res = escape_codes(input);
    match res {
        Ok(res) => {
            return Ok(res);
        }
        Err(e) => {
            match e {
                nom::Err::Error(sub_error) => {
                    // If fail to match than we have escape code in the first char
                    // we check in fail to match and not incomplete as we might get more text that might be escape code
                    if matches!(sub_error.code, ErrorKind::Tag) {
                        let single_res = take_single(input);

                        if single_res.is_ok() {
                            let (str, matched_string) = single_res.unwrap();
                            return Ok((str, AnsiSequence::Text(matched_string)));
                        }
                    }
                    return Err(nom::Err::Error(sub_error));
                }
                _ => {
                    return Err(e);
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
            Ok((
                EMPTY_AS_BYTES,
                AnsiSequence::SetGraphicsMode(Vec::from_slice(&[41]).unwrap())
            ))
        );
        assert_eq!(
            parse_escape(RESET_CODE.as_bytes(), true),
            Ok((
                EMPTY_AS_BYTES,
                AnsiSequence::SetGraphicsMode(Vec::from_slice(&[0]).unwrap())
            ))
        );
    }
}
