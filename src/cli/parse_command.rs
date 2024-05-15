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

pub fn run_parse_command(matches: &clap::ArgMatches) {
    let split_by_lines = matches.get_one::<bool>("split-lines").unwrap();

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

    if !split_by_lines {
        print_json_without_split_by_lines(json_output_format, contents, parse_options);
    } else {
        if json_output_format {
            print_json_with_split_by_lines(contents, parse_options)
        } else if json_line_output_format {
            print_json_line_with_split_by_lines(contents, parse_options);
        } else if flat_json_line_output_format {
            print_flat_json_line(contents, parse_options);
        }
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
fn print_json_without_split_by_lines(
    json_output_format: bool,
    contents: String,
    parse_options: ParseOptions,
) {
    let spans_iter = contents.parse_ansi_as_spans(parse_options);

    // Start output as json
    if json_output_format {
        println!("[");
    }

    let mut print_first = true;

    for span in spans_iter {
        if json_output_format && !print_first {
            // Print from prev object
            print!(",")
        }

        print_first = false;
        let span_json = SpanJson::create_from_span(&span);
        let span_json_str = serde_json::to_string(&span_json).unwrap();

        println!("{}", span_json_str);
    }

    // Print ending array
    if json_output_format {
        println!("]");
    }
}

// TODO - change to iterator and consume it by either printing to stdout or file
fn print_only_start_of_line_styles(
    json_output_format: bool,
    contents: String,
    parse_options: ParseOptions,
) {
    let lines_iter = contents.parse_ansi_as_spans_by_lines(parse_options);

    // Start output as json
    if json_output_format {
        println!("[");
    }

    let mut print_first = true;

    for line in lines_iter {
        if json_output_format && !print_first {
            // Print from prev object
            print!(",")
        }

        let mut start_of_line_style_span: Span;

        if line.is_empty() {
            start_of_line_style_span = Span::empty()
        } else {
            start_of_line_style_span = line[0].clone().with_text("".to_string());
        }

        print_first = false;
        let span_json = SpanJson::create_from_span(&start_of_line_style_span);
        let span_json_str = serde_json::to_string(&span_json).unwrap();

        println!("{}", span_json_str);
    }

    // Print ending array
    if json_output_format {
        println!("]");
    }
}

// TODO - change to iterator and consume it by either printing to stdout or file
fn print_json_with_split_by_lines(contents: String, parse_options: ParseOptions) {
    let lines_iter = contents.parse_ansi_as_spans_by_lines(parse_options);

    // Start of all lines
    println!("[");

    let mut is_first = true;

    for line in lines_iter {
        if !is_first {
            // Print from prev object
            print!(",")
        }

        is_first = false;

        // Start of line
        println!("[");
        let mut is_first_in_line = true;

        for span in line {
            if !is_first_in_line {
                // Printing on the next line so we don't have to deal with knowing when the item is the last
                // Print from prev object
                print!(",")
            }
            is_first_in_line = false;
            let json_span = SpanJson::create_from_span(&span);

            println!("{}", serde_json::to_string(&json_span).unwrap());
        }

        // End of line
        println!("]");
    }

    // End of all lines
    println!("]");
}

// TODO - change to iterator and consume it by either printing to stdout or file
fn print_json_line_with_split_by_lines(contents: String, parse_options: ParseOptions) {
    let lines_iter = contents.parse_ansi_as_spans_by_lines(parse_options);
    let mut is_first = true;

    for line in lines_iter {
        if !is_first {
            // If not first line, should go to next line before printing current line
            print!("\n")
        }

        is_first = false;

        // Start of line, need to print here as each line must be valid JSON
        print!("[");
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
