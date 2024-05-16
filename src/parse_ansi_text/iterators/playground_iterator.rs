use std::fmt::Display;
use std::io;
use std::iter::Iterator;

use get_chunk::iterator::FileIter;

use crate::parse_ansi_text::ansi::colors::{BLACK_BACKGROUND_CODE, RED_BACKGROUND_CODE};
use crate::parse_ansi_text::ansi::constants::RESET_CODE;
use crate::parse_ansi_text::iterators::custom_ansi_parse_iterator::AnsiParseIterator;

// TODO - change type here
fn run() -> io::Result<u8> {
    // TODO - create string iterator that can be swaopped to file iterator or stdin or whatever
    // TODO - try the https://crates.io/crates/get_chunk crate

    let black_background_code = BLACK_BACKGROUND_CODE.to_string();

    let mut first_part_for_black_background = black_background_code.clone();
    let second_part_for_black_background =
        first_part_for_black_background.split_off(black_background_code.len() / 2);

    let input_chunks: Vec<String> = vec![
        RED_BACKGROUND_CODE.to_string(),
        "Hello, World!".to_string(),
        RESET_CODE.to_string(),
        " ".to_string(),
        // Split same style to two parts to make sure it works
        first_part_for_black_background,
        second_part_for_black_background,
        "Goodbye".to_string(),
        " world!".to_string(),
    ];
    let iterator = RandomStringsIterator { vec: input_chunks, index: 0 };


    let file_iter = FileIter::new("file.txt")?;

    // or
    // let file_iter = FileIter::try_from(File::open("file.txt")?)?;
    // ...
    // for chunk in file_iter {
    //     match chunk {
    //         Ok(data) => {
    //
    //         }
    //         Err(_) => break,
    //     }
    // }

    // let file_iterator = file_iter.into_iter().map(|item| item.expect("Failed to get file chunk")).expect("Converting file chunk to UTF-8 string failed"));
    // 
    // let iterator = AnsiParseIterator::create(Box::new(file_iterator));

    //
    // let ansi_parse_iterator = AnsiParseIterator {
    //     pending_string: "".to_string(),
    //     iterator: Box::new(file_iterator),
    // };
    // AnsiParseIterator {
    //     dat: iterator.into_iter()
    // }
    // iterator.for_each(|item| {
    //     println!("{:#?}", item);
    // });

    // TODO - find a better way to create iterator from input, just a function that get the
    // "".random_strings(input_chunks)
    // TODO - ansi parse on the iterator

    return Ok(0);
}

pub trait RandomStrings {
    fn random_strings(&self, vec: Vec<String>) -> RandomStringsIterator;
}

impl RandomStrings for str {
    fn random_strings(&self, vec: Vec<String>) -> RandomStringsIterator {
        RandomStringsIterator { vec, index: 0 }
    }
}

#[derive(Debug)]
pub struct RandomStringsIterator {
    vec: Vec<String>,
    index: usize,
}

impl Iterator for RandomStringsIterator {
    type Item = String;

    // https://users.rust-lang.org/t/how-to-write-iterator-adapter/8835/2
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len() {
            let item = self.vec[self.index].clone();
            self.index += 1;
            return Some(item);
        }

        return None;
    }
}

// TODO - remove this and use proper iterator
#[derive(Debug)]
pub struct CharsIterator {
    pub(crate) str: String,
    pub(crate) index: usize,
}

impl Iterator for CharsIterator {
    type Item = String;

    // https://users.rust-lang.org/t/how-to-write-iterator-adapter/8835/2
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.str.len() {
            let item = self.str.chars().nth(self.index).unwrap().to_string();
            self.index += 1;
            return Some(item);
        }

        return None;
    }
}
