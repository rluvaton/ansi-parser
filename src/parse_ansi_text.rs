

use futures::stream::StreamExt;
use ansi::types::Span;
use crate::compose_async_steams;
use crate::parse_ansi_text::ansi_text_to_output::stream_helpers::merge_text_output;
use crate::parse_ansi_text::ansi_text_to_output::stream_parse::parse_ansi;
use crate::parse_ansi_text::ansi_output_to_spans::parse_ansi_as_spans::{convert_ansi_output_to_spans};
use crate::parse_ansi_text::ansi_output_to_spans::parse_ansi_split_by_lines_as_spans::{convert_ansi_output_to_lines_of_spans, Line};


use crate::parse_ansi_text::parse_options::ParseOptions;
use crate::test_utils::{async_chars_stream};

mod tests;
pub mod parse_options;
pub mod parse_text_matching_single_span;
pub mod ansi_output_to_spans;
pub mod ansi;
pub mod raw_ansi_parse;
pub mod ansi_text_to_output;

// TODO - remove convert string to iterator - done this to test that the iterator works

pub async fn parse_ansi_text(str: &str) -> Vec<Span> {
    //Parse the first two blocks in the list
    //By parsing it this way, it allows you to iterate over the
    //elements returned.
    //
    //The parser only every holds a reference to the data,
    //so there is no allocation.

    let output: Vec<Span> = compose_async_steams!(
        || async_chars_stream(str.to_string()),
        parse_ansi,
        merge_text_output,
        |output| convert_ansi_output_to_spans(output, ParseOptions::default())
    ).await.collect::<Vec<Span>>().await;
    
    return output;
}

pub async fn parse_ansi_text_with_options(str: &str, options: ParseOptions) -> Vec<Span> {
    let output: Vec<Span> = compose_async_steams!(
        || async_chars_stream(str.to_string()),
        parse_ansi,
        merge_text_output,
        |output| convert_ansi_output_to_spans(output, options)
    ).await.collect::<Vec<Span>>().await;

    return output;
}

pub async fn parse_ansi_text_split_by_lines(str: &str, options: ParseOptions) -> Vec<Line> {
    return compose_async_steams!(
        || async_chars_stream(str.to_string()),
        parse_ansi,
        merge_text_output,
        |output| convert_ansi_output_to_lines_of_spans(output, options)
    ).await.collect::<Vec<Line>>().await;
}
