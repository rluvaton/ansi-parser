use crate::parse_ansi_text::raw_ansi_parse::{AnsiSequence, Output, parse_escape, Text};

#[derive(Debug, PartialEq, Clone)]
pub struct ParseAnsiResult<'a> {
    pub output: Option<Output<'a>>,
    pub size: usize,
    pub pending_string: &'a [u8],
}

pub fn parse_ansi_continues(
    value: &[u8],
) -> ParseAnsiResult {
    let mut buf = value;
    let pending_text_size_before = buf.len();

    return match parse_escape(buf, true) {
        Ok((pending, seq)) => {
            buf = pending;

            let size = pending_text_size_before - buf.len();

            match seq {
                AnsiSequence::Text(str) => ParseAnsiResult {
                    output: Some(Output::TextBlock(Text {
                        text: str,
                    })),
                    size,
                    pending_string: buf,
                },
                _ => ParseAnsiResult {
                    output: Some(Output::Escape(seq)),
                    pending_string: buf,
                    size
                },
            }
        }
        Err(_) => ParseAnsiResult {
            output: None,
            size: 0,
            pending_string: buf,
        },
    };
}
