use crate::mapping_file::read::{
    get_initial_style_for_line_from_file, get_mapping_file_ready_to_read,
};
use crate::parse_ansi_text::parse_ansi_as_spans_iterator::*;
use crate::parse_ansi_text::parse_options::ParseOptions;
use crate::parse_ansi_text::types::{Span, SpanJson};
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use crate::cli::format::json_single_span::*;
use crate::cli::format::json_span_lines::*;
use crate::cli::format::json_line_single_span::*;
use crate::cli::format::json_line_span_lines::*;
use crate::cli::format::flat_json_line_span_lines::*;
use crate::parse_ansi_text::custom_ansi_parse_iterator::AnsiParseIterator;
use crate::parse_ansi_text::parse_ansi_split_by_lines_as_spans_iterator::ParseAnsiAsSpansByLinesIterator;
use crate::parse_ansi_text::{parse_ansi_text_split_by_lines, parse_ansi_text_with_options};

// TODO - in order to save memory and not read the entire file to memory
//        we should have a way to have an iterator over the file that yield the spans
//        currently, the parse_ansi lib is not designed to work with iterators
//        so we need to yield the current span and the next span

pub fn run_parse_command(matches: &clap::ArgMatches) {
    let split_by_lines = *matches.get_one::<bool>("split-lines").unwrap();

    let from_line = matches.get_one::<u16>("from-line");
    let to_line = matches.get_one::<u16>("to-line");
    let mapping_file = matches.get_one::<String>("mapping-file");

    let file_path = matches
        .get_one::<String>("file")
        .expect("Should have been able to get the file path");

    // get_lines_in_range_if_needed_from_file_path(&file_path, mapping_file, from_line, to_line);
    
    // TODO - don't load entire file to memory and instead iterate on it
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    if mapping_file.is_some() {
        panic!("Mapping file is not supported yet");
    }

    let format = matches
        .get_one::<String>("format")
        .expect("Should have been able to get the format");
    let flat_json_line_output_format = format == "flat-json-line";
    let json_output_format = format == "json";
    let json_line_output_format = format == "json-line";

    if !split_by_lines && flat_json_line_output_format {
        panic!("'flat-json-line' option is only available when 'split-lines' is enabled");
    }

    // println!("The file passed is: {}", file_path);

    // println!("With text:\n{contents}");

    let parse_options = ParseOptions::default();

    // TODO - change this to iterator instead of Vec for better performance
    // TODO - make ansi parse iterator split by line if needed for better performance instead of keeping all for 1 huge line
    let parse_ansi_from_text_iterator = AnsiParseIterator::create_from_str(contents);

    // return output;

    // TODO - find a more generic way to have where to output and and the format of the output instead of having multiple if statements with the same code
    //        Ideally should be like this:
    //        content
    //            .compose(parse_ansi_as_spans(parse_options))
    //            .compose(format_output(format))
    if json_output_format {
        if !split_by_lines {
            let parse_ansi_as_spans_iterator = ParseAnsiAsSpansIterator {
                iter: parse_ansi_from_text_iterator,
                current_span: Span::empty(),
            };
            
            print_strings_to_stdout(parse_ansi_as_spans_iterator.to_span_json());
        } else {
            print_strings_to_stdout(ParseAnsiAsSpansByLinesIterator::create(parse_ansi_from_text_iterator, parse_options)
                .to_json_string_in_span_lines());
        }

        return;
    }

    if json_line_output_format {
        if !split_by_lines {
            let parse_ansi_as_spans_iterator = ParseAnsiAsSpansIterator {
                iter: parse_ansi_from_text_iterator,
                current_span: Span::empty(),
            };

            print_strings_to_stdout(parse_ansi_as_spans_iterator.to_span_json_line());
        } else {
            print_strings_to_stdout(ParseAnsiAsSpansByLinesIterator::create(parse_ansi_from_text_iterator, parse_options).to_json_line_string_in_span_lines());
        }

        return;
    }

    if flat_json_line_output_format {
        print_strings_to_stdout(ParseAnsiAsSpansByLinesIterator::create(parse_ansi_from_text_iterator, parse_options).to_flat_json_line_string_in_span_lines());
    }
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
fn get_spans_in_range_if_needed_from_file_path(
    file_path: &str,
    mapping_file_path: Option<&String>,
    from_line: Option<&u16>,
    to_line: Option<&u16>,
) -> Vec<Vec<Span>> {
    if from_line.is_none() && to_line.is_none() {
        let file_content = fs::read_to_string(file_path).unwrap();

        return parse_ansi_text_split_by_lines(file_content.as_str(), ParseOptions::default())
    }

    let mut reader =
        my_reader::BufReader::open(file_path).expect("Should have been able to open the file");
    let mut buffer = String::new();
    let mut line_number: usize = 1;
    let mut line_number_u16: u16 = 1;

    let from_line_value = *from_line.unwrap_or(&0);

    // TODO - if have mapping file can just read line by line and have a proper iterator

    // TODO - parse line by line and get the trailing new line from mapping file

    // TODO - require having mapping file
    mapping_file_path.expect("Mapping file is required when using from-line or to-line");

    let ready_data_for_reading_file = get_mapping_file_ready_to_read(PathBuf::from(
        OsString::from(mapping_file_path.unwrap().clone()),
    ));

    if ready_data_for_reading_file.is_none() {
        panic!("Could not get ready mapping data for reading file");
    }

    let (mut file, content_start_offset, line_length) = ready_data_for_reading_file.unwrap();

    let mut all_lines: Vec<Vec<Span>> = vec![];

    while let Some(line) = reader.read_line(&mut buffer) {
        if line_number_u16 < from_line_value {
            continue;
        }

        if to_line.is_some() && line_number_u16 > *to_line.unwrap() {
            break;
        }

        let initial_span = get_initial_style_for_line_from_file(
            &mut file,
            line_length,
            content_start_offset,
            line_number,
        )
        .unwrap();

        // TODO - change this to use the file iterator directly
        let line_spans = parse_ansi_text_with_options(line.unwrap().clone().as_str(), ParseOptions::default().with_initial_span(initial_span));
        
        all_lines.push(line_spans);

        line_number += 1;
        line_number_u16 += 1;
    }

    return all_lines;
}

// From: https://stackoverflow.com/a/45882510/5923666
mod my_reader {
    use std::{
        fs::File,
        io::{self, prelude::*},
    };

    pub struct BufReader {
        reader: io::BufReader<File>,
    }

    impl BufReader {
        pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
            let file = File::open(path)?;
            let reader = io::BufReader::new(file);

            Ok(Self { reader })
        }

        pub fn read_line<'buf>(
            &mut self,
            buffer: &'buf mut String,
        ) -> Option<io::Result<&'buf mut String>> {
            buffer.clear();

            self.reader
                .read_line(buffer)
                .map(|u| if u == 0 { None } else { Some(buffer) })
                .transpose()
        }
    }
}

