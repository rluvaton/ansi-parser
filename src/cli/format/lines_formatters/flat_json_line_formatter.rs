use std::iter::Iterator;

use genawaiter::{rc::gen, yield_};
use serde::{Deserialize, Serialize};
use crate::parse_ansi_text::ansi::types::Span;
use crate::parse_ansi_text::ansi_output_to_spans::parse_ansi_split_by_lines_as_spans::Line;
use crate::traits::ToJson;

// Just to have a struct for the type in the tests

#[derive(Debug, PartialEq, Clone, Deserialize)]
struct LineMarker {
    #[serde(rename(serialize = "type_", deserialize = "type"))]
    type_: String
}

const NEW_LINE_MARKER: &str = r#"{ "type": "new line" }"#;

pub fn flat_json_line_formatter<I: Iterator<Item=Line>>(iter: I) -> impl Iterator<Item = String> {
    return gen!({
        for line in iter {
            for span in line.spans.iter() {
                yield_!(span.to_json());
            }
            yield_!(NEW_LINE_MARKER.to_string());
        }
    }).into_iter();

}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use crate::cli::format::spans_formatters::single_item_json_formatter::json_single_item_formatter;

    use crate::parse_ansi_text::ansi::style::{Brightness, TextStyle};
    use crate::parse_ansi_text::ansi::types::Span;

    use super::*;


    #[test]
    fn test_formatter_each_span_is_different_line_and_have_delimiter() {
        let lines: Vec<Line> = vec![
            Line {
                location_in_file: 0,
                spans: vec![
                    Span::empty()
                        .with_text("Hello, World!".as_bytes().to_vec())
                        .with_brightness(Brightness::Bold),
                    Span::empty()
                        .with_text(" ".as_bytes().to_vec()),
                    Span::empty()
                        .with_text("This is another span".as_bytes().to_vec())
                        .with_text_style(TextStyle::Italic | TextStyle::Underline)
                ]
            },
            Line {
                location_in_file: 10,
                spans: vec![
                    Span::empty()
                        .with_text("how are you".as_bytes().to_vec())
                        .with_brightness(Brightness::Dim),
                    Span::empty()
                        .with_text(" ".as_bytes().to_vec()),
                    Span::empty()
                        .with_text("good".as_bytes().to_vec())
                        .with_text_style(TextStyle::Strikethrough)
                ]
            }
        ];

        let spans_lines = lines.clone().iter().map(|line| line.spans.clone()).collect::<Vec<Vec<Span>>>();
        let number_of_spans = spans_lines.iter().map(|spans| spans.len()).sum::<usize>();

        let outputs_iter = flat_json_line_formatter(lines.clone().into_iter());

        let outputs: Vec<String> = outputs_iter.collect();
        
        assert_eq!(outputs.len(), number_of_spans + spans_lines.len());
        
        let mut i = 0;

        for line in spans_lines {
            for span in line {
                let output_span = sonic_rs::from_str::<Span>(outputs[i].as_str()).expect("Failed to parse span");
                
                assert_eq!(output_span, span);
                i += 1;
            }
            
            let output_line_marker = sonic_rs::from_str::<LineMarker>(outputs[i].as_str()).expect("Failed to parse line marker");

            assert_eq!(output_line_marker, LineMarker { type_: "new line".to_string() });
            
            i += 1;
        }
    }
}
