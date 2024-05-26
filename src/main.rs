extern crate clap;

use std::env;



use crate::cli::definition::get_cli;
use crate::cli::mapping_file_command::run_create_mapping_file_command;
use crate::cli::parse_command::run_parse_command;

mod parse_ansi_text;
mod mapping_file;
mod cli;
mod iterators;
mod streams_helpers;
mod files;
mod test_utils;
mod traits;
mod parse_file;
pub mod format;
mod output;


#[tokio::main]
async fn main() {
    let matches = get_cli().get_matches();
    
    let command = matches.subcommand_name().expect("Should have been able to get the command");
    
    if command == "parse" {
        run_parse_command(matches.subcommand_matches("parse").expect("Should have been able to get the parse subcommand")).await;
    } else if command == "mapping" {
        let matches = matches.subcommand_matches("mapping").expect("Should have been able to get the mapping subcommand");
        
        let command = matches.subcommand_name().expect("Should have been able to get the mapping subcommand");
        
        if command == "create" {
            run_create_mapping_file_command(matches.subcommand_matches("create").expect("Should have been able to get the create subcommand")).await;
        } else {
            panic!("Unknown mapping subcommand: {}", command);
        }
    } else {
        panic!("Unknown command: {}", command);
    }
}


fn get_file_path_in_current_dir(file_name: &str) -> String {
    env::current_dir().unwrap().as_path().join(file_name).to_str().unwrap().to_string()
}
