use genawaiter::sync::{Co, Gen};

use crate::files::file_reader::FileReader;
use crate::parse_ansi_text::ansi::types::Span;
use crate::parse_ansi_text::ansi_output_to_spans::parse_ansi_as_spans::convert_ansi_output_to_spans_continues;
use crate::parse_ansi_text::ansi_output_to_spans::parse_ansi_as_spans::ResultType;
use crate::parse_ansi_text::ansi_text_to_output::str_part_parse::parse_ansi_continues;
use crate::parse_ansi_text::ansi_text_to_output::str_part_parse::ParseAnsiResult;
use crate::parse_ansi_text::raw_ansi_parse::Output;
use crate::parse_ansi_text::raw_ansi_parse::Text;
use crate::parse_file::types::ReadAnsiFileOptions;


// Using the low level API instead of relying on the gen! or producer! macros for better editor support and debugging
async fn read_ansi_file_to_spans_producer(options: ReadAnsiFileOptions, co: Co<Span>) {
    let file_reader = FileReader::new(options.file_options);

    let mut pending_string: Vec<u8> = vec![];

    let mut current_span: Span = options
        .parse_options
        .initial_span
        .clone()
        .replace_default_color_with_none();

    for item in file_reader {
        let mut value = item;

        if pending_string.is_empty() {
            pending_string = value;
        } else {
            pending_string.append(value.as_mut());
        }

        let mut pending = pending_string.as_slice();
        let mut result: ParseAnsiResult = parse_ansi_continues(pending);

        while let Some(ready_output) = result.output {
            let span_result =
                convert_ansi_output_to_spans_continues(ready_output, &mut current_span);

            match span_result {
                ResultType::Parse(next_span) => {
                    co.yield_(current_span).await;

                    current_span = next_span;
                }
                ResultType::Skip => {
                    current_span = Span::empty();
                }
                ResultType::WaitForNext => {
                    // Do nothing with the current span
                }
            }

            pending = result.pending_string;
            result = parse_ansi_continues(pending);
        }

        pending_string = result.pending_string.to_vec();
    }

    // Add last span if it has text
    if !current_span.text.is_empty() {
        let ready_output = Output::TextBlock(Text {
            text: pending_string.as_slice(),
        });

        convert_ansi_output_to_spans_continues(ready_output, &mut current_span);

        co.yield_(current_span).await;
    }
}

pub fn read_ansi_file_to_spans(options: ReadAnsiFileOptions) -> impl Iterator<Item=Span> {
    return Gen::new(|co| read_ansi_file_to_spans_producer(options, co)).into_iter();
}
