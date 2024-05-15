use std::fs;
use crate::parse_ansi_text::parse_ansi_as_spans_iterator::ParseAnsiAsSpans;
use crate::parse_ansi_text::parse_ansi_split_by_lines_as_spans_iterator::ParseAnsiAsSpansByLines;
use crate::parse_ansi_text::parse_options::ParseOptions;
use crate::parse_ansi_text::types::{Span, SpanJson};

pub fn run_parse_command(matches: &clap::ArgMatches) {
    let split_by_lines = matches.get_one::<bool>("split-lines").unwrap();

    let format = matches.get_one::<String>("format").expect("Should have been able to get the format");
    let flat_json_line_output_format = format == "flat-json-line";
    let json_output_format = format == "json";
    let json_line_output_format = format == "json-line";

    if !split_by_lines && flat_json_line_output_format {
        panic!("'flat-json-line' option is only available when 'split-lines' is enabled");
    }

    let file_path = matches.get_one::<String>("file").expect("Should have been able to get the file path");
    // println!("The file passed is: {}", file_path);

    // TODO - don't load entire file to memory and instead iterate on it
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

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


// TODO - change to iterator and consume it by either printing to stdout or file
fn print_json_without_split_by_lines(json_output_format: bool, contents: String, parse_options: ParseOptions) {
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
fn print_only_start_of_line_styles(json_output_format: bool, contents: String, parse_options: ParseOptions) {
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
