// Taken from ansi_parse and modify

use atoi::atoi;
use heapless::Vec;
use nom::branch::alt;
use nom::bytes::streaming::{tag, take, take_until};
use nom::character::streaming::{digit0, digit1};
use nom::combinator::{map, map_res, opt, value};
use nom::error::ErrorKind;
use nom::{AsBytes, error, IResult};
use nom::sequence::{delimited, preceded, tuple};

use crate::parse_ansi_text::raw_ansi_parse::enums::AnsiSequence;

const ESCAPE_AS_BYTES: &[u8] = "\u{1b}".as_bytes();
const EMPTY_AS_BYTES: &[u8] = &[];

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
        return atoi::<u8>(s).ok_or(nom::Err::Error(nom::error::Error::new(input, ErrorKind::Digit)));
    })(input)
}

// TODO kind of ugly, would prefer to pass in the default so we could use it for
// all escapes with defaults (not just those that default to 1).
fn parse_def_cursor_int(input: &[u8]) -> IResult<&[u8], u32> {
    map_res(digit1, |s: &[u8]| -> Result<u32, error::Error<&[u8]>> {
        let result = atoi::<u32>(s);
        
        if result.is_none() {
            return Ok(1);
        }
        
        return Ok(result.unwrap());
    })(input)
}

fn cursor_pos(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    map(
        tuple((
            tag("\u{1b}[".as_bytes()),
            parse_def_cursor_int,
            opt(tag(";".as_bytes())),
            parse_def_cursor_int,
            alt((tag("H".as_bytes()), tag("f".as_bytes()))),
        )),
        |(_, x, _, y, _)| AnsiSequence::CursorPos(x, y),
    )(input)
}

fn escape(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    value(AnsiSequence::Text("\u{1b}".as_bytes()), tag("\u{1b}".as_bytes()))(input)
}

fn cursor_up(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    preceded(tag("\u{1b}".as_bytes()), map(delimited(tag("[".as_bytes()), parse_def_cursor_int, tag("A".as_bytes())), |am| {
        AnsiSequence::CursorUp(am)
    }))(input)
}

fn cursor_down(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    preceded(tag("\u{1b}".as_bytes()), map(delimited(tag("[".as_bytes()), parse_def_cursor_int, tag("B".as_bytes())), |am| {
        AnsiSequence::CursorDown(am)
    }))(input)
}

fn cursor_forward(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    preceded(tag("\u{1b}".as_bytes()), map(delimited(tag("[".as_bytes()), parse_def_cursor_int, tag("C".as_bytes())), |am| {
        AnsiSequence::CursorForward(am)
    }))(input)
}

fn cursor_backward(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    preceded(tag("\u{1b}".as_bytes()), map(delimited(tag("[".as_bytes()), parse_def_cursor_int, tag("D".as_bytes())), |am| {
        AnsiSequence::CursorBackward(am)
    }))(input)
}

fn graphics_mode1(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    map(delimited(tag("\u{1b}[".as_bytes()), parse_u8, tag("m".as_bytes())), |val| {
        let mode =
            Vec::from_slice(&[val]).expect("Vec::from_slice should allocate sufficient size");
        AnsiSequence::SetGraphicsMode(mode)
    })(input)
}

fn graphics_mode2(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    map(
        tuple((tag("\u{1b}[".as_bytes()), parse_u8, tag(";".as_bytes()), parse_u8, tag("m".as_bytes()))),
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
            tag("\u{1b}[".as_bytes()),
            parse_u8,
            tag(";".as_bytes()),
            parse_u8,
            tag(";".as_bytes()),
            parse_u8,
            tag("m".as_bytes()),
        )),
        |(_, val1, _, val2, _, val3, _)| {
            let mode = Vec::from_slice(&[val1, val2, val3])
                .expect("Vec::from_slice should allocate sufficient size");
            AnsiSequence::SetGraphicsMode(mode)
        },
    )(input)
}

fn graphics_mode4(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    value(AnsiSequence::SetGraphicsMode(Vec::new()), tag("\u{1b}[m".as_bytes()))(input)
}

fn graphics_mode5(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    map(
        tuple((
            tag("\u{1b}[".as_bytes()),
            parse_u8,
            tag(";".as_bytes()),
            parse_u8,
            tag(";".as_bytes()),
            parse_u8,
            tag(";".as_bytes()),
            parse_u8,
            tag(";".as_bytes()),
            parse_u8,
            tag("m".as_bytes()),
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
    map(delimited(tag("\u{1b}[=".as_bytes()), parse_u8, tag("h".as_bytes())), |val| {
        AnsiSequence::SetMode(val)
    })(input)
}

fn reset_mode(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    map(delimited(tag("\u{1b}[=".as_bytes()), parse_u8, tag("l".as_bytes())), |val| {
        AnsiSequence::ResetMode(val)
    })(input)
}

fn set_top_and_bottom(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    preceded(tag("\u{1b}".as_bytes()), map(
        tuple((tag("[".as_bytes()), parse_u32, tag(";".as_bytes()), parse_u32, tag("r".as_bytes()))),
        |(_, x, _, y, _)| AnsiSequence::SetTopAndBottom(x, y),
    ))(input)
}

tag_parser!(cursor_save, "\u{1b}[s".as_bytes(), AnsiSequence::CursorSave);
tag_parser!(cursor_restore, "\u{1b}[u".as_bytes(), AnsiSequence::CursorRestore);
tag_parser!(erase_display, "\u{1b}[2J".as_bytes(), AnsiSequence::EraseDisplay);
tag_parser!(erase_line, "\u{1b}[K".as_bytes(), AnsiSequence::EraseLine);
tag_parser!(hide_cursor, "\u{1b}[?25l".as_bytes(), AnsiSequence::HideCursor);
tag_parser!(show_cursor, "\u{1b}[?25h".as_bytes(), AnsiSequence::ShowCursor);
tag_parser!(cursor_to_app, "\u{1b}[?1h".as_bytes(), AnsiSequence::CursorToApp);
tag_parser!(set_new_line_mode, "\u{1b}[20h".as_bytes(), AnsiSequence::SetNewLineMode);
tag_parser!(set_col_132, "\u{1b}[?3h".as_bytes(), AnsiSequence::SetCol132);
tag_parser!(set_smooth_scroll, "\u{1b}[?4h".as_bytes(), AnsiSequence::SetSmoothScroll);
tag_parser!(set_reverse_video, "\u{1b}[?5h".as_bytes(), AnsiSequence::SetReverseVideo);
tag_parser!(set_origin_rel, "\u{1b}[?6h".as_bytes(), AnsiSequence::SetOriginRelative);
tag_parser!(set_auto_wrap, "\u{1b}[?7h".as_bytes(), AnsiSequence::SetAutoWrap);
tag_parser!(set_auto_repeat, "\u{1b}[?8h".as_bytes(), AnsiSequence::SetAutoRepeat);
tag_parser!(set_interlacing, "\u{1b}[?9h".as_bytes(), AnsiSequence::SetInterlacing);
tag_parser!(set_linefeed, "\u{1b}[20l".as_bytes(), AnsiSequence::SetLineFeedMode);
tag_parser!(set_cursorkey, "\u{1b}[?1l".as_bytes(), AnsiSequence::SetCursorKeyToCursor);
tag_parser!(set_vt52, "\u{1b}[?2l".as_bytes(), AnsiSequence::SetVT52);
tag_parser!(set_col80, "\u{1b}[?3l".as_bytes(), AnsiSequence::SetCol80);
tag_parser!(set_jump_scroll, "\u{1b}[?4l".as_bytes(), AnsiSequence::SetJumpScrolling);
tag_parser!(set_normal_video, "\u{1b}[?5l".as_bytes(), AnsiSequence::SetNormalVideo);
tag_parser!(set_origin_abs, "\u{1b}[?6l".as_bytes(), AnsiSequence::SetOriginAbsolute);
tag_parser!(reset_auto_wrap, "\u{1b}[?7l".as_bytes(), AnsiSequence::ResetAutoWrap);
tag_parser!(reset_auto_repeat, "\u{1b}[?8l".as_bytes(), AnsiSequence::ResetAutoRepeat);
tag_parser!(reset_interlacing, "\u{1b}[?9l".as_bytes(), AnsiSequence::ResetInterlacing);

tag_parser!(set_alternate_keypad, "\u{1b}=".as_bytes(), AnsiSequence::SetAlternateKeypad);
tag_parser!(set_numeric_keypad, "\u{1b}>".as_bytes(), AnsiSequence::SetNumericKeypad);
tag_parser!(set_uk_g0, "\u{1b}(A".as_bytes(), AnsiSequence::SetUKG0);
tag_parser!(set_uk_g1, "\u{1b})A".as_bytes(), AnsiSequence::SetUKG1);
tag_parser!(set_us_g0, "\u{1b}(B".as_bytes(), AnsiSequence::SetUSG0);
tag_parser!(set_us_g1, "\u{1b})B".as_bytes(), AnsiSequence::SetUSG1);
tag_parser!(set_g0_special, "\u{1b}(0".as_bytes(), AnsiSequence::SetG0SpecialChars);
tag_parser!(set_g1_special, "\u{1b})0".as_bytes(), AnsiSequence::SetG1SpecialChars);
tag_parser!(set_g0_alternate, "\u{1b}(1".as_bytes(), AnsiSequence::SetG0AlternateChar);
tag_parser!(set_g1_alternate, "\u{1b})1".as_bytes(), AnsiSequence::SetG1AlternateChar);
tag_parser!(set_g0_graph, "\u{1b}(2".as_bytes(), AnsiSequence::SetG0AltAndSpecialGraph);
tag_parser!(set_g1_graph, "\u{1b})2".as_bytes(), AnsiSequence::SetG1AltAndSpecialGraph);
tag_parser!(set_single_shift2, "\u{1b}N".as_bytes(), AnsiSequence::SetSingleShift2);
tag_parser!(set_single_shift3, "\u{1b}O".as_bytes(), AnsiSequence::SetSingleShift3);

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

fn until_escape(s: &[u8]) -> IResult<&[u8], &[u8]> {
    take_until(ESCAPE_AS_BYTES)(s)
}

fn take_single(s: &[u8]) -> IResult<&[u8], &[u8]> {
    take(1usize)(s)
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
                    // TODO - avoid to string
                    return Ok((str, AnsiSequence::Text(matched_string)));
                }
            }
            Err(err) => {
                if complete_string && matches!(err, nom::Err::Incomplete(_) ) {
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
                    if matches!(sub_error.code, ErrorKind::Tag)  {
                        let single_res = take_single(input);
                        
                        if single_res.is_ok() {
                            let (str, matched_string) = single_res.unwrap();
                            // TODO - avoid to string
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
        assert_eq!(parse_escape(RED_BACKGROUND_CODE.as_bytes(), true), Ok((EMPTY_AS_BYTES, AnsiSequence::SetGraphicsMode(Vec::from_slice(&[41]).unwrap()))));
        assert_eq!(parse_escape(RESET_CODE.as_bytes(), true), Ok((EMPTY_AS_BYTES, AnsiSequence::SetGraphicsMode(Vec::from_slice(&[0]).unwrap()))));
    }
}
