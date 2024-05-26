use std::iter::Iterator;

use genawaiter::{rc::gen, yield_};

use crate::parse_ansi_text::ansi::types::Span;
use crate::traits::ToJson;

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
                .with_text(b"Hello, World!".to_vec())
                .with_brightness(Brightness::Bold),
            Span::empty()
                .with_text(b" ".to_vec()),
            Span::empty()
                .with_text(b"This is another span".to_vec())
                .with_text_style(TextStyle::Italic | TextStyle::Underline)
        ];
        
        let outputs_iter = json_single_item_formatter(spans.clone().into_iter());
        
        let outputs: Vec<String> = outputs_iter.collect();
        
        // parse each item
        let mut i = 0;
        for output in outputs.iter() {
            let item = Span::from_json(output.as_str()).expect("Failed to parse json");
            
            assert_eq!(item, spans[i]);
            
            i += 1;
        }
        
        assert_eq!(i, spans.len());
    }

}
