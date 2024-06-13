// use peak_alloc::PeakAlloc;
//
// #[global_allocator]
// static PEAK_ALLOC: PeakAlloc = PeakAlloc;
extern crate clap;
extern crate core;

use nom::IResult;

mod cli;

use std::env;
use ansi_parser_extended::parse_ansi_text::raw_ansi_parse::{AnsiSequence, parse_escape_complete, parse_escape_only_text_and_graphics, parse_escape_only_text_and_graphics_manual_simd};
const file_path: &str = "/Users/rluvaton/dev/personal/ansi-viewer/examples/fixtures/huge.ans";


fn run_cli() {
    let args: Vec<String> = env::args().collect();
    let fn_type_string = args.get(1).expect("Should have been able to get the function type").clone();
    let fn_type = fn_type_string.as_str();

    let file_content = std::fs::read(file_path).expect("Failed to read file");
    let content = file_content.as_slice();

    match fn_type {
        "complete" => {
            run_parse_fn_res(content, parse_escape_complete);
        },
        "text_graphics" => {
            run_parse_fn_res(content, parse_escape_only_text_and_graphics);
        },
        "simd" => {
            run_parse_fn_option(content, parse_escape_only_text_and_graphics_manual_simd)
        },

        _ => panic!("Unknown function type: {}", fn_type)
    }


    //
    // let matches = get_cli().get_matches();
    //
    // let command = matches
    //     .subcommand_name()
    //     .expect("Should have been able to get the command");
    //
    // if command == "parse" {
    //     run_parse_command(
    //         matches
    //             .subcommand_matches("parse")
    //             .expect("Should have been able to get the parse subcommand"),
    //     );
    //     return;
    // }
    //
    // if command == "mapping" {
    //     let matches = matches
    //         .subcommand_matches("mapping")
    //         .expect("Should have been able to get the mapping subcommand");
    //
    //     let command = matches
    //         .subcommand_name()
    //         .expect("Should have been able to get the mapping subcommand");
    //
    //     if command == "create" {
    //         run_create_mapping_file_command(
    //             matches
    //                 .subcommand_matches("create")
    //                 .expect("Should have been able to get the create subcommand"),
    //         );
    //     } else {
    //         panic!("Unknown mapping subcommand: {}", command);
    //     }
    //
    //     return;
    // }
    //
    // panic!("Unknown command: {}", command);
}

fn run_simd(content: &[u8]) {
    let mut res = parse_escape_only_text_and_graphics_manual_simd(content, true);

    loop {
        if res.is_none() {
            break;
        }

        res = parse_escape_only_text_and_graphics_manual_simd(res.unwrap().0, true);
    }
}


fn run_parse_fn_res<F>(mut content: &[u8], parse_fn: F) where F: Fn(&[u8], bool) -> IResult<&[u8], AnsiSequence> {
    let mut res = parse_fn(content, true);

    loop {
        if res.is_err() {
            break;
        }

        content = res.unwrap().0;

        res = parse_fn(content, true);
    }
}
fn run_parse_fn_option<F>(mut content: &[u8], parse_fn: F) where F: Fn(&[u8], bool) -> Option<(&[u8], AnsiSequence)> {
    let mut res = parse_fn(content, true);

    loop {
        if res.is_none() {
            break;
        }

        content = res.unwrap().0;

        res = parse_fn(content, true);
    }
}


fn main() {
    run_cli();

    // https://crates.io/crates/peak_alloc
    // let peak_mem = PEAK_ALLOC.peak_usage_as_mb();
    // println!("The max amount that was used {}", peak_mem);
}
