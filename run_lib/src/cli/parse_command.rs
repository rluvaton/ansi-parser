use std::ffi::OsString;
use std::path::PathBuf;
use ansi_parser_extended::parse_file::types::ReadAnsiFileOptions;
use genawaiter::stack::let_gen_using;
use ansi_parser_extended::files::file_reader::FileReaderOptions;
use ansi_parser_extended::parse_ansi_text::ansi::types::Span;
use ansi_parser_extended::parse_ansi_text::parse_options::ParseOptions;
use ansi_parser_extended::parse_file::file_to_lines_of_spans::read_ansi_file_to_lines_producer;
use ansi_parser_extended::parse_file::file_to_spans::read_ansi_file_to_spans_producer;
use ansi_parser_extended::parse_file::from_middle_of_file::get_from_middle_of_the_file_info;


pub fn run_parse_command(matches: &clap::ArgMatches) {
    let split_by_lines = *matches.get_one::<bool>("split-lines").unwrap();

    let from_line = matches.get_one::<usize>("from-line");
    let to_line = matches.get_one::<usize>("to-line");
    let mapping_file = matches.get_one::<String>("mapping-file");

    let file_path = matches
        .get_one::<String>("file")
        .expect("Should have been able to get the file path");

    let output = matches
        .get_one::<String>("output")
        .expect("Should have been able to get the output destination");

    let input_file_path = PathBuf::from(OsString::from(file_path));

    let format = matches
        .get_one::<String>("format")
        .expect("Should have been able to get the format");

    let flat_json_line_output_format = format == "flat-json-line";

    if !split_by_lines && flat_json_line_output_format {
        panic!("'flat-json-line' option is only available when 'split-lines' is enabled");
    }

    let middle_of_file_info =
        get_from_middle_of_the_file_info(input_file_path, from_line, to_line, mapping_file);

    let file_reader_options = FileReaderOptions {
        file_path: file_path.clone(),
        chunk_size_in_bytes: Some(1024 * 1024 * 10), // 10MB
        from_bytes: middle_of_file_info.from_bytes,
        to_bytes: middle_of_file_info.to_bytes,
    };
    let parse_options = ParseOptions::default()
        .with_initial_span(middle_of_file_info.initial_span.unwrap_or(Span::empty()));

    if !split_by_lines {
        let options = ReadAnsiFileOptions {
            file_options: file_reader_options,
            parse_options,
        };

        let_gen_using!(spans_iterator, |co| read_ansi_file_to_spans_producer(options, co));

        spans_iterator.into_iter().for_each(|_| {
            // Noop
        });
    } else {
        let options = ReadAnsiFileOptions {
            file_options: file_reader_options,
            parse_options,
        };

        let_gen_using!(lines_iterator, |co| read_ansi_file_to_lines_producer(options, co));

        lines_iterator.into_iter().for_each(|_| {
            // Noop
        });
    }
}
