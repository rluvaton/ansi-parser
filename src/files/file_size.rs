use std::fs;
use std::path::PathBuf;

pub fn get_file_size(path: PathBuf) -> usize {
    let metadata = fs::metadata(path).expect("Failed to get metadata");
    metadata.len() as usize
}
