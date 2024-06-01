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


#[cfg(test)]
mod tests {
    use crate::iterators::compose::ComposeByIterator;
    use crate::parse_ansi_text::ansi::colors::*;
    use crate::parse_ansi_text::ansi::constants::*;
    use crate::parse_ansi_text::ansi::types::Span;
    use crate::test_utils::chars_iterator;

    use super::*;

    // TODO - add tests
}
