// use ansi_parser::AnsiSequence;
use crate::parse_ansi_text::ansi::colors::*;
use crate::parse_ansi_text::ansi::style::*;
use crate::parse_ansi_text::raw_ansi_parse::parsers::ESCAPE_AS_BYTES;
use crate::parse_ansi_text::raw_ansi_parse::AnsiSequence;

pub enum AnsiSequenceType {
    Unsupported,
    Reset,
    ForegroundColor(Color),
    BackgroundColor(Color),
    Brightness(Brightness),
    TextStyle(TextStyle),
}

pub fn old_ansi_sequence_to_new<'a>(seq: ansi_parser::AnsiSequence) -> AnsiSequence<'a> {
    match seq {
        ansi_parser::AnsiSequence::Escape => AnsiSequence::Text(ESCAPE_AS_BYTES),
        ansi_parser::AnsiSequence::CursorPos(a, b) => AnsiSequence::CursorPos(a, b),
        ansi_parser::AnsiSequence::CursorUp(a) => AnsiSequence::CursorUp(a),
        ansi_parser::AnsiSequence::CursorDown(a) => AnsiSequence::CursorDown(a),
        ansi_parser::AnsiSequence::CursorForward(a) => AnsiSequence::CursorForward(a),
        ansi_parser::AnsiSequence::CursorBackward(a) => AnsiSequence::CursorBackward(a),
        ansi_parser::AnsiSequence::CursorSave => AnsiSequence::CursorSave,
        ansi_parser::AnsiSequence::CursorRestore => AnsiSequence::CursorRestore,
        ansi_parser::AnsiSequence::EraseDisplay => AnsiSequence::EraseDisplay,
        ansi_parser::AnsiSequence::EraseLine => AnsiSequence::EraseLine,
        // TODO - use SetGraphicsMode1Byte when only 1 byte
        ansi_parser::AnsiSequence::SetGraphicsMode(a) => AnsiSequence::SetGraphicsMode(a),
        ansi_parser::AnsiSequence::SetMode(a) => AnsiSequence::SetMode(a),
        ansi_parser::AnsiSequence::ResetMode(a) => AnsiSequence::ResetMode(a),
        ansi_parser::AnsiSequence::HideCursor => AnsiSequence::HideCursor,
        ansi_parser::AnsiSequence::ShowCursor => AnsiSequence::ShowCursor,
        ansi_parser::AnsiSequence::CursorToApp => AnsiSequence::CursorToApp,
        ansi_parser::AnsiSequence::SetNewLineMode => AnsiSequence::SetNewLineMode,
        ansi_parser::AnsiSequence::SetCol132 => AnsiSequence::SetCol132,
        ansi_parser::AnsiSequence::SetSmoothScroll => AnsiSequence::SetSmoothScroll,
        ansi_parser::AnsiSequence::SetReverseVideo => AnsiSequence::SetReverseVideo,
        ansi_parser::AnsiSequence::SetOriginRelative => AnsiSequence::SetOriginRelative,
        ansi_parser::AnsiSequence::SetAutoWrap => AnsiSequence::SetAutoWrap,
        ansi_parser::AnsiSequence::SetAutoRepeat => AnsiSequence::SetAutoRepeat,
        ansi_parser::AnsiSequence::SetInterlacing => AnsiSequence::SetInterlacing,
        ansi_parser::AnsiSequence::SetLineFeedMode => AnsiSequence::SetLineFeedMode,
        ansi_parser::AnsiSequence::SetCursorKeyToCursor => AnsiSequence::SetCursorKeyToCursor,
        ansi_parser::AnsiSequence::SetVT52 => AnsiSequence::SetVT52,
        ansi_parser::AnsiSequence::SetCol80 => AnsiSequence::SetCol80,
        ansi_parser::AnsiSequence::SetJumpScrolling => AnsiSequence::SetJumpScrolling,
        ansi_parser::AnsiSequence::SetNormalVideo => AnsiSequence::SetNormalVideo,
        ansi_parser::AnsiSequence::SetOriginAbsolute => AnsiSequence::SetOriginAbsolute,
        ansi_parser::AnsiSequence::ResetAutoWrap => AnsiSequence::ResetAutoWrap,
        ansi_parser::AnsiSequence::ResetAutoRepeat => AnsiSequence::ResetAutoRepeat,
        ansi_parser::AnsiSequence::ResetInterlacing => AnsiSequence::ResetInterlacing,
        ansi_parser::AnsiSequence::SetAlternateKeypad => AnsiSequence::SetAlternateKeypad,
        ansi_parser::AnsiSequence::SetNumericKeypad => AnsiSequence::SetNumericKeypad,
        ansi_parser::AnsiSequence::SetUKG0 => AnsiSequence::SetUKG0,
        ansi_parser::AnsiSequence::SetUKG1 => AnsiSequence::SetUKG1,
        ansi_parser::AnsiSequence::SetUSG0 => AnsiSequence::SetUSG0,
        ansi_parser::AnsiSequence::SetUSG1 => AnsiSequence::SetUSG1,
        ansi_parser::AnsiSequence::SetG0SpecialChars => AnsiSequence::SetG0SpecialChars,
        ansi_parser::AnsiSequence::SetG1SpecialChars => AnsiSequence::SetG1SpecialChars,
        ansi_parser::AnsiSequence::SetG0AlternateChar => AnsiSequence::SetG0AlternateChar,
        ansi_parser::AnsiSequence::SetG1AlternateChar => AnsiSequence::SetG1AlternateChar,
        ansi_parser::AnsiSequence::SetG0AltAndSpecialGraph => AnsiSequence::SetG0AltAndSpecialGraph,
        ansi_parser::AnsiSequence::SetG1AltAndSpecialGraph => AnsiSequence::SetG1AltAndSpecialGraph,
        ansi_parser::AnsiSequence::SetSingleShift2 => AnsiSequence::SetSingleShift2,
        ansi_parser::AnsiSequence::SetSingleShift3 => AnsiSequence::SetSingleShift3,
        ansi_parser::AnsiSequence::SetTopAndBottom(a, b) => AnsiSequence::SetTopAndBottom(a, b),
    }
}

pub fn get_type_from_ansi_sequence(seq: &AnsiSequence) -> AnsiSequenceType {
    if !is_ansi_sequence_code_supported(&seq) {
        println!("Unsupported ansi sequence: {:?}", seq);
        return AnsiSequenceType::Unsupported;
    }

    // println!("Supported Ansi sequence: {:?}", seq);

    // Instead of match as we only support single, change to if not set graphics mode panic

    match seq {
        // TODO - what it means?
        AnsiSequence::SetGraphicsMode1Byte(byte) => {
            return get_style_type(*byte);
        }
        AnsiSequence::SetGraphicsModePredefinedColor(predefined_color) => {
            return get_predefined_color_type(*predefined_color);
        }
        AnsiSequence::SetGraphicsModeEightBitColor(color_type, color) => {
            return get_8_bit_color_type(*color_type, *color);
        }
        AnsiSequence::SetGraphicsMode(vec) => {
            // println!("SetGraphicsMode: {:?}", vec);

            if vec.len() == 0 {
                println!("Unrecognized graphics mode: {:?}", vec);
                return AnsiSequenceType::Unsupported;
            }

            if vec[0] == 0 {
                return AnsiSequenceType::Reset;
            }

            let color_type = get_color_type(vec);

            // TODO - should replace here default with none color?
            match color_type {
                ColorType::Foreground(color) => {
                    return AnsiSequenceType::ForegroundColor(color);
                }
                ColorType::Background(color) => {
                    return AnsiSequenceType::BackgroundColor(color);
                }
                _ => {}
            }

            let brightness = get_brightness_type(vec[0]);

            if brightness != Brightness::None {
                return AnsiSequenceType::Brightness(brightness);
            }

            let style = get_text_style_type(vec[0]);

            if style != TextStyle::None {
                return AnsiSequenceType::TextStyle(style);
            }

            println!("Unrecognized graphics mode: {:?}", vec);
        }

        _ => {
            // Should not be here
            panic!("supported ANSI sequence have no handling: {:?}", seq);
        }
    }

    return AnsiSequenceType::Unsupported;
}

pub fn is_ansi_sequence_code_supported(seq: &AnsiSequence) -> bool {
    let supported = match seq {
        AnsiSequence::Text(_) => true,

        // TODO - what it means?
        AnsiSequence::SetGraphicsMode(_) => true,
        AnsiSequence::SetGraphicsModeSimd(_) => true,
        AnsiSequence::SetGraphicsMode1Byte(_) => true,
        AnsiSequence::SetGraphicsModeEightBitColor(_, _) => true,
        AnsiSequence::SetGraphicsModePredefinedColor(_) => true,

        // -- Unsupported --

        // TODO - change to _ for all unsupported

        // TODO - what it means?
        // AnsiSequence::Escape => false,

        // TODO - what it means?
        AnsiSequence::SetMode(_) => false,

        // TODO - what it means?
        AnsiSequence::ResetMode(_) => false,

        AnsiSequence::CursorPos(_, _) => false,
        AnsiSequence::CursorUp(_) => false,
        AnsiSequence::CursorDown(_) => false,
        AnsiSequence::CursorForward(_) => false,
        AnsiSequence::CursorBackward(_) => false,
        AnsiSequence::CursorSave => false,
        AnsiSequence::CursorRestore => false,

        AnsiSequence::EraseDisplay => false,
        AnsiSequence::EraseLine => false,

        AnsiSequence::HideCursor => false,
        AnsiSequence::ShowCursor => false,
        AnsiSequence::CursorToApp => false,
        AnsiSequence::SetNewLineMode => false,
        AnsiSequence::SetCol132 => false,
        AnsiSequence::SetSmoothScroll => false,
        AnsiSequence::SetReverseVideo => false,
        AnsiSequence::SetOriginRelative => false,
        AnsiSequence::SetAutoWrap => false,
        AnsiSequence::SetAutoRepeat => false,
        AnsiSequence::SetInterlacing => false,
        AnsiSequence::SetLineFeedMode => false,
        AnsiSequence::SetCursorKeyToCursor => false,
        AnsiSequence::SetVT52 => false,
        AnsiSequence::SetCol80 => false,
        AnsiSequence::SetJumpScrolling => false,
        AnsiSequence::SetNormalVideo => false,
        AnsiSequence::SetOriginAbsolute => false,
        AnsiSequence::ResetAutoWrap => false,
        AnsiSequence::ResetAutoRepeat => false,
        AnsiSequence::ResetInterlacing => false,
        AnsiSequence::SetAlternateKeypad => false,
        AnsiSequence::SetNumericKeypad => false,
        AnsiSequence::SetUKG0 => false,
        AnsiSequence::SetUKG1 => false,
        AnsiSequence::SetUSG0 => false,
        AnsiSequence::SetUSG1 => false,
        AnsiSequence::SetG0SpecialChars => false,
        AnsiSequence::SetG1SpecialChars => false,
        AnsiSequence::SetG0AlternateChar => false,
        AnsiSequence::SetG1AlternateChar => false,
        AnsiSequence::SetG0AltAndSpecialGraph => false,
        AnsiSequence::SetG1AltAndSpecialGraph => false,
        AnsiSequence::SetSingleShift2 => false,
        AnsiSequence::SetSingleShift3 => false,
        AnsiSequence::SetTopAndBottom(_, _) => false,
    };

    return supported;
}
