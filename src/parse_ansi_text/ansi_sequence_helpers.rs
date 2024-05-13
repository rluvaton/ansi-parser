use ansi_parser::AnsiSequence;
use crate::parse_ansi_text::colors::*;
use crate::parse_ansi_text::style::*;

pub enum AnsiSequenceType {
    Unsupported,
    Reset,
    ForegroundColor(Color),
    BackgroundColor(Color),
    Brightness(Brightness),
    TextStyle(TextStyle),
}

pub fn get_type_from_ansi_sequence(seq: &AnsiSequence) -> AnsiSequenceType {
    if !is_ansi_sequence_code_supported(&seq) {
        println!("Unsupported ansi sequence: {:?}", seq);
        return AnsiSequenceType::Unsupported;
    }

    println!("Supported Ansi sequence: {:?}", seq);
    
    // Instead of match as we only support single, change to if not set graphics mode panic

    match seq {
        // TODO - what it means?
        AnsiSequence::SetGraphicsMode(vec) => {
            println!("SetGraphicsMode: {:?}", vec);
            
            if vec.len() == 0 {
                println!("Unrecognized graphics mode: {:?}", vec);
                return AnsiSequenceType::Unsupported;
            }
            
            if vec[0] == 0 {
                return AnsiSequenceType::Reset;
            }
            
            let color_type = get_color_type(vec);
            
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
        },

        _ => {
            // Should not be here
            panic!("supported ANSI sequence have no handling: {:?}", seq);
        }
    }
    
            return AnsiSequenceType::Unsupported;
}

pub fn is_ansi_sequence_code_supported(seq: &AnsiSequence) -> bool {
    let supported = match seq {

        // TODO - what it means?
        AnsiSequence::SetGraphicsMode(_) => true,

        // -- Unsupported --

        // TODO - change to _ for all unsupported

        // TODO - what it means?
        AnsiSequence::Escape => false,

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