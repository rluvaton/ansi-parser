use std::iter::Iterator;
use crate::parse_ansi_text::types::Span;

pub struct SplitToLines<IteratorType> {
    iter: IteratorType,
    line: Option<Vec<Span>>,
    pending_span: Option<Span>,
}

/**
* Same implementation as the following in TypeScript:
* ```ts
* function* splitToLines(chunks: Iterator<string>): Iterator<string> {
*     let line = '';
*     for (let chunk of chunks) {
*         while(chunk.includes('\n')) {
*             const i = chunk.indexOf('\n');
*             line += chunk.slice(0, i);
*             yield line;
*             line = '';
*             chunk = chunk.slice(i + 1);
*         }
*
*         line += chunk;
*     }
*
*     if (line) {
*         yield line;
*     }
* }
* ```
*/
impl<IteratorType> Iterator for SplitToLines<IteratorType>
    where
        IteratorType: Iterator<Item = Span>,
{
    // Output item
    type Item = Vec<Span>;

    // https://users.rust-lang.org/t/how-to-write-iterator-adapter/8835/2
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // Have some span from previous iteration that was cut off
        if self.pending_span.is_some() {
            let mut pending_span = self.pending_span.as_mut().unwrap();

            // If this span still contain text, than extract the 2 spans, one with the text until the new line and the other with the rest of the text
            if pending_span.text.contains("\n") {
                let i = pending_span.text.find("\n").unwrap();

                // Create new span with the text until the newline
                let new_span = pending_span.clone().with_text(pending_span.text[..i].to_string());
                
                let mut line = self.line.clone().unwrap();
                if !new_span.text.is_empty() {
                    line.push(new_span);
                }

                self.line = Some(vec![]);

                // Remove the string from it
                pending_span.text = pending_span.text[(i + 1)..].to_string();
                self.pending_span = Some(pending_span.clone());

                return Some(line);
            }
            
            if !pending_span.text.is_empty() {
                self.line.as_mut().unwrap().push(pending_span.clone());
            }

            self.pending_span = None;
        }

        while let Some(span) = self.iter.next() {
            if span.text.contains("\n") {
                let i = span.text.find("\n").unwrap();

                let new_span = span.clone()
                    .with_text(span.text[..i].to_string());

                let mut line = self.line.clone().unwrap();
                if !new_span.text.is_empty() {
                    line.push(new_span.clone());
                }

                self.line = Some(vec![]);

                // Remove the string from it
                self.pending_span = Some(span.clone().with_text(span.text[(i + 1)..].to_string()));
                return Some(line);
            }

            if !span.text.is_empty() {
                self.line.get_or_insert(vec![]).push(span);
            }
        }

        if self.line.is_some() {
            let line = self.line.clone().unwrap();
            self.line = None;
            
            return Some(line);
        }
        
        return None;
    }
}

impl<IteratorType> SplitToLines<IteratorType> {
    pub fn new(iter: IteratorType) -> Self {
        Self { iter, line: Some(vec![]), pending_span: None }
    }
}

pub trait SplitToLinesByIterator: Iterator<Item = Span> + Sized {
    fn to_span_lines(self) -> SplitToLines<Self> {
        SplitToLines::new(self)
    }
}

impl<IteratorType: Iterator<Item = Span>> SplitToLinesByIterator for IteratorType {}

#[cfg(test)]
mod tests {
    use pretty_assertions::{assert_eq};
    use crate::parse_ansi_text::colors::*;
    use crate::parse_ansi_text::constants::RESET_CODE;
    use crate::parse_ansi_text::parse_ansi_as_spans_iterator::ParseAnsiAsSpans;
    use crate::parse_ansi_text::parse_options::ParseOptions;
    use super::*;

    #[test]
    fn split_to_lines_should_work() {
        let input = "";

        let chunks = vec![
            RED_FOREGROUND_CODE.to_string() + "abc" + RESET_CODE,
            YELLOW_FOREGROUND_CODE.to_string() + "d\nef\ng" + RESET_CODE,
            CYAN_FOREGROUND_CODE.to_string() + "hij" + RESET_CODE
        ]
            .join("")
            .to_string();

        let lines: Vec<Vec<Span>> = chunks.parse_ansi_as_spans(ParseOptions::default()).to_span_lines().collect();

        let expected = vec![
            // Line 1:
            vec![
                Span::empty().with_text("abc".to_string()).with_color(Color::Red),
                Span::empty().with_text("d".to_string()).with_color(Color::Yellow)
            ],

            // Line 2:
            vec![
                Span::empty().with_text("ef".to_string()).with_color(Color::Yellow)
            ],

            // Line 3:
            vec![
                Span::empty().with_text("g".to_string()).with_color(Color::Yellow),
                Span::empty().with_text("hij".to_string()).with_color(Color::Cyan)
            ],
        ];

        assert_eq!(lines, expected);
    }
}
