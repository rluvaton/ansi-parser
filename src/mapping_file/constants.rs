use crate::parse_ansi_text::ansi::colors::*;
use crate::parse_ansi_text::ansi::style::*;

pub const DELIMITER: &str = "\n";

// LINE LENGTH IS THE MAXIMUM LENGTH THAT IS REQUIRED TO HAVE ALL SUPPORTED STYLES
pub const FIRST_PART_LINE_LENGTH: usize = BOLD_CODE.len()
    + ITALIC_CODE.len()
    + INVERSE_CODE.len()
    + UNDERLINE_CODE.len()
    + STRIKETHROUGH_CODE.len()
    + LARGEST_RGB_FOREGROUND_CODE.len()
    + LARGEST_RGB_BACKGROUND_CODE.len();

pub const SECOND_PART_LINE_LENGTH: usize = u64::MAX.to_ne_bytes().len();

pub const FULL_LINE_LENGTH: usize =
    FIRST_PART_LINE_LENGTH + SECOND_PART_LINE_LENGTH + DELIMITER.len();
