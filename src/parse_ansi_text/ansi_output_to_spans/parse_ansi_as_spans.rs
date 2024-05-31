use std::iter::Iterator;

use genawaiter::{rc::gen, yield_};

use crate::parse_ansi_text::ansi::ansi_sequence_helpers::{
    get_type_from_ansi_sequence, AnsiSequenceType,
};
use crate::parse_ansi_text::ansi::colors::Color;
use crate::parse_ansi_text::ansi::types::Span;
use crate::parse_ansi_text::parse_options::ParseOptions;
use crate::parse_ansi_text::raw_ansi_parse::Output;

pub enum ResultType {
    // Span here is the next span to be used
    Parse(Span),
    Skip,
    WaitForNext,
}

pub fn convert_ansi_output_to_spans_continues<'a>(
    output: Output<'a>,
    current_span: &'a mut Span,
) -> ResultType {
    return match output {
        Output::TextBlock(text) => {
            current_span.text.append(text.text.to_vec().as_mut());
            ResultType::WaitForNext
        }
        Output::Escape(seq) => {
            let sequence_type = get_type_from_ansi_sequence(&seq);

            match sequence_type {
                AnsiSequenceType::Unsupported => ResultType::WaitForNext,
                AnsiSequenceType::Reset => {
                    // Ignore spans that are just empty text even if they have style as this won't be shown
                    if current_span.text.len() > 0 {
                        return ResultType::Parse(Span::empty());
                    }

                    ResultType::Skip
                }
                AnsiSequenceType::ForegroundColor(mut color) => {
                    // Default color is same as none
                    if matches!(color, Color::Default) {
                        color = Color::None;
                    }

                    // TODO - add here that if current color is default or None and new color is default or none don't treat as different
                    if current_span.text.len() > 0 && current_span.color != color {
                        return ResultType::Parse(
                            current_span
                                .clone()
                                .with_text(vec![])
                                // Apply the color
                                .with_color(color),
                        );
                    }

                    current_span.color = color;
                    ResultType::WaitForNext
                }
                AnsiSequenceType::BackgroundColor(mut color) => {
                    // Default color is same as none
                    if matches!(color, Color::Default) {
                        color = Color::None;
                    }

                    if current_span.text.len() > 0 && current_span.bg_color != color {
                        return ResultType::Parse(
                            current_span
                                .clone()
                                .with_text(vec![])
                                // Apply the background color
                                .with_bg_color(color),
                        );
                    }
                    current_span.bg_color = color;
                    ResultType::WaitForNext
                }
                AnsiSequenceType::Brightness(brightness) => {
                    if current_span.text.len() > 0 && current_span.brightness != brightness {
                        return ResultType::Parse(
                            current_span
                                .clone()
                                .with_text(vec![])
                                // Apply the background color
                                .with_brightness(brightness),
                        );
                    }
                    current_span.brightness = brightness;
                    ResultType::WaitForNext
                }
                AnsiSequenceType::TextStyle(style) => {
                    if current_span.text.len() > 0 && current_span.text_style != style {
                        return ResultType::Parse(
                            current_span
                                .clone()
                                .with_text(vec![])
                                // Merge the style
                                .with_text_style(current_span.text_style | style),
                        );
                    }
                    // Merge the style
                    current_span.text_style = current_span.text_style | style;
                    ResultType::WaitForNext
                }
            }
        }
    };
}

#[allow(unused_variables, unused_mut)]
pub fn convert_ansi_output_to_spans<'a, I: Iterator<Item = Output<'a>>>(
    mut input: I,
    options: ParseOptions,
) -> impl Iterator<Item = Span> {
    #[allow(unused_variables, unused_mut)]
    let mut current_span: Span = options
        .initial_span
        .clone()
        .replace_default_color_with_none();

    #[allow(clippy::needless_return)]
    return gen!({
        let mut current_span: Span = options
            .initial_span
            .clone()
            .replace_default_color_with_none();

        for output in input {
            let span_result = convert_ansi_output_to_spans_continues(output, &mut current_span);

            match span_result {
                ResultType::Parse(next_span) => {
                    yield_!(current_span);

                    current_span = next_span;
                }
                ResultType::Skip => {
                    current_span = Span::empty();
                }
                ResultType::WaitForNext => {
                    // Do nothing with the current span
                }
            }
        }

        // Add last span if it has text
        if current_span.text.len() > 0 {
            yield_!(current_span);
        }
    })
    .into_iter();
}

#[cfg(test)]
mod tests {
    use crate::iterators::compose::ComposeByIterator;
    use crate::parse_ansi_text::ansi::colors::*;
    use crate::parse_ansi_text::ansi::constants::*;
    use crate::parse_ansi_text::ansi::types::Span;
    use crate::parse_ansi_text::ansi_text_to_output::parse::parse_ansi;
    use crate::test_utils::chars_iterator;

    use super::*;

    #[test]
    fn should_parse_chars_iterator_correctly() {
        let input_str = vec![
            RED_BACKGROUND_CODE.to_string(),
            "Hello, World!".to_string(),
            RESET_CODE.to_string(),
        ]
        .join("");

        let output: Vec<Span> = chars_iterator(input_str.clone())
            .compose(parse_ansi)
            .compose(|iter| convert_ansi_output_to_spans(iter, ParseOptions::default()))
            .collect();

        let expected = vec![Span::empty()
            .with_text(b"Hello, World!".to_vec())
            .with_bg_color(Color::Red)];
        assert_eq!(output, expected);
    }

    #[test]
    fn should_be_available_as_iterator() {
        let input_str = [RED_BACKGROUND_CODE, "Hello, World!", RESET_CODE].join("");

        let output: Vec<Span> = vec![input_str.as_bytes().to_vec()]
            .into_iter()
            .compose(parse_ansi)
            .compose(|iter| convert_ansi_output_to_spans(iter, ParseOptions::default()))
            .collect();

        let expected = vec![Span::empty()
            .with_text("Hello, World!".to_string().as_bytes().to_vec())
            .with_bg_color(Color::Red)];

        assert_eq!(output, expected);
    }
}
