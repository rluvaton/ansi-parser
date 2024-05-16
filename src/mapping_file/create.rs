use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use crate::mapping_file::constants::*;
use crate::parse_ansi_text::parse_ansi_split_by_lines_as_spans_iterator::ParseAnsiAsSpansByLinesIterator;
use crate::parse_ansi_text::parse_options::ParseOptions;
use crate::parse_ansi_text::types::Span;

// The format for the mapping is
// <line-length>
// <initial-style-for-line-0><padding until reach line-length?>
// <initial-style-for-line-1><padding until reach line-length?>
// ...
// <initial-style-for-line-n><padding until reach line-length?>

pub fn create_mapping_text(contents: String) -> String {
    let initial_span_for_each_line = ParseAnsiAsSpansByLinesIterator::create_from_str(contents, ParseOptions::default())
        .map(|line| if line.is_empty() {Span::empty()} else {line[0].clone().with_text("".to_string())})
        .collect::<Vec<Span>>();

    let initial_style_for_each_line =
        initial_span_for_each_line
            .into_iter()
            .map(|span| span.serialize_to_ansi_string())
            .map(|ansi_string| {
                let ansi_len = ansi_string.len();

                return ansi_string + " ".repeat(LINE_LENGTH - ansi_len).as_str() + DELIMITER;
            })
            .collect::<Vec<String>>();

    return FULL_LINE_LENGTH.to_string() + DELIMITER + initial_style_for_each_line.join("").as_str();
}

// TODO - create a function that accept a string iterator
pub fn create_mapping_file(file_path: PathBuf, contents: String) {
    let mut file = File::create(file_path).expect("create mapping file failed");

    let header = FULL_LINE_LENGTH.to_string() + DELIMITER;

    file.write(header.as_bytes()).expect("write header to file failed");

    let lines_iterators = ParseAnsiAsSpansByLinesIterator::create_from_str(contents, ParseOptions::default());

    for line in lines_iterators {
        let initial_span_for_line = if line.is_empty() {Span::empty()} else {line[0].clone().with_text("".to_string())};

        let initial_style_for_line_ansi_string = initial_span_for_line.serialize_to_ansi_string();

        let ansi_len = initial_style_for_line_ansi_string.len();

        let padding = " ".repeat(LINE_LENGTH - ansi_len);

        let line_text = initial_style_for_line_ansi_string + padding.as_str() + DELIMITER;

        // append line to file
        file.write(line_text.as_bytes()).expect("write line to file failed");
    }
}
