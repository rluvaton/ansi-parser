use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::files::file_reader::FileReaderOptions;
use crate::mapping_file::constants::*;
use crate::parse_ansi_text::ansi::types::Span;
use crate::parse_ansi_text::parse_options::ParseOptions;
use crate::parse_file::file_to_lines_of_spans::read_ansi_file_to_lines;
use crate::parse_file::types::ReadAnsiFileOptions;
use crate::types::Line;

// The format for the mapping is
// <line-length>
// <initial-style-for-line-0><padding until reach line-length?>
// <initial-style-for-line-1><padding until reach line-length?>
// ...
// <initial-style-for-line-n><padding until reach line-length?>

pub async fn create_mapping_file_from_input_path(
    output_mapping_file_path: PathBuf,
    input_file_path: PathBuf,
) {
    let mut file = File::create(output_mapping_file_path).expect("create mapping file failed");

    let header = FULL_LINE_LENGTH.to_string() + DELIMITER;

    file.write_all(header.as_bytes())
        .expect("write header to file failed");


    let output = read_ansi_file_to_lines(ReadAnsiFileOptions {
        file_options: FileReaderOptions {
            file_path: input_file_path
                .to_str()
                .expect("input file path is not valid")
                .to_string(),
            chunk_size_in_bytes: Some(1024 * 1024 * 10), // 10MB
            from_bytes: None,
            to_bytes: None,
        },
        parse_options: ParseOptions::default(),
    });
    
    output.for_each(|value| {
        // append line to file
        file.write_all(&create_line_map(value))
            .expect("write line to file failed");
    });
}


fn create_line_map(line: Line) -> Vec<u8> {
    let initial_span_for_line = if line.spans.is_empty() {
        Span::empty()
    } else {
        line.spans[0].clone().with_text(vec![])
    };

    let initial_style_for_line_ansi_string = initial_span_for_line.serialize_to_ansi_string();

    let ansi_len = initial_style_for_line_ansi_string.len();

    let first_part_padding = " "
        .repeat(FIRST_PART_LINE_LENGTH - ansi_len)
        .as_bytes()
        .to_vec();

    let location_in_file = line.location_in_file.to_ne_bytes();

    return [
        initial_style_for_line_ansi_string,
        first_part_padding,
        location_in_file.to_vec(),
        DELIMITER.as_bytes().to_vec(),
    ]
    .concat();
}
