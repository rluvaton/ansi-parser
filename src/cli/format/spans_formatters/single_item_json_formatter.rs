use std::iter::Iterator;
use async_stream::stream;
use futures_core::Stream;
use genawaiter::{rc::gen, yield_};
use crate::parse_ansi_text::ansi::types::{Span};
use crate::traits::ToJson;

pub struct SpansJsonLineDisplay<IteratorType> {
    iter: IteratorType,
    yielded_first_item: bool
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

            return Some(str.to_string() + span.to_json().as_str());
        }

        return None;
    }
}

pub async fn spans_json_line<S: Stream<Item = Span>>(input: S) -> impl Stream<Item = String> {
    stream! {
        // Can replace the loop here with just json line single span, as it's the same thing
        for await span in input {
            yield span.to_json();
        }
    }
}

impl<IteratorType> SpansJsonLineDisplay<IteratorType> {
    pub fn new(iter: IteratorType) -> Self {
        Self { iter, yielded_first_item: false }
    }
}

pub trait SpansJsonLineDisplayByIterator: Iterator<Item = Span> + Sized {
    fn to_span_json_line(self) -> SpansJsonLineDisplay<Self> {
        SpansJsonLineDisplay::new(self)
    }
}

impl<IteratorType: Iterator<Item = Span>> SpansJsonLineDisplayByIterator for IteratorType {}

pub fn json_single_item_formatter<I: Iterator<Item=Span>>(iter: I) -> impl Iterator<Item = String> {
    return gen!({
        for span in iter {
            yield_!(span.to_json());
        }
    }).into_iter();
    
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use crate::parse_ansi_text::ansi::style::{Brightness, TextStyle};

    use super::*;

    #[test]
    fn test_formatter_each_item_is_valid_json() {
        let spans: Vec<Span> = vec![
            Span::empty()
                .with_text("Hello, World!".as_bytes().to_vec())
                .with_brightness(Brightness::Bold),
            Span::empty()
                .with_text(" ".as_bytes().to_vec()),
            Span::empty()
                .with_text("This is another span".as_bytes().to_vec())
                .with_text_style(TextStyle::Italic | TextStyle::Underline)
        ];
        
        let outputs_iter = json_single_item_formatter(spans.into_iter());
        
        let outputs: Vec<String> = outputs_iter.collect();
        
        // parse each
        
        assert_eq!(outputs.len(), 3);
    }

}
