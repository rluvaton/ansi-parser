use async_stream::stream;
use futures_core::Stream;
use crate::parse_ansi_text::raw_ansi_parse::{Output, Text};

pub async fn merge_text_output<S: Stream<Item = Output>>(input: S) -> impl Stream<Item = Output> {
    stream! {
        let mut text_blocks_vec: Vec<Text> = Vec::new();

        for await value in input {
            match value {
                Output::TextBlock(txt) => {
                    text_blocks_vec.push(txt);
                },
                _ => {
                    if !text_blocks_vec.is_empty() {
                        yield Output::TextBlock(Text {
                            text: text_blocks_vec.iter().map(|x| x.text.as_str()).collect::<String>(),
                            location_in_text: text_blocks_vec.first().unwrap().location_in_text,
                        });
                        text_blocks_vec.clear();
                        text_blocks_vec.shrink_to_fit();
                    }
                    yield value;
                }
            
            }
        }
        
        if !text_blocks_vec.is_empty() {
            yield Output::TextBlock(Text {
                text: text_blocks_vec.iter().map(|x| x.text.as_str()).collect::<String>(),
                location_in_text: text_blocks_vec.first().unwrap().location_in_text,
            });
        }
    }
}
