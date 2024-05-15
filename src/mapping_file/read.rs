use std::str;

use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use crate::mapping_file::constants::*;
use crate::parse_ansi_text::parse_text_matching_single_span::parse_text_matching_single_span;
use crate::parse_ansi_text::types::Span;

pub fn get_initial_style_for_line(mapping_text: String, line_number: usize) -> Span {
    if(line_number < 1) {
        panic!("Line number must be at least 1");
    }

    // TODO - can avoid cloning?
    let get_mapping_file_metadata_result = get_mapping_metadata(mapping_text.clone());

    if get_mapping_file_metadata_result.is_none() {
        panic!("Invalid mapping file");
    }

    let (content_start_offset, line_length) = get_mapping_file_metadata_result.unwrap();

    let offset_in_text = content_start_offset + ((line_number - 1) * line_length);

    if(offset_in_text >= mapping_text.len()) {
        panic!("Invalid mapping, line number is missing");
    }

    if(offset_in_text + line_length > mapping_text.len()) {
        panic!("Invalid mapping, each line is not the same length");
    }

    // TODO - make sure not to include \0
    let line_style = mapping_text[offset_in_text..offset_in_text + line_length].to_string();

    // To make sure there is no empty span
    let spans = parse_text_matching_single_span(&line_style);

    return spans.with_text("".to_string());
    // TODO - throw if offset does not exists in file
}


pub fn get_initial_style_for_line_from_file(file_path: String, line_number: usize) -> Span {
    let mut file = File::open(file_path).expect("open mapping file failed");

    let line_length = get_mapping_file_metadata(&mut file);

    if line_length.is_none() {
        panic!("Invalid mapping file, should have at least one line");
    }

    let (content_start_offset, line_length) = line_length.unwrap();
    

    let mut requested_line_initial_style = vec![0u8; line_length];

    // TODO - should differentiate between seek problem or index out of bounds
    file.seek(SeekFrom::Start(content_start_offset as u64)).expect("Go to matching line failed");
    
    file.read_exact(&mut requested_line_initial_style).expect("Read matching line failed");
    
    let line_style = String::from_utf8(requested_line_initial_style).expect("Converting requested line to UTF-8 string failed");
    
    return parse_text_matching_single_span(&line_style).with_text("".to_string());
}

// First item in returned tuple is the content_start_offset and the second is the line_length
fn get_mapping_file_metadata(f: &mut File) -> Option<(usize, usize)> {
    let mut buf = vec![0u8; 1000];

    f.read_exact(&mut buf).expect("Try read mapping header failed");

    let s = match str::from_utf8(&buf) {
        Ok(v) => v,
        Err(e) => {
            println!("Invalid UTF-8 sequence: {}", e);
            return None;
        }
    };

    return get_mapping_metadata(s.to_string());
}

// First item in returned tuple is the content_start_offset and the second is the line_length
fn get_mapping_metadata(header_and_more: String) -> Option<(usize, usize)> {
    let header_end_index = header_and_more.find(DELIMITER);

    if header_end_index.is_none() {
        println!("Invalid mapping file, should have at least one line");
        return None;
    }

    let header_end_index = header_end_index.unwrap();

    let content_start_offset = header_end_index + DELIMITER.len();

    let line_length_str = &header_and_more[0..header_end_index];
    let line_length_result = line_length_str.parse::<usize>();

    if line_length_result.is_err() {
        panic!("Invalid mapping file, first line should be a number");
    }

    let line_length = line_length_result.unwrap();

    return Some((content_start_offset, line_length));
}

// TODO - add function that given file path get initial style for line
