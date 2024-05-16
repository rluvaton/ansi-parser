use ansi_parser::AnsiParser;

use types::Span;

use crate::parse_ansi_text::parse_ansi_as_spans_iterator::*;
use crate::parse_ansi_text::parse_ansi_split_by_lines_as_spans_iterator::*;
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
pub mod parse_text_matching_single_span;
mod playground_iterator;
pub mod custom_ansi_parse_iterator;

pub fn parse_ansi_text(str: &str) -> Vec<Span> {
    //Parse the first two blocks in the list
    //By parsing it this way, it allows you to iterate over the
    //elements returned.
    //
    //The parser only every holds a reference to the data,
    //so there is no allocation.

    
    let output: Vec<Span> = ParseAnsiAsSpansIterator::create_from_str(str.to_string(), ParseOptions::default()).collect::<Vec<Span>>();
    
    return output;
}

pub fn parse_ansi_text_with_options(str: &str, options: ParseOptions) -> Vec<Span> {
    let output: Vec<Span> = ParseAnsiAsSpansIterator::create_from_str(str.to_string(), options).collect::<Vec<Span>>();

    return output;
}

pub fn parse_ansi_text_split_by_lines(str: &str, options: ParseOptions) -> Vec<Vec<Span>> {
    return ParseAnsiAsSpansByLinesIterator::create_from_str(str.to_string(), options).collect();
}