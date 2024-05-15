use ansi_parser::AnsiParser;

use types::Span;

use crate::parse_ansi_text::parse_ansi_as_spans_iterator::ParseAnsiAsSpans;
use crate::parse_ansi_text::parse_ansi_split_by_lines_as_spans_iterator::ParseAnsiAsSpansByLines;
use crate::parse_ansi_text::parse_options::ParseOptions;

pub mod types;
pub mod colors;
pub mod constants;
pub mod style;
mod ansi_sequence_helpers;
mod tests;
pub mod parse_ansi_as_spans_iterator;
pub mod parse_options;
pub mod parse_ansi_split_by_lines_as_spans_iterator;
mod split_spans_to_lines;
pub mod parse_text_matching_single_span;

pub fn parse_ansi_text(str: &str) -> Vec<Span> {
    //Parse the first two blocks in the list
    //By parsing it this way, it allows you to iterate over the
    //elements returned.
    //
    //The parser only every holds a reference to the data,
    //so there is no allocation.
    return str
        .parse_ansi_as_spans(ParseOptions::default())
        .collect();
}

pub fn parse_ansi_text_with_options(str: &str, options: ParseOptions) -> Vec<Span> {
    return str
        .parse_ansi_as_spans(options)
        .collect();
}

pub fn parse_ansi_text_split_by_lines(str: &str, options: ParseOptions) -> Vec<Vec<Span>> {
    return str
        .parse_ansi_as_spans_by_lines(options)
        .collect();
}