use ansi::types::Span;

use crate::parse_ansi_text::parse_options::ParseOptions;
use crate::types::Line;

pub mod ansi;
pub mod ansi_output_to_spans;
pub mod ansi_text_to_output;
pub mod parse_options;
pub mod parse_text_matching_single_span;
pub mod raw_ansi_parse;

// TODO - remove convert string to iterator - done this to test that the iterator works

pub async fn parse_ansi_text(str: &str) -> Vec<Span> {
    //Parse the first two blocks in the list
    //By parsing it this way, it allows you to iterate over the
    //elements returned.
    //
    //The parser only every holds a reference to the data,
    //so there is no allocation.

    return vec![];
    // let output: Vec<Span> = compose_async_steams!(
    //     || async_chars_stream(str.to_string()),
    //     parse_ansi,
    //     // TODO
    //     // merge_text_output,
    //     |output| convert_ansi_output_to_spans(output, ParseOptions::default())
    // ).await.collect::<Vec<Span>>().await;
    //
    // return output;
}

pub async fn parse_ansi_text_with_options(str: &str, options: ParseOptions) -> Vec<Span> {
    return vec![];
    // let output: Vec<Span> = compose_async_steams!(
    //     || async_chars_stream(str.to_string()),
    //     parse_ansi,
    //     // TODO
    //     // merge_text_output,
    //     |output| convert_ansi_output_to_spans(output, options)
    // ).await.collect::<Vec<Span>>().await;
    //
    // return output;
}

pub fn parse_ansi_text_split_by_lines(str: &str, options: ParseOptions) -> Vec<Line> {
    return vec![];
    // return compose_async_steams!(
    //     || async_chars_stream(str.to_string()),
    //     parse_ansi,
    //     // TODO -
    //     // merge_text_output,
    //     |output| convert_ansi_output_to_lines_of_spans(output, options)
    // ).await.collect::<Vec<Line>>().await;
}
