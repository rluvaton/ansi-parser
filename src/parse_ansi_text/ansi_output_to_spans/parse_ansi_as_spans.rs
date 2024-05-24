use std::iter::Iterator;

use async_stream::stream;
use tokio_stream::Stream;

use crate::parse_ansi_text::ansi::ansi_sequence_helpers::{AnsiSequenceType, get_type_from_ansi_sequence};
use crate::parse_ansi_text::ansi::colors::Color;
use crate::parse_ansi_text::ansi::types::Span;
use crate::parse_ansi_text::parse_options::ParseOptions;
use crate::parse_ansi_text::raw_ansi_parse::Output;

pub async fn convert_ansi_output_to_spans<'a, S: Stream<Item = Output<'a>>>(input: S, options: ParseOptions) -> impl Stream<Item = Span> {
    stream! {
        let mut current_span: Span = options
                .initial_span
                .clone()
                .replace_default_color_with_none();

        for await output in input {
            match output {
                Output::TextBlock(text) => {
                    current_span.text = [current_span.text, text.text.to_vec()].concat();
                }
                Output::Escape(seq) => {
                    let sequence_type = get_type_from_ansi_sequence(&seq);

                    match sequence_type {
                        AnsiSequenceType::Unsupported => {
                            continue;
                        }
                        AnsiSequenceType::Reset => {
                            // Ignore spans that are just empty text even if they have style as this won't be shown
                            if current_span.text.len() > 0 {
                                let span = current_span.clone();
                                current_span = Span::empty();
                                yield span;
                                continue;
                            }

                            current_span = Span::empty();
                        }
                        AnsiSequenceType::ForegroundColor(mut color) => {
                            // Default color is same as none
                            if matches!(color, Color::Default) {
                                color = Color::None;
                            }

                            // TODO - add here that if current color is default or None and new color is default or none don't treat as different
                            if current_span.text.len() > 0 && current_span.color != color
                            {
                                let span = current_span.clone();
                                current_span = current_span
                                    .clone()
                                    .with_text(vec![])
                                    // Apply the color
                                    .with_color(color);

                                yield span;
                                continue;
                            }

                            current_span.color = color;
                        }
                        AnsiSequenceType::BackgroundColor(mut color) => {
                            // Default color is same as none
                            if matches!(color, Color::Default) {
                                color = Color::None;
                            }

                            if current_span.text.len() > 0
                                && current_span.bg_color != color
                            {
                                let span = current_span.clone();
                                current_span = current_span
                                    .clone()
                                    .with_text(vec![])
                                    // Apply the background color
                                    .with_bg_color(color);

                                yield span;
                                continue;
                            }
                            current_span.bg_color = color;
                        }
                        AnsiSequenceType::Brightness(brightness) => {
                            if current_span.text.len() > 0
                                && current_span.brightness != brightness
                            {
                                let span = current_span.clone();
                                current_span = current_span
                                    .clone()
                                    .with_text(vec![])
                                    // Apply the background color
                                    .with_brightness(brightness);

                                yield span;
                                continue;
                            }
                            current_span.brightness = brightness;
                        }
                        AnsiSequenceType::TextStyle(style) => {
                            if current_span.text.len() > 0
                                && current_span.text_style != style
                            {
                                let span = current_span.clone();
                                current_span = current_span
                                    .clone()
                                    .with_text(vec![])
                                    // Merge the style
                                    .with_text_style(current_span.text_style | style);

                                yield span;
                                continue;
                            }
                            // Merge the style
                            current_span.text_style = current_span.text_style | style;
                        }
                    }
                }
            }
        }

        // Add last span if it has text
        if current_span.text.len() > 0 {
            yield current_span;
        }
    }
}

#[cfg(test)]
mod tests {
    use futures::stream::StreamExt;
    use pretty_assertions::assert_eq;

    use crate::compose_async_steams;
    use crate::parse_ansi_text::ansi::colors::*;
    use crate::parse_ansi_text::ansi::constants::*;
    use crate::parse_ansi_text::ansi::types::*;
    use crate::parse_ansi_text::ansi_text_to_output::helpers::merge_text_output;
    use crate::parse_ansi_text::ansi_text_to_output::stream_parse::*;
    use crate::test_utils::*;

    use super::*;

    #[tokio::test]
    async fn stream_should_parse_chars_iterator_correctly() {
        let input_str = vec![
            RED_BACKGROUND_CODE.to_string(),
            "Hello, World!".to_string(),
            RESET_CODE.to_string(),
        ]
            .join("");

        let output: Vec<Span> = compose_async_steams!(
            || async_chars_stream(input_str.clone()),
            parse_ansi,
            // TODO
            // merge_text_output,
            |output| convert_ansi_output_to_spans(output, ParseOptions::default())
        ).await.collect::<Vec<Span>>().await;
        
        let expected = vec![Span::empty()
            .with_text("Hello, World!".to_string().as_bytes().to_vec())
            .with_bg_color(Color::Red)];
        assert_eq!(output, expected);
    }

    #[tokio::test]
    async fn stream_should_be_available_as_iterator() {
        let input_str = [RED_BACKGROUND_CODE, "Hello, World!", RESET_CODE].join("");

        let output: Vec<Span> = compose_async_steams!(
            || async_stream_from_vector(vec![input_str.as_bytes().to_vec()]),
            parse_ansi,
            // TODO
            // merge_text_output,
            |output| convert_ansi_output_to_spans(output, ParseOptions::default())
        ).await.collect::<Vec<Span>>().await;

        let expected = vec![Span::empty()
            .with_text("Hello, World!".to_string().as_bytes().to_vec())
            .with_bg_color(Color::Red)];
        assert_eq!(output, expected);
    }
}
