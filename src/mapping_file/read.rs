use std::str;

use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::PathBuf;
use crate::mapping_file::constants::*;
use crate::parse_ansi_text::parse_text_matching_single_span::parse_text_matching_single_span;
use crate::parse_ansi_text::types::Span;

pub fn get_initial_style_for_line(mapping_text: String, line_number: usize) -> Option<Span> {
    if(line_number < 1) {
        panic!("Line number must be at least 1");
    }

    // TODO - can avoid cloning?
    let get_mapping_file_metadata_result = get_mapping_metadata(mapping_text.clone());

    if get_mapping_file_metadata_result.is_none() {
        println!("Invalid mapping file");

        // TODO - throw instead of returning None
        return None
    }

    let (content_start_offset, line_length) = get_mapping_file_metadata_result.unwrap();

    let offset_in_text = content_start_offset + ((line_number - 1) * line_length);

    if(offset_in_text >= mapping_text.len()) {
        println!("Invalid mapping, line number is missing");

        // TODO - throw instead of returning None
        return None;
    }

    if(offset_in_text + line_length > mapping_text.len()) {
        println!("Invalid mapping, each line is not the same length");
        
        // TODO - throw instead of returning None
        return None;
    }

    let line_style = mapping_text[offset_in_text..offset_in_text + line_length].to_string();

    // To make sure there is no empty span
    let span = parse_text_matching_single_span(&line_style);

    return Some(span.with_text("".to_string()));
}


pub fn get_initial_style_for_line_from_file_path(file_path: PathBuf, line_number: usize) -> Option<Span> {
    if(line_number < 1) {
        panic!("Line number must be at least 1");
    }

    get_mapping_file_ready_to_read(file_path).and_then(|(mut file, content_start_offset, line_length)| {
        return get_initial_style_for_line_from_file(&mut file, line_number, content_start_offset, line_length);
    })
}

// This is useful when wanting to avoid opening the file multiple times - like reading block of lines
pub fn get_initial_style_for_line_from_file(file: &mut File, line_number: usize, content_start_offset: usize, line_length: usize) -> Option<Span> {
    if(line_number < 1) {
        panic!("Line number must be at least 1");
    }

    // Create a buffer to read the line style with the expected length of the line
    let mut requested_line_initial_style = vec![0u8; line_length];


    let offset_in_text = content_start_offset + ((line_number - 1) * line_length);

    // Go to the matching line position
    // TODO - should differentiate between seek problem or index out of bounds
    file.seek(SeekFrom::Start(offset_in_text as u64)).expect("Go to matching line failed");

    file.read_exact(&mut requested_line_initial_style).expect("Read matching line failed");

    let line_style = String::from_utf8(requested_line_initial_style).expect("Converting requested line to UTF-8 string failed");

    return Some(parse_text_matching_single_span(&line_style).clone().with_text("".to_string()));
}

pub fn get_mapping_file_ready_to_read(file_path: PathBuf) -> Option<(File, usize, usize)> {
    // TODO - make sure the file is not closed when the function finish
    let mut file = File::open(file_path).expect("open mapping file failed");

    let get_mapping_file_metadata_result = get_mapping_file_metadata(&mut file);

    if get_mapping_file_metadata_result.is_none() {
        println!("Invalid mapping file");

        // TODO - throw instead of returning None
        return None
    }

    let (content_start_offset, line_length) = get_mapping_file_metadata_result.unwrap();
    
    return Some((file, content_start_offset, line_length));
}

// First item in returned tuple is the content_start_offset and the second is the line_length
fn get_mapping_file_metadata(f: &mut File) -> Option<(usize, usize)> {
    let mut buf = vec![0u8; 1000];

    // TODO - make sure that the buffer is read completely and not partially
    f.read(&mut buf).expect("Try read mapping header failed");

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
