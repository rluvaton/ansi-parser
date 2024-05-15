use crate::mapping_file::read::{
    get_initial_style_for_line_from_file, get_mapping_file_ready_to_read,
};
use crate::parse_ansi_text::parse_ansi_as_spans_iterator::ParseAnsiAsSpans;
use crate::parse_ansi_text::parse_ansi_split_by_lines_as_spans_iterator::ParseAnsiAsSpansByLines;
use crate::parse_ansi_text::parse_options::ParseOptions;
use crate::parse_ansi_text::types::{Span, SpanJson};
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use crate::cli::format::json_line_span_lines::SpansLineJsonLineDisplayByIterator;
use crate::cli::format::json_single_span::SpansJsonDisplayByIterator;
use crate::cli::format::json_span_lines::SpansLineJsonDisplayByIterator;

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


    if json_output_format {
        if !split_by_lines {
            print_strings_to_stdout(contents.parse_ansi_as_spans(parse_options).to_span_json());
        } else {
            print_strings_to_stdout(contents.parse_ansi_as_spans_by_lines(parse_options).to_json_string_in_span_lines());
        }

        return;
    }
    
    if json_line_output_format {
        if !split_by_lines {
            print_strings_to_stdout(contents.parse_ansi_as_spans(parse_options).to_span_json_line());
        } else {
            print_strings_to_stdout(contents.parse_ansi_as_spans_by_lines(parse_options).to_json_line_string_in_span_lines());
        }

        return;
    }
    
    if flat_json_line_output_format {
        print_strings_to_stdout(contents.parse_ansi_as_spans_by_lines(parse_options).to_flat_json_line_string_in_span_lines());
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

        return file_content
            .parse_ansi_as_spans_by_lines(ParseOptions::default())
            .collect::<Vec<Vec<Span>>>();
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

        let line_spans = line
            .unwrap()
            .clone()
            .parse_ansi_as_spans(ParseOptions::default().with_initial_span(initial_span))
            .collect::<Vec<Span>>();
        
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


// TODO - change to iterator and consume it by either printing to stdout or file
fn print_flat_json_line(contents: String, parse_options: ParseOptions) {
    let lines_iter = contents.parse_ansi_as_spans_by_lines(parse_options);
    let mut is_first = true;

    for line in lines_iter {
        if !is_first {
            // If not first line, should go to next line before printing current line

            // If not first line than print that starting a new line
            // { "type": "new line" }
            print!("\n{{ \"type\": \"new line\" }}\n");
        }

        is_first = false;

        let mut is_first_in_line = true;

        for span in line {
            if !is_first_in_line {
                // Printing on the next line so we don't have to deal with knowing when the item is the last
                // Print from prev object
                print!(",")
            }
            is_first_in_line = false;
            let json_span = SpanJson::create_from_span(&span);

            print!("{}", serde_json::to_string(&json_span).unwrap());
        }

        // End of line
        print!("]");
    }
}
