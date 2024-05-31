use std::iter::Iterator;

use crate::types::Line;
use genawaiter::{rc::gen, yield_};

pub fn json_lines_single_item_formatter<I: Iterator<Item = Line>>(
    iter: I,
) -> impl Iterator<Item = String> {
    return gen!({
        for line in iter {
            yield_!(sonic_rs::to_string(&line.spans).unwrap());
        }
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
    fn test_formatter_each_item_is_valid_json_line() {
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

        let outputs_iter = json_lines_single_item_formatter(lines.clone().into_iter());

        let spans_lines = lines
            .clone()
            .iter()
            .map(|line| line.spans.clone())
            .collect::<Vec<Vec<Span>>>();

        let outputs: Vec<String> = outputs_iter.collect();

        // parse each item
        let mut i = 0;
        for output in outputs.iter() {
            let output_spans = sonic_rs::from_str::<Vec<Span>>(output.as_str())
                .expect("Failed to parse json array");

            assert_eq!(output_spans, spans_lines[i]);

            i += 1;
        }

        assert_eq!(i, spans_lines.len());
    }
}
