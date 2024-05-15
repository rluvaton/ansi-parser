// Span consumer iterator that get a span and just print it, making sure the entire iterator is output correctly in JSON 

// this means that should support both Vec<Span> and Vec<Vec<Span>>

use std::iter::Iterator;
use crate::cli::format::json_single_span::{SpansJsonDisplay, SpansJsonDisplayByIterator};
use crate::parse_ansi_text::types::{Span, SpanJson};

pub struct SpansLineFlatJsonLineDisplay<'a, IteratorType> {
    iter: IteratorType,
    line_iter: Option<Box<dyn Iterator<Item = String> + 'a>>,
    yielded_first_item: bool
}

impl<'a, IteratorType> Iterator for SpansLineFlatJsonLineDisplay<'a, IteratorType>
    where
        IteratorType: Iterator<Item = Vec<Span>>,
{
    // Output item
    type Item = String;

    // https://users.rust-lang.org/t/how-to-write-iterator-adapter/8835/2
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // TODO - implement

        if self.line_iter.is_some() {
            let line_iter = self.line_iter.as_mut().unwrap();
            while let Some(str) = line_iter.next() {
                return Some(str)
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
            self.line_iter = Some(Box::new(SpansJsonDisplay::new(line.into_iter())));

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

impl<'a, IteratorType> SpansLineFlatJsonLineDisplay<'a, IteratorType> {
    pub fn new(iter: IteratorType) -> Self {
        Self { iter, line_iter: None, yielded_first_item: false }
    }
}

pub trait SpansLineFlatJsonLineDisplayByIterator: Iterator<Item = Vec<Span>> + Sized {
    fn to_flat_json_line_string_in_span_lines<'a>(self) -> SpansLineFlatJsonLineDisplay<'a, Self> {
        SpansLineFlatJsonLineDisplay::new(self)
    }
}

impl<IteratorType: Iterator<Item = Vec<Span>>> SpansLineFlatJsonLineDisplayByIterator for IteratorType {}
