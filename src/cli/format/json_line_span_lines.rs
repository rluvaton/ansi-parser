// Span consumer iterator that get a span and just print it, making sure the entire iterator is output correctly in JSON

// this means that should support both Vec<Span> and Vec<Vec<Span>>

use async_stream::stream;
use futures_core::Stream;
use std::iter::Iterator;

use crate::cli::format::json_single_span::SpansJsonDisplay;
use crate::parse_ansi_text::ansi::types::SpanJson;
use crate::parse_ansi_text::iterators::parse_ansi_split_by_lines_as_spans_iterator::Line;

pub struct SpansLineJsonLineDisplay<'a, IteratorType> {
    iter: IteratorType,
    line_iter: Option<Box<dyn Iterator<Item = String> + 'a>>,
    yielded_first_item: bool,
}

impl<'a, IteratorType> Iterator for SpansLineJsonLineDisplay<'a, IteratorType>
where
    IteratorType: Iterator<Item = Line>,
{
    // Output item
    type Item = String;

    // https://users.rust-lang.org/t/how-to-write-iterator-adapter/8835/2
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.line_iter.is_some() {
            let line_iter = self.line_iter.as_mut().unwrap();
            while let Some(str) = line_iter.next() {
                return Some(str);
            }

            self.line_iter = None;
        }

        while let Some(line) = self.iter.next() {
            let mut str: &str = "";

            if self.yielded_first_item {
                // Print from prev object
                str = ",";
            }

            self.yielded_first_item = true;
            self.line_iter = Some(Box::new(SpansJsonDisplay::new(line.spans.into_iter())));

            let line_iter = self.line_iter.as_mut().unwrap().next();

            // line iterator should not be empty

            if line_iter.is_some() {
                let line_iter = line_iter.unwrap();
                return Some(str.to_string() + line_iter.as_str());
            }
        }

        return None;
    }
}
pub async fn spans_lines_json_lines<S: Stream<Item = Line>>(
    input: S,
) -> impl Stream<Item = String> {
    stream! {
        for await line in input {
            let str = line.spans.iter().map(|span| {
                let span_json = SpanJson::create_from_span(&span);
                let span_json_str = serde_json::to_string(&span_json).unwrap();
                span_json_str
            }).collect::<Vec<String>>().join(",");

            yield "[".to_string() + str.as_str() + "]";
        }
    }
}

impl<'a, IteratorType> SpansLineJsonLineDisplay<'a, IteratorType> {
    pub fn new(iter: IteratorType) -> Self {
        Self {
            iter,
            line_iter: None,
            yielded_first_item: false,
        }
    }
}

pub trait SpansLineJsonLineDisplayByIterator: Iterator<Item = Line> + Sized {
    fn to_json_line_string_in_span_lines<'a>(self) -> SpansLineJsonLineDisplay<'a, Self> {
        SpansLineJsonLineDisplay::new(self)
    }
}

impl<IteratorType: Iterator<Item = Line>> SpansLineJsonLineDisplayByIterator for IteratorType {}
