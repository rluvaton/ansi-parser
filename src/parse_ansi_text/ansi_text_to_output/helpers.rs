use crate::parse_ansi_text::raw_ansi_parse::{Output, Text};

use std::iter::Iterator;

use genawaiter::{rc::gen, yield_};

pub fn merge_text_output<'a, I: Iterator<Item = Output<'a>>>(input: I) -> impl Iterator<Item = Output<'a>> {
    return gen!({
         let mut text_blocks_vec: Vec<Text> = Vec::new();

        for value in input {
            match value {
                Output::TextBlock(txt) => {
                    text_blocks_vec.push(txt);
                },
                _ => {
                    if !text_blocks_vec.is_empty() {
                        yield_!(Output::TextBlock(Text {
                            // TODO - avoid leak
                            text: text_blocks_vec.iter().map(|x| x.text.to_vec()).reduce(|a, b| [a.clone(), b.clone()].concat()).unwrap().leak(),
                            location_in_text: text_blocks_vec.first().unwrap().location_in_text,
                        }));
                        text_blocks_vec.clear();
                        text_blocks_vec.shrink_to_fit();
                    }
                    yield_!(value);
                }
            
            }
        }
        
        if !text_blocks_vec.is_empty() {
            yield_!(Output::TextBlock(Text {
                            // TODO - avoid leak
                
                text: text_blocks_vec.iter().map(|x| x.text.to_vec()).reduce(|a, b| [a.clone(), b.clone()].concat()).unwrap().leak(),
                location_in_text: text_blocks_vec.first().unwrap().location_in_text,
            }));
        }
    }).into_iter();
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use crate::parse_ansi_text::raw_ansi_parse::AnsiSequence;

    use super::*;


    #[test]
    fn should_merge_text_output_to_one() {
        let outputs: Vec<Output> = vec![
            Output::TextBlock(Text {
                text: b"Hello, World!",
                location_in_text: 0,
            }),
            Output::TextBlock(Text {
                text: b"How are you",
                location_in_text: 10,
            })
        ];

        let merged_outputs = merge_text_output(outputs.into_iter());

        let merged_outputs: Vec<Output> = merged_outputs.collect();

        assert_eq!(merged_outputs, vec![
            Output::TextBlock(Text {
                text: b"Hello, World!How are you",
                location_in_text: 0,
            })
        ]);
    }

    #[test]
    fn should_merge_text_output() {
        let outputs: Vec<Output> = vec![
            Output::TextBlock(Text {
                text: b"Hello, World!",
                location_in_text: 0,
            }),
            Output::TextBlock(Text {
                text: b"How are you",
                location_in_text: 10,
            }),
            Output::Escape(AnsiSequence::SetMode(0)),

            Output::TextBlock(Text {
                text: b"Im good",
                location_in_text: 13,
            }),
            Output::TextBlock(Text {
                text: b"Great",
                location_in_text: 16,
            }),
            Output::Escape(AnsiSequence::SetMode(1)),
        ];

        let merged_outputs = merge_text_output(outputs.into_iter());

        let merged_outputs: Vec<Output> = merged_outputs.collect();

        assert_eq!(merged_outputs, vec![
            Output::TextBlock(Text {
                text: b"Hello, World!How are you",
                location_in_text: 0,
            }),
            Output::Escape(AnsiSequence::SetMode(0)),
            Output::TextBlock(Text {
                text: b"Im goodGreat",
                location_in_text: 13,
            }),
            Output::Escape(AnsiSequence::SetMode(1)),
        ]);
    }

    #[test]
    fn should_not_merge_non_text_outputs() {
        let outputs: Vec<Output> = vec![
            Output::TextBlock(Text {
                text: b"Hello, World!",
                location_in_text: 0,
            }),
            Output::Escape(AnsiSequence::SetMode(0)),
            Output::TextBlock(Text {
                text: b"How are you",
                location_in_text: 10,
            })
        ];

        let merged_outputs = merge_text_output(outputs.clone().into_iter());

        let merged_outputs: Vec<Output> = merged_outputs.collect();

        assert_eq!(merged_outputs, outputs);
    }
}
