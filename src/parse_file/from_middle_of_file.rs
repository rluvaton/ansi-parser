use std::ffi::OsString;
use std::path::PathBuf;

use crate::mapping_file::read::{get_line_metadata_from_file, get_mapping_file_ready_to_read};
use crate::parse_ansi_text::ansi::types::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct FromMiddleOfFile {
    pub from_bytes: Option<usize>,
    pub to_bytes: Option<usize>,
    pub initial_span: Option<Span>,
}

pub fn get_from_middle_of_the_file_info(
    input_file: PathBuf,
    from_line: Option<&usize>,
    to_line: Option<&usize>,
    mapping_file: Option<&String>,
) -> FromMiddleOfFile {
    if mapping_file.is_none() {
        return get_from_middle_of_the_file_info_without_mapping(input_file, from_line, to_line);
    }

    let mapping_file_path = PathBuf::from(OsString::from(mapping_file.clone().unwrap()));

    let (mut file, content_start_offset, line_length) =
        get_mapping_file_ready_to_read(mapping_file_path).expect("Failed to read mapping file");

    let mut from_bytes: Option<usize> = None;
    let mut to_bytes: Option<usize> = None;
    let mut initial_span: Option<Span> = None;

    if from_line.is_some() {
        let from = get_line_metadata_from_file(
            &mut file,
            *from_line.unwrap(),
            content_start_offset,
            line_length,
        )
        .expect("from should exists");

        from_bytes = Some(from.location_in_original_file);
        initial_span = Some(from.initial_span);
    }

    if to_line.is_some() {
        let to = get_line_metadata_from_file(
            &mut file,
            *to_line.unwrap(),
            content_start_offset,
            line_length,
        )
        .expect("to should exists");

        to_bytes = Some(to.location_in_original_file);
    };

    return FromMiddleOfFile {
        from_bytes,
        to_bytes,
        initial_span,
    };
}

pub fn get_from_middle_of_the_file_info_without_mapping(
    input_file: PathBuf,
    from_line: Option<&usize>,
    to_line: Option<&usize>,
) -> FromMiddleOfFile {
    if from_line.is_none() && to_line.is_none() {
        return FromMiddleOfFile {
            from_bytes: None,
            to_bytes: None,
            initial_span: None,
        };
    }

    // Calculate style until the requested line and return it
    todo!("Calculate style until the requested line and return it");
}
