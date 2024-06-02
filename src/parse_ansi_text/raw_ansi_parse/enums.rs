// Taken from ansi_parse and modify

///The following are the implemented ANSI escape sequences. More to be added.
#[derive(Debug, PartialEq, Clone)]
pub enum AnsiSequence<'a> {
    // TODO - change to &str?
    Text(&'a [u8]),
    CursorPos(u32, u32),
    CursorUp(u32),
    CursorDown(u32),
    CursorForward(u32),
    CursorBackward(u32),
    CursorSave,
    CursorRestore,
    EraseDisplay,
    EraseLine,
    SetGraphicsMode(heapless::Vec<u8, 5>),
    SetMode(u8),
    ResetMode(u8),
    HideCursor,
    ShowCursor,
    CursorToApp,
    SetNewLineMode,
    SetCol132,
    SetSmoothScroll,
    SetReverseVideo,
    SetOriginRelative,
    SetAutoWrap,
    SetAutoRepeat,
    SetInterlacing,
    SetLineFeedMode,
    SetCursorKeyToCursor,
    SetVT52,
    SetCol80,
    SetJumpScrolling,
    SetNormalVideo,
    SetOriginAbsolute,
    ResetAutoWrap,
    ResetAutoRepeat,
    ResetInterlacing,
    SetAlternateKeypad,
    SetNumericKeypad,
    SetUKG0,
    SetUKG1,
    SetUSG0,
    SetUSG1,
    SetG0SpecialChars,
    SetG1SpecialChars,
    SetG0AlternateChar,
    SetG1AlternateChar,
    SetG0AltAndSpecialGraph,
    SetG1AltAndSpecialGraph,
    SetSingleShift2,
    SetSingleShift3,
    SetTopAndBottom(u32, u32),
}

use core::fmt::{Display, Formatter, Result as DisplayResult};
impl Display for AnsiSequence<'_> {
    fn fmt(&self, formatter: &mut Formatter) -> DisplayResult {
        write!(formatter, "\u{1b}")?;

        use AnsiSequence::*;
        match self {
            Text(text) => write!(formatter, "{}", String::from_utf8(text.to_vec()).unwrap()),
            SetGraphicsMode(vec) => match vec.len() {
                0 => write!(formatter, "[m"),
                1 => write!(formatter, "[{}m", vec[0]),
                2 => write!(formatter, "[{};{}m", vec[0], vec[1]),
                3 => write!(formatter, "[{};{};{}m", vec[0], vec[1], vec[2]),
                5 => write!(
                    formatter,
                    "[{};{};{};{};{}m",
                    vec[0], vec[1], vec[2], vec[3], vec[4]
                ),
                _ => unreachable!(),
            },
            SetMode(mode) => write!(formatter, "[={}h", mode),
            ResetMode(mode) => write!(formatter, "[={}l", mode),
            _ => write!(formatter, "<other>"),
        }
    }
}
