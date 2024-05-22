// Span consumer iterator that get a span and just print it, making sure the entire iterator is output correctly in JSON

// this means that should support both Vec<Span> and Vec<Vec<Span>>

use crate::parse_ansi_text::ansi::types::{Span, SpanJson};
use async_stream::stream;
use futures_core::Stream;
use std::iter::Iterator;

pub struct SpansJsonDisplay<IteratorType> {
    iter: IteratorType,
    yielded_opening: bool,
    yielded_closing: bool,
    yielded_first_item: bool,
}

impl<IteratorType> Iterator for SpansJsonDisplay<IteratorType>
where
    IteratorType: Iterator<Item = Span>,
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

        // Can replace the loop here with just json line single span, as it's the same thing

        while let Some(span) = self.iter.next() {
            let mut str: &str = "";

            if self.yielded_first_item {
                // Print from prev object
                str = ",";
            }

            self.yielded_first_item = true;
            let span_json = SpanJson::create_from_span(&span);
            let span_json_str = serde_json::to_string(&span_json).unwrap();

            return Some(str.to_string() + span_json_str.as_str());
        }

        if !self.yielded_closing {
            self.yielded_closing = true;

            return Some("\n]".to_string());
        }

        return None;
    }
}

pub async fn spans_valid_json<S: Stream<Item = Span>>(input: S) -> impl Stream<Item = String> {
    stream! {
        let mut yielded_first_item = false;
        yield "[\n".to_string();

        // Can replace the loop here with just json line single span, as it's the same thing
        for await span in input {
            let mut str: &str = "";

            if yielded_first_item {
                // Print from prev object
                str = ",";
            }


            yielded_first_item = true;
            let span_json = SpanJson::create_from_span(&span);
            let span_json_str = serde_json::to_string(&span_json).unwrap();

            yield str.to_string() + span_json_str.as_str();
        }

        yield "\n]".to_string();

    }
}

impl<IteratorType> SpansJsonDisplay<IteratorType> {
    pub fn new(iter: IteratorType) -> Self {
        Self {
            iter,
            yielded_opening: false,
            yielded_closing: false,
            yielded_first_item: false,
        }
    }
}

pub trait SpansJsonDisplayByIterator: Iterator<Item = Span> + Sized {
    fn to_span_json(self) -> SpansJsonDisplay<Self> {
        SpansJsonDisplay::new(self)
    }
}

impl<IteratorType: Iterator<Item = Span>> SpansJsonDisplayByIterator for IteratorType {}
