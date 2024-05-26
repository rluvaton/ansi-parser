use std::ffi::OsString;
use std::path::PathBuf;

use crate::files::file_reader::FileReaderOptions;
use crate::format::spans_formatters::spans_formats::{
    format_spans, get_span_format_from_string, SpanFormat,
};
use crate::iterators::compose::ComposeByIterator;
use crate::output::{
    get_output_destination, get_output_destination_from_string, OutputDestination,
};
use crate::parse_ansi_text::ansi::types::Span;
use crate::parse_ansi_text::parse_options::ParseOptions;
use crate::parse_file::file_to_spans::{read_ansi_file_to_spans};
use crate::parse_file::from_middle_of_file::get_from_middle_of_the_file_info;
use crate::parse_file::types::ReadAnsiFileOptions;

// TODO - in order to save memory and not read the entire file to memory
//        we should have a way to have an iterator over the file that yield the spans
//        currently, the parse_ansi lib is not designed to work with iterators
//        so we need to yield the current span and the next span

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

    let middle_of_file_info = get_from_middle_of_the_file_info(input_file_path, from_line, to_line, mapping_file);

    let file_reader_options = FileReaderOptions {
        file_path: file_path.clone(),
        chunk_size_in_bytes: Some(1024 * 1024 * 10), // 10MB
        from_bytes: middle_of_file_info.from_bytes,
        to_bytes: middle_of_file_info.to_bytes,
    };
    let parse_options = ParseOptions::default()
        .with_initial_span(middle_of_file_info.initial_span.unwrap_or(Span::empty()));

    if !split_by_lines {
        run_for_spans(
            file_reader_options,
            parse_options,
            get_span_format_from_string(format).expect("Invalid format"),
            get_output_destination_from_string(output).expect("Invalid format"),
        );
    } else {
        todo!("Implemented split by lines");
    }
}

fn run_for_spans(
    file_options: FileReaderOptions,
    parse_options: ParseOptions,
    span_format: SpanFormat,
    output_dest: OutputDestination,
) {
    let options = ReadAnsiFileOptions {
        file_options,
        parse_options,
    };

    read_ansi_file_to_spans(options)
        .compose(|iter| format_spans(Box::new(iter), span_format))
        .for_each(get_output_destination(output_dest));
}

