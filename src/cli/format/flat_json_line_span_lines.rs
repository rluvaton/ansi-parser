// Span consumer iterator that get a span and just print it, making sure the entire iterator is output correctly in JSON 

// this means that should support both Vec<Span> and Vec<Vec<Span>>

use std::iter::Iterator;
use async_stream::stream;
use futures_core::Stream;

use crate::parse_ansi_text::ansi_output_to_spans::parse_ansi_split_by_lines_as_spans::Line;
use crate::traits::ToJson;

pub async fn spans_lines_flat_json_lines<S: Stream<Item = Line>>(input: S) -> impl Stream<Item = String> {
    stream! {
        for await line in input {
            for span in line.spans.iter() {
                yield span.to_json();
            }
            yield "{ \"type\": \"new line\" }".to_string();
        }
    }
}
