use crate::parse_ansi_text::ansi::types::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub spans: Vec<Span>,
    pub location_in_file: usize,
}
