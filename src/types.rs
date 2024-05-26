use crate::parse_ansi_text::ansi::types::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub(crate) spans: Vec<Span>,
    pub(crate) location_in_file: usize,
}
