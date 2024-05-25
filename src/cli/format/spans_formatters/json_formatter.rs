use std::iter::Iterator;

use genawaiter::{rc::gen, yield_};

use crate::parse_ansi_text::ansi::types::Span;
use crate::traits::ToJson;

pub fn json_formatter<I: Iterator<Item=Span>>(iter: I) -> impl Iterator<Item = String> {
    return gen!({
        let mut yielded_first_item = false;
        yield_!("[\n".to_string());
    
        // Can replace the loop here with just json line single span, as it's the same thing
        for span in iter {
            let mut str: &str = "";
    
            if yielded_first_item {
            // Print from prev object
                str = ",";
            }
    
    
            yielded_first_item = true;
    
            yield_!(str.to_string() + span.to_json().as_str());
        }
    
        yield_!("\n]".to_string());
    }).into_iter();

}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::parse_ansi_text::ansi::style::{Brightness, TextStyle};

    use super::*;

    #[test]
    fn test_formatter_combined_output_is_valid_json() {
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

        let outputs_iter = json_formatter(spans.clone().into_iter());

        let outputs: Vec<String> = outputs_iter.collect();
        
        let outputs = outputs.join("");
        
        let output_spans = Span::from_json_array(outputs.as_str()).expect("Failed to parse json array");
        
        assert_eq!(output_spans, spans);
    }
}
