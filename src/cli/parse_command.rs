use std::cmp::max;
use std::ffi::OsString;
use std::path::PathBuf;

use crate::cli::format::flat_json_line_span_lines::*;
use crate::cli::format::json_line_single_span::*;
use crate::cli::format::json_line_span_lines::*;
use crate::cli::format::json_single_span::*;
use crate::cli::format::json_span_lines::*;
use crate::iterators::file_iterator_helpers::create_file_iterator_in_range;
use crate::mapping_file::read::get_initial_style_for_line_from_file_path;
use crate::parse_ansi_text::ansi::types::Span;
use crate::parse_ansi_text::iterators::parse_ansi_as_spans_iterator::*;
use crate::parse_ansi_text::iterators::parse_ansi_split_by_lines_as_spans_iterator::ParseAnsiAsSpansByLinesIterator;
use crate::parse_ansi_text::parse_options::ParseOptions;

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

    let buf_file_path = PathBuf::from(OsString::from(file_path));

    let format = matches
        .get_one::<String>("format")
        .expect("Should have been able to get the format");
    let flat_json_line_output_format = format == "flat-json-line";
    let json_output_format = format == "json";
    let json_line_output_format = format == "json-line";

    if !split_by_lines && flat_json_line_output_format {
        panic!("'flat-json-line' option is only available when 'split-lines' is enabled");
    }

    let mut output_iterator: Box<dyn Iterator<Item=String>>;

    if !split_by_lines {
        let parse_ansi_as_spans_iterator = ParseAnsiAsSpansIterator::create_from_file_path(
            buf_file_path,
            ParseOptions::default(),
        );
        
        if json_output_format {
            output_iterator = Box::new(parse_ansi_as_spans_iterator.to_span_json());
        } else if json_line_output_format {
            output_iterator = Box::new(parse_ansi_as_spans_iterator.to_span_json_line());
        } else {
            panic!("Invalid format")
        }
    } else {
        let mut parse_ansi_as_spans_iterator = get_spans_in_range_if_needed_from_file_path(
            buf_file_path.clone(),
            mapping_file,
            from_line,
            to_line,
        );
        
        if json_output_format {
            output_iterator = Box::new(parse_ansi_as_spans_iterator.to_json_string_in_span_lines());
        } else if json_line_output_format {
            output_iterator = Box::new(parse_ansi_as_spans_iterator.to_json_line_string_in_span_lines());
        } else if flat_json_line_output_format {
            output_iterator = Box::new(parse_ansi_as_spans_iterator.to_flat_json_line_string_in_span_lines());
        } else {
            panic!("Invalid format")
        }
    }
    
    print_strings_to_stdout(output_iterator);
}

fn print_strings_to_stdout<I>(strings: I)
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    for s in strings {
        println!("{}", s.as_ref());
    }
}

// TODO - return iterator instead of Vec for better performance to not wait for the entire file to be read or load it to memory
fn get_spans_in_range_if_needed_from_file_path<'a>(
    file_path: PathBuf,
    mapping_file_path: Option<&String>,
    from_line: Option<&usize>,
    to_line: Option<&usize>,
) -> Box<dyn Iterator<Item = Vec<Span>>> {
    if from_line.is_none() && to_line.is_none() {
        return Box::new(ParseAnsiAsSpansByLinesIterator::create_from_file_path(
            file_path,
            ParseOptions::default(),
        ));
    }

    if mapping_file_path.is_none() {
        // Using slow path since we calculate everything
        return get_spans_in_range_without_mapping_file(file_path, from_line, to_line);
    }

    let from_line_value = *from_line.unwrap_or(&0);

    mapping_file_path.expect("Mapping file is required when using from-line or to-line");

    let initial_style = get_initial_style_for_line_from_file_path(
        PathBuf::from(OsString::from(mapping_file_path.unwrap().clone())),
        from_line_value,
    );

    if initial_style.is_none() {
        // TODO - avoid panicking and instead return error or empty
        panic!("Could not get ready mapping data for reading file");
    }

    let file_iterator_in_range = create_file_iterator_in_range(
        PathBuf::from(OsString::from(file_path)),
        from_line,
        to_line,
    );

    return Box::new(
        ParseAnsiAsSpansByLinesIterator::create_from_string_iterator(
            file_iterator_in_range,
            ParseOptions::default().with_initial_span(initial_style.unwrap()),
        ),
    );
}

// TODO - return iterator instead of Vec for better performance to not wait for the entire file to be read or load it to memory
fn get_spans_in_range_without_mapping_file<'a>(
    file_path: PathBuf,
    from_line: Option<&usize>,
    to_line: Option<&usize>,
) -> Box<dyn Iterator<Item = Vec<Span>>> {
    let iterator =
        ParseAnsiAsSpansByLinesIterator::create_from_file_path(file_path, ParseOptions::default());

    if from_line.is_some() && to_line.is_some() {
        return Box::new(
            iterator
                .skip(max(*from_line.unwrap(), 1) - 1)
                .take(*to_line.unwrap() - *from_line.unwrap() as usize),
        );
    }

    if from_line.is_some() {
        return Box::new(iterator.skip(*from_line.unwrap() as usize - 1));
    }

    if to_line.is_some() {
        return Box::new(iterator.take(*to_line.unwrap() as usize));
    }

    return Box::new(iterator);
}
