use std::ffi::OsString;

use std::path::PathBuf;

use crate::mapping_file::create::create_mapping_file_from_input_path;

pub fn run_create_mapping_file_command(matches: &clap::ArgMatches) {
    let input_path = matches
        .get_one::<String>("input")
        .expect("Should have been able to get the input file path");
    let output_path = matches
        .get_one::<String>("output")
        .expect("Should have been able to get the output file path");

    create_mapping_file_from_input_path(
        PathBuf::from(OsString::from(output_path)),
        PathBuf::from(OsString::from(input_path)),
    );

    println!("Done");
}
