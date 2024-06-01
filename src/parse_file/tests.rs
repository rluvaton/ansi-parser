#[cfg(test)]
use pretty_assertions::assert_eq;
use tempfile::NamedTempFile;
use test_case::test_case;

use crate::files::file_reader::FileReaderOptions;
use crate::parse_ansi_text::ansi::colors::*;
use crate::parse_ansi_text::ansi::constants::*;
use crate::parse_ansi_text::ansi::style::*;
use crate::parse_ansi_text::ansi::types::*;
use crate::parse_ansi_text::parse_options::ParseOptions;
use crate::parse_file::file_to_lines_of_spans::read_ansi_file_to_lines;
use crate::parse_file::file_to_spans::read_ansi_file_to_spans;
use crate::parse_file::types::ReadAnsiFileOptions;
use crate::types::Line;

fn get_tmp_file_path() -> String {
    return NamedTempFile::new()
        .expect("create temp file")
        .into_temp_path()
        .to_str()
        .expect("convert to string")
        .to_string();
}

fn create_tmp_file(input: String) -> String {
    let file_path = get_tmp_file_path();

    std::fs::write(file_path.clone(), input).expect("Failed to write to file");

    return file_path;
}

fn parse_ansi_text_with_options(input: &str, parse_options: ParseOptions) -> Vec<Span> {
    let tmp_file_path = create_tmp_file(input.to_string());
    let spans_iterator = read_ansi_file_to_spans(ReadAnsiFileOptions {
        parse_options,
        file_options: FileReaderOptions {
            file_path: tmp_file_path,

            chunk_size_in_bytes: None,
            from_bytes: None,
            to_bytes: None,
        },
    });

    return spans_iterator.collect();
}

fn parse_ansi_text(input: &str) -> Vec<Span> {
    return parse_ansi_text_with_options(input, ParseOptions::default());
}

fn parse_ansi_text_split_by_lines_with_options(
    input: &str,
    parse_options: ParseOptions,
) -> Vec<Line> {
    let tmp_file_path = create_tmp_file(input.to_string());
    let lines_iterator = read_ansi_file_to_lines(ReadAnsiFileOptions {
        parse_options,
        file_options: FileReaderOptions {
            file_path: tmp_file_path,

            chunk_size_in_bytes: None,
            from_bytes: None,
            to_bytes: None,
        },
    });

    return lines_iterator.collect();
}

fn parse_ansi_text_split_by_lines(input: &str) -> Vec<Line> {
    return parse_ansi_text_split_by_lines_with_options(input, ParseOptions::default());
}

#[test]
fn empty_text_should_return_empty_array() {
    let input = "";
    let expected = vec![];
    assert_eq!(parse_ansi_text(input), expected);
}

// -------------
// No ANSI codes
// -------------

#[test]
fn single_text_without_ansi_codes_should_return_array_with_one_unstyled_span() {
    let input = "Hello, world!";
    let expected = vec![Span::empty().with_text("Hello, world!".to_string().into_bytes())];
    assert_eq!(parse_ansi_text(input), expected);
}

#[test]
fn multiline_text_without_ansi_codes_should_return_array_with_one_unstyled_span() {
    let input = "Hello, world!\nhow are you";
    let expected =
        vec![Span::empty().with_text("Hello, world!\nhow are you".to_string().into_bytes())];
    assert_eq!(parse_ansi_text(input), expected);
}

// -------------
// Single style
// -------------

// TODO - add tests for RGB and 8-bit colors
// TODO - add test for default color

#[test_case(Color::Red, RED_FOREGROUND_CODE; "Red foreground")]
#[test_case(Color::Black, BLACK_FOREGROUND_CODE; "Black foreground")]
#[test_case(Color::Green, GREEN_FOREGROUND_CODE; "Green foreground")]
#[test_case(Color::Yellow, YELLOW_FOREGROUND_CODE; "Yellow foreground")]
#[test_case(Color::Blue, BLUE_FOREGROUND_CODE; "Blue foreground")]
#[test_case(Color::Magenta, MAGENTA_FOREGROUND_CODE; "Magenta foreground")]
#[test_case(Color::Cyan, CYAN_FOREGROUND_CODE; "Cyan foreground")]
#[test_case(Color::White, WHITE_FOREGROUND_CODE; "White foreground")]
#[test_case(Color::BrightRed, BRIGHT_RED_FOREGROUND_CODE; "Bright Red foreground")]
#[test_case(Color::BrightBlack, BRIGHT_BLACK_FOREGROUND_CODE; "Bright Black foreground")]
#[test_case(Color::BrightGreen, BRIGHT_GREEN_FOREGROUND_CODE; "Bright Green foreground")]
#[test_case(Color::BrightYellow, BRIGHT_YELLOW_FOREGROUND_CODE; "Bright Yellow foreground")]
#[test_case(Color::BrightBlue, BRIGHT_BLUE_FOREGROUND_CODE; "Bright Blue foreground")]
#[test_case(Color::BrightMagenta, BRIGHT_MAGENTA_FOREGROUND_CODE; "Bright Magenta foreground")]
#[test_case(Color::BrightCyan, BRIGHT_CYAN_FOREGROUND_CODE; "Bright Cyan foreground")]
#[test_case(Color::BrightWhite, BRIGHT_WHITE_FOREGROUND_CODE; "Bright White foreground")]
async fn single_foreground_color_with_no_other_styles(expected_color: Color, color_code: &str) {
    let input = [color_code, "Hello, world!", RESET_CODE].join("");
    let expected = vec![Span::empty()
        .with_color(expected_color)
        .with_text("Hello, world!".to_string().into_bytes())];
    assert_eq!(parse_ansi_text(&input), expected);
}

#[test_case(Color::Red, RED_BACKGROUND_CODE; "Red background")]
#[test_case(Color::Black, BLACK_BACKGROUND_CODE; "Black background")]
#[test_case(Color::Green, GREEN_BACKGROUND_CODE; "Green background")]
#[test_case(Color::Yellow, YELLOW_BACKGROUND_CODE; "Yellow background")]
#[test_case(Color::Blue, BLUE_BACKGROUND_CODE; "Blue background")]
#[test_case(Color::Magenta, MAGENTA_BACKGROUND_CODE; "Magenta background")]
#[test_case(Color::Cyan, CYAN_BACKGROUND_CODE; "Cyan background")]
#[test_case(Color::White, WHITE_BACKGROUND_CODE; "White background")]
#[test_case(Color::BrightRed, BRIGHT_RED_BACKGROUND_CODE; "Bright Red background")]
#[test_case(Color::BrightBlack, BRIGHT_BLACK_BACKGROUND_CODE; "Bright Black background")]
#[test_case(Color::BrightGreen, BRIGHT_GREEN_BACKGROUND_CODE; "Bright Green background")]
#[test_case(Color::BrightYellow, BRIGHT_YELLOW_BACKGROUND_CODE; "Bright Yellow background")]
#[test_case(Color::BrightBlue, BRIGHT_BLUE_BACKGROUND_CODE; "Bright Blue background")]
#[test_case(Color::BrightMagenta, BRIGHT_MAGENTA_BACKGROUND_CODE; "Bright Magenta background")]
#[test_case(Color::BrightCyan, BRIGHT_CYAN_BACKGROUND_CODE; "Bright Cyan background")]
#[test_case(Color::BrightWhite, BRIGHT_WHITE_BACKGROUND_CODE; "Bright White background")]
async fn single_background_color_with_no_other_styles(expected_color: Color, color_code: &str) {
    let input = [color_code, "Hello, world!", RESET_CODE].join("");
    let expected = vec![Span::empty()
        .with_bg_color(expected_color)
        .with_text("Hello, world!".to_string().into_bytes())];
    assert_eq!(parse_ansi_text(&input), expected);
}

#[test_case(TextStyle::Italic, ITALIC_CODE; "Italic text")]
#[test_case(TextStyle::Underline, UNDERLINE_CODE; "Underline text")]
#[test_case(TextStyle::Inverse, INVERSE_CODE; "Inverse text")]
#[test_case(TextStyle::Strikethrough, STRIKETHROUGH_CODE; "Strikethrough text")]
async fn single_text_style_with_no_other_styles(
    expected_text_style: TextStyle,
    text_style_code: &str,
) {
    let input = [text_style_code, "Hello, world!", RESET_CODE].join("");
    let expected = vec![Span::empty()
        .with_text_style(expected_text_style)
        .with_text("Hello, world!".to_string().into_bytes())];
    assert_eq!(parse_ansi_text(&input), expected);
}

#[test_case(Brightness::Bold, BOLD_CODE; "Bold text")]
#[test_case(Brightness::Dim, DIM_CODE; "Dim text")]
async fn single_brightness_with_no_other_styles(
    expected_brightness: Brightness,
    brightness_code: &str,
) {
    let input = [brightness_code, "Hello, world!", RESET_CODE].join("");
    let expected = vec![Span::empty()
        .with_brightness(expected_brightness)
        .with_text("Hello, world!".to_string().into_bytes())];
    assert_eq!(parse_ansi_text(&input), expected);
}

// -----------------------------------------------------------------------
// color/brightness override when no text before and without reset
// -----------------------------------------------------------------------

#[test]
fn foreground_color_should_replace_prev_foreground_color_when_no_text_in_between() {
    let input = [
        BLACK_FOREGROUND_CODE,
        RED_FOREGROUND_CODE,
        "Hello, world!",
        RESET_CODE,
    ]
        .join("");
    let expected = vec![Span {
        color: Color::Red,

        text: "Hello, world!".to_string().into_bytes(),
        bg_color: Color::None,
        brightness: Brightness::None,
        text_style: TextStyle::None,
    }];
    assert_eq!(parse_ansi_text(&input), expected);
}

#[test]
fn background_color_should_replace_prev_background_color_when_no_text_in_between() {
    let input = [
        BLACK_BACKGROUND_CODE,
        RED_BACKGROUND_CODE,
        "Hello, world!",
        RESET_CODE,
    ]
        .join("");
    let expected = vec![Span {
        bg_color: Color::Red,

        color: Color::None,
        text: "Hello, world!".to_string().into_bytes(),
        brightness: Brightness::None,
        text_style: TextStyle::None,
    }];
    assert_eq!(parse_ansi_text(&input), expected);
}

#[test]
fn brightness_should_replace_prev_brightness_when_no_text_in_between() {
    let input = [BOLD_CODE, DIM_CODE, "Hello, world!", RESET_CODE].join("");
    let expected = vec![Span {
        brightness: Brightness::Dim,

        color: Color::None,
        bg_color: Color::None,
        text: "Hello, world!".to_string().into_bytes(),
        text_style: TextStyle::None,
    }];
    assert_eq!(parse_ansi_text(&input), expected);
}

// -----------------------------------------------------------------------
// style override when no text before and with reset
// -----------------------------------------------------------------------

#[test]
fn foreground_color_should_replace_prev_foreground_color_after_reset_when_no_text_in_between() {
    let input = [
        BLACK_FOREGROUND_CODE,
        RESET_CODE,
        RED_FOREGROUND_CODE,
        "Hello, world!",
        RESET_CODE,
    ]
        .join("");
    let expected = vec![Span {
        color: Color::Red,

        text: "Hello, world!".to_string().into_bytes(),
        bg_color: Color::None,
        brightness: Brightness::None,
        text_style: TextStyle::None,
    }];
    assert_eq!(parse_ansi_text(&input), expected);
}

#[test]
fn background_color_should_replace_prev_background_color_after_reset_when_no_text_in_between() {
    let input = [
        BLACK_BACKGROUND_CODE,
        RESET_CODE,
        RED_BACKGROUND_CODE,
        "Hello, world!",
        RESET_CODE,
    ]
        .join("");
    let expected = vec![Span {
        bg_color: Color::Red,

        color: Color::None,
        text: "Hello, world!".to_string().into_bytes(),
        brightness: Brightness::None,
        text_style: TextStyle::None,
    }];
    assert_eq!(parse_ansi_text(&input), expected);
}

#[test]
fn brightness_should_replace_prev_brightness_after_reset_when_no_text_in_between() {
    let input = [BOLD_CODE, RESET_CODE, DIM_CODE, "Hello, world!", RESET_CODE].join("");
    let expected = vec![Span {
        brightness: Brightness::Dim,

        color: Color::None,
        bg_color: Color::None,
        text: "Hello, world!".to_string().into_bytes(),
        text_style: TextStyle::None,
    }];
    assert_eq!(parse_ansi_text(&input), expected);
}

#[test]
fn text_style_should_replace_prev_text_style_after_reset_when_no_text_in_between() {
    let input = [
        ITALIC_CODE,
        RESET_CODE,
        UNDERLINE_CODE,
        "Hello, world!",
        RESET_CODE,
    ]
        .join("");
    let expected = vec![Span {
        text_style: TextStyle::Underline,

        color: Color::None,
        bg_color: Color::None,
        brightness: Brightness::None,
        text: "Hello, world!".to_string().into_bytes(),
    }];
    assert_eq!(parse_ansi_text(&input), expected);
}

// ---------------------------------------------------------------------------------------
// Color/Style/Brightness changed after some text without reset and no other style before
// ---------------------------------------------------------------------------------------

#[test]
fn when_foreground_color_change_after_some_text_without_reset_should_create_a_new_span_with_new_foreground_color() {
    let input = [
        BLACK_FOREGROUND_CODE,
        "Hello, world!",
        RED_FOREGROUND_CODE,
        "How are you?",
        RESET_CODE,
    ]
        .join("");
    let expected = vec![
        Span {
            color: Color::Black,

            text: "Hello, world!".to_string().into_bytes(),
            bg_color: Color::None,
            brightness: Brightness::None,
            text_style: TextStyle::None,
        },
        Span {
            color: Color::Red,

            text: "How are you?".to_string().into_bytes(),
            bg_color: Color::None,
            brightness: Brightness::None,
            text_style: TextStyle::None,
        },
    ];
    assert_eq!(parse_ansi_text(&input), expected);
}

#[test]
fn when_rgb_values_in_foreground_color_change_after_some_text_without_reset_should_create_a_new_span_with_new_foreground_color() {
    let input = [
        RGB_FOREGROUND_CODE(188, 29, 68).as_str(),
        "Hello, world!",
        RGB_FOREGROUND_CODE(255, 19, 94).as_str(),
        "How are you?",
        RESET_CODE,
    ]
        .join("");
    let expected = vec![
        Span {
            color: Color::Rgb(188, 29, 68),

            text: "Hello, world!".to_string().into_bytes(),
            bg_color: Color::None,
            brightness: Brightness::None,
            text_style: TextStyle::None,
        },
        Span {
            color: Color::Rgb(255, 19, 94),

            text: "How are you?".to_string().into_bytes(),
            bg_color: Color::None,
            brightness: Brightness::None,
            text_style: TextStyle::None,
        },
    ];
    assert_eq!(parse_ansi_text(&input), expected);
}

#[test]
fn when_background_color_change_after_some_text_without_reset_should_create_a_new_span_with_new_background_color() {
    let input = [
        BLACK_BACKGROUND_CODE,
        "Hello, world!",
        RED_BACKGROUND_CODE,
        "How are you?",
        RESET_CODE,
    ]
        .join("");
    let expected = vec![
        Span {
            bg_color: Color::Black,

            text: "Hello, world!".to_string().into_bytes(),
            color: Color::None,
            brightness: Brightness::None,
            text_style: TextStyle::None,
        },
        Span {
            bg_color: Color::Red,

            text: "How are you?".to_string().into_bytes(),
            color: Color::None,
            brightness: Brightness::None,
            text_style: TextStyle::None,
        },
    ];
    assert_eq!(parse_ansi_text(&input), expected);
}

#[test]
fn when_rgb_background_color_change_after_some_text_without_reset_should_create_a_new_span_with_new_background_color() {
    let input = [
        RGB_BACKGROUND_CODE(188, 29, 68).as_str(),
        "Hello, world!",
        RGB_BACKGROUND_CODE(255, 19, 94).as_str(),
        "How are you?",
        RESET_CODE,
    ]
        .join("");
    let expected = vec![
        Span {
            bg_color: Color::Rgb(188, 29, 68),

            text: "Hello, world!".to_string().into_bytes(),
            color: Color::None,
            brightness: Brightness::None,
            text_style: TextStyle::None,
        },
        Span {
            bg_color: Color::Rgb(255, 19, 94),

            text: "How are you?".to_string().into_bytes(),
            color: Color::None,
            brightness: Brightness::None,
            text_style: TextStyle::None,
        },
    ];
    assert_eq!(parse_ansi_text(&input), expected);
}

#[test]
fn when_brightness_change_after_some_text_without_reset_should_create_a_new_span_with_new_brightness() {
    let input = [
        BOLD_CODE,
        "Hello, world!",
        DIM_CODE,
        "How are you?",
        RESET_CODE,
    ]
        .join("");
    let expected = vec![
        Span {
            brightness: Brightness::Bold,

            text: "Hello, world!".to_string().into_bytes(),
            color: Color::None,
            bg_color: Color::None,
            text_style: TextStyle::None,
        },
        Span {
            brightness: Brightness::Dim,

            text: "How are you?".to_string().into_bytes(),
            color: Color::None,
            bg_color: Color::None,
            text_style: TextStyle::None,
        },
    ];
    assert_eq!(parse_ansi_text(&input), expected);
}

#[test]
fn when_text_style_change_after_some_text_without_reset_should_create_a_new_span_with_merged_text_style() {
    let input = [
        ITALIC_CODE,
        "Hello, world!",
        UNDERLINE_CODE,
        "How are you?",
        RESET_CODE,
    ]
        .join("");
    let expected = vec![
        Span {
            text_style: TextStyle::Italic,

            text: "Hello, world!".to_string().into_bytes(),
            bg_color: Color::None,
            brightness: Brightness::None,
            color: Color::None,
        },
        Span {
            text_style: TextStyle::Italic | TextStyle::Underline,

            text: "How are you?".to_string().into_bytes(),
            bg_color: Color::None,
            brightness: Brightness::None,
            color: Color::None,
        },
    ];
    assert_eq!(parse_ansi_text(&input), expected);
}

// ----------------------------------------------------------------------------------------------
// Color/Style/Brightness changed after some text without text afterward with other style before
// ----------------------------------------------------------------------------------------------

#[test]
fn when_foreground_color_change_after_some_text_without_reset_should_create_a_new_span_with_prev_style_and_new_foreground_color() {
    let input = [
        ITALIC_CODE,
        BOLD_CODE,
        WHITE_BACKGROUND_CODE,
        BLACK_FOREGROUND_CODE,
        "Hello, world!",
        RED_FOREGROUND_CODE,
        "How are you?",
        RESET_CODE,
    ]
        .join("");
    let expected = vec![
        Span {
            color: Color::Black,

            text: "Hello, world!".to_string().into_bytes(),
            bg_color: Color::White,
            brightness: Brightness::Bold,
            text_style: TextStyle::Italic,
        },
        Span {
            color: Color::Red,

            text: "How are you?".to_string().into_bytes(),
            bg_color: Color::White,
            brightness: Brightness::Bold,
            text_style: TextStyle::Italic,
        },
    ];
    assert_eq!(parse_ansi_text(&input), expected);
}

#[test]
fn when_background_color_change_after_some_text_without_reset_should_create_a_new_span_with_prev_style_and_new_background_color() {
    let input = [
        ITALIC_CODE,
        BOLD_CODE,
        WHITE_FOREGROUND_CODE,
        BLACK_BACKGROUND_CODE,
        "Hello, world!",
        RED_BACKGROUND_CODE,
        "How are you?",
        RESET_CODE,
    ]
        .join("");
    let expected = vec![
        Span {
            bg_color: Color::Black,

            text: "Hello, world!".to_string().into_bytes(),
            color: Color::White,
            brightness: Brightness::Bold,
            text_style: TextStyle::Italic,
        },
        Span {
            bg_color: Color::Red,

            text: "How are you?".to_string().into_bytes(),
            color: Color::White,
            brightness: Brightness::Bold,
            text_style: TextStyle::Italic,
        },
    ];
    assert_eq!(parse_ansi_text(&input), expected);
}

#[test]
fn when_brightness_change_after_some_text_without_reset_should_create_a_new_span_with_prev_style_and_new_brightness() {
    let input = [
        ITALIC_CODE,
        WHITE_FOREGROUND_CODE,
        BLACK_BACKGROUND_CODE,
        BOLD_CODE,
        "Hello, world!",
        DIM_CODE,
        "How are you?",
        RESET_CODE,
    ]
        .join("");
    let expected = vec![
        Span {
            brightness: Brightness::Bold,

            text: "Hello, world!".to_string().into_bytes(),
            color: Color::White,
            bg_color: Color::Black,
            text_style: TextStyle::Italic,
        },
        Span {
            brightness: Brightness::Dim,

            text: "How are you?".to_string().into_bytes(),
            color: Color::White,
            bg_color: Color::Black,
            text_style: TextStyle::Italic,
        },
    ];
    assert_eq!(parse_ansi_text(&input), expected);
}

#[test]
fn when_text_style_change_after_some_text_without_reset_should_create_a_new_span_with_prev_style_and_merged_text_style() {
    let input = [
        WHITE_FOREGROUND_CODE,
        BLACK_BACKGROUND_CODE,
        BOLD_CODE,
        ITALIC_CODE,
        "Hello, world!",
        UNDERLINE_CODE,
        "How are you?",
        RESET_CODE,
    ]
        .join("");
    let expected = vec![
        Span {
            text_style: TextStyle::Italic,

            text: "Hello, world!".to_string().into_bytes(),
            color: Color::White,
            bg_color: Color::Black,
            brightness: Brightness::Bold,
        },
        Span {
            text_style: TextStyle::Italic | TextStyle::Underline,

            text: "How are you?".to_string().into_bytes(),
            color: Color::White,
            bg_color: Color::Black,
            brightness: Brightness::Bold,
        },
    ];
    assert_eq!(parse_ansi_text(&input), expected);
}

// ------------------------------------------------------------------------------------------
// Color/Style/Brightness first set after some text without reset with no other style before
// ------------------------------------------------------------------------------------------

#[test]
fn when_foreground_color_added_after_some_text_without_reset_should_create_a_new_span_with_new_foreground_color() {
    let input = [
        "Hello, world!",
        RED_FOREGROUND_CODE,
        "How are you?",
        RESET_CODE,
    ]
        .join("");
    let expected = vec![
        Span {
            color: Color::None,

            text: "Hello, world!".to_string().into_bytes(),
            bg_color: Color::None,
            brightness: Brightness::None,
            text_style: TextStyle::None,
        },
        Span {
            color: Color::Red,

            text: "How are you?".to_string().into_bytes(),
            bg_color: Color::None,
            brightness: Brightness::None,
            text_style: TextStyle::None,
        },
    ];
    assert_eq!(parse_ansi_text(&input), expected);
}

#[test]
fn when_background_color_added_after_some_text_without_reset_should_create_a_new_span_with_new_background_color() {
    let input = [
        "Hello, world!",
        RED_BACKGROUND_CODE,
        "How are you?",
        RESET_CODE,
    ]
        .join("");
    let expected = vec![
        Span {
            bg_color: Color::None,

            text: "Hello, world!".to_string().into_bytes(),
            color: Color::None,
            brightness: Brightness::None,
            text_style: TextStyle::None,
        },
        Span {
            bg_color: Color::Red,

            text: "How are you?".to_string().into_bytes(),
            color: Color::None,
            brightness: Brightness::None,
            text_style: TextStyle::None,
        },
    ];
    assert_eq!(parse_ansi_text(&input), expected);
}

#[test]
fn when_brightness_added_after_some_text_without_reset_should_create_a_new_span_with_new_brightness() {
    let input = ["Hello, world!", DIM_CODE, "How are you?", RESET_CODE].join("");
    let expected = vec![
        Span {
            brightness: Brightness::None,

            text: "Hello, world!".to_string().into_bytes(),
            color: Color::None,
            bg_color: Color::None,
            text_style: TextStyle::None,
        },
        Span {
            brightness: Brightness::Dim,

            text: "How are you?".to_string().into_bytes(),
            color: Color::None,
            bg_color: Color::None,
            text_style: TextStyle::None,
        },
    ];
    assert_eq!(parse_ansi_text(&input), expected);
}

#[test]
fn when_text_style_added_after_some_text_without_reset_should_create_a_new_span_with_new_text_style() {
    let input = ["Hello, world!", UNDERLINE_CODE, "How are you?", RESET_CODE].join("");
    let expected = vec![
        Span {
            text_style: TextStyle::None,

            text: "Hello, world!".to_string().into_bytes(),
            bg_color: Color::None,
            brightness: Brightness::None,
            color: Color::None,
        },
        Span {
            text_style: TextStyle::Underline,

            text: "How are you?".to_string().into_bytes(),
            bg_color: Color::None,
            brightness: Brightness::None,
            color: Color::None,
        },
    ];
    assert_eq!(parse_ansi_text(&input), expected);
}

// ------------------------------------------------------------------------------------------------
// Color/Style/Brightness first set after some text without text afterward with other style before
// ------------------------------------------------------------------------------------------------

#[test]
fn when_foreground_color_added_after_some_text_without_reset_should_create_a_new_span_with_prev_style_and_new_foreground_color() {
    let input = [
        ITALIC_CODE,
        BOLD_CODE,
        BLACK_BACKGROUND_CODE,
        "Hello, world!",
        RED_FOREGROUND_CODE,
        "How are you?",
        RESET_CODE,
    ]
        .join("");
    let expected = vec![
        Span {
            color: Color::None,

            text: "Hello, world!".to_string().into_bytes(),
            bg_color: Color::Black,
            brightness: Brightness::Bold,
            text_style: TextStyle::Italic,
        },
        Span {
            color: Color::Red,

            text: "How are you?".to_string().into_bytes(),
            bg_color: Color::Black,
            brightness: Brightness::Bold,
            text_style: TextStyle::Italic,
        },
    ];
    assert_eq!(parse_ansi_text(&input), expected);
}

#[test]
fn when_background_color_added_after_some_text_without_reset_should_create_a_new_span_with_prev_style_and_new_background_color() {
    let input = [
        ITALIC_CODE,
        BOLD_CODE,
        RED_FOREGROUND_CODE,
        "Hello, world!",
        RED_BACKGROUND_CODE,
        "How are you?",
        RESET_CODE,
    ]
        .join("");
    let expected = vec![
        Span {
            bg_color: Color::None,

            text: "Hello, world!".to_string().into_bytes(),
            color: Color::Red,
            brightness: Brightness::Bold,
            text_style: TextStyle::Italic,
        },
        Span {
            bg_color: Color::Red,

            text: "How are you?".to_string().into_bytes(),
            color: Color::Red,
            brightness: Brightness::Bold,
            text_style: TextStyle::Italic,
        },
    ];
    assert_eq!(parse_ansi_text(&input), expected);
}

#[test]
fn when_brightness_added_after_some_text_without_reset_should_create_a_new_span_with_prev_style_and_new_brightness() {
    let input = [
        ITALIC_CODE,
        RED_FOREGROUND_CODE,
        BLACK_BACKGROUND_CODE,
        "Hello, world!",
        DIM_CODE,
        "How are you?",
        RESET_CODE,
    ]
        .join("");
    let expected = vec![
        Span {
            brightness: Brightness::None,

            text: "Hello, world!".to_string().into_bytes(),
            color: Color::Red,
            bg_color: Color::Black,
            text_style: TextStyle::Italic,
        },
        Span {
            brightness: Brightness::Dim,

            text: "How are you?".to_string().into_bytes(),
            color: Color::Red,
            bg_color: Color::Black,
            text_style: TextStyle::Italic,
        },
    ];
    assert_eq!(parse_ansi_text(&input), expected);
}

#[test]
fn when_text_style_added_after_some_text_without_reset_should_create_a_new_span_with_prev_style_and_new_text_style() {
    let input = [
        DIM_CODE,
        RED_FOREGROUND_CODE,
        BLACK_BACKGROUND_CODE,
        "Hello, world!",
        UNDERLINE_CODE,
        "How are you?",
        RESET_CODE,
    ]
        .join("");
    let expected = vec![
        Span {
            text_style: TextStyle::None,

            text: "Hello, world!".to_string().into_bytes(),
            color: Color::Red,
            bg_color: Color::Black,
            brightness: Brightness::Dim,
        },
        Span {
            text_style: TextStyle::Underline,

            text: "How are you?".to_string().into_bytes(),
            color: Color::Red,
            bg_color: Color::Black,
            brightness: Brightness::Dim,
        },
    ];
    assert_eq!(parse_ansi_text(&input), expected);
}

// --------------------------------------------------------------------------------------------
// Color/Style/Brightness first set after some text without text afterward with no other style
// --------------------------------------------------------------------------------------------

#[test]
fn when_foreground_color_added_after_some_text_without_reset_should_not_use_the_new_style_on_prev_span() {
    let input = ["Hello, world!", RED_FOREGROUND_CODE].join("");
    let expected = vec![Span {
        color: Color::None,

        text: "Hello, world!".to_string().into_bytes(),
        bg_color: Color::None,
        brightness: Brightness::None,
        text_style: TextStyle::None,
    }];
    assert_eq!(parse_ansi_text(&input), expected);
}

#[test]
fn when_background_color_added_after_some_text_without_reset_should_not_use_the_new_style_on_prev_span() {
    let input = ["Hello, world!", RED_BACKGROUND_CODE].join("");
    let expected = vec![Span {
        bg_color: Color::None,

        text: "Hello, world!".to_string().into_bytes(),
        color: Color::None,
        brightness: Brightness::None,
        text_style: TextStyle::None,
    }];
    assert_eq!(parse_ansi_text(&input), expected);
}

#[test]
fn when_brightness_added_after_some_text_without_reset_should_not_use_the_new_style_on_prev_span() {
    let input = ["Hello, world!", DIM_CODE].join("");
    let expected = vec![Span {
        brightness: Brightness::None,

        text: "Hello, world!".to_string().into_bytes(),
        color: Color::None,
        bg_color: Color::None,
        text_style: TextStyle::None,
    }];
    assert_eq!(parse_ansi_text(&input), expected);
}

#[test]
fn when_text_style_added_after_some_text_without_reset_should_not_use_the_new_style_on_prev_span() {
    let input = ["Hello, world!", UNDERLINE_CODE].join("");
    let expected = vec![Span {
        text_style: TextStyle::None,

        text: "Hello, world!".to_string().into_bytes(),
        bg_color: Color::None,
        brightness: Brightness::None,
        color: Color::None,
    }];
    assert_eq!(parse_ansi_text(&input), expected);
}

// ------------------------------------------------------------
// Style combination
// ------------------------------------------------------------

#[test]
fn should_append_text_styles() {
    let input = [
        ITALIC_CODE,
        UNDERLINE_CODE,
        INVERSE_CODE,
        STRIKETHROUGH_CODE,
        "Hello, world!",
        RESET_CODE,
    ]
        .join("");
    let expected = vec![Span {
        text_style: TextStyle::Italic
            | TextStyle::Underline
            | TextStyle::Inverse
            | TextStyle::Strikethrough,

        text: "Hello, world!".to_string().into_bytes(),
        bg_color: Color::None,
        color: Color::None,
        brightness: Brightness::None,
    }];
    assert_eq!(parse_ansi_text(&input), expected);
}

// -----------------------------------------------------------------------------------------------------
// Style added after text should create a new span with the same color/brightness and merged text style
// -----------------------------------------------------------------------------------------------------

#[test]
fn style_added_after_text_should_create_new_span_and_merge_with_style_before() {
    let input = [
        ITALIC_CODE,
        UNDERLINE_CODE,
        "Hello, world!",
        INVERSE_CODE,
        "How are you?",
        RESET_CODE,
    ]
        .join("");

    let expected = vec![
        Span {
            text_style: TextStyle::Italic | TextStyle::Underline,

            text: "Hello, world!".to_string().into_bytes(),
            color: Color::None,
            bg_color: Color::None,
            brightness: Brightness::None,
        },
        Span {
            text_style: TextStyle::Italic | TextStyle::Underline | TextStyle::Inverse,

            text: "How are you?".to_string().into_bytes(),
            color: Color::None,
            bg_color: Color::None,
            brightness: Brightness::None,
        },
    ];
    assert_eq!(parse_ansi_text(&input), expected);
}

#[test_case(RED_FOREGROUND_CODE; "Red foreground added after text again")]
#[test_case(GREEN_BACKGROUND_CODE; "Green background added after text again")]
#[test_case(ITALIC_CODE; "Italic text added after text again")]
#[test_case(DIM_CODE; "Dim text added after text again")]
async fn same_style_apply_after_text_should_not_create_new_span_for_next_text(
    same_style_code: &str,
) {
    let input = [
        RED_FOREGROUND_CODE,
        GREEN_BACKGROUND_CODE,
        ITALIC_CODE,
        DIM_CODE,
        "Hello, world!",
        same_style_code,
        "How are you?",
        RESET_CODE,
    ]
        .join("");

    let expected = vec![Span {
        text_style: TextStyle::Italic,

        text: "Hello, world!How are you?".to_string().into_bytes(),
        color: Color::Red,
        bg_color: Color::Green,
        brightness: Brightness::Dim,
    }];
    assert_eq!(parse_ansi_text(&input), expected);
}

// -------------------------------
// Parse options with initial span
// -------------------------------

#[test]
fn span_should_have_the_same_style_as_the_initial_span() {
    let input = ["Hello, world!", RESET_CODE].join("");
    let expected = vec![Span {
        color: Color::Red,

        text: "Hello, world!".to_string().into_bytes(),
        bg_color: Color::None,
        brightness: Brightness::None,
        text_style: TextStyle::None,
    }];

    let parse_options =
        ParseOptions::default().with_initial_span(Span::empty().with_color(Color::Red));

    assert_eq!(
        parse_ansi_text_with_options(&input, parse_options),
        expected
    );
}

#[test]
fn non_first_spans_should_not_have_the_same_style_as_the_initial_span() {
    let input = ["Hello, world!", RESET_CODE, "How are you?"].join("");
    let expected = vec![
        Span {
            color: Color::Red,

            text: "Hello, world!".to_string().into_bytes(),
            bg_color: Color::None,
            brightness: Brightness::None,
            text_style: TextStyle::None,
        },
        Span {
            color: Color::None,

            text: "How are you?".to_string().into_bytes(),
            bg_color: Color::None,
            brightness: Brightness::None,
            text_style: TextStyle::None,
        },
    ];

    let parse_options =
        ParseOptions::default().with_initial_span(Span::empty().with_color(Color::Red));

    assert_eq!(
        parse_ansi_text_with_options(&input, parse_options),
        expected
    );
}

// ----------------------------------
// Parse options with split by lines
// ----------------------------------

#[test]
fn spans_should_have_same_style_when_split_by_line() {
    let input = [
        RGB_BACKGROUND_CODE(188, 29, 68).to_string(),
        RGB_FOREGROUND_CODE(255, 19, 94).to_string(),
        ITALIC_CODE.to_string(),
        UNDERLINE_CODE.to_string(),
        BOLD_CODE.to_string(),
        "Hello, world!\nHow are you?".to_string(),
        RESET_CODE.to_string(),
    ]
        .join("");
    let expected = vec![
        Line {
            spans: vec![Span {
                text: "Hello, world!".to_string().into_bytes(),
                color: Color::Rgb(255, 19, 94),
                bg_color: Color::Rgb(188, 29, 68),
                brightness: Brightness::Bold,
                text_style: TextStyle::Italic | TextStyle::Underline,
            }],
            location_in_file: 0,
        },
        Line {
            spans: vec![Span {
                text: "How are you?".to_string().into_bytes(),
                color: Color::Rgb(255, 19, 94),
                bg_color: Color::Rgb(188, 29, 68),
                brightness: Brightness::Bold,
                text_style: TextStyle::Italic | TextStyle::Underline,
            }],
            location_in_file: input.find("How are you?").unwrap(),
        },
    ];

    let parse_options =
        ParseOptions::default().with_initial_span(Span::empty().with_color(Color::Red));

    let actual = parse_ansi_text_split_by_lines_with_options(&input, parse_options);
    assert_eq!(actual, expected);
}

#[test]
fn multiple_styles() {
    let input = [
        &*RGB_BACKGROUND_CODE(188, 29, 68),
        &*RGB_FOREGROUND_CODE(255, 19, 94),
        ITALIC_CODE,
        UNDERLINE_CODE,
        BOLD_CODE,
        "Hello, world!",
        RESET_CODE,
    ]
        .join("");
    let expected = vec![Span {
        text: "Hello, world!".to_string().into_bytes(),
        color: Color::Rgb(255, 19, 94),
        bg_color: Color::Rgb(188, 29, 68),
        brightness: Brightness::Bold,
        text_style: TextStyle::Italic | TextStyle::Underline,
    }];
    assert_eq!(parse_ansi_text(&input), expected);
}


// ----------------------------------
// Location of each line
// ----------------------------------


#[derive(Debug, PartialEq)]
struct LineForLocationTests {
    text_in_line: String,
    location_in_file: usize,
}

fn convert_line_to_line_for_location_tests(line: &Line) -> LineForLocationTests {
    LineForLocationTests {
        text_in_line: line.spans.iter().map(|span| String::from_utf8_lossy(&span.text).to_string()).collect::<String>(),
        location_in_file: line.location_in_file,
    }
}

fn convert_lines_to_location_tests_lines(lines: &Vec<Line>) -> Vec<LineForLocationTests> {
    lines.iter().map(convert_line_to_line_for_location_tests).collect()
}

#[test]
fn only_text_without_styles_should_have_correct_location() {
    let input = [
        "Hello, world!\n",
        "How are you?\n",
        "Good",
    ]
        .join("");
    let expected = vec![
        LineForLocationTests {
            text_in_line: "Hello, world!".to_string(),
            location_in_file: 0,
        },
        LineForLocationTests {
            text_in_line: "How are you?".to_string(),
            location_in_file: input.find("How are you?").unwrap(),
        },
        LineForLocationTests {
            text_in_line: "Good".to_string(),
            location_in_file: input.find("Good").unwrap(),
        },
    ];

    let parse_options = ParseOptions::default();

    let actual = parse_ansi_text_split_by_lines_with_options(&input, parse_options);
    let actual = convert_lines_to_location_tests_lines(&actual);

    assert_eq!(actual, expected);
}

#[test]
fn only_text_without_styles_should_have_correct_location_when_having_initial_style() {
    let input = [
        "Hello, world!\n",
        "How are you?\n",
        "Good",
    ].join("");
    let expected = vec![
        LineForLocationTests {
            text_in_line: "Hello, world!".to_string(),
            location_in_file: 0,
        },
        LineForLocationTests {
            text_in_line: "How are you?".to_string(),
            location_in_file: input.find("How are you?").unwrap(),
        },
        LineForLocationTests {
            text_in_line: "Good".to_string(),
            location_in_file: input.find("Good").unwrap(),
        },
    ];

    let parse_options = ParseOptions::default().with_initial_span(Span::empty().with_color(Color::Red));

    let actual = parse_ansi_text_split_by_lines_with_options(&input, parse_options);
    let actual = convert_lines_to_location_tests_lines(&actual);

    assert_eq!(actual, expected);
}

#[test]
fn first_line_have_styles_before_text_should_still_have_correct_location() {
    let input = [
        RGB_BACKGROUND_CODE(188, 29, 68).as_str(),
        "Hello, world!\n",
        "How are you?\n",
        "Good",
    ]
        .join("");
    let expected = vec![
        LineForLocationTests {
            text_in_line: "Hello, world!".to_string(),
            location_in_file: 0,
        },
        LineForLocationTests {
            text_in_line: "How are you?".to_string(),
            location_in_file: input.find("How are you?").unwrap(),
        },
        LineForLocationTests {
            text_in_line: "Good".to_string(),
            location_in_file: input.find("Good").unwrap(),
        },
    ];

    let parse_options = ParseOptions::default();

    let actual = parse_ansi_text_split_by_lines_with_options(&input, parse_options);
    let actual = convert_lines_to_location_tests_lines(&actual);
    assert_eq!(actual, expected);
}

#[test]
fn first_line_have_conflicting_styles_before_text_should_still_have_correct_location() {
    let input = [
        RGB_BACKGROUND_CODE(188, 29, 68).as_str(),
        RGB_BACKGROUND_CODE(150, 20, 10).as_str(),
        "Hello, world!\n",
        "How are you?\n",
        "Good",
    ]
        .join("");
    let expected = vec![
        LineForLocationTests {
            text_in_line: "Hello, world!".to_string(),
            location_in_file: 0,
        },
        LineForLocationTests {
            text_in_line: "How are you?".to_string(),
            location_in_file: input.find("How are you?").unwrap(),
        },
        LineForLocationTests {
            text_in_line: "Good".to_string(),
            location_in_file: input.find("Good").unwrap(),
        },
    ];

    let parse_options = ParseOptions::default();

    let actual = parse_ansi_text_split_by_lines_with_options(&input, parse_options);
    let actual = convert_lines_to_location_tests_lines(&actual);

    assert_eq!(actual, expected);
}

#[test]
fn first_line_have_multiple_spans_still_have_correct_locations() {
    let input = [
        RGB_BACKGROUND_CODE(188, 29, 68).as_str(),
        "Hello, ",
        ITALIC_CODE,
        BOLD_CODE,
        "world!\n",
        "How are you?\n",
        "Good",
    ]
        .join("");
    let expected = vec![
        LineForLocationTests {
            text_in_line: "Hello, world!".to_string(),
            location_in_file: 0,
        },
        LineForLocationTests {
            text_in_line: "How are you?".to_string(),
            location_in_file: input.find("How are you?").unwrap(),
        },
        LineForLocationTests {
            text_in_line: "Good".to_string(),
            location_in_file: input.find("Good").unwrap(),
        },
    ];

    let parse_options = ParseOptions::default();

    let actual = parse_ansi_text_split_by_lines_with_options(&input, parse_options);
    let actual = convert_lines_to_location_tests_lines(&actual);

    assert_eq!(actual, expected);
}

#[test]
fn when_lines_start_with_style_should_have_the_styles_be_the_initial_location() {
    let input = [
        "Hello, world!\n",
        BOLD_CODE,
        "How are you?\n",
        "Good",
    ]
        .join("");
    let expected = vec![
        LineForLocationTests {
            text_in_line: "Hello, world!".to_string(),
            location_in_file: 0,
        },
        LineForLocationTests {
            text_in_line: "How are you?".to_string(),
            location_in_file: input.find("How are you?").unwrap() - BOLD_CODE.len(),
        },
        LineForLocationTests {
            text_in_line: "Good".to_string(),
            location_in_file: input.find("Good").unwrap(),
        },
    ];

    let parse_options = ParseOptions::default();

    let actual = parse_ansi_text_split_by_lines_with_options(&input, parse_options);
    let actual = convert_lines_to_location_tests_lines(&actual);

    assert_eq!(actual, expected);
}


#[test]
fn lines_without_any_text_or_style_should_still_have_correct_location() {
    let input = [
        "Hello, world!\n",
        "\n",
        "How are you?\n",
        "Good",
    ]
        .join("");
    let expected = vec![
        LineForLocationTests {
            text_in_line: "Hello, world!".to_string(),
            location_in_file: 0,
        },
        LineForLocationTests {
            text_in_line: "".to_string(),
            // First new line + 1
            location_in_file: input.find("\n").unwrap() + 1,
        },
        LineForLocationTests {
            text_in_line: "How are you?".to_string(),
            location_in_file: input.find("How are you?").unwrap(),
        },
        LineForLocationTests {
            text_in_line: "Good".to_string(),
            location_in_file: input.find("Good").unwrap(),
        },
    ];

    let parse_options = ParseOptions::default();

    let actual = parse_ansi_text_split_by_lines_with_options(&input, parse_options);
    let actual = convert_lines_to_location_tests_lines(&actual);

    assert_eq!(actual, expected);
}
