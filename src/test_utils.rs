use async_stream::stream;
use futures_core::Stream;

pub fn chars_stream(str: String) -> impl Stream<Item = String> {
    stream! {
        let chars = str.chars();
        for c in chars {
            yield c.to_string();
        }
    }
}

pub fn chars_stream_str(str: &str) -> impl Stream<Item = Box<str>> + '_ {
    stream! {
        let chars = str.chars();
        for c in chars {
            yield c.to_string().into_boxed_str();
        }
    }
}

pub async fn async_chars_stream(str: String) -> impl Stream<Item = String> {
    stream! {
        let chars = str.chars();
        for c in chars {
            yield c.to_string();
        }
    }
}

pub async fn async_chars_stream_str(str: &str) -> impl Stream<Item = Box<str>> + '_ {
    stream! {
        let chars = str.chars();
        for c in chars {
            yield c.to_string().into_boxed_str();
        }
    }
}

// TODO - avoid many functions with similar name
pub async fn async_chars_stream_string(str: String) -> impl Stream<Item = Box<str>> {
    stream! {
        let chars = str.chars();
        for c in chars {
            yield c.to_string().into_boxed_str();
        }
    }
}

pub async fn async_stream_from_vector<T>(vec: Vec<T>) -> impl Stream<Item = T> {
    stream! {
        for item in vec {
            yield item;
        }
    }
}


