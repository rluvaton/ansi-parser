use genawaiter::{rc::gen, yield_};

use crate::files::file_reader::FileReader;
use crate::parse_ansi_text::ansi::types::Span;
use crate::parse_ansi_text::ansi_output_to_spans::parse_ansi_as_line_of_spans::{
    convert_ansi_output_lines_of_spans_continues, ResultType,
};
use crate::parse_ansi_text::ansi_text_to_output::str_part_parse::parse_ansi_continues;
use crate::parse_ansi_text::ansi_text_to_output::str_part_parse::ParseAnsiResult;
use crate::parse_ansi_text::raw_ansi_parse::Output;
use crate::parse_ansi_text::raw_ansi_parse::Text;
use crate::parse_file::types::ReadAnsiFileOptions;
use crate::types::Line;

pub fn read_ansi_file_to_lines(options: ReadAnsiFileOptions) -> impl Iterator<Item = Line> {
    let file_reader = FileReader::new(options.file_options);

    let mut current_location_until_pending_string: usize = 0;

    let current_span: Span = options
        .parse_options
        .initial_span
        .clone()
        .replace_default_color_with_none();
    let mut current_line = Line {
        location_in_file: 0,
        spans: vec![current_span],
    };
    let mut pending_string: Vec<u8> = vec![];

    #[allow(clippy::needless_return)]
    return gen!({
        for item in file_reader {
            let mut value = item;

            if pending_string.is_empty() {
                pending_string = value;
            } else {
                pending_string.append(value.as_mut());
            }

            let mut pending = pending_string.as_slice();
            let mut result: ParseAnsiResult =
                parse_ansi_continues(pending, current_location_until_pending_string);
            current_location_until_pending_string = result.current_location_until_pending_string;

            while let Some(ready_output) = result.output {
                let mut lines_result = convert_ansi_output_lines_of_spans_continues(
                    Some(ready_output),
                    &mut current_line,
                );

                while let ResultType::Parse(next_line) = lines_result {
                    yield_!(current_line);

                    current_line = next_line;

                    lines_result =
                        convert_ansi_output_lines_of_spans_continues(None, &mut current_line);
                }

                pending = result.pending_string;
                result = parse_ansi_continues(pending, current_location_until_pending_string);
                current_location_until_pending_string =
                    result.current_location_until_pending_string;
            }

            pending_string = result.pending_string.to_vec();
        }

        let ready_output = Output::TextBlock(Text {
            text: pending_string.as_slice(),
            // TODO - this may not be right
            location_in_text: current_location_until_pending_string,
        });

        let mut lines_result =
            convert_ansi_output_lines_of_spans_continues(Some(ready_output), &mut current_line);

        while let ResultType::Parse(next_line) = lines_result {
            yield_!(current_line);

            current_line = next_line;

            lines_result = convert_ansi_output_lines_of_spans_continues(None, &mut current_line);
        }

        let last_span = current_line.spans.last();

        if let Some(last_span) = last_span {
            if last_span.text.is_empty() {
                current_line.spans.pop();
            }
        }

        // Yielding the last ;ome
        yield_!(current_line);
    })
    .into_iter();
}
