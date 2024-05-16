#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use tempfile::*;
    use crate::mapping_file::create::*;
    use crate::mapping_file::read::*;
    use crate::mapping_file::*;
    use pretty_assertions::assert_eq;

    use crate::parse_ansi_text::constants::RESET_CODE;
    use crate::parse_ansi_text::parse_text_matching_single_span::parse_text_matching_single_span;
    use crate::parse_ansi_text::types::Span;

    fn get_tmp_file_path() -> PathBuf {
        return NamedTempFile::new().expect("create temp file").into_temp_path().to_path_buf();
    }

    // --------------------------------
    // Create Mapping text from string
    // --------------------------------

    #[test]
    fn in_memory_output_should_have_line_length_before_first_delimiter() {
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
    fn in_memory_output_should_have_same_number_of_lines_when_calculated_by_line_length() {
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
    fn in_memory_output_should_have_same_number_of_lines_when_calculated_by_line_numbers() {
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
    fn in_memory_output_should_have_correct_length() {
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
    fn in_memory_mapping_should_include_initial_style_for_each_line() {
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

    // -------------------------------------
    // Create mapping text file from string
    // -------------------------------------

    #[test]
    fn file_output_should_have_line_length_before_first_delimiter() {
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

        let tmp_file_path = get_tmp_file_path();

        create_mapping_file(tmp_file_path.clone(), input.to_string());

        let mapping_file_content = std::fs::read_to_string(tmp_file_path.clone()).unwrap();

        let first_line = mapping_file_content
            .splitn(
                // 2 and not 1 as splitn return in the last element the rest of the string
                2, DELIMITER,
            )
            .collect::<Vec<&str>>()[0];

        let expected = FULL_LINE_LENGTH.to_string();

        assert_eq!(first_line, expected);
    }

    #[test]
    fn file_output_should_have_same_number_of_lines_when_calculated_by_line_length() {
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

        let tmp_file_path = get_tmp_file_path();

        create_mapping_file(tmp_file_path.clone(), input.to_string());

        let mapping_file_content = std::fs::read_to_string(tmp_file_path.clone()).unwrap();

        let number_of_lines_in_mapping =
            (mapping_file_content.len() - mapping_file_content.find(DELIMITER).unwrap()) / FULL_LINE_LENGTH;

        assert_eq!(number_of_lines_in_mapping, input_lines.len());
    }

    #[test]
    fn file_output_should_have_same_number_of_lines_when_calculated_by_line_numbers() {
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

        let tmp_file_path = get_tmp_file_path();

        create_mapping_file(tmp_file_path.clone(), input.to_string());

        let mapping_file_content = std::fs::read_to_string(tmp_file_path.clone()).unwrap();

        let number_of_lines_in_mapping = mapping_file_content.lines().count() - 1; // -1 to remove the header

        assert_eq!(number_of_lines_in_mapping, input_lines.len());
    }

    #[test]
    fn file_output_should_have_correct_length() {
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

        let tmp_file_path = get_tmp_file_path();

        create_mapping_file(tmp_file_path.clone(), input.to_string());

        let mapping_file_content = std::fs::read_to_string(tmp_file_path.clone()).unwrap();

        assert_eq!(
            mapping_file_content.len(),
            LINE_LENGTH.to_string().len() + DELIMITER.len() + input_lines.len() * FULL_LINE_LENGTH
        );
    }

    #[test]
    fn file_mapping_should_include_initial_style_for_each_line() {
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

        let tmp_file_path = get_tmp_file_path();

        create_mapping_file(tmp_file_path.clone(), input.to_string());

        let mapping_file_content = std::fs::read_to_string(tmp_file_path.clone()).unwrap();

        let mapping_output_initial_style_for_each_line = mapping_file_content
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

    // ---------------------------------------------
    // Create mapping text file from input file path
    // ---------------------------------------------

    #[test]
    fn file_input_and_output_should_have_line_length_before_first_delimiter() {
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
        

        let tmp_input_file_path = get_tmp_file_path();
        let tmp_mapping_file_path = get_tmp_file_path();
        
        std::fs::write(tmp_input_file_path.clone(), input.to_string()).expect("write input file failed");

        create_mapping_file_from_input_path(tmp_mapping_file_path.clone(), tmp_input_file_path.clone());

        let mapping_file_content = std::fs::read_to_string(tmp_mapping_file_path.clone()).unwrap();

        let first_line = mapping_file_content
            .splitn(
                // 2 and not 1 as splitn return in the last element the rest of the string
                2, DELIMITER,
            )
            .collect::<Vec<&str>>()[0];

        let expected = FULL_LINE_LENGTH.to_string();

        assert_eq!(first_line, expected);
    }

    #[test]
    fn file_input_and_output_should_have_same_number_of_lines_when_calculated_by_line_length() {
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

        let tmp_input_file_path = get_tmp_file_path();
        let tmp_mapping_file_path = get_tmp_file_path();

        std::fs::write(tmp_input_file_path.clone(), input.to_string()).expect("write input file failed");

        create_mapping_file_from_input_path(tmp_mapping_file_path.clone(), tmp_input_file_path.clone());

        let mapping_file_content = std::fs::read_to_string(tmp_mapping_file_path.clone()).unwrap();

        let number_of_lines_in_mapping =
            (mapping_file_content.len() - mapping_file_content.find(DELIMITER).unwrap()) / FULL_LINE_LENGTH;

        assert_eq!(number_of_lines_in_mapping, input_lines.len());
    }

    #[test]
    fn file_input_and_output_should_have_same_number_of_lines_when_calculated_by_line_numbers() {
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


        let tmp_input_file_path = get_tmp_file_path();
        let tmp_mapping_file_path = get_tmp_file_path();

        std::fs::write(tmp_input_file_path.clone(), input.to_string()).expect("write input file failed");

        create_mapping_file_from_input_path(tmp_mapping_file_path.clone(), tmp_input_file_path.clone());

        let mapping_file_content = std::fs::read_to_string(tmp_mapping_file_path.clone()).unwrap();

        let number_of_lines_in_mapping = mapping_file_content.lines().count() - 1; // -1 to remove the header

        assert_eq!(number_of_lines_in_mapping, input_lines.len());
    }

    #[test]
    fn file_input_and_output_should_have_correct_length() {
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


        let tmp_input_file_path = get_tmp_file_path();
        let tmp_mapping_file_path = get_tmp_file_path();

        std::fs::write(tmp_input_file_path.clone(), input.to_string()).expect("write input file failed");

        create_mapping_file_from_input_path(tmp_mapping_file_path.clone(), tmp_input_file_path.clone());

        let mapping_file_content = std::fs::read_to_string(tmp_mapping_file_path.clone()).unwrap();

        assert_eq!(
            mapping_file_content.len(),
            LINE_LENGTH.to_string().len() + DELIMITER.len() + input_lines.len() * FULL_LINE_LENGTH
        );
    }

    #[test]
    fn file_input_and_output_mapping_should_include_initial_style_for_each_line() {
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


        let tmp_input_file_path = get_tmp_file_path();
        let tmp_mapping_file_path = get_tmp_file_path();

        std::fs::write(tmp_input_file_path.clone(), input.to_string()).expect("write input file failed");

        create_mapping_file_from_input_path(tmp_mapping_file_path.clone(), tmp_input_file_path.clone());

        let mapping_file_content = std::fs::read_to_string(tmp_mapping_file_path.clone()).unwrap();

        let mapping_output_initial_style_for_each_line = mapping_file_content
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

    // ---------------------
    // Consume Mapping text
    // ---------------------

    #[test]
    fn in_memory_should_return_initial_span_for_text_with_one_line() {
        let input = BLACK_BACKGROUND_CODE.to_string()
            + "Hello, "
            + RESET_CODE
            + CYAN_BACKGROUND_CODE
            + BOLD_CODE
            + "world!";

        let mapping_text = create_mapping_text(input.to_string());

        let initial_style = get_initial_style_for_line(mapping_text.clone(), 1);

        let expected = Span::empty().with_bg_color(Color::Black);

        assert_eq!(initial_style, Some(expected));
    }

    #[test]
    fn in_memory_should_return_correct_initial_style_for_each_line() {
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

        let mapping_text = create_mapping_text(input.to_string());

        let mut initial_style_for_each_line: Vec<Option<Span>> = vec![];

        for i in 0..input_lines.len() {
            let initial_style = get_initial_style_for_line(mapping_text.clone(), i + 1);

            initial_style_for_each_line.push(initial_style);
        }

        let expected = [
            Some(Span::empty().with_bg_color(Color::Black)),
            Some(
                Span::empty()
                    .with_bg_color(Color::Cyan)
                    .with_brightness(Brightness::Bold),
            ),
            Some(Span::empty()), // No style at all
            Some(Span::empty()), // No style at the beginning
            Some(Span::empty()), // No style at all
            Some(Span::empty().with_text_style(TextStyle::Italic | TextStyle::Underline)),
            Some(
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
            ),
            // Same style from prev line
            Some(
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
            ),
        ];

        assert_eq!(initial_style_for_each_line, expected);
    }

    // ---------------------
    // Consume Mapping file
    // ---------------------

    #[test]
    fn file_path_should_return_initial_span_for_text_with_one_line() {
        let input = BLACK_BACKGROUND_CODE.to_string()
            + "Hello, "
            + RESET_CODE
            + CYAN_BACKGROUND_CODE
            + BOLD_CODE
            + "world!";

        let tmp_file_path = get_tmp_file_path();

        create_mapping_file(tmp_file_path.clone(), input.to_string());

        let initial_style = get_initial_style_for_line_from_file_path(tmp_file_path.clone(), 1);

        let expected = Span::empty().with_bg_color(Color::Black);

        assert_eq!(initial_style, Some(expected));
    }
    
    #[test]
    fn file_path_should_return_initial_span_for_line_in_the_middle() {
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
        ];

        let input = input_lines.join("\n");

        let tmp_file_path = get_tmp_file_path();

        create_mapping_file(tmp_file_path.clone(), input.to_string());

        let initial_style = get_initial_style_for_line_from_file_path(tmp_file_path.clone(), 2);

        let expected = Span::empty().with_bg_color(Color::Cyan).with_brightness(Brightness::Bold);

        assert_eq!(initial_style, Some(expected));
    }

    #[test]
    fn file_path_should_return_correct_initial_style_for_each_line() {
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

        let tmp_file_path = get_tmp_file_path();

        create_mapping_file(tmp_file_path.clone(), input.to_string());

        let mut initial_style_for_each_line: Vec<Option<Span>> = vec![];

        for i in 0..input_lines.len() {
            let initial_style = get_initial_style_for_line_from_file_path(tmp_file_path.clone(), i + 1);

            initial_style_for_each_line.push(initial_style);
        }

        let expected = [
            Some(Span::empty().with_bg_color(Color::Black)),
            Some(
                Span::empty()
                    .with_bg_color(Color::Cyan)
                    .with_brightness(Brightness::Bold),
            ),
            Some(Span::empty()), // No style at all
            Some(Span::empty()), // No style at the beginning
            Some(Span::empty()), // No style at all
            Some(Span::empty().with_text_style(TextStyle::Italic | TextStyle::Underline)),
            Some(
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
            ),
            // Same style from prev line
            Some(
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
            ),
        ];

        assert_eq!(initial_style_for_each_line, expected);
    }

    #[test]
    fn file_path_should_return_correct_initial_style_for_each_line_when_requesting_from_end_to_start() {
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

        let tmp_file_path = get_tmp_file_path();

        create_mapping_file(tmp_file_path.clone(), input.to_string());

        let mut initial_style_for_each_line: Vec<Option<Span>> = vec![];

        for i in (0..input_lines.len()).rev() {
            let initial_style = get_initial_style_for_line_from_file_path(tmp_file_path.clone(), i + 1);

            initial_style_for_each_line.push(initial_style.clone());
        }

        // We read at the opposite order so we need to reverse to get the correct order of lines
        initial_style_for_each_line.reverse();

        let expected = [
            Some(Span::empty().with_bg_color(Color::Black)),
            Some(
                Span::empty()
                    .with_bg_color(Color::Cyan)
                    .with_brightness(Brightness::Bold),
            ),
            Some(Span::empty()), // No style at all
            Some(Span::empty()), // No style at the beginning
            Some(Span::empty()), // No style at all
            Some(Span::empty().with_text_style(TextStyle::Italic | TextStyle::Underline)),
            Some(
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
            ),
            // Same style from prev line
            Some(
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
            ),
        ];

        assert_eq!(initial_style_for_each_line, expected);
    }

    // -----------------------------------
    // Consume Mapping file with open file
    // -----------------------------------

    #[test]
    fn file_should_return_correct_initial_style_for_each_line() {
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

        let tmp_file_path = get_tmp_file_path();

        create_mapping_file(tmp_file_path.clone(), input.to_string());

        let mut initial_style_for_each_line: Vec<Option<Span>> = vec![];

        let ready_data_for_reading_file= get_mapping_file_ready_to_read(tmp_file_path.clone());

        assert_eq!(ready_data_for_reading_file.is_none(), false);

        let (mut file, content_start_offset, line_length) = ready_data_for_reading_file.unwrap();

        for i in 0..input_lines.len() {
            let initial_style = get_initial_style_for_line_from_file(&mut file, i + 1, content_start_offset, line_length);

            initial_style_for_each_line.push(initial_style);
        }

        let expected = [
            Some(Span::empty().with_bg_color(Color::Black)),
            Some(
                Span::empty()
                    .with_bg_color(Color::Cyan)
                    .with_brightness(Brightness::Bold),
            ),
            Some(Span::empty()), // No style at all
            Some(Span::empty()), // No style at the beginning
            Some(Span::empty()), // No style at all
            Some(Span::empty().with_text_style(TextStyle::Italic | TextStyle::Underline)),
            Some(
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
            ),
            // Same style from prev line
            Some(
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
            ),
        ];

        assert_eq!(initial_style_for_each_line, expected);
    }

    #[test]
    fn file_should_return_correct_initial_style_for_each_line_from_end_to_start() {
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

        let tmp_file_path = get_tmp_file_path();

        create_mapping_file(tmp_file_path.clone(), input.to_string());

        let mut initial_style_for_each_line: Vec<Option<Span>> = vec![];

        let ready_data_for_reading_file= get_mapping_file_ready_to_read(tmp_file_path.clone());

        assert_eq!(ready_data_for_reading_file.is_none(), false);

        let (mut file, content_start_offset, line_length) = ready_data_for_reading_file.unwrap();

        for i in (0..input_lines.len()).rev() {
            let initial_style = get_initial_style_for_line_from_file(&mut file, i + 1, content_start_offset, line_length);

            initial_style_for_each_line.push(initial_style);
        }
        
        // We read at the opposite order so we need to reverse to get the correct order of lines
        initial_style_for_each_line.reverse();

        let expected = [
            Some(Span::empty().with_bg_color(Color::Black)),
            Some(
                Span::empty()
                    .with_bg_color(Color::Cyan)
                    .with_brightness(Brightness::Bold),
            ),
            Some(Span::empty()), // No style at all
            Some(Span::empty()), // No style at the beginning
            Some(Span::empty()), // No style at all
            Some(Span::empty().with_text_style(TextStyle::Italic | TextStyle::Underline)),
            Some(
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
            ),
            // Same style from prev line
            Some(
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
            ),
        ];

        assert_eq!(initial_style_for_each_line, expected);
    }

    //
    // #[test]
    // fn should_throw_for_missing_line_in_mapping() {
    //     let input_lines = [
    //         // Style from start of the line
    //         BLACK_BACKGROUND_CODE.to_string()
    //             + "Hello, "
    //             + RESET_CODE
    //             + CYAN_BACKGROUND_CODE
    //             + BOLD_CODE
    //             + "world!",
    //         // Style from prev line
    //         "how are you ".to_string() + DIM_CODE + "I'm fine" + RESET_CODE,
    //     ];
    //
    //     let input = input_lines.join("\n");
    //
    //     let mapping_text = create_mapping_text(input.to_string());
    //
    //     let initial_style = get_initial_style_for_line(mapping_text.clone(), 6);
    //
    //     assert_eq!(initial_style, Span::empty());
    // }
}
