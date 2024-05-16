use std::fs::File;
use std::path::PathBuf;
use get_chunk::iterator::FileIter;

pub fn create_file_iterator(input_file_path: PathBuf) -> Box<dyn Iterator<Item=String>> {
    let input_file = File::open(input_file_path).expect("opening input file path failed");

    let file_iter = FileIter::try_from(input_file).expect("create input file iterator failed");
    let file_string_iterator = file_iter.into_iter().map(|item| String::from_utf8(item.expect("Failed to get file chunk")).expect("Converting file chunk to UTF-8 string failed"));

    return Box::new(file_string_iterator);
}