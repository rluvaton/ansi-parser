use std::ops::Deref;
use std::str;

use nom::AsBytes;

use crate::files::file_reader::{FileReader, FileReaderOptions};
use crate::parse_ansi_text::ansi::ansi_sequence_helpers::{AnsiSequenceType, get_type_from_ansi_sequence};
use crate::parse_ansi_text::ansi::colors::Color;
use crate::parse_ansi_text::ansi::style::{Brightness, TextStyle};
use crate::parse_ansi_text::ansi::types::{Span};
use crate::parse_ansi_text::ansi_text_to_output::str_part_parse::parse_single_ansi;
use crate::parse_ansi_text::parse_options::ParseOptions;
use crate::parse_ansi_text::raw_ansi_parse::{Output, Text};
use crate::traits::ToJson;

pub fn tmp_parse(file_path: String, options: ParseOptions) {

    let file_reader = FileReader::new(FileReaderOptions {
        file_path: file_path.clone(),
        chunk_size_in_bytes: Some(1024 * 1024 * 10), // 10MB
        
        // Start from location
        from_bytes: None,
        to_bytes: None,
    });
    
    let mut current_location_until_pending_string: usize = 0;
    let mut pending_string: Vec<u8> = vec![];

    let mut current_span: Span = options
            .initial_span
            .clone()
            .replace_default_color_with_none();

    let mut yielded_first_item = false;
    print!("[\n");

    for item in file_reader {
        let value = item;

        // --------- Parse
        pending_string = [pending_string, value].concat();
        // pending_string.push_str(value.as_ref());


        let result = parse_single_ansi(pending_string.as_bytes(), current_location_until_pending_string);
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
                    let text = current_span.to_json();

                    // println!("{}", serde_json::to_string(&span_json).unwrap());
                    // simd_json::to_string(&span_json).unwrap();
                    // serde_json::to_string(&span_json).unwrap();
                    // writer.write(",")
                    // serde_json::to_writer(&writer, &span_json).unwrap();
                    // io::stdout().write(b",\n").unwrap();

                    // return str.to_string() + span_json_str.as_str() + "\n";

                    // print!("{}\n", current_span.to_json());

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


        pending_string = result.pending_string.to_vec();

        // last_pending = result.pending_string.clone();
    }

    if !pending_string.is_empty() {
        let ready_output = Output::TextBlock(Text {
            text: pending_string.as_bytes(),
            // TODO - this is not right
            location_in_text: 0,
        });

        let span_result = convert_ansi_output_to_spans(ready_output, &mut current_span);

        match span_result {
            ResultType::Parse(_) => {

                // let text = String::from_utf8_lossy(current_span.text.deref());
                // TODO - do something with current span

                // let string_to_output = spans_valid_json(current_span, &mut yielded_first_item);
                // print!("{}", string_to_output);
                // writer.write(",")
                // serde_json::to_string(&span_json).unwrap();
                // simd_json::to_string(&span_json).unwrap();
                current_span.to_json();
                // println!("{}", current_span.to_json());
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
            current_span.text = [current_span.text.to_vec(), text.text.to_vec()].concat();
            // current_span.text.push_str(text.text);
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
                            .with_text(vec![])
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
                            .with_text(vec![])
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
                            .with_text(vec![])
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
                            .with_text(vec![])
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

            return str.to_string() + span.to_json().as_str() + "\n";
        // }

        // yield "\n]".to_string();

}
