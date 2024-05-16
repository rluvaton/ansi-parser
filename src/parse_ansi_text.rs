use ansi_parser::AnsiParser;

use ansi::types::Span;
use crate::parse_ansi_text::iterators::parse_ansi_as_spans_iterator::ParseAnsiAsSpansIterator;
use crate::parse_ansi_text::iterators::parse_ansi_split_by_lines_as_spans_iterator::ParseAnsiAsSpansByLinesIterator;
use crate::parse_ansi_text::iterators::playground_iterator::CharsIterator;

use crate::parse_ansi_text::parse_options::ParseOptions;

mod tests;
pub mod parse_options;
pub mod parse_text_matching_single_span;
pub mod iterators;
pub mod ansi;

// TODO - remove convert string to iterator - done this to test that the iterator works

pub fn parse_ansi_text(str: &str) -> Vec<Span> {
    //Parse the first two blocks in the list
    //By parsing it this way, it allows you to iterate over the
    //elements returned.
    //
    //The parser only every holds a reference to the data,
    //so there is no allocation.

    
    let output: Vec<Span> = ParseAnsiAsSpansIterator::create_from_string_iterator(Box::new(CharsIterator {
        index: 0,
        str: str.to_string(),
    }), ParseOptions::default()).collect::<Vec<Span>>();
    
    return output;
}

pub fn parse_ansi_text_with_options(str: &str, options: ParseOptions) -> Vec<Span> {
    let output: Vec<Span> = ParseAnsiAsSpansIterator::create_from_string_iterator(Box::new(CharsIterator {
        index: 0,
        str: str.to_string(),
    }), options).collect::<Vec<Span>>();

    return output;
}

pub fn parse_ansi_text_split_by_lines(str: &str, options: ParseOptions) -> Vec<Vec<Span>> {
    return ParseAnsiAsSpansByLinesIterator::create_from_string_iterator(Box::new(CharsIterator {
        index: 0,
        str: str.to_string(),
    }), options).collect();
}