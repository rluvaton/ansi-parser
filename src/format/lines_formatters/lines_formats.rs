use crate::format::lines_formatters::{
    flat_json_line_formatter::flat_json_line_formatter,
    json_formatter::json_lines_formatter,
    single_item_json_formatter::json_lines_single_item_formatter,
};
use crate::types::Line;

#[derive(Debug, Clone, PartialEq)]
pub enum LineFormat {
    JSON,
    SingleJsonLine,
    FlatJsonLine,
}

pub fn format_lines(
    iter: Box<dyn Iterator<Item = Line>>,
    format: LineFormat,
) -> Box<dyn Iterator<Item = String>> {
    match format {
        LineFormat::JSON => Box::new(json_lines_formatter(iter)),
        LineFormat::SingleJsonLine => Box::new(json_lines_single_item_formatter(iter)),
        LineFormat::FlatJsonLine => Box::new(flat_json_line_formatter(iter)),
    }
}

pub fn get_line_format_from_string(output_format: &str) -> Option<LineFormat> {
    if output_format == "json" {
        return Some(LineFormat::JSON);
    }
    
    if output_format == "json-line" {
        return Some(LineFormat::SingleJsonLine);
    }
    
    if output_format == "flat-json-line" {
        return Some(LineFormat::FlatJsonLine);
    }
    
    return None;
}