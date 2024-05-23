use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::mapping_file::constants::*;
use crate::parse_ansi_text::ansi::types::Span;
use crate::parse_ansi_text::iterators::parse_ansi_split_by_lines_as_spans_iterator::{Line, ParseAnsiAsSpansByLinesIterator};
use crate::parse_ansi_text::parse_options::ParseOptions;

// The format for the mapping is
// <line-length>
// <initial-style-for-line-0><padding until reach line-length?>
// <initial-style-for-line-1><padding until reach line-length?>
// ...
// <initial-style-for-line-n><padding until reach line-length?>


pub fn create_mapping_file(file_path: PathBuf, contents: String) {
    let mut file = File::create(file_path).expect("create mapping file failed");

    let lines_iterators = ParseAnsiAsSpansByLinesIterator::create_from_str(contents, ParseOptions::default());
    
    write_mapping_file_from_iterator(&mut file, lines_iterators);
}


pub fn create_mapping_file_from_input_path(output_mapping_file_path: PathBuf, input_file_path: PathBuf) {
    let mut file = File::create(output_mapping_file_path).expect("create mapping file failed");

    let iterator = ParseAnsiAsSpansByLinesIterator::create_from_file_path(input_file_path, ParseOptions::default());

    write_mapping_file_from_iterator(&mut file, iterator);
}

fn write_mapping_file_from_iterator(file: &mut File, iterator: ParseAnsiAsSpansByLinesIterator) {
    let header = FULL_LINE_LENGTH.to_string() + DELIMITER;
    
    file.write(header.as_bytes()).expect("write header to file failed");

    for line in iterator {
        // append line to file
        file.write(&*create_line_map(line)).expect("write line to file failed");
    }
}

fn create_line_map(line: Line) -> Vec<u8> {
    let initial_span_for_line = if line.spans.is_empty() { Span::empty() } else { line.spans[0].clone().with_text("".to_string()) };

    let initial_style_for_line_ansi_string = initial_span_for_line.serialize_to_ansi_string();

    let ansi_len = initial_style_for_line_ansi_string.len();

    let first_part_padding = " ".repeat(FIRST_PART_LINE_LENGTH - ansi_len);

    let location_in_file = line.location_in_file.to_ne_bytes();

    
    return [initial_style_for_line_ansi_string.as_bytes(), first_part_padding.as_bytes(), location_in_file.as_slice(), DELIMITER.as_bytes()].concat();
}
