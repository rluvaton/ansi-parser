use std::ffi::OsString;
use std::path::PathBuf;
use ansi_parser_extended::parse_file::types::ReadAnsiFileOptions;
use ansi_parser_extended::files::file_reader::FileReaderOptions;
use ansi_parser_extended::parse_ansi_text::ansi::types::Span;
use ansi_parser_extended::parse_ansi_text::parse_options::ParseOptions;
use ansi_parser_extended::parse_file::file_to_lines_of_spans::read_ansi_file_to_lines;
use ansi_parser_extended::parse_file::file_to_spans::read_ansi_file_to_spans;
use ansi_parser_extended::parse_file::from_middle_of_file::get_from_middle_of_the_file_info;


pub fn run_parse_command(matches: &clap::ArgMatches) {
    let split_by_lines = *matches.get_one::<bool>("split-lines").unwrap();

    let from_line = matches.get_one::<usize>("from-line");
    let to_line = matches.get_one::<usize>("to-line");
    let mapping_file = matches.get_one::<String>("mapping-file");

    let file_path = matches
        .get_one::<String>("file")
        .expect("Should have been able to get the file path");

    let input_file_path = PathBuf::from(OsString::from(file_path));

    let middle_of_file_info =
        get_from_middle_of_the_file_info(input_file_path, from_line.copied(), to_line.copied(), mapping_file.cloned());

    let file_reader_options = FileReaderOptions {
        file_path: file_path.clone(),
        chunk_size_in_bytes: Some(1024 * 1024 * 10), // 10MB
        from_bytes: middle_of_file_info.from_bytes,
        to_bytes: middle_of_file_info.to_bytes,
    };
    let parse_options = ParseOptions::default()
        .with_initial_span(middle_of_file_info.initial_span.unwrap_or(Span::empty()));

    let options = ReadAnsiFileOptions {
        file_options: file_reader_options,
        parse_options,
    };

    if !split_by_lines {
        let spans_iterator = read_ansi_file_to_spans(options);

        spans_iterator.into_iter().for_each(|_| {
            // Noop
        });
    } else {
        let lines_iterator = read_ansi_file_to_lines(options);

        lines_iterator.into_iter().for_each(|_| {
            // Noop
        });
    }
}
