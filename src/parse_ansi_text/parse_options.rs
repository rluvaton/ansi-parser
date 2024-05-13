use crate::parse_ansi_text::types::Span;

pub struct ParseOptions {
    pub split_spans_to_lines: bool,
    pub initial_span: Span,
}

impl ParseOptions {
    pub fn default() -> ParseOptions {
        ParseOptions {
            split_spans_to_lines: false,
            initial_span: Span::empty(),
        }
    }
    
    pub fn with_split_spans_to_lines(mut self, split_spans_to_lines: bool) -> ParseOptions {
        self.split_spans_to_lines = split_spans_to_lines;
        self
    }
    
    pub fn with_initial_span(mut self, initial_span: Span) -> ParseOptions {
        self.initial_span = initial_span;
        self
    }
}