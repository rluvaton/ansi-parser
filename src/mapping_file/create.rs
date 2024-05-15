use crate::mapping_file::constants::*;
use crate::parse_ansi_text::parse_ansi_split_by_lines_as_spans_iterator::ParseAnsiAsSpansByLines;
use crate::parse_ansi_text::parse_options::ParseOptions;
use crate::parse_ansi_text::types::Span;

// The format for the mapping is
// <line-length>
// <initial-style-for-line-0><padding until reach line-length?>
// <initial-style-for-line-1><padding until reach line-length?>
// ...
// <initial-style-for-line-n><padding until reach line-length?>

pub fn create_mapping_text(contents: String) -> String {
    let initial_span_for_each_line = contents
        .parse_ansi_as_spans_by_lines(ParseOptions::default())
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
