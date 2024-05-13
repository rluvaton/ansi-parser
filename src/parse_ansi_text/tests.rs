
#[cfg(test)]
mod tests {
    use test_case::{test_case, test_matrix};
    
    use crate::parse_ansi_text::colors::*;
    use crate::parse_ansi_text::constants::*;
    use crate::parse_ansi_text::style::*;
    use crate::parse_ansi_text::types::*;
    use crate::parse_ansi_text::{parse_ansi_text, parse_ansi_text_with_options};
    use crate::parse_ansi_text::parse_options::ParseOptions;

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
        let expected = vec![Span::empty().with_text("Hello, world!".to_string())];
        assert_eq!(parse_ansi_text(input), expected);
    }

    #[test]
    fn multiline_text_without_ansi_codes_should_return_array_with_one_unstyled_span() {
        let input = "Hello, world!\nhow are you";
        let expected = vec![Span::empty().with_text("Hello, world!\nhow are you".to_string())];
        assert_eq!(parse_ansi_text(input), expected);
    }

    // -------------
    // Single style
    // -------------


    #[test_case(Color::Red, RED_FOREGROUND_CODE ; "Red foreground")]
    #[test_case(Color::Black, BLACK_FOREGROUND_CODE ; "Black foreground")]
    #[test_case(Color::Green, GREEN_FOREGROUND_CODE ; "Green foreground")]
    #[test_case(Color::Yellow, YELLOW_FOREGROUND_CODE ; "Yellow foreground")]
    #[test_case(Color::Blue, BLUE_FOREGROUND_CODE ; "Blue foreground")]
    #[test_case(Color::Magenta, MAGENTA_FOREGROUND_CODE ; "Magenta foreground")]
    #[test_case(Color::Cyan, CYAN_FOREGROUND_CODE ; "Cyan foreground")]
    #[test_case(Color::White, WHITE_FOREGROUND_CODE ; "White foreground")]
    fn single_foreground_color_with_no_other_styles(expected_color: Color, color_code: &str) {
        let input = [
            color_code,
            "Hello, world!",
            RESET_CODE,
        ].join("");
        let expected = vec![
            Span::empty()
                .with_color(expected_color)
                .with_text("Hello, world!".to_string())
        ];
        assert_eq!(parse_ansi_text(&input), expected);
    }

    #[test_case(Color::Red, RED_BACKGROUND_CODE ; "Red background")]
    #[test_case(Color::Black, BLACK_BACKGROUND_CODE ; "Black background")]
    #[test_case(Color::Green, GREEN_BACKGROUND_CODE ; "Green background")]
    #[test_case(Color::Yellow, YELLOW_BACKGROUND_CODE ; "Yellow background")]
    #[test_case(Color::Blue, BLUE_BACKGROUND_CODE ; "Blue background")]
    #[test_case(Color::Magenta, MAGENTA_BACKGROUND_CODE ; "Magenta background")]
    #[test_case(Color::Cyan, CYAN_BACKGROUND_CODE ; "Cyan background")]
    #[test_case(Color::White, WHITE_BACKGROUND_CODE ; "White background")]
    fn single_background_color_with_no_other_styles(expected_color: Color, color_code: &str) {
        let input = [
            color_code,
            "Hello, world!",
            RESET_CODE,
        ].join("");
        let expected = vec![
            Span::empty()
                .with_bg_color(expected_color)
                .with_text("Hello, world!".to_string())
        ];
        assert_eq!(parse_ansi_text(&input), expected);
    }

    #[test_case(TextStyle::Italic, ITALIC_CODE ; "Italic text")]
    #[test_case(TextStyle::Underline, UNDERLINE_CODE ; "Underline text")]
    #[test_case(TextStyle::Inverse, INVERSE_CODE ; "Inverse text")]
    #[test_case(TextStyle::Strikethrough, STRIKETHROUGH_CODE ; "Strikethrough text")]
    fn single_text_style_with_no_other_styles(expected_text_style: TextStyle, text_style_code: &str) {
        let input = [
            text_style_code,
            "Hello, world!",
            RESET_CODE,
        ].join("");
        let expected = vec![

            Span::empty()
                .with_text_style(expected_text_style)
                .with_text("Hello, world!".to_string())
        ];
        assert_eq!(parse_ansi_text(&input), expected);
    }

    #[test_case(Brightness::Bold, BOLD_CODE ; "Bold text")]
    #[test_case(Brightness::Dim, DIM_CODE ; "Dim text")]
    fn single_brightness_with_no_other_styles(expected_brightness: Brightness, brightness_code: &str) {
        let input = [
            brightness_code,
            "Hello, world!",
            RESET_CODE,
        ].join("");
        let expected = vec![
            Span::empty()
                .with_brightness(expected_brightness)
                .with_text("Hello, world!".to_string())
        ];
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
        ].join("");
        let expected = vec![Span {
            color: Color::Red,

            text: "Hello, world!".to_string(),
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
        ].join("");
        let expected = vec![Span {
            bg_color: Color::Red,

            color: Color::None,
            text: "Hello, world!".to_string(),
            brightness: Brightness::None,
            text_style: TextStyle::None,
        }];
        assert_eq!(parse_ansi_text(&input), expected);
    }

    #[test]
    fn brightness_should_replace_prev_brightness_when_no_text_in_between() {
        let input = [
            BOLD_CODE,
            DIM_CODE,
            "Hello, world!",
            RESET_CODE,
        ].join("");
        let expected = vec![Span {
            brightness: Brightness::Dim,

            color: Color::None,
            bg_color: Color::None,
            text: "Hello, world!".to_string(),
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
        ].join("");
        let expected = vec![Span {
            color: Color::Red,

            text: "Hello, world!".to_string(),
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
        ].join("");
        let expected = vec![Span {
            bg_color: Color::Red,

            color: Color::None,
            text: "Hello, world!".to_string(),
            brightness: Brightness::None,
            text_style: TextStyle::None,
        }];
        assert_eq!(parse_ansi_text(&input), expected);
    }

    #[test]
    fn brightness_should_replace_prev_brightness_after_reset_when_no_text_in_between() {
        let input = [
            BOLD_CODE,
            RESET_CODE,
            DIM_CODE,
            "Hello, world!",
            RESET_CODE,
        ].join("");
        let expected = vec![Span {
            brightness: Brightness::Dim,

            color: Color::None,
            bg_color: Color::None,
            text: "Hello, world!".to_string(),
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
        ].join("");
        let expected = vec![Span {
            text_style: TextStyle::Underline,

            color: Color::None,
            bg_color: Color::None,
            brightness: Brightness::None,
            text: "Hello, world!".to_string(),
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
        ].join("");
        let expected = vec![Span {
            color: Color::Black,

            text: "Hello, world!".to_string(),
            bg_color: Color::None,
            brightness: Brightness::None,
            text_style: TextStyle::None,
        }, Span {
            color: Color::Red,

            text: "How are you?".to_string(),
            bg_color: Color::None,
            brightness: Brightness::None,
            text_style: TextStyle::None,
        }];
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
        ].join("");
        let expected = vec![Span {
            color: Color::Rgb(188, 29, 68),

            text: "Hello, world!".to_string(),
            bg_color: Color::None,
            brightness: Brightness::None,
            text_style: TextStyle::None,
        }, Span {
            color: Color::Rgb(255, 19, 94),

            text: "How are you?".to_string(),
            bg_color: Color::None,
            brightness: Brightness::None,
            text_style: TextStyle::None,
        }];
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
        ].join("");
        let expected = vec![Span {
            bg_color: Color::Black,

            text: "Hello, world!".to_string(),
            color: Color::None,
            brightness: Brightness::None,
            text_style: TextStyle::None,
        }, Span {
            bg_color: Color::Red,

            text: "How are you?".to_string(),
            color: Color::None,
            brightness: Brightness::None,
            text_style: TextStyle::None,
        }];
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
        ].join("");
        let expected = vec![Span {
            bg_color: Color::Rgb(188, 29, 68),

            text: "Hello, world!".to_string(),
            color: Color::None,
            brightness: Brightness::None,
            text_style: TextStyle::None,
        }, Span {
            bg_color: Color::Rgb(255, 19, 94),

            text: "How are you?".to_string(),
            color: Color::None,
            brightness: Brightness::None,
            text_style: TextStyle::None,
        }];
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
        ].join("");
        let expected = vec![Span {
            brightness: Brightness::Bold,

            text: "Hello, world!".to_string(),
            color: Color::None,
            bg_color: Color::None,
            text_style: TextStyle::None,
        }, Span {
            brightness: Brightness::Dim,

            text: "How are you?".to_string(),
            color: Color::None,
            bg_color: Color::None,
            text_style: TextStyle::None,
        }];
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
        ].join("");
        let expected = vec![Span {
            text_style: TextStyle::Italic,

            text: "Hello, world!".to_string(),
            bg_color: Color::None,
            brightness: Brightness::None,
            color: Color::None,
        }, Span {
            text_style: TextStyle::Italic | TextStyle::Underline,

            text: "How are you?".to_string(),
            bg_color: Color::None,
            brightness: Brightness::None,
            color: Color::None,
        }];
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
        ].join("");
        let expected = vec![Span {
            color: Color::Black,

            text: "Hello, world!".to_string(),
            bg_color: Color::White,
            brightness: Brightness::Bold,
            text_style: TextStyle::Italic,
        }, Span {
            color: Color::Red,

            text: "How are you?".to_string(),
            bg_color: Color::White,
            brightness: Brightness::Bold,
            text_style: TextStyle::Italic,
        }];
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
        ].join("");
        let expected = vec![Span {
            bg_color: Color::Black,

            text: "Hello, world!".to_string(),
            color: Color::White,
            brightness: Brightness::Bold,
            text_style: TextStyle::Italic,
        }, Span {
            bg_color: Color::Red,

            text: "How are you?".to_string(),
            color: Color::White,
            brightness: Brightness::Bold,
            text_style: TextStyle::Italic,
        }];
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
        ].join("");
        let expected = vec![Span {
            brightness: Brightness::Bold,

            text: "Hello, world!".to_string(),
            color: Color::White,
            bg_color: Color::Black,
            text_style: TextStyle::Italic,
        }, Span {
            brightness: Brightness::Dim,

            text: "How are you?".to_string(),
            color: Color::White,
            bg_color: Color::Black,
            text_style: TextStyle::Italic,
        }];
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
        ].join("");
        let expected = vec![Span {
            text_style: TextStyle::Italic,

            text: "Hello, world!".to_string(),
            color: Color::White,
            bg_color: Color::Black,
            brightness: Brightness::Bold,
        }, Span {
            text_style: TextStyle::Italic | TextStyle::Underline,

            text: "How are you?".to_string(),
            color: Color::White,
            bg_color: Color::Black,
            brightness: Brightness::Bold,
        }];
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
        ].join("");
        let expected = vec![Span {
            color: Color::None,

            text: "Hello, world!".to_string(),
            bg_color: Color::None,
            brightness: Brightness::None,
            text_style: TextStyle::None,
        }, Span {
            color: Color::Red,

            text: "How are you?".to_string(),
            bg_color: Color::None,
            brightness: Brightness::None,
            text_style: TextStyle::None,
        }];
        assert_eq!(parse_ansi_text(&input), expected);
    }

    #[test]
    fn when_background_color_added_after_some_text_without_reset_should_create_a_new_span_with_new_background_color() {
        let input = [
            "Hello, world!",
            RED_BACKGROUND_CODE,
            "How are you?",
            RESET_CODE,
        ].join("");
        let expected = vec![Span {
            bg_color: Color::None,

            text: "Hello, world!".to_string(),
            color: Color::None,
            brightness: Brightness::None,
            text_style: TextStyle::None,
        }, Span {
            bg_color: Color::Red,

            text: "How are you?".to_string(),
            color: Color::None,
            brightness: Brightness::None,
            text_style: TextStyle::None,
        }];
        assert_eq!(parse_ansi_text(&input), expected);
    }

    #[test]
    fn when_brightness_added_after_some_text_without_reset_should_create_a_new_span_with_new_brightness() {
        let input = [
            "Hello, world!",
            DIM_CODE,
            "How are you?",
            RESET_CODE,
        ].join("");
        let expected = vec![Span {
            brightness: Brightness::None,

            text: "Hello, world!".to_string(),
            color: Color::None,
            bg_color: Color::None,
            text_style: TextStyle::None,
        }, Span {
            brightness: Brightness::Dim,

            text: "How are you?".to_string(),
            color: Color::None,
            bg_color: Color::None,
            text_style: TextStyle::None,
        }];
        assert_eq!(parse_ansi_text(&input), expected);
    }

    #[test]
    fn when_text_style_added_after_some_text_without_reset_should_create_a_new_span_with_new_text_style() {
        let input = [
            "Hello, world!",
            UNDERLINE_CODE,
            "How are you?",
            RESET_CODE,
        ].join("");
        let expected = vec![Span {
            text_style: TextStyle::None,

            text: "Hello, world!".to_string(),
            bg_color: Color::None,
            brightness: Brightness::None,
            color: Color::None,
        }, Span {
            text_style: TextStyle::Underline,

            text: "How are you?".to_string(),
            bg_color: Color::None,
            brightness: Brightness::None,
            color: Color::None,
        }];
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
        ].join("");
        let expected = vec![Span {
            color: Color::None,

            text: "Hello, world!".to_string(),
            bg_color: Color::Black,
            brightness: Brightness::Bold,
            text_style: TextStyle::Italic,
        }, Span {
            color: Color::Red,

            text: "How are you?".to_string(),
            bg_color: Color::Black,
            brightness: Brightness::Bold,
            text_style: TextStyle::Italic,
        }];
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
        ].join("");
        let expected = vec![Span {
            bg_color: Color::None,

            text: "Hello, world!".to_string(),
            color: Color::Red,
            brightness: Brightness::Bold,
            text_style: TextStyle::Italic,
        }, Span {
            bg_color: Color::Red,

            text: "How are you?".to_string(),
            color: Color::Red,
            brightness: Brightness::Bold,
            text_style: TextStyle::Italic,
        }];
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
        ].join("");
        let expected = vec![Span {
            brightness: Brightness::None,

            text: "Hello, world!".to_string(),
            color: Color::Red,
            bg_color: Color::Black,
            text_style: TextStyle::Italic,
        }, Span {
            brightness: Brightness::Dim,

            text: "How are you?".to_string(),
            color: Color::Red,
            bg_color: Color::Black,
            text_style: TextStyle::Italic,
        }];
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
        ].join("");
        let expected = vec![Span {
            text_style: TextStyle::None,

            text: "Hello, world!".to_string(),
            color: Color::Red,
            bg_color: Color::Black,
            brightness: Brightness::Dim,
        }, Span {
            text_style: TextStyle::Underline,

            text: "How are you?".to_string(),
            color: Color::Red,
            bg_color: Color::Black,
            brightness: Brightness::Dim,
        }];
        assert_eq!(parse_ansi_text(&input), expected);
    }

    // --------------------------------------------------------------------------------------------
    // Color/Style/Brightness first set after some text without text afterward with no other style
    // --------------------------------------------------------------------------------------------

    #[test]
    fn when_foreground_color_added_after_some_text_without_reset_should_not_use_the_new_style_on_prev_span() {
        let input = [
            "Hello, world!",
            RED_FOREGROUND_CODE,
        ].join("");
        let expected = vec![Span {
            color: Color::None,

            text: "Hello, world!".to_string(),
            bg_color: Color::None,
            brightness: Brightness::None,
            text_style: TextStyle::None,
        }];
        assert_eq!(parse_ansi_text(&input), expected);
    }

    #[test]
    fn when_background_color_added_after_some_text_without_reset_should_not_use_the_new_style_on_prev_span() {
        let input = [
            "Hello, world!",
            RED_BACKGROUND_CODE,
        ].join("");
        let expected = vec![Span {
            bg_color: Color::None,

            text: "Hello, world!".to_string(),
            color: Color::None,
            brightness: Brightness::None,
            text_style: TextStyle::None,
        }];
        assert_eq!(parse_ansi_text(&input), expected);
    }

    #[test]
    fn when_brightness_added_after_some_text_without_reset_should_not_use_the_new_style_on_prev_span() {
        let input = [
            "Hello, world!",
            DIM_CODE,
        ].join("");
        let expected = vec![Span {
            brightness: Brightness::None,

            text: "Hello, world!".to_string(),
            color: Color::None,
            bg_color: Color::None,
            text_style: TextStyle::None,
        }];
        assert_eq!(parse_ansi_text(&input), expected);
    }

    #[test]
    fn when_text_style_added_after_some_text_without_reset_should_not_use_the_new_style_on_prev_span() {
        let input = [
            "Hello, world!",
            UNDERLINE_CODE,
        ].join("");
        let expected = vec![Span {
            text_style: TextStyle::None,

            text: "Hello, world!".to_string(),
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
        ].join("");
        let expected = vec![Span {
            text_style: TextStyle::Italic | TextStyle::Underline | TextStyle::Inverse | TextStyle::Strikethrough,

            text: "Hello, world!".to_string(),
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
        ].join("");

        let expected = vec![Span {
            text_style: TextStyle::Italic | TextStyle::Underline,

            text: "Hello, world!".to_string(),
            color: Color::None,
            bg_color: Color::None,
            brightness: Brightness::None,
        }, Span {
            text_style: TextStyle::Italic | TextStyle::Underline | TextStyle::Inverse,

            text: "How are you?".to_string(),
            color: Color::None,
            bg_color: Color::None,
            brightness: Brightness::None,
        }];
        assert_eq!(parse_ansi_text(&input), expected);
    }

    #[test_case(RED_FOREGROUND_CODE ; "Red foreground added after text again")]
    #[test_case(GREEN_BACKGROUND_CODE ; "Green background added after text again")]
    #[test_case(ITALIC_CODE ; "Italic text added after text again")]
    #[test_case(DIM_CODE ; "Dim text added after text again")]
    fn same_style_apply_after_text_should_not_create_new_span_for_next_text(same_style_code: &str) {
        let input = [
            RED_FOREGROUND_CODE,
            GREEN_BACKGROUND_CODE,
            ITALIC_CODE,
            DIM_CODE,
            "Hello, world!",
            same_style_code,
            "How are you?",
            RESET_CODE,
        ].join("");

        let expected = vec![Span {
            text_style: TextStyle::Italic,

            text: "Hello, world!How are you?".to_string(),
            color: Color::Red,
            bg_color: Color::Green,
            brightness: Brightness::Dim,
        }];
        assert_eq!(parse_ansi_text(&input), expected);
    }

    // -----------------------
    // Test with initial span
    // -----------------------

    #[test]
    fn span_should_have_the_same_style_as_the_initial_span() {
        let input = [
            "Hello, world!",
            RESET_CODE,
        ].join("");
        let expected = vec![Span {
            color: Color::Red,

            text: "Hello, world!".to_string(),
            bg_color: Color::None,
            brightness: Brightness::None,
            text_style: TextStyle::None,
        }];

        let parse_options = ParseOptions::default()
            .with_initial_span(
                Span::empty()
                    .with_color(Color::Red)
            );

        assert_eq!(parse_ansi_text_with_options(&input, parse_options), expected);
    }

    #[test]
    fn non_first_spans_should_not_have_the_same_style_as_the_initial_span() {
        let input = [
            "Hello, world!",
            RESET_CODE,
            "How are you?",
        ].join("");
        let expected = vec![Span {
            color: Color::Red,

            text: "Hello, world!".to_string(),
            bg_color: Color::None,
            brightness: Brightness::None,
            text_style: TextStyle::None,
        }, Span {
            color: Color::None,

            text: "How are you?".to_string(),
            bg_color: Color::None,
            brightness: Brightness::None,
            text_style: TextStyle::None,
        }];

        let parse_options = ParseOptions::default()
            .with_initial_span(
                Span::empty()
                    .with_color(Color::Red)
            );

        assert_eq!(parse_ansi_text_with_options(&input, parse_options), expected);
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
        ].join("");
        let expected = vec![Span {
            text: "Hello, world!".to_string(),
            color: Color::Rgb(255, 19, 94),
            bg_color: Color::Rgb(188, 29, 68),
            brightness: Brightness::Bold,
            text_style: TextStyle::Italic | TextStyle::Underline,
        }];
        assert_eq!(parse_ansi_text(&input), expected);
    }

}
