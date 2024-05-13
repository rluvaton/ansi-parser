pub mod types;
pub mod colors;
pub mod constants;
pub mod style;
mod ansi_sequence_helpers;
mod tests;

use ansi_parser::{Output, AnsiParser};

use types::Span;
use crate::parse_ansi_text::ansi_sequence_helpers::*;


pub fn parse_ansi_text(str: &str) -> Vec<Span> {


    //Parse the first two blocks in the list
    //By parsing it this way, it allows you to iterate over the
    //elements returned.
    //
    //The parser only every holds a reference to the data,
    //so there is no allocation.
    let parsed: Vec<Output> = str
        .ansi_parse()
        // .take(2)
        .collect();

    let spans: Vec<Span> = create_span_from_output(parsed);
    
    return spans;
}



// TODO - replace argument to be iterator and return type to be iterator for best performance
fn create_span_from_output(parsed: Vec<Output>) -> Vec<Span> {
    if parsed.len() == 0 {
        return vec![];
    }

    let mut all_spans: Vec<Span> = vec![];

    let mut span: Span = Span::empty();

    for output in parsed {
        // println!("Output: {:?}", output);

        match output {
            Output::TextBlock(text) => {
                // println!("Text block: {}", text);
                span.text.push_str(text);
            },
            Output::Escape(seq) => {
                let sequence_type = get_type_from_ansi_sequence(&seq);
                
                match sequence_type {
                    AnsiSequenceType::Unsupported => {
                        continue;
                    },
                    AnsiSequenceType::Reset => {
                        // Ignore spans that are just empty text even if they have style as this won't be shown
                        if span.text.len() > 0 {
                            all_spans.push(span.clone());
                        }
                        
                        span = Span::empty();
                    },
                    AnsiSequenceType::ForegroundColor(color) => {
                        if span.text.len() > 0 && span.color != color {
                            all_spans.push(span.clone());
                            span = span.with_text("".to_string());
                        }
                        span.color = color;
                    },
                    AnsiSequenceType::BackgroundColor(color) => {
                        if span.text.len() > 0 && span.bg_color != color {
                            all_spans.push(span.clone());
                            span = span.with_text("".to_string());
                        }
                        span.bg_color = color;
                    },
                    AnsiSequenceType::Brightness(brightness) => {
                        if span.text.len() > 0 && span.brightness != brightness {
                            all_spans.push(span.clone());
                            span = span.with_text("".to_string());
                        }
                        span.brightness = brightness;
                    },
                    AnsiSequenceType::TextStyle(style) => {
                        if span.text.len() > 0 && span.text_style != style {
                            all_spans.push(span.clone());
                            span = span.with_text("".to_string());
                        }
                        // Merge the style
                        span.text_style = span.text_style | style;
                    },
                }
            },
            
        }
    }

    // Add last span if it has text
    if span.text.len() > 0 {
        // No need to clone as it's the last one
        all_spans.push(span);
    }

    return all_spans
}

