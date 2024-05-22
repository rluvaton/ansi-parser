use crate::compose_async_steams;
use crate::parse_ansi_text::iterators::custom_ansi_parse_iterator::{merge_text_output, parse_ansi};
use crate::parse_ansi_text::iterators::parse_ansi_as_spans_iterator::convert_ansi_output_to_spans;
use crate::parse_ansi_text::iterators::parse_ansi_split_by_lines_as_spans_iterator::{
    convert_ansi_output_to_lines_of_spans, Line,
};
use ansi::types::Span;
use futures::stream::StreamExt;

use crate::parse_ansi_text::parse_options::ParseOptions;
use crate::test_utils::{async_chars_stream, async_chars_stream_str};

pub mod ansi;
pub mod iterators;
pub mod parse_options;
pub mod parse_text_matching_single_span;
mod tests;

// TODO - remove convert string to iterator - done this to test that the iterator works

pub async fn parse_ansi_text(str: &str) -> Vec<Span> {
    //Parse the first two blocks in the list
    //By parsing it this way, it allows you to iterate over the
    //elements returned.
    //
    //The parser only every holds a reference to the data,
    //so there is no allocation.

    let output: Vec<Span> =
        compose_async_steams!(
            || async_chars_stream_str(str),
            parse_ansi,
            merge_text_output,
            |output| convert_ansi_output_to_spans(output, ParseOptions::default()))
        .await
        .collect::<Vec<Span>>()
        .await;

    return output;
}

pub async fn parse_ansi_text_with_options(str: &str, options: ParseOptions) -> Vec<Span> {
    
    let output: Vec<Span> =
        compose_async_steams!(
            || async_chars_stream_str(str),
            parse_ansi,
            merge_text_output,
            |output| convert_ansi_output_to_spans(output, options))
        .await
        .collect::<Vec<Span>>()
        .await;

    return output;
}

pub async fn parse_ansi_text_split_by_lines(str: &str, options: ParseOptions) -> Vec<Line> {
    return compose_async_steams!(
        || async_chars_stream_str(str),
        parse_ansi,
        merge_text_output,
        |output| convert_ansi_output_to_lines_of_spans(output, options))
    .await
    .collect::<Vec<Line>>()
    .await;
    //
    // return ParseAnsiAsSpansByLinesIterator::create_from_string_iterator(Box::new(CharsIterator {
    //     index: 0,
    //     str: str.to_string(),
    // }), options).collect();
}
