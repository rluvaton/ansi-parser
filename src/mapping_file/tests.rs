#[cfg(test)]
mod tests {
    use std::io::BufRead;
    use std::path::PathBuf;

    use pretty_assertions::assert_eq;
    use tempfile::*;

    use crate::mapping_file::constants::*;
    use crate::mapping_file::create::*;
    use crate::mapping_file::read::*;
    use crate::parse_ansi_text::ansi::colors::*;
    use crate::parse_ansi_text::ansi::constants::RESET_CODE;
    use crate::parse_ansi_text::ansi::style::*;
    use crate::parse_ansi_text::ansi::types::Span;
    use crate::parse_ansi_text::parse_text_matching_single_span::parse_text_matching_single_span;

    fn get_tmp_file_path() -> PathBuf {
        return NamedTempFile::new()
            .expect("create temp file")
            .into_temp_path()
            .to_path_buf();
    }

    fn calculate_chars_until_line(lines: Vec<String>, line_number: usize) -> usize {
        let mut count = 0;

        for i in 0..line_number {
            count += lines[i].chars().count() + 1; // +1 for the newline
        }

        return count;
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

        std::fs::write(tmp_input_file_path.clone(), input.to_string())
            .expect("write input file failed");

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

        std::fs::write(tmp_input_file_path.clone(), input.to_string())
            .expect("write input file failed");

        create_mapping_file_from_input_path(tmp_mapping_file_path.clone(), tmp_input_file_path.clone());

        let mapping_file_content = std::fs::read(tmp_mapping_file_path.clone()).unwrap();
        let number_of_lines_in_mapping = (mapping_file_content.len()
            - mapping_file_content
            .iter()
            .position(|item| *item == b'\n')
            .unwrap())
            / FULL_LINE_LENGTH;

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

        std::fs::write(tmp_input_file_path.clone(), input.to_string())
            .expect("write input file failed");

        create_mapping_file_from_input_path(tmp_mapping_file_path.clone(), tmp_input_file_path.clone());

        let mapping_file_content = std::fs::read(tmp_mapping_file_path.clone()).unwrap();

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

        std::fs::write(tmp_input_file_path.clone(), input.to_string())
            .expect("write input file failed");

        create_mapping_file_from_input_path(tmp_mapping_file_path.clone(), tmp_input_file_path.clone());

        let mapping_file_content = std::fs::read(tmp_mapping_file_path.clone()).unwrap();

        assert_eq!(
            mapping_file_content.len(),
            FIRST_PART_LINE_LENGTH.to_string().len()
                + DELIMITER.len()
                + input_lines.len() * FULL_LINE_LENGTH
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

        std::fs::write(tmp_input_file_path.clone(), input.to_string())
            .expect("write input file failed");

        create_mapping_file_from_input_path(tmp_mapping_file_path.clone(), tmp_input_file_path.clone());

        let mapping_file_content = std::fs::read(tmp_mapping_file_path.clone()).unwrap();

        let mapping_output_initial_style_for_each_line = mapping_file_content
            // split_inclusive So last line won't be treated as empty
            .split_inclusive(|item| *item == DELIMITER.as_bytes()[0])
            .into_iter()
            // Skip the header
            .skip(1)
            .map(|line| {
                String::from_utf8(line[0..FIRST_PART_LINE_LENGTH].to_owned())
                    .expect("convert to string")
            })
            .map(|line| {
                parse_text_matching_single_span(line.as_str())
                    // Reset string as it's not irrelevant here
                    .with_text("".to_string().into_bytes())
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

        let tmp_input_file_path = get_tmp_file_path();
        let tmp_mapping_file_path = get_tmp_file_path();

        std::fs::write(tmp_input_file_path.clone(), input.to_string())
            .expect("write input file failed");

        create_mapping_file_from_input_path(tmp_mapping_file_path.clone(), tmp_input_file_path.clone());

        let line_metadata = get_line_metadata_from_file_path(tmp_mapping_file_path.clone(), 1);

        let expected = Span::empty().with_bg_color(Color::Black);

        assert_eq!(
            line_metadata,
            Some(MappingItem {
                initial_span: expected,
                location_in_original_file: 0,
            })
        );
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

        let tmp_input_file_path = get_tmp_file_path();
        let tmp_mapping_file_path = get_tmp_file_path();

        std::fs::write(tmp_input_file_path.clone(), input.to_string())
            .expect("write input file failed");

        create_mapping_file_from_input_path(tmp_mapping_file_path.clone(), tmp_input_file_path.clone());

        let line_metadata = get_line_metadata_from_file_path(tmp_mapping_file_path.clone(), 2);

        let expected = Span::empty()
            .with_bg_color(Color::Cyan)
            .with_brightness(Brightness::Bold);

        assert_eq!(
            line_metadata,
            Some(MappingItem {
                initial_span: expected,
                location_in_original_file: input.find("\n").unwrap() + 1,
            })
        );
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

        let tmp_input_file_path = get_tmp_file_path();
        let tmp_mapping_file_path = get_tmp_file_path();

        std::fs::write(tmp_input_file_path.clone(), input.to_string())
            .expect("write input file failed");

        create_mapping_file_from_input_path(tmp_mapping_file_path.clone(), tmp_input_file_path.clone());

        let mut all_lines_metadata: Vec<Option<MappingItem>> = vec![];

        for i in 0..input_lines.len() {
            let line_metadata = get_line_metadata_from_file_path(tmp_mapping_file_path.clone(), i + 1);

            all_lines_metadata.push(line_metadata);
        }

        let expected = [
            Some(MappingItem {
                initial_span: Span::empty().with_bg_color(Color::Black),
                location_in_original_file: 0,
            }),
            Some(MappingItem {
                initial_span: Span::empty()
                    .with_bg_color(Color::Cyan)
                    .with_brightness(Brightness::Bold),
                location_in_original_file: calculate_chars_until_line(input_lines.to_vec(), 1),
            }),
            Some(MappingItem {
                initial_span: Span::empty(), // No style at all
                location_in_original_file: calculate_chars_until_line(input_lines.to_vec(), 2),
            }),
            Some(MappingItem {
                initial_span: Span::empty(), // No style at the beginning
                location_in_original_file: calculate_chars_until_line(input_lines.to_vec(), 3),
            }),
            Some(MappingItem {
                initial_span: Span::empty(), // No style at all
                location_in_original_file: calculate_chars_until_line(input_lines.to_vec(), 4),
            }),
            Some(MappingItem {
                initial_span: Span::empty().with_text_style(TextStyle::Italic | TextStyle::Underline),
                location_in_original_file: calculate_chars_until_line(input_lines.to_vec(), 5),
            }),
            Some(MappingItem {
                initial_span: Span::empty()
                    .with_brightness(Brightness::Bold)
                    .with_text_style(
                        TextStyle::Italic
                            | TextStyle::Inverse
                            | TextStyle::Underline
                            | TextStyle::Strikethrough,
                    )
                    .with_color(Color::Rgb(255, 255, 255))
                    .with_bg_color(Color::Rgb(255, 255, 255)),
                location_in_original_file: calculate_chars_until_line(input_lines.to_vec(), 6),
            }),
            // Same style from prev line
            Some(MappingItem {
                initial_span: Span::empty()
                    .with_brightness(Brightness::Bold)
                    .with_text_style(
                        TextStyle::Italic
                            | TextStyle::Inverse
                            | TextStyle::Underline
                            | TextStyle::Strikethrough,
                    )
                    .with_color(Color::Rgb(255, 255, 255))
                    .with_bg_color(Color::Rgb(255, 255, 255)),
                location_in_original_file: calculate_chars_until_line(input_lines.to_vec(), 7),
            }),
        ];

        assert_eq!(all_lines_metadata, expected);
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

        let tmp_input_file_path = get_tmp_file_path();
        let tmp_mapping_file_path = get_tmp_file_path();

        std::fs::write(tmp_input_file_path.clone(), input.to_string())
            .expect("write input file failed");

        create_mapping_file_from_input_path(tmp_mapping_file_path.clone(), tmp_input_file_path.clone());

        let mut all_lines_metadata: Vec<Option<MappingItem>> = vec![];

        for i in (0..input_lines.len()).rev() {
            let line_metadata = get_line_metadata_from_file_path(tmp_mapping_file_path.clone(), i + 1);

            all_lines_metadata.push(line_metadata.clone());
        }

        // We read at the opposite order so we need to reverse to get the correct order of lines
        all_lines_metadata.reverse();

        let expected = [
            Some(MappingItem {
                initial_span: Span::empty().with_bg_color(Color::Black),
                location_in_original_file: 0,
            }),
            Some(MappingItem {
                initial_span: Span::empty()
                    .with_bg_color(Color::Cyan)
                    .with_brightness(Brightness::Bold),
                location_in_original_file: calculate_chars_until_line(input_lines.to_vec(), 1),
            }),
            Some(MappingItem {
                initial_span: Span::empty(), // No style at all
                location_in_original_file: calculate_chars_until_line(input_lines.to_vec(), 2),
            }),
            Some(MappingItem {
                initial_span: Span::empty(), // No style at the beginning
                location_in_original_file: calculate_chars_until_line(input_lines.to_vec(), 3),
            }),
            Some(MappingItem {
                initial_span: Span::empty(), // No style at all
                location_in_original_file: calculate_chars_until_line(input_lines.to_vec(), 4),
            }),
            Some(MappingItem {
                initial_span: Span::empty().with_text_style(TextStyle::Italic | TextStyle::Underline),
                location_in_original_file: calculate_chars_until_line(input_lines.to_vec(), 5),
            }),
            Some(MappingItem {
                initial_span: Span::empty()
                    .with_brightness(Brightness::Bold)
                    .with_text_style(
                        TextStyle::Italic
                            | TextStyle::Inverse
                            | TextStyle::Underline
                            | TextStyle::Strikethrough,
                    )
                    .with_color(Color::Rgb(255, 255, 255))
                    .with_bg_color(Color::Rgb(255, 255, 255)),
                location_in_original_file: calculate_chars_until_line(input_lines.to_vec(), 6),
            }),
            // Same style from prev line
            Some(MappingItem {
                initial_span: Span::empty()
                    .with_brightness(Brightness::Bold)
                    .with_text_style(
                        TextStyle::Italic
                            | TextStyle::Inverse
                            | TextStyle::Underline
                            | TextStyle::Strikethrough,
                    )
                    .with_color(Color::Rgb(255, 255, 255))
                    .with_bg_color(Color::Rgb(255, 255, 255)),
                location_in_original_file: calculate_chars_until_line(input_lines.to_vec(), 7),
            }),
        ];

        assert_eq!(all_lines_metadata, expected);
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

        let tmp_input_file_path = get_tmp_file_path();
        let tmp_mapping_file_path = get_tmp_file_path();

        std::fs::write(tmp_input_file_path.clone(), input.to_string())
            .expect("write input file failed");

        create_mapping_file_from_input_path(tmp_mapping_file_path.clone(), tmp_input_file_path.clone());

        let mut initial_style_for_each_line: Vec<Option<MappingItem>> = vec![];

        let ready_data_for_reading_file = get_mapping_file_ready_to_read(tmp_mapping_file_path.clone());

        assert_eq!(ready_data_for_reading_file.is_none(), false);

        let (mut file, content_start_offset, line_length) = ready_data_for_reading_file.unwrap();

        for i in 0..input_lines.len() {
            let initial_style =
                get_line_metadata_from_file(&mut file, i + 1, content_start_offset, line_length);

            initial_style_for_each_line.push(initial_style);
        }

        let expected = [
            Some(MappingItem {
                initial_span: Span::empty().with_bg_color(Color::Black),
                location_in_original_file: 0,
            }),
            Some(MappingItem {
                initial_span: Span::empty()
                    .with_bg_color(Color::Cyan)
                    .with_brightness(Brightness::Bold),
                location_in_original_file: calculate_chars_until_line(input_lines.to_vec(), 1),
            }),
            Some(MappingItem {
                initial_span: Span::empty(), // No style at all
                location_in_original_file: calculate_chars_until_line(input_lines.to_vec(), 2),
            }),
            Some(MappingItem {
                initial_span: Span::empty(), // No style at the beginning
                location_in_original_file: calculate_chars_until_line(input_lines.to_vec(), 3),
            }),
            Some(MappingItem {
                initial_span: Span::empty(), // No style at all
                location_in_original_file: calculate_chars_until_line(input_lines.to_vec(), 4),
            }),
            Some(MappingItem {
                initial_span: Span::empty().with_text_style(TextStyle::Italic | TextStyle::Underline),
                location_in_original_file: calculate_chars_until_line(input_lines.to_vec(), 5),
            }),
            Some(MappingItem {
                initial_span: Span::empty()
                    .with_brightness(Brightness::Bold)
                    .with_text_style(
                        TextStyle::Italic
                            | TextStyle::Inverse
                            | TextStyle::Underline
                            | TextStyle::Strikethrough,
                    )
                    .with_color(Color::Rgb(255, 255, 255))
                    .with_bg_color(Color::Rgb(255, 255, 255)),
                location_in_original_file: calculate_chars_until_line(input_lines.to_vec(), 6),
            }),
            // Same style from prev line
            Some(MappingItem {
                initial_span: Span::empty()
                    .with_brightness(Brightness::Bold)
                    .with_text_style(
                        TextStyle::Italic
                            | TextStyle::Inverse
                            | TextStyle::Underline
                            | TextStyle::Strikethrough,
                    )
                    .with_color(Color::Rgb(255, 255, 255))
                    .with_bg_color(Color::Rgb(255, 255, 255)),
                location_in_original_file: calculate_chars_until_line(input_lines.to_vec(), 7),
            }),
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

        let tmp_input_file_path = get_tmp_file_path();
        let tmp_mapping_file_path = get_tmp_file_path();

        std::fs::write(tmp_input_file_path.clone(), input.to_string())
            .expect("write input file failed");

        create_mapping_file_from_input_path(tmp_mapping_file_path.clone(), tmp_input_file_path.clone());

        let mut initial_style_for_each_line: Vec<Option<MappingItem>> = vec![];

        let ready_data_for_reading_file = get_mapping_file_ready_to_read(tmp_mapping_file_path.clone());

        assert_eq!(ready_data_for_reading_file.is_none(), false);

        let (mut file, content_start_offset, line_length) = ready_data_for_reading_file.unwrap();

        for i in (0..input_lines.len()).rev() {
            let initial_style =
                get_line_metadata_from_file(&mut file, i + 1, content_start_offset, line_length);

            initial_style_for_each_line.push(initial_style);
        }

        // We read at the opposite order so we need to reverse to get the correct order of lines
        initial_style_for_each_line.reverse();

        let expected = [
            Some(MappingItem {
                initial_span: Span::empty().with_bg_color(Color::Black),
                location_in_original_file: 0,
            }),
            Some(MappingItem {
                initial_span: Span::empty()
                    .with_bg_color(Color::Cyan)
                    .with_brightness(Brightness::Bold),
                location_in_original_file: calculate_chars_until_line(input_lines.to_vec(), 1),
            }),
            Some(MappingItem {
                initial_span: Span::empty(), // No style at all
                location_in_original_file: calculate_chars_until_line(input_lines.to_vec(), 2),
            }),
            Some(MappingItem {
                initial_span: Span::empty(), // No style at the beginning
                location_in_original_file: calculate_chars_until_line(input_lines.to_vec(), 3),
            }),
            Some(MappingItem {
                initial_span: Span::empty(), // No style at all
                location_in_original_file: calculate_chars_until_line(input_lines.to_vec(), 4),
            }),
            Some(MappingItem {
                initial_span: Span::empty().with_text_style(TextStyle::Italic | TextStyle::Underline),
                location_in_original_file: calculate_chars_until_line(input_lines.to_vec(), 5),
            }),
            Some(MappingItem {
                initial_span: Span::empty()
                    .with_brightness(Brightness::Bold)
                    .with_text_style(
                        TextStyle::Italic
                            | TextStyle::Inverse
                            | TextStyle::Underline
                            | TextStyle::Strikethrough,
                    )
                    .with_color(Color::Rgb(255, 255, 255))
                    .with_bg_color(Color::Rgb(255, 255, 255)),
                location_in_original_file: calculate_chars_until_line(input_lines.to_vec(), 6),
            }),
            // Same style from prev line
            Some(MappingItem {
                initial_span: Span::empty()
                    .with_brightness(Brightness::Bold)
                    .with_text_style(
                        TextStyle::Italic
                            | TextStyle::Inverse
                            | TextStyle::Underline
                            | TextStyle::Strikethrough,
                    )
                    .with_color(Color::Rgb(255, 255, 255))
                    .with_bg_color(Color::Rgb(255, 255, 255)),
                location_in_original_file: calculate_chars_until_line(input_lines.to_vec(), 7),
            }),
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
