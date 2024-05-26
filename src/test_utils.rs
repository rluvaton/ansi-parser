use async_stream::stream;
use futures_core::Stream;

use genawaiter::{rc::gen, yield_};
pub fn chars_stream(str: String) -> impl Stream<Item=String> {
    stream! {
        let chars = str.chars();
        for c in chars {
            yield c.to_string();
        }
    }
}
pub fn chars_iterator(str: String) -> impl Iterator<Item=Vec<u8>> {
    return gen!({
        let chars = str.chars();
        for c in chars {
            yield_!(c.to_string().into_bytes());
        }
    }).into_iter();
}

pub fn chars_iterator2(str: String) -> impl Iterator<Item=&'static [u8]> {
    return gen!({
        let chars = str.chars();
        for c in chars {
            yield_!(c.to_string().leak().as_bytes());
        }
    }).into_iter();
}

pub async fn async_chars_stream(str: String) -> impl Stream<Item=Vec<u8>> {
    stream! {
        let chars = str.chars();
        for c in chars {
            yield c.to_string().as_bytes().to_vec();
        }
    }
}

pub async fn async_str_chars_stream(str: &str) -> impl Stream<Item=&str> {
    stream! {
        for i in 0..str.len() {
            // it's just a hack for str[i]
            yield str.split_at(i).1.split_at(1).0;
        }
    }
}

pub async fn async_stream_from_vector<T>(vec: Vec<T>) -> impl Stream<Item=T> {
    stream! {
        for item in vec {
            yield item;
        }
    }
}
