use std::ffi::OsString;
use std::path::PathBuf;
use crate::files::file_reader::FileReaderOptions;
use crate::files::file_size::get_file_size;

use crate::mapping_file::read::{get_line_metadata_from_file, get_mapping_file_ready_to_read};
use crate::parse_ansi_text::ansi::types::Span;
use crate::parse_ansi_text::parse_options::ParseOptions;
use crate::parse_file::file_to_lines_of_spans::read_ansi_file_to_lines;
use crate::parse_file::types::ReadAnsiFileOptions;
use crate::types::Line;

#[derive(Debug, Clone, PartialEq)]
pub struct FromMiddleOfFile {
    pub from_bytes: Option<usize>,
    pub to_bytes: Option<usize>,
    pub initial_span: Option<Span>,
}

// Avoid using when mapping file is not provided and the file is big and the from_line is not the first line
// because it will parse the lines until the max(from_line, to_line)
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
            // + 1 to get the line after the last line to have the to bytes
            to_line.unwrap() + 1,
            content_start_offset,
            line_length,
        );

        if to.is_none() {
            // File size if the line does not exist or it's the last line in the file
            to_bytes = Some(get_file_size(input_file));
        } else {
            to_bytes = Some(to.unwrap().location_in_original_file - 1);
        }
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

    let file_reader_options = FileReaderOptions {
        file_path: input_file.clone().into_os_string().into_string().expect("Failed to convert path"),
        from_bytes: None,
        chunk_size_in_bytes: Some(1024 * 1024 * 10), // 10MB
        to_bytes: None,
    };
    let parse_options = ParseOptions::default();

    let options = ReadAnsiFileOptions {
        file_options: file_reader_options,
        parse_options,
    };


    let mut lines_iterator: Box<dyn Iterator<Item = Line>> = Box::new(read_ansi_file_to_lines(options));

    if from_line.is_some() {
        lines_iterator = Box::new(lines_iterator.skip(*from_line.unwrap()));
    }

    if to_line.is_some() {
        let number_of_lines = to_line.unwrap() - from_line.unwrap_or(&0);

        // + 1 to get the line after the last line to have the to bytes, if the line does not exists we are in pro
        lines_iterator = Box::new(lines_iterator.take(number_of_lines + 1));
    }

    let lines = lines_iterator.collect::<Vec<Line>>();

    let from = lines.first();

    let mut from_bytes: Option<usize> = None;
    let mut to_bytes: Option<usize> = None;
    let mut initial_span: Option<Span> = None;

    if let Some(from) = from {
        from_bytes = Some(from.location_in_file);

        if !from.spans.is_empty() {
            initial_span = Some(from.spans[0].clone().with_text(vec![]));
        }
    }

    if to_line.is_some() {
        let got_all_lines = lines.len() == to_line.unwrap() - from_line.unwrap_or(&0) + 1;

        if got_all_lines {
            // the line after requested line - 1 to get the end of the requested line
            to_bytes = Some(lines.last().unwrap().location_in_file - 1);
        } else {
            // File size if the line does not exist or it's the last line in the file
            to_bytes = Some(get_file_size(input_file));
        }
    }

    return FromMiddleOfFile {
        from_bytes,
        to_bytes,
        initial_span,
    };
}
