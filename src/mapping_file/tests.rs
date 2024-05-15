#[cfg(test)]
mod tests {
    use pretty_assertions::{assert_eq};
    use crate::mapping_file::create::*;
    use crate::mapping_file::*;

    use crate::parse_ansi_text::constants::RESET_CODE;
    use crate::parse_ansi_text::parse_text_matching_single_span::parse_text_matching_single_span;
    use crate::parse_ansi_text::types::Span;

    // ------------
    // Format
    // ------------

    #[test]
    fn output_should_have_line_length_before_first_delimiter() {
        let input = [
            // Style from start of the line
            BLACK_BACKGROUND_CODE.to_string()
                + "Hello, "
                + RESET_CODE
                + CYAN_BACKGROUND_CODE
                + BOLD_CODE
                + "world!",
            // Style from prev line
            "how are you ".to_string() + DIM_CODE + "I'm fine" + RESET_CODE,
            // No Style
            "Great to hear".to_string(),
        ]
        .join("\n");

        let output = create_mapping_text(input.to_string());
        let first_line = output
            .splitn(
                // 2 and not 1 as splitn return in the last element the rest of the string
                2, DELIMITER,
            )
            .collect::<Vec<&str>>()[0];

        let expected = FULL_LINE_LENGTH.to_string();

        assert_eq!(first_line, expected);
    }

    #[test]
    fn output_should_have_same_number_of_lines_when_calculated_by_line_length() {
        let input_lines = [
            // Style from start of the line
            BLACK_BACKGROUND_CODE.to_string()
                + "Hello, "
                + RESET_CODE
                + CYAN_BACKGROUND_CODE
                + BOLD_CODE
                + "world!",
            // Style from prev line
            "how are you ".to_string() + DIM_CODE + "I'm fine" + RESET_CODE,
            // No Style
            "Great to hear".to_string(),
            // No style in the beginning and style in the end
            "I'm happy".to_string() + BOLD_CODE + "!" + RESET_CODE,
            // Empty line
            "".to_string(),
            // Text style in the beginning
            ITALIC_CODE.to_string()
                + UNDERLINE_CODE
                + "this is line with multiple text style"
                + RESET_CODE,
            // All Possible style combined
            BOLD_CODE.to_string()
                + ITALIC_CODE
                + INVERSE_CODE
                + UNDERLINE_CODE
                + STRIKETHROUGH_CODE
                + RGB_FOREGROUND_CODE(255, 255, 255).as_str()
                + RGB_BACKGROUND_CODE(255, 255, 255).as_str()
                + "this is line with all possible styles",
            // Empty line with style from prev line
            "".to_string(),
        ];

        let input = input_lines.join("\n");

        let output = create_mapping_text(input.to_string());

        let number_of_lines_in_mapping =
            (output.len() - output.find(DELIMITER).unwrap()) / FULL_LINE_LENGTH;

        assert_eq!(number_of_lines_in_mapping, input_lines.len());
    }

    #[test]
    fn output_should_have_same_number_of_lines_when_calculated_by_line_numbers() {
        let input_lines = [
            // Style from start of the line
            BLACK_BACKGROUND_CODE.to_string()
                + "Hello, "
                + RESET_CODE
                + CYAN_BACKGROUND_CODE
                + BOLD_CODE
                + "world!",
            // Style from prev line
            "how are you ".to_string() + DIM_CODE + "I'm fine" + RESET_CODE,
            // No Style
            "Great to hear".to_string(),
            // No style in the beginning and style in the end
            "I'm happy".to_string() + BOLD_CODE + "!" + RESET_CODE,
            // Empty line
            "".to_string(),
            // Text style in the beginning
            ITALIC_CODE.to_string()
                + UNDERLINE_CODE
                + "this is line with multiple text style"
                + RESET_CODE,
            // All Possible style combined
            BOLD_CODE.to_string()
                + ITALIC_CODE
                + INVERSE_CODE
                + UNDERLINE_CODE
                + STRIKETHROUGH_CODE
                + RGB_FOREGROUND_CODE(255, 255, 255).as_str()
                + RGB_BACKGROUND_CODE(255, 255, 255).as_str()
                + "this is line with all possible styles",
            // Empty line with style from prev line
            "".to_string(),
        ];

        let input = input_lines.join("\n");

        let output = create_mapping_text(input.to_string());

        let number_of_lines_in_mapping = output.lines().count() - 1; // -1 to remove the header

        assert_eq!(number_of_lines_in_mapping, input_lines.len());
    }

    #[test]
    fn output_should_have_correct_length() {
        let input_lines = [
            // Style from start of the line
            BLACK_BACKGROUND_CODE.to_string()
                + "Hello, "
                + RESET_CODE
                + CYAN_BACKGROUND_CODE
                + BOLD_CODE
                + "world!",
            // Style from prev line
            "how are you ".to_string() + DIM_CODE + "I'm fine" + RESET_CODE,
            // No Style
            "Great to hear".to_string(),
            // No style in the beginning and style in the end
            "I'm happy".to_string() + BOLD_CODE + "!" + RESET_CODE,
            // Empty line
            "".to_string(),
            // Text style in the beginning
            ITALIC_CODE.to_string()
                + UNDERLINE_CODE
                + "this is line with multiple text style"
                + RESET_CODE,
            // All Possible style combined
            BOLD_CODE.to_string()
                + ITALIC_CODE
                + INVERSE_CODE
                + UNDERLINE_CODE
                + STRIKETHROUGH_CODE
                + RGB_FOREGROUND_CODE(255, 255, 255).as_str()
                + RGB_BACKGROUND_CODE(255, 255, 255).as_str()
                + "this is line with all possible styles",
            // Empty line with style from prev line
            "".to_string(),
        ];

        let input = input_lines.join("\n");

        let output = create_mapping_text(input.to_string());

        assert_eq!(
            output.len(),
            LINE_LENGTH.to_string().len() + DELIMITER.len() + input_lines.len() * FULL_LINE_LENGTH
        );
    }

    #[test]
    fn mapping_should_include_initial_style_for_each_line() {
        let input_lines = [
            // Style from start of the line
            BLACK_BACKGROUND_CODE.to_string()
                + "Hello, "
                + RESET_CODE
                + CYAN_BACKGROUND_CODE
                + BOLD_CODE
                + "world!",
            // Style from prev line
            "how are you ".to_string() + DIM_CODE + "I'm fine" + RESET_CODE,
            // No Style
            "Great to hear".to_string(),
            // No style in the beginning and style in the end
            "I'm happy".to_string() + BOLD_CODE + "!" + RESET_CODE,
            // Empty line without style
            "".to_string(),
            // Text style in the beginning
            ITALIC_CODE.to_string()
                + UNDERLINE_CODE
                + "this is line with multiple text style"
                + RESET_CODE,
            // All Possible style combined
            BOLD_CODE.to_string()
                + ITALIC_CODE
                + INVERSE_CODE
                + UNDERLINE_CODE
                + STRIKETHROUGH_CODE
                + RGB_FOREGROUND_CODE(255, 255, 255).as_str()
                + RGB_BACKGROUND_CODE(255, 255, 255).as_str()
                + "this is line with all possible styles",
            
            // Non-empty line with style from prev line
            "hey".to_string(),
        ];

        let input = input_lines.join("\n");
        
        assert_eq!(input.split("\n").collect::<Vec<&str>>().len(), 8);

        let output = create_mapping_text(input.to_string());

        let mapping_output_initial_style_for_each_line = output
            // split_inclusive So last line won't be treated as empty
            .split_inclusive(DELIMITER)
            .into_iter()
            // Skip the header
            .skip(1)
            .map(|line| {
                parse_text_matching_single_span(line)
                    // Reset string as it's not irrelevant here
                    .with_text("".to_string())
            })
            .collect::<Vec<Span>>();


        let expected = [
            Span::empty().with_bg_color(Color::Black),
            Span::empty()
                .with_bg_color(Color::Cyan)
                .with_brightness(Brightness::Bold),
            Span::empty(), // No style at all
            Span::empty(), // No style at the beginning
            Span::empty(), // No style at all
            Span::empty().with_text_style(TextStyle::Italic | TextStyle::Underline),
            Span::empty()
                .with_brightness(Brightness::Bold)
                .with_text_style(
                    TextStyle::Italic
                        | TextStyle::Inverse
                        | TextStyle::Underline
                        | TextStyle::Strikethrough,
                )
                .with_color(Color::Rgb(255, 255, 255))
                .with_bg_color(Color::Rgb(255, 255, 255)),
            
            // Same style from prev line
            Span::empty()
                .with_brightness(Brightness::Bold)
                .with_text_style(
                    TextStyle::Italic
                        | TextStyle::Inverse
                        | TextStyle::Underline
                        | TextStyle::Strikethrough,
                )
                .with_color(Color::Rgb(255, 255, 255))
                .with_bg_color(Color::Rgb(255, 255, 255)),
        ];

        assert_eq!(mapping_output_initial_style_for_each_line, expected);
    }
}
