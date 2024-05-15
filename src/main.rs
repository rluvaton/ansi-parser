extern crate clap;

use std::env;

use ansi_parser;

use crate::cli::definition::get_cli;
use crate::cli::parse_command::run_parse_command;

mod parse_ansi_text;
mod mapping_file;
mod cli;

fn main() {
    let matches = get_cli().get_matches();
    
    // TODO - check which command currently running
    let command = matches.subcommand_name().expect("Should have been able to get the command");
    
    if command == "parse" {
        run_parse_command(matches.subcommand_matches("parse").expect("Should have been able to get the parse subcommand"));
    } else {
        panic!("Unknown command: {}", command);
    }
}


fn get_file_path_in_current_dir(file_name: &str) -> String {
    env::current_dir().unwrap().as_path().join(file_name).to_str().unwrap().to_string()
}
