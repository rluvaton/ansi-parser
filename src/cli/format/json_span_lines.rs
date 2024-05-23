// Span consumer iterator that get a span and just print it, making sure the entire iterator is output correctly in JSON

// this means that should support both Vec<Span> and Vec<Vec<Span>>

use std::iter::Iterator;

use async_stream::stream;
use futures_core::Stream;
use futures_util::stream;

use crate::cli::format::json_single_span::{spans_valid_json, SpansJsonDisplay};
use crate::parse_ansi_text::ansi_output_to_spans::parse_ansi_split_by_lines_as_spans::Line;

pub struct SpansLineJsonDisplay<'a, IteratorType> {
    iter: IteratorType,
    line_iter: Option<Box<dyn Iterator<Item = String> + 'a>>,
    yielded_opening: bool,
    yielded_closing: bool,
    yielded_first_item: bool,
}

impl<'a, IteratorType> Iterator for SpansLineJsonDisplay<'a, IteratorType>
where
    IteratorType: Iterator<Item = Line>,
{
    // Output item
    type Item = String;

    // https://users.rust-lang.org/t/how-to-write-iterator-adapter/8835/2
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if !self.yielded_opening {
            self.yielded_opening = true;

            return Some("[\n".to_string());
        }

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

        if !self.yielded_closing {
            self.yielded_closing = true;

            return Some("\n]".to_string());
        }

        return None;
    }
}

pub async fn spans_lines_valid_json<S: Stream<Item = Line>>(
    input: S,
) -> impl Stream<Item = String> {
    stream! {
        let mut yielded_first_item = false;
        yield "[\n".to_string();

        for await line in input {
            let mut str: &str = "";

            if yielded_first_item {
                // Print from prev object
                str = ",";
            }


            yielded_first_item = true;

            let line_iter = spans_valid_json(stream::iter(line.spans));
            for await span_str in line_iter.await {
                yield str.to_string() + span_str.as_str();

                str = "";
            }
        }

        yield "\n]".to_string();
    }
}

impl<'a, IteratorType> SpansLineJsonDisplay<'a, IteratorType> {
    pub fn new(iter: IteratorType) -> Self {
        Self {
            iter,
            line_iter: None,
            yielded_opening: false,
            yielded_closing: false,
            yielded_first_item: false,
        }
    }
}

pub trait SpansLineJsonDisplayByIterator: Iterator<Item = Line> + Sized {
    fn to_json_string_in_span_lines<'a>(self) -> SpansLineJsonDisplay<'a, Self> {
        SpansLineJsonDisplay::new(self)
    }
}

impl<IteratorType: Iterator<Item = Line>> SpansLineJsonDisplayByIterator for IteratorType {}
