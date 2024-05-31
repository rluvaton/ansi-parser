use genawaiter::{rc::gen, yield_};

pub fn chars_iterator(str: String) -> impl Iterator<Item=Vec<u8>> {
    return gen!({
        let chars = str.chars();
        for c in chars {
            yield_!(c.to_string().into_bytes());
        }
    }).into_iter();
}


