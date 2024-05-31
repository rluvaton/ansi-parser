// Taken from ansi_parse and modify

use memchr::memchr;

use atoi::atoi;
use heapless::Vec;
use nom::branch::alt;
use nom::bytes::streaming::{tag, take, take_until};
use nom::character::streaming::{digit0, digit1};
use nom::combinator::{map, map_res, opt, value};
use nom::error::ErrorKind;
use nom::sequence::{delimited, preceded, tuple};
use nom::{error, AsBytes, FindToken, IResult};

use crate::parse_ansi_text::raw_ansi_parse::enums::AnsiSequence;

pub const ESCAPE_AS_BYTES: &[u8] = b"\x1b";
const EMPTY_AS_BYTES: &[u8] = b"";

macro_rules! tag_parser {
    ($sig:ident, $tag:expr, $ret:expr) => {
        fn $sig(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
            value($ret, tag($tag))(input)
        }
    };
}

fn parse_u32(input: &[u8]) -> IResult<&[u8], u32, error::Error<&[u8]>> {
    map_res(digit1, |s: &[u8]| -> Result<u32, error::Error<&[u8]>> {
        return Ok(u32::from_be_bytes(s.try_into().unwrap()));
    })(input)
}

fn parse_u8(input: &[u8]) -> IResult<&[u8], u8> {
    map_res(digit1, |s: &[u8]| {
        return atoi::<u8>(s).ok_or(nom::Err::Error(nom::error::Error::new(
            input,
            ErrorKind::Digit,
        )));
    })(input)
}

// TODO kind of ugly, would prefer to pass in the default so we could use it for
// all escapes with defaults (not just those that default to 1).
fn parse_def_cursor_int(input: &[u8]) -> IResult<&[u8], u32> {
    map(digit0, |s: &[u8]| atoi::<u32>(s).unwrap_or(1))(input)
}

fn cursor_pos(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    map(
        tuple((
            tag(b"\x1b["),
            parse_def_cursor_int,
            opt(tag(b";")),
            parse_def_cursor_int,
            alt((tag(b"H"), tag(b"f"))),
        )),
        |(_, x, _, y, _)| AnsiSequence::CursorPos(x, y),
    )(input)
}

fn escape(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    value(AnsiSequence::Text(ESCAPE_AS_BYTES), tag(ESCAPE_AS_BYTES))(input)
}

fn cursor_up(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    preceded(
        tag(ESCAPE_AS_BYTES),
        map(
            delimited(tag(b"["), parse_def_cursor_int, tag(b"A")),
            |am| AnsiSequence::CursorUp(am),
        ),
    )(input)
}

fn cursor_down(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    preceded(
        tag(ESCAPE_AS_BYTES),
        map(
            delimited(tag(b"["), parse_def_cursor_int, tag(b"B")),
            |am| AnsiSequence::CursorDown(am),
        ),
    )(input)
}

fn cursor_forward(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    preceded(
        tag(ESCAPE_AS_BYTES),
        map(
            delimited(tag(b"["), parse_def_cursor_int, tag(b"C")),
            |am| AnsiSequence::CursorForward(am),
        ),
    )(input)
}

fn cursor_backward(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    preceded(
        tag(ESCAPE_AS_BYTES),
        map(
            delimited(tag(b"["), parse_def_cursor_int, tag(b"D")),
            |am| AnsiSequence::CursorBackward(am),
        ),
    )(input)
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

fn set_mode(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    map(delimited(tag(b"\x1b[="), parse_u8, tag(b"h")), |val| {
        AnsiSequence::SetMode(val)
    })(input)
}

fn reset_mode(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    map(delimited(tag(b"\x1b[="), parse_u8, tag(b"l")), |val| {
        AnsiSequence::ResetMode(val)
    })(input)
}

fn set_top_and_bottom(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    preceded(
        tag(ESCAPE_AS_BYTES),
        map(
            tuple((tag(b"["), parse_u32, tag(b";"), parse_u32, tag(b"r"))),
            |(_, x, _, y, _)| AnsiSequence::SetTopAndBottom(x, y),
        ),
    )(input)
}

tag_parser!(cursor_save, b"\x1b[s", AnsiSequence::CursorSave);
tag_parser!(cursor_restore, b"\x1b[u", AnsiSequence::CursorRestore);
tag_parser!(erase_display, b"\x1b[2J", AnsiSequence::EraseDisplay);
tag_parser!(erase_line, b"\x1b[K", AnsiSequence::EraseLine);
tag_parser!(hide_cursor, b"\x1b[?25l", AnsiSequence::HideCursor);
tag_parser!(show_cursor, b"\x1b[?25h", AnsiSequence::ShowCursor);
tag_parser!(cursor_to_app, b"\x1b[?1h", AnsiSequence::CursorToApp);
tag_parser!(set_new_line_mode, b"\x1b[20h", AnsiSequence::SetNewLineMode);
tag_parser!(set_col_132, b"\x1b[?3h", AnsiSequence::SetCol132);
tag_parser!(
    set_smooth_scroll,
    b"\x1b[?4h",
    AnsiSequence::SetSmoothScroll
);
tag_parser!(
    set_reverse_video,
    b"\x1b[?5h",
    AnsiSequence::SetReverseVideo
);
tag_parser!(set_origin_rel, b"\x1b[?6h", AnsiSequence::SetOriginRelative);
tag_parser!(set_auto_wrap, b"\x1b[?7h", AnsiSequence::SetAutoWrap);
tag_parser!(set_auto_repeat, b"\x1b[?8h", AnsiSequence::SetAutoRepeat);
tag_parser!(set_interlacing, b"\x1b[?9h", AnsiSequence::SetInterlacing);
tag_parser!(set_linefeed, b"\x1b[20l", AnsiSequence::SetLineFeedMode);
tag_parser!(
    set_cursorkey,
    b"\x1b[?1l",
    AnsiSequence::SetCursorKeyToCursor
);
tag_parser!(set_vt52, b"\x1b[?2l", AnsiSequence::SetVT52);
tag_parser!(set_col80, b"\x1b[?3l", AnsiSequence::SetCol80);
tag_parser!(set_jump_scroll, b"\x1b[?4l", AnsiSequence::SetJumpScrolling);
tag_parser!(set_normal_video, b"\x1b[?5l", AnsiSequence::SetNormalVideo);
tag_parser!(set_origin_abs, b"\x1b[?6l", AnsiSequence::SetOriginAbsolute);
tag_parser!(reset_auto_wrap, b"\x1b[?7l", AnsiSequence::ResetAutoWrap);
tag_parser!(
    reset_auto_repeat,
    b"\x1b[?8l",
    AnsiSequence::ResetAutoRepeat
);
tag_parser!(
    reset_interlacing,
    b"\x1b[?9l",
    AnsiSequence::ResetInterlacing
);

tag_parser!(
    set_alternate_keypad,
    b"\x1b=",
    AnsiSequence::SetAlternateKeypad
);
tag_parser!(set_numeric_keypad, b"\x1b>", AnsiSequence::SetNumericKeypad);
tag_parser!(set_uk_g0, b"\x1b(A", AnsiSequence::SetUKG0);
tag_parser!(set_uk_g1, b"\x1b)A", AnsiSequence::SetUKG1);
tag_parser!(set_us_g0, b"\x1b(B", AnsiSequence::SetUSG0);
tag_parser!(set_us_g1, b"\x1b)B", AnsiSequence::SetUSG1);
tag_parser!(set_g0_special, b"\x1b(0", AnsiSequence::SetG0SpecialChars);
tag_parser!(set_g1_special, b"\x1b)0", AnsiSequence::SetG1SpecialChars);
tag_parser!(
    set_g0_alternate,
    b"\x1b(1",
    AnsiSequence::SetG0AlternateChar
);
tag_parser!(
    set_g1_alternate,
    b"\x1b)1",
    AnsiSequence::SetG1AlternateChar
);
tag_parser!(
    set_g0_graph,
    b"\x1b(2",
    AnsiSequence::SetG0AltAndSpecialGraph
);
tag_parser!(
    set_g1_graph,
    b"\x1b)2",
    AnsiSequence::SetG1AltAndSpecialGraph
);
tag_parser!(set_single_shift2, b"\x1bN", AnsiSequence::SetSingleShift2);
tag_parser!(set_single_shift3, b"\x1bO", AnsiSequence::SetSingleShift3);

fn combined(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    // `alt` only supports up to 21 parsers, and nom doesn't seem to
    // have an alternative with higher variability.
    // So we simply nest them.
    alt((
        alt((
            // TODO - remove escape
            // escape,
            cursor_pos,
            cursor_up,
            cursor_down,
            cursor_forward,
            cursor_backward,
            cursor_save,
            cursor_restore,
            erase_display,
            erase_line,
            graphics_mode,
            set_mode,
            reset_mode,
            hide_cursor,
            show_cursor,
            cursor_to_app,
            set_new_line_mode,
            set_col_132,
            set_smooth_scroll,
            set_reverse_video,
            set_origin_rel,
        )),
        alt((
            set_auto_wrap,
            set_auto_repeat,
            set_interlacing,
            set_linefeed,
            set_cursorkey,
            set_vt52,
            set_col80,
            set_jump_scroll,
            set_normal_video,
            set_origin_abs,
            reset_auto_wrap,
            reset_auto_repeat,
            reset_interlacing,
            set_top_and_bottom,
            set_alternate_keypad,
            set_numeric_keypad,
            set_uk_g0,
            set_uk_g1,
            set_us_g0,
            set_us_g1,
            set_g0_special,
        )),
        set_g1_special,
        set_g0_alternate,
        set_g1_alternate,
        set_g0_graph,
        set_g1_graph,
        set_single_shift2,
        set_single_shift3,
    ))(input)
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
