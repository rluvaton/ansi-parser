use std::cmp::max;
use std::ffi::OsString;
use std::path::PathBuf;
use std::pin::Pin;

use futures::pin_mut;
use futures_core::Stream;
use futures_util::stream::StreamExt;

use crate::cli::format::flat_json_line_span_lines::*;
use crate::cli::format::json_line_single_span::*;
use crate::cli::format::json_line_span_lines::*;
use crate::cli::format::json_single_span::spans_valid_json;
use crate::cli::format::json_span_lines::*;
use crate::cli::parse_temp::tmp_parse;
use crate::compose_async_steams;
use crate::files::streams::{read_file_by_chunks, read_file_by_chunks_from_to_locations};
use crate::mapping_file::read::{get_line_metadata_from_file, get_mapping_file_ready_to_read};
use crate::parse_ansi_text::ansi_text_to_output::stream_helpers::merge_text_output;
use crate::parse_ansi_text::ansi_text_to_output::stream_parse::parse_ansi;
use crate::parse_ansi_text::ansi_output_to_spans::parse_ansi_as_spans::*;
use crate::parse_ansi_text::ansi_output_to_spans::parse_ansi_split_by_lines_as_spans::{
    convert_ansi_output_to_lines_of_spans, Line,
};
use crate::parse_ansi_text::parse_options::ParseOptions;
use crate::streams_helpers::unwrap_items;

// TODO - in order to save memory and not read the entire file to memory
//        we should have a way to have an iterator over the file that yield the spans
//        currently, the parse_ansi lib is not designed to work with iterators
//        so we need to yield the current span and the next span

pub async fn run_parse_command(matches: &clap::ArgMatches) {
    let split_by_lines = *matches.get_one::<bool>("split-lines").unwrap();

    let from_line = matches.get_one::<usize>("from-line");
    let to_line = matches.get_one::<usize>("to-line");
    let mapping_file = matches.get_one::<String>("mapping-file");

    let file_path = matches
        .get_one::<String>("file")
        .expect("Should have been able to get the file path");

    let buf_file_path = PathBuf::from(OsString::from(file_path));

    let format = matches
        .get_one::<String>("format")
        .expect("Should have been able to get the format");
    let flat_json_line_output_format = format == "flat-json-line";
    let json_output_format = format == "json";
    let json_line_output_format = format == "json-line";

    if !split_by_lines && flat_json_line_output_format {
        panic!("'flat-json-line' option is only available when 'split-lines' is enabled");
    }

    tmp_parse(file_path.to_string(), ParseOptions::default());
    //
    //
    // let output_iterator: Pin<Box<dyn Stream<Item = String>>>;
    //
    // if !split_by_lines {
    //     let parsed_ansi = compose_async_steams!(
    //         // TODO - change this chunks
    //         || read_file_by_chunks(&file_path, 1024),
    //         unwrap_items,
    //         parse_ansi,
    //         merge_text_output,
    //         |output| convert_ansi_output_to_spans(output, ParseOptions::default())
    //     );
    //
    //     if json_output_format {
    //         output_iterator =
    //             Box::pin(compose_async_steams!(|| parsed_ansi, spans_valid_json).await);
    //     } else if json_line_output_format {
    //         output_iterator =
    //             Box::pin(compose_async_steams!(|| parsed_ansi, spans_json_line).await);
    //     } else {
    //         panic!("Invalid format")
    //     }
    // } else {
    //     let parse_ansi_as_spans_iterator = get_spans_in_range_if_needed_from_file_path(
    //         buf_file_path.clone(),
    //         mapping_file,
    //         from_line,
    //         to_line,
    //     );
    //     if json_output_format {
    //         output_iterator = Box::pin(
    //             compose_async_steams!(|| parse_ansi_as_spans_iterator, spans_lines_valid_json)
    //                 .await,
    //         );
    //     } else if json_line_output_format {
    //         output_iterator = Box::pin(
    //             compose_async_steams!(|| parse_ansi_as_spans_iterator, spans_lines_json_lines)
    //                 .await,
    //         );
    //     } else if flat_json_line_output_format {
    //         output_iterator = Box::pin(
    //             compose_async_steams!(|| parse_ansi_as_spans_iterator, spans_lines_flat_json_lines)
    //                 .await,
    //         );
    //     } else {
    //         panic!("Invalid format")
    //     }
    // }
    //
    // let output = matches
    //     .get_one::<String>("output")
    //     .expect("Should have been able to get the output destination");
    //
    // if output == "stdout" {
    //     print_stream_of_strings_to_stdout(output_iterator).await;
    // } else if output == "sink" {
    //     sink(output_iterator).await;
    // } else {
    //     panic!("Invalid output destination");
    // }
}

async fn print_stream_of_strings_to_stdout<S: Stream<Item = String>>(stream: S) {
    pin_mut!(stream); // needed for iteration

    while let Some(value) = stream.next().await {
        println!("got {}", value);
    }
}
async fn sink<S: Stream<Item = String>>(stream: S) {
    pin_mut!(stream); // needed for iteration

    while let Some(_) = stream.next().await {
        // println!("got {}", value);
    }
}

// TODO - return iterator instead of Vec for better performance to not wait for the entire file to be read or load it to memory
async fn get_spans_in_range_if_needed_from_file_path<'a>(
    file_path: PathBuf,
    mapping_file_path: Option<&String>,
    from_line: Option<&usize>,
    to_line: Option<&usize>,
) -> Pin<Box<dyn Stream<Item = Line>>> {
    if from_line.is_none() && to_line.is_none() {
        return Box::pin(
            compose_async_steams!(
                // TODO - change this chunks
                || read_file_by_chunks(&file_path.to_str().unwrap(), 1024),
                unwrap_items,
                parse_ansi,
                merge_text_output,
                |output| convert_ansi_output_to_lines_of_spans(output, ParseOptions::default())
            )
            .await,
        );
    }

    if mapping_file_path.is_none() {
        // Using slow path since we calculate everything
        return Box::pin(
            get_spans_in_range_without_mapping_file(file_path, from_line, to_line).await,
        );
    }

    let from_line_value = *from_line.unwrap_or(&0);

    mapping_file_path.expect("Mapping file is required when using from-line or to-line");

    let ready_data_for_reading_file = get_mapping_file_ready_to_read(PathBuf::from(
        OsString::from(mapping_file_path.unwrap().clone()),
    ));

    let (mut file, content_start_offset, line_length) = ready_data_for_reading_file.unwrap();

    let from_line_metadata = get_line_metadata_from_file(
        &mut file,
        from_line_value,
        content_start_offset,
        line_length,
    );

    if from_line_metadata.is_none() {
        // TODO - avoid panicking and instead return error or empty
        panic!("Could not get ready mapping data for reading file");
    }

    if to_line.is_some() && to_line.unwrap() < &from_line_value {
        panic!("to-line must be greater than from-line");
    }

    let from_line_index_in_file = Some(
        from_line_metadata
            .clone()
            .unwrap()
            .location_in_original_file,
    );
    let mut to_line_index_in_file = None;

    if to_line.is_some() {
        let to_line_metadata = get_line_metadata_from_file(
            &mut file,
            *to_line.unwrap(),
            content_start_offset,
            line_length,
        );

        // TODO - What if the last, should not panic
        if to_line_metadata.is_none() {
            // TODO - avoid panicking and instead return error or empty
            panic!("Could not get ready mapping data for reading file");
        }

        to_line_index_in_file = Some(to_line_metadata.unwrap().location_in_original_file);
    }

    return Box::pin(
        compose_async_steams!(
            // TODO - change this chunks
            || read_file_by_chunks_from_to_locations(
                &file_path.to_str().unwrap(),
                1024,
                from_line_index_in_file,
                to_line_index_in_file,
            ),
            unwrap_items,
            parse_ansi,
            merge_text_output,
            |output| convert_ansi_output_to_lines_of_spans(
                output,
                ParseOptions::default().with_initial_span(from_line_metadata.unwrap().initial_span)
            )
        )
        .await,
    );
}

// TODO - return iterator instead of Vec for better performance to not wait for the entire file to be read or load it to memory
async fn get_spans_in_range_without_mapping_file<'a>(
    file_path: PathBuf,
    from_line: Option<&usize>,
    to_line: Option<&usize>,
) -> Pin<Box<dyn Stream<Item = Line>>> {
    let lines_stream = compose_async_steams!(
        // TODO - change this chunks
        || read_file_by_chunks(&file_path.to_str().unwrap(), 1024),
        unwrap_items,
        parse_ansi,
        merge_text_output,
        |output| convert_ansi_output_to_lines_of_spans(output, ParseOptions::default())
    )
    .await;

    if from_line.is_some() && to_line.is_some() {
        return Box::pin(
            lines_stream
                .skip(max(*from_line.unwrap(), 1) - 1)
                .take(*to_line.unwrap() - *from_line.unwrap()),
        );
    }

    if from_line.is_some() {
        return Box::pin(lines_stream.skip(max(*from_line.unwrap(), 1) - 1));
    }

    if to_line.is_some() {
        return Box::pin(lines_stream.take(*to_line.unwrap() - *from_line.unwrap()));
    }

    return Box::pin(lines_stream);
}
