use crate::format::spans_formatters::{
    json_formatter::json_formatter, single_item_json_formatter::json_single_item_formatter,
};
use crate::parse_ansi_text::ansi::types::Span;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum SpanFormat {
    JSON,
    SingleJsonItem,
}

pub fn format_spans(
    iter: Box<dyn Iterator<Item = Span>>,
    format: SpanFormat,
) -> Box<dyn Iterator<Item = String>> {
    match format {
        SpanFormat::JSON => Box::new(json_formatter(iter)),
        SpanFormat::SingleJsonItem => Box::new(json_single_item_formatter(iter)),
    }
}


pub fn get_span_format_from_string(output_format: &str) -> Option<SpanFormat> {
    if output_format == "json" {
        return Some(SpanFormat::JSON);
    }

    if output_format == "json-line" {
        return Some(SpanFormat::SingleJsonItem);
    }


    return None;
}