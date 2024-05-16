use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use get_chunk::iterator::FileIter;

pub fn create_file_iterator(input_file_path: PathBuf) -> Box<dyn Iterator<Item=String>> {
    let input_file = File::open(input_file_path).expect("opening input file path failed");

    let file_iter = FileIter::try_from(input_file).expect("create input file iterator failed");
    let file_string_iterator = file_iter.into_iter().map(|item| String::from_utf8(item.expect("Failed to get file chunk")).expect("Converting file chunk to UTF-8 string failed"));

    return Box::new(file_string_iterator);
}


// TODO - make from and to line optional and use them only if they are not None
pub fn create_file_iterator_in_range(input_file_path: PathBuf, from_line: Option<&u16>, to_line: Option<&u16>) -> Box<dyn Iterator<Item=String>> {
    if from_line.is_none() && to_line.is_none() {
        return create_file_iterator(input_file_path);
    }

    let input_file = File::open(input_file_path).expect("opening input file path failed");

    let reader = BufReader::new(input_file);

    let mut lines_iter = reader.lines();

    if from_line.is_some() && to_line.is_some() {
        return Box::new(lines_iter.skip(*from_line.unwrap() as usize - 1).take(*to_line.unwrap() as usize - *from_line.unwrap() as usize).map(|line| line.expect("Failed to get line")));
    }

    if from_line.is_some() {
        return Box::new(lines_iter.skip(*from_line.unwrap() as usize - 1).map(|line| line.expect("Failed to get line")));
    }

    if to_line.is_some() {
        return Box::new(lines_iter.take(*to_line.unwrap() as usize).map(|line| line.expect("Failed to get line")));
    }

    panic!("Should not reach here");
}
