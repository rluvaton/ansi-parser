use std::iter::Iterator;

use crate::types::Line;
use genawaiter::{rc::gen, yield_};

pub fn json_lines_formatter<I: Iterator<Item = Line>>(iter: I) -> impl Iterator<Item = String> {
    return gen!({
        let mut yielded_first_item = false;
        yield_!("[\n".to_string());

        // Can replace the loop here with just json line single span, as it's the same thing
        for line in iter {
            let mut str: &str = "";

            if yielded_first_item {
                // Print from prev object
                str = ",";
            }

            yielded_first_item = true;

            // line.spans.to_json().as_str()

            yield_!(str.to_string() + sonic_rs::to_string(&line.spans).unwrap().as_str());
        }

        yield_!("\n]".to_string());
    })
    .into_iter();
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::parse_ansi_text::ansi::style::{Brightness, TextStyle};
    use crate::parse_ansi_text::ansi::types::Span;

    use super::*;

    #[test]
    fn test_formatter_combined_output_is_valid_json() {
        let lines: Vec<Line> = vec![
            Line {
                location_in_file: 0,
                spans: vec![
                    Span::empty()
                        .with_text(b"Hello, World!".to_vec())
                        .with_brightness(Brightness::Bold),
                    Span::empty().with_text(b" ".to_vec()),
                    Span::empty()
                        .with_text(b"This is another span".to_vec())
                        .with_text_style(TextStyle::Italic | TextStyle::Underline),
                ],
            },
            Line {
                location_in_file: 10,
                spans: vec![
                    Span::empty()
                        .with_text(b"how are you".to_vec())
                        .with_brightness(Brightness::Dim),
                    Span::empty().with_text(b" ".to_vec()),
                    Span::empty()
                        .with_text(b"good".to_vec())
                        .with_text_style(TextStyle::Strikethrough),
                ],
            },
        ];

        let spans_lines = lines
            .clone()
            .iter()
            .map(|line| line.spans.clone())
            .collect::<Vec<Vec<Span>>>();

        let outputs_iter = json_lines_formatter(lines.clone().into_iter());

        let outputs: Vec<String> = outputs_iter.collect();

        let outputs = outputs.join("");

        let output_spans = sonic_rs::from_str::<Vec<Vec<Span>>>(outputs.as_str())
            .expect("Failed to parse json array");

        assert_eq!(output_spans, spans_lines);
    }
}
