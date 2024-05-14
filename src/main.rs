use std::env;
use std::fs;
use ansi_parser;

extern crate clap;
use clap::{App, Arg};
use crate::parse_ansi_text::parse_ansi_as_spans_iterator::ParseAnsiAsSpans;
use crate::parse_ansi_text::parse_ansi_split_by_lines_as_spans_iterator::ParseAnsiAsSpansByLines;
use crate::parse_ansi_text::parse_options::ParseOptions;
use crate::parse_ansi_text::types::{Span, SpanJson};

mod parse_ansi_text;

fn main() {
    let matches = App::new("My Test Program")
        .version("1.0.0")
        .author("Raz Luvaton")
        .about("Parse ANSI text")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .required(true)
            .takes_value(true)
            .help("file to read")
            // TODO - remove this default value
            .default_value("../examples/2-lines.ans"))
        .arg(Arg::with_name("split-lines")
            .short("s")
            .long("split-lines")
            .required(false)
            .takes_value(false)
            .help("Whether should return array of lines where each line contains spans in that line"))
        
        // TODO - add support
        // .arg(Arg::with_name("output")
        //     .short("o")
        //     .long("output")
        //     .required(false)
        //     .takes_value(true)
        //     .help("Where to output")
        //     .possible_values(&["stdout", "file"])
        //     .default_value("stdout"))

        // .arg(Arg::with_name("output-path")
        //     .long("output-path")
        //     .required_if("output", "file")
        //     .takes_value(true)
        //     .help("Output JSON file (when output option is file")
        // )

        .arg(Arg::with_name("json-output-format")
            .long("json")
            .takes_value(false)
            .conflicts_with_all(&["json-line-output-format", "flat-json-line-output-format"])
            .help("output all the span in a valid JSON format this is the default format"))

        .arg(Arg::with_name("json-line-output-format")
            .long("json-line")
            .takes_value(false)

            .help("Each line of output is a valid JSON, there are no comma between lines and the output is not wrapped with [ and ].\n\nWhen split-lines is true, each line of output will match line in the input, all spans for the same input line will be at the same line in the output"))

        .arg(Arg::with_name("flat-json-line-output-format")
            .long("flat-json-line")
            .takes_value(false)

            .help("Each line of output is a valid JSON, there are no comma between lines and the output is not wrapped with [ and ].\n\nobject with property type: 'new line' will be printed between lines to mark new line"))

        .arg(Arg::with_name("only-style-for-start-of-line")
            .long("only-style-for-start-of-line")
            .takes_value(false)

            .help("Only output style for the start of each line without text, this helps reading files and to know which style to apply at the beginning"))
        
        // TODO - add initial span to parse with and line/index ranges for reading the file
        .get_matches();

    let mut split_by_lines = matches.is_present("split-lines");
    let flat_json_line_output_format = matches.is_present("flat-json-line-output-format");
    let mut json_output_format = matches.is_present("json-output-format");
    let json_line_output_format = matches.is_present("json-line-output-format");
    let only_style_for_start_of_line = matches.is_present("only-style-for-start-of-line");
    
    if !split_by_lines && flat_json_line_output_format {
        panic!("'flat-json-line' option is only available when 'split-lines' is enabled");
    }
    
    if only_style_for_start_of_line {
        split_by_lines = true;
    }
    
    if !flat_json_line_output_format && !json_line_output_format && !json_output_format {
        // JSON output is the default format
        json_output_format = true
    }

    let file_path = matches.value_of("file").unwrap();
    // println!("The file passed is: {}", file_path);

    // TODO - don't load entire file to memory and instead iterate on it
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // println!("With text:\n{contents}");
    
    let parse_options = ParseOptions::default();
    
    if only_style_for_start_of_line {
        print_only_start_of_line_styles(json_output_format, contents, parse_options);
    } else if !split_by_lines {
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

fn get_file_path_in_current_dir(file_name: &str) -> String {
    env::current_dir().unwrap().as_path().join(file_name).to_str().unwrap().to_string()
}
