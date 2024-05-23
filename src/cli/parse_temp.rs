use std::ffi::OsString;
use std::path::PathBuf;

use get_chunk::iterator::FileIter;

use crate::parse_ansi_text::ansi::ansi_sequence_helpers::{AnsiSequenceType, get_type_from_ansi_sequence};
use crate::parse_ansi_text::ansi::colors::Color;
use crate::parse_ansi_text::ansi::types::{Span, SpanJson};
use crate::parse_ansi_text::ansi_text_to_output::str_part_parse::parse_single_ansi;
use crate::parse_ansi_text::parse_options::ParseOptions;
use crate::parse_ansi_text::raw_ansi_parse::{Output, Text};
use crate::parse_ansi_text::raw_ansi_parse::output::TextWithString;

pub fn tmp_parse(file_path: String, options: ParseOptions) {

    let input_file_path = PathBuf::from(OsString::from(file_path));
    let input_file = std::fs::File::open(input_file_path).expect("opening input file path failed");

    let file_iter = FileIter::try_from(input_file).expect("create input file iterator failed");

    let mut current_location_until_pending_string: usize = 0;
    let mut pending_string: String = "".to_string();

    let mut current_span: Span = options
            .initial_span
            .clone()
            .replace_default_color_with_none();

    let mut yielded_first_item = false;
    print!("[\n");
    let mut last_vec: Vec<TextWithString> = Vec::new();
    
    for item in file_iter.into_iter() {
        let mut text_blocks_vec: Vec<Text> = Vec::new();
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
            match output {
                Output::TextBlock(txt) => {
                    text_blocks_vec.push(txt);
                },
                _ => {
                    if !text_blocks_vec.as_slice().is_empty() {
                        let joined = text_blocks_vec.iter().map(|x| x.text).collect::<String>();
                        let ready_output = Output::TextBlock(Text {
                            text: joined.as_str(),
                            location_in_text: text_blocks_vec.first().unwrap().location_in_text,
                        });

                        let span_result = convert_ansi_output_to_spans(ready_output, &mut current_span);
                        
                        match span_result {
                            ResultType::Parse(nextSpan) => {
                                // TODO - do something with current span
                                let string_to_output = spans_valid_json(current_span, &mut yielded_first_item);
                                print!("{}", string_to_output);
                                
                                current_span = nextSpan;
                            }
                            ResultType::Skip => {
                                current_span = Span::empty();
                            }
                            ResultType::WaitForNext => {
                                // Do nothing with the current span
                            }
                        }

                        text_blocks_vec = vec![];
                        // text_blocks_vec.clear();
                        // text_blocks_vec.shrink_to_fit();
                    }
                    let ready_output = output;

                    let span_result = convert_ansi_output_to_spans(ready_output, &mut current_span);

                    match span_result {
                        ResultType::Parse(nextSpan) => {
                            // TODO - do something with current span

                            let string_to_output = spans_valid_json(current_span, &mut yielded_first_item);
                            print!("{}", string_to_output);
                            
                            current_span = nextSpan;
                        }
                        ResultType::Skip => {
                            current_span = Span::empty();
                        }
                        ResultType::WaitForNext => {
                            // Do nothing with the current span
                        }
                    }
                }

            }
        }

        // ------------ until here merge

        last_vec = Vec::with_capacity(text_blocks_vec.len());
        if !last_vec.is_empty() {
            let joined = last_vec.iter().map(|x| x.text.as_str()).collect::<String>();
            last_vec.push(TextWithString {
                text: joined,
                location_in_text: last_vec.first().unwrap().location_in_text,
            });
        }
        
        pending_string = result.pending_string;

        // last_pending = result.pending_string.clone();
    }
    
    if !pending_string.is_empty() {

        last_vec.push(TextWithString {
            text: pending_string,
            location_in_text: current_location_until_pending_string,
        });
    }
    
    
    
    if !last_vec.is_empty() {
        let joined = last_vec.iter().map(|x| x.text.as_str()).collect::<String>();
        let ready_output = Output::TextBlock(Text {
            text: joined.as_str(),
            location_in_text: last_vec.first().unwrap().location_in_text,
        });
    
        let span_result = convert_ansi_output_to_spans(ready_output, &mut current_span);
    
        match span_result {
            ResultType::Parse(nextSpan) => {
                // TODO - do something with current span
    
                let string_to_output = spans_valid_json(current_span, &mut yielded_first_item);
                print!("{}", string_to_output);
                
                current_span = nextSpan;
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
        // TODO - do something with current_span
        let string_to_output = spans_valid_json(current_span, &mut yielded_first_item);
        print!("{}", string_to_output);
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