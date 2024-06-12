// Taken from ansi_parse and modify

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


fn escape_codes(input: &[u8]) -> IResult<&[u8], AnsiSequence> {
    // removed the preceding tag so we can match it in the value
    graphics_mode(input)
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
                    // TODO - only reach here, in incomplete case, so need to return empty string?
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
