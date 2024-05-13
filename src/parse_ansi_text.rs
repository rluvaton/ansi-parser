use ansi_parser::AnsiParser;

use types::Span;

use crate::parse_ansi_text::ansi_parse_iterator_to_span_iterator::ParseAnsiAsSpans;
use crate::parse_ansi_text::parse_options::ParseOptions;

pub mod types;
pub mod colors;
pub mod constants;
pub mod style;
mod ansi_sequence_helpers;
mod tests;
mod ansi_parse_iterator_to_span_iterator;
mod parse_options;

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