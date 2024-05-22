use crate::parse_ansi_text::ansi::types::{Span, SpanJson};
use async_stream::stream;
use futures_core::Stream;
use std::iter::Iterator;

pub struct SpansJsonLineDisplay<IteratorType> {
    iter: IteratorType,
    yielded_first_item: bool,
}

impl<IteratorType> Iterator for SpansJsonLineDisplay<IteratorType>
where
    IteratorType: Iterator<Item = Span>,
{
    // Output item
    type Item = String;

    // https://users.rust-lang.org/t/how-to-write-iterator-adapter/8835/2
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
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

        return None;
    }
}

pub async fn spans_json_line<S: Stream<Item = Span>>(input: S) -> impl Stream<Item = String> {
    stream! {
        // Can replace the loop here with just json line single span, as it's the same thing
        for await span in input {
            let span_json = SpanJson::create_from_span(&span);
            let span_json_str = serde_json::to_string(&span_json).unwrap();

            yield span_json_str;
        }
    }
}

impl<IteratorType> SpansJsonLineDisplay<IteratorType> {
    pub fn new(iter: IteratorType) -> Self {
        Self {
            iter,
            yielded_first_item: false,
        }
    }
}

pub trait SpansJsonLineDisplayByIterator: Iterator<Item = Span> + Sized {
    fn to_span_json_line(self) -> SpansJsonLineDisplay<Self> {
        SpansJsonLineDisplay::new(self)
    }
}

impl<IteratorType: Iterator<Item = Span>> SpansJsonLineDisplayByIterator for IteratorType {}
