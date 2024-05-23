use std::ffi::OsString;
use std::path::PathBuf;

use get_chunk::ChunkSize;
use get_chunk::iterator::FileIter;

use crate::parse_ansi_text::ansi::ansi_sequence_helpers::{AnsiSequenceType, get_type_from_ansi_sequence};
use crate::parse_ansi_text::ansi::colors::Color;
use crate::parse_ansi_text::ansi::style::{Brightness, TextStyle};
use crate::parse_ansi_text::ansi::types::{Span, SpanJson};
use crate::parse_ansi_text::ansi_text_to_output::str_part_parse::parse_single_ansi;
use crate::parse_ansi_text::parse_options::ParseOptions;
use crate::parse_ansi_text::raw_ansi_parse::{Output, Text};

pub fn tmp_parse(file_path: String, options: ParseOptions) {

    let input_file_path = PathBuf::from(OsString::from(file_path));
    let input_file = std::fs::File::open(input_file_path).expect("opening input file path failed");

    let file_iter = FileIter::try_from(input_file).expect("create input file iterator failed");
    let file_iter = file_iter.set_mode(ChunkSize::Bytes(1024 * 1024 * 10));

    let mut current_location_until_pending_string: usize = 0;
    let mut pending_string: String = "".to_string();

    let mut current_span: Span = options
            .initial_span
            .clone()
            .replace_default_color_with_none();

    let mut yielded_first_item = false;
    print!("[\n");

    for item in file_iter.into_iter() {
        let item = item.expect("Failed to get file chunk");
        let value = String::from_utf8_lossy(item.as_ref());

        // --------- Parse
        pending_string.push_str(value.as_ref());


        let result = parse_single_ansi(pending_string.as_str(), current_location_until_pending_string);
        current_location_until_pending_string = result.current_location_until_pending_string;
        //
        // for output in result.output {
        //     yield item;
        // }

        // ------------ until here parsed

        // ------ Merge
        for output in result.output {
            let ready_output = output;

            let span_result = convert_ansi_output_to_spans(ready_output, &mut current_span);

            match span_result {
                ResultType::Parse(next_span) => {
                    // TODO - do something with current span

                    // let string_to_output = spans_valid_json(current_span, &mut yielded_first_item);

                    // let span_json = SpanJson::create_from_span(&current_span);
                    let span_json = SpanJson {
                        text: current_span.text,

                        // Colors
                        color: SpanJson::get_color_str_from_color(current_span.color),
                        bg_color: SpanJson::get_color_str_from_color(current_span.bg_color),

                        // Brightness
                        bold: current_span.brightness == Brightness::Bold,
                        dim: current_span.brightness == Brightness::Dim,

                        // Text style
                        italic: current_span.text_style & TextStyle::Italic != TextStyle::empty(),
                        underline: current_span.text_style & TextStyle::Underline != TextStyle::empty(),
                        inverse: current_span.text_style & TextStyle::Inverse != TextStyle::empty(),
                        strikethrough: current_span.text_style & TextStyle::Strikethrough != TextStyle::empty(),
                    };

                    // println!("{}", serde_json::to_string(&span_json).unwrap());
                    // simd_json::to_string(&span_json).unwrap();
                    sonic_rs::to_string(&span_json).unwrap();
                    // serde_json::to_string(&span_json).unwrap();
                    // writer.write(",")
                    // serde_json::to_writer(&writer, &span_json).unwrap();
                    // io::stdout().write(b",\n").unwrap();

                    // return str.to_string() + span_json_str.as_str() + "\n";

                    // print!("{}\n", sonic_rs::to_string(&span_json).unwrap());

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

        // ------------ until here merge


        pending_string = result.pending_string;

        // last_pending = result.pending_string.clone();
    }

    if !pending_string.is_empty() {
        let ready_output = Output::TextBlock(Text {
            text: pending_string.as_str(),
            // TODO - this is not right
            location_in_text: 0,
        });

        let span_result = convert_ansi_output_to_spans(ready_output, &mut current_span);

        match span_result {
            ResultType::Parse(_) => {
                // TODO - do something with current span

                // let string_to_output = spans_valid_json(current_span, &mut yielded_first_item);
                // print!("{}", string_to_output);
                let span_json = SpanJson {
                    text: current_span.text,

                    // Colors
                    color: SpanJson::get_color_str_from_color(current_span.color),
                    bg_color: SpanJson::get_color_str_from_color(current_span.bg_color),

                    // Brightness
                    bold: current_span.brightness == Brightness::Bold,
                    dim: current_span.brightness == Brightness::Dim,

                    // Text style
                    italic: current_span.text_style & TextStyle::Italic != TextStyle::empty(),
                    underline: current_span.text_style & TextStyle::Underline != TextStyle::empty(),
                    inverse: current_span.text_style & TextStyle::Inverse != TextStyle::empty(),
                    strikethrough: current_span.text_style & TextStyle::Strikethrough != TextStyle::empty(),
                };

                // writer.write(",")
                // serde_json::to_string(&span_json).unwrap();
                // simd_json::to_string(&span_json).unwrap();
                sonic_rs::to_string(&span_json).unwrap();
                // println!("{}", sonic_rs::to_string(&span_json).unwrap());
            },
            (_) => {}
        }
    }


    print!("]\n");
}

enum ResultType {
    // Span here is the next span to be used
    Parse(Span),
    Skip,
    WaitForNext
}

// TODO - if none than should keep the same current span, otherwise should create a new span and finish with the current
pub fn convert_ansi_output_to_spans<'a>(output: Output<'a>, current_span: &'a mut Span) -> ResultType {
    match output {
        Output::TextBlock(text) => {
            current_span.text.push_str(text.text);
            return ResultType::WaitForNext;
        }
        Output::Escape(seq) => {
            let sequence_type = get_type_from_ansi_sequence(&seq);

            match sequence_type {
                AnsiSequenceType::Unsupported => {
                    return ResultType::WaitForNext;
                }
                AnsiSequenceType::Reset => {
                    // Ignore spans that are just empty text even if they have style as this won't be shown
                    if current_span.text.len() > 0 {
                        return ResultType::Parse(Span::empty());
                    }

                    return ResultType::Skip;
                }
                AnsiSequenceType::ForegroundColor(mut color) => {
                    // Default color is same as none
                    if matches!(color, Color::Default) {
                        color = Color::None;
                    }

                    // TODO - add here that if current color is default or None and new color is default or none don't treat as different
                    if current_span.text.len() > 0 && current_span.color != color {
                        return ResultType::Parse(current_span
                            .clone()
                            .with_text("".to_string())
                            // Apply the color
                            .with_color(color));
                    }

                    current_span.color = color;
                    return ResultType::WaitForNext;
                }
                AnsiSequenceType::BackgroundColor(mut color) => {
                    // Default color is same as none
                    if matches!(color, Color::Default) {
                        color = Color::None;
                    }

                    if current_span.text.len() > 0 && current_span.bg_color != color {
                        return ResultType::Parse(current_span
                            .clone()
                            .with_text("".to_string())
                             // Apply the background color
                             .with_bg_color(color)
                        );

                    }
                    current_span.bg_color = color;
                    return ResultType::WaitForNext;
                }
                AnsiSequenceType::Brightness(brightness) => {
                    if current_span.text.len() > 0 && current_span.brightness != brightness {
                        return ResultType::Parse(current_span
                            .clone()
                            .with_text("".to_string())
                            // Apply the background color
                            .with_brightness(brightness)
                        );
                    }
                    current_span.brightness = brightness;
                    return ResultType::WaitForNext;
                }
                AnsiSequenceType::TextStyle(style) => {
                    if current_span.text.len() > 0 && current_span.text_style != style {
                        return ResultType::Parse(current_span
                            .clone()
                            .with_text("".to_string())
                            // Merge the style
                            .with_text_style(current_span.text_style | style)
                        );
                    }
                    // Merge the style
                    current_span.text_style = current_span.text_style | style;
                    return ResultType::WaitForNext;
                }
            }
        }
    }
}


pub fn spans_valid_json(span: Span, mut yielded_first_item: &bool) -> String {
        // let mut yielded_first_item = false;
        // yield "[\n".to_string();

        // Can replace the loop here with just json line single span, as it's the same thing
        // for await span in input {
            let mut str: &str = "";

            if *yielded_first_item {
                // Print from prev object
                str = ",";
            }


            yielded_first_item = &true;
            let span_json = SpanJson::create_from_span(&span);
            let span_json_str = serde_json::to_string(&span_json).unwrap();

            return str.to_string() + span_json_str.as_str() + "\n";
        // }

        // yield "\n]".to_string();

}
