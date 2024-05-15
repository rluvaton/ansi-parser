use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;

use crate::mapping_file::create::create_mapping_file;

pub fn run_create_mapping_file_command(matches: &clap::ArgMatches) {
    let input_path = matches.get_one::<String>("input").expect("Should have been able to get the input file path");
    let output_path = matches.get_one::<String>("output").expect("Should have been able to get the output file path");

    // TODO - don't load entire file to memory and instead iterate on it
    let contents = fs::read_to_string(input_path)
        .expect("Should have been able to read the input file");
    
    create_mapping_file(PathBuf::from(OsString::from(output_path)), contents);
   
    println!("Done");
}
