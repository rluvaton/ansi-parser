use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use futures::pin_mut;
use futures_core::Stream;
use futures_util::StreamExt;

use crate::compose_async_steams;
use crate::files::file_reader::{FileReader, FileReaderOptions};
use crate::mapping_file::constants::*;
use crate::parse_ansi_text::ansi::types::Span;
use crate::parse_ansi_text::ansi_output_to_spans::parse_ansi_split_by_lines_as_spans::{convert_ansi_output_to_lines_of_spans, Line};
use crate::parse_ansi_text::ansi_text_to_output::helpers::merge_text_output;
use crate::parse_ansi_text::ansi_text_to_output::stream_parse::parse_ansi;
use crate::parse_ansi_text::parse_options::ParseOptions;
use crate::streams_helpers::iterator_to_async_stream;

// The format for the mapping is
// <line-length>
// <initial-style-for-line-0><padding until reach line-length?>
// <initial-style-for-line-1><padding until reach line-length?>
// ...
// <initial-style-for-line-n><padding until reach line-length?>



pub async fn create_mapping_file_from_input_path(output_mapping_file_path: PathBuf, input_file_path: PathBuf) {
    let mut file = File::create(output_mapping_file_path).expect("create mapping file failed");


    let file_reader = FileReader::new(FileReaderOptions {
        file_path: input_file_path.to_str().expect("input file path is not valid").to_string(),
        chunk_size_in_bytes: Some(1024 * 1024 * 10), // 10MB

        // Read whole file
        from_bytes: None,
        to_bytes: None,
    });
    
    let output = compose_async_steams!(
        || iterator_to_async_stream(file_reader),
        parse_ansi,
        // TODO -
        // merge_text_output,
        |output| convert_ansi_output_to_lines_of_spans(output, ParseOptions::default())
    ).await;

    write_mapping_file(&mut file, output).await;
}


async fn write_mapping_file(file: &mut File, input: impl Stream<Item = Line>)
{
    let header = FULL_LINE_LENGTH.to_string() + DELIMITER;

    file.write(header.as_bytes()).expect("write header to file failed");

    pin_mut!(input); // needed for iteration

    while let Some(value) = input.next().await {
        // append line to file
        file.write(&*create_line_map(value)).expect("write line to file failed");
    }
}


fn create_line_map(line: Line) -> Vec<u8> {
    let initial_span_for_line = if line.spans.is_empty() { Span::empty() } else { line.spans[0].clone().with_text(vec![]) };

    let initial_style_for_line_ansi_string = initial_span_for_line.serialize_to_ansi_string();

    let ansi_len = initial_style_for_line_ansi_string.len();

    let first_part_padding = " ".repeat(FIRST_PART_LINE_LENGTH - ansi_len).as_bytes().to_vec();

    let location_in_file = line.location_in_file.to_ne_bytes();

    return [initial_style_for_line_ansi_string, first_part_padding, location_in_file.to_vec(), DELIMITER.as_bytes().to_vec()].concat();
}
