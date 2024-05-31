use std::fs::File;
use std::io::{BufRead, Read, Seek, SeekFrom};
use std::path::PathBuf;

use crate::mapping_file::constants::*;
use crate::parse_ansi_text::ansi::types::Span;
use crate::parse_ansi_text::parse_text_matching_single_span::parse_text_matching_single_span;

#[derive(PartialEq, Debug, Clone)]
pub struct MappingItem {
    pub initial_span: Span,
    pub location_in_original_file: usize,
}

pub fn get_initial_style_for_line(mapping_text: String, line_number: usize) -> Option<MappingItem> {
    if line_number < 1 {
        panic!("Line number must be at least 1");
    }

    // TODO - can avoid cloning?
    let get_mapping_file_metadata_result = get_mapping_metadata(mapping_text.clone());

    if get_mapping_file_metadata_result.is_none() {
        println!("Invalid mapping file");

        // TODO - throw instead of returning None
        return None;
    }

    let (content_start_offset, line_length) = get_mapping_file_metadata_result.unwrap();

    let offset_in_text = content_start_offset + ((line_number - 1) * line_length);

    if offset_in_text >= mapping_text.len() {
        println!("Invalid mapping, line number is missing");

        // TODO - throw instead of returning None
        return None;
    }

    if offset_in_text + line_length > mapping_text.len() {
        println!("Invalid mapping, each line is not the same length");

        // TODO - throw instead of returning None
        return None;
    }

    let line_style = mapping_text
        [offset_in_text..offset_in_text + line_length - SECOND_PART_LINE_LENGTH]
        .to_string();

    let location_in_original_file = mapping_text
        [offset_in_text + line_length - SECOND_PART_LINE_LENGTH..offset_in_text + line_length]
        .as_bytes();

    let reading_of_number = u64::from_ne_bytes(location_in_original_file.try_into().unwrap()); //convert the array to a variable of type usize

    // To make sure there is no empty span
    let span = parse_text_matching_single_span(&line_style);

    return Some(MappingItem {
        initial_span: span.with_text(vec![]),
        location_in_original_file: reading_of_number as usize,
    });
}

pub fn get_line_metadata_from_file_path(
    file_path: PathBuf,
    line_number: usize,
) -> Option<MappingItem> {
    if line_number < 1 {
        panic!("Line number must be at least 1");
    }

    get_mapping_file_ready_to_read(file_path).and_then(
        |(mut file, content_start_offset, line_length)| {
            return get_line_metadata_from_file(
                &mut file,
                line_number,
                content_start_offset,
                line_length,
            );
        },
    )
}

// This is useful when wanting to avoid opening the file multiple times - like reading block of lines
pub fn get_line_metadata_from_file(
    file: &mut File,
    line_number: usize,
    content_start_offset: usize,
    line_length: usize,
) -> Option<MappingItem> {
    if line_number < 1 {
        panic!("Line number must be at least 1");
    }

    // Create a buffer to read the line style with the expected length of the line
    let mut requested_line_initial_style = vec![0u8; line_length - SECOND_PART_LINE_LENGTH];
    let mut requested_line_original_location = vec![0u8; SECOND_PART_LINE_LENGTH];

    let offset_in_text = content_start_offset + ((line_number - 1) * line_length);

    // Go to the matching line position
    // TODO - should differentiate between seek problem or index out of bounds
    file.seek(SeekFrom::Start(offset_in_text as u64))
        .expect("Go to matching line failed");

    file.read_exact(&mut requested_line_initial_style)
        .expect("Read matching line failed");

    // Go to the matching line position
    // TODO - should differentiate between seek problem or index out of bounds
    file.seek(SeekFrom::Start(
        (offset_in_text + line_length - SECOND_PART_LINE_LENGTH) as u64,
    ))
    .expect("Go to matching line failed");

    file.read_exact(&mut requested_line_original_location)
        .expect("Read matching line failed");

    let line_style = String::from_utf8(requested_line_initial_style)
        .expect("Converting requested line to UTF-8 string failed");

    return Some(MappingItem {
        initial_span: parse_text_matching_single_span(&line_style)
            .clone()
            .with_text(vec![]),
        location_in_original_file: u64::from_ne_bytes(
            requested_line_original_location.try_into().unwrap(),
        ) as usize,
    });
}

pub fn get_mapping_file_ready_to_read(file_path: PathBuf) -> Option<(File, usize, usize)> {
    // TODO - make sure the file is not closed when the function finish
    let mut file = File::open(file_path).expect("open mapping file failed");

    let get_mapping_file_metadata_result = get_mapping_file_metadata(&mut file);

    if get_mapping_file_metadata_result.is_none() {
        println!("Invalid mapping file");

        // TODO - throw instead of returning None
        return None;
    }

    let (content_start_offset, line_length) = get_mapping_file_metadata_result.unwrap();

    return Some((file, content_start_offset, line_length));
}

// First item in returned tuple is the content_start_offset and the second is the line_length
fn get_mapping_file_metadata(f: &mut File) -> Option<(usize, usize)> {
    let mut buf = vec![0u8; 1000];

    // TODO - make sure that the buffer is read completely and not partially
    f.read(&mut buf).expect("Try read mapping header failed");

    let header = buf
        .lines()
        .next()
        .expect("No lines in mapping file")
        .expect("No content in mapping file");

    return get_mapping_metadata(header);
}

// First item in returned tuple is the content_start_offset and the second is the line_length
fn get_mapping_metadata(header: String) -> Option<(usize, usize)> {
    if header.len() < 1 {
        println!("Invalid mapping file, should have at least one line");
        return None;
    }

    let line_length_result = header.parse::<usize>();

    if line_length_result.is_err() {
        panic!("Invalid mapping file, first line should be a number");
    }

    let line_length = line_length_result.unwrap();

    return Some((header.len(), line_length));
}
