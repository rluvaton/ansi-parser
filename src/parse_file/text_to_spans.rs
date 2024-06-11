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
async fn buffer_to_spans_producer(mut buffer: &[u8], co: Co<Span>) {

    let mut current_span: Span = Span::empty();

    let mut result: ParseAnsiResult = parse_ansi_continues(buffer);

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

        buffer = result.pending_string;
        result = parse_ansi_continues(buffer);
    }

    // Add last span if it has text
    if !current_span.text.is_empty() {
        let ready_output = Output::TextBlock(Text {
            text: result.pending_string,
        });

        convert_ansi_output_to_spans_continues(ready_output, &mut current_span);

        co.yield_(current_span).await;
    }
}

pub fn buffer_to_spans(buffer: &[u8]) -> impl Iterator<Item=Span> + '_ {
    return Gen::new(|co| buffer_to_spans_producer(buffer, co)).into_iter();
}
