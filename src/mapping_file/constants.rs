use std::clone::Clone;
use crate::parse_ansi_text::colors::*;
use crate::parse_ansi_text::style::*;

pub const DELIMITER: &str = "\n";

// LINE LENGTH IS THE MAXIMUM LENGTH THAT IS REQUIRED TO HAVE ALL SUPPORTED STYLES
pub const LINE_LENGTH: usize =
    BOLD_CODE.len() +
        ITALIC_CODE.len() +
        INVERSE_CODE.len() +
        UNDERLINE_CODE.len() +
        STRIKETHROUGH_CODE.len() +
        LARGEST_RGB_FOREGROUND_CODE.len() +
        LARGEST_RGB_BACKGROUND_CODE.len();

pub const FULL_LINE_LENGTH: usize = LINE_LENGTH + DELIMITER.len();
