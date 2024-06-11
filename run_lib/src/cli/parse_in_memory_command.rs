use std::ffi::OsString;
use std::path::PathBuf;

use ansi_parser_extended::parse_file::text_to_lines_of_spans::buffer_to_lines;
use ansi_parser_extended::parse_file::text_to_spans::buffer_to_spans;

pub fn run_parse_command_in_memory(matches: &clap::ArgMatches) {
    let split_by_lines = *matches.get_one::<bool>("split-lines").unwrap();

    let file_path = matches
        .get_one::<String>("file")
        .expect("Should have been able to get the file path");

    let input_file_path = PathBuf::from(OsString::from(file_path));

    let file_content = std::fs::read(&input_file_path).expect("Failed to read file");


    if !split_by_lines {
        let spans_iterator = buffer_to_spans(file_content.as_slice());

        spans_iterator.into_iter().for_each(|_| {
            // Noop
        });
    } else {
        let lines_iterator = buffer_to_lines(file_content.as_slice());

        lines_iterator.into_iter().for_each(|_| {
            // Noop
        });
    }
}
