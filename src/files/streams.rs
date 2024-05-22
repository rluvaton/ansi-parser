use std::ffi::OsString;
use std::path::PathBuf;
use std::pin::Pin;

use async_stream::stream;
use get_chunk::iterator::FileIter;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncSeekExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::Stream;

macro_rules! send_string_buffer_chunk {
    // match rule which matches multiple expressions in an argument
    ($buffer:ident, $size:expr, $tx:ident) => {
        let s = String::from_utf8_lossy($buffer[..$size].as_ref()).to_string();

        if $tx.send(Ok(s)).await.is_err() {
            break;
        }
    };
}

pub async fn read_file_by_chunks_tokio(
    file_path: &str,
    chunk_size: usize,
) -> io::Result<impl Stream<Item = io::Result<String>>> {
    let (tx, rx) = mpsc::channel(10);
    let mut file = File::open(file_path).await?;
    let mut buffer = vec![0; chunk_size];

    tokio::spawn(async move {
        loop {
            match file.read(&mut buffer).await {
                Ok(0) => break, // EOF reached
                Ok(n) => {
                    send_string_buffer_chunk!(buffer, n, tx);
                }
                Err(e) => {
                    let _ = tx.send(Err(e)).await;
                    break;
                }
            }
        }
    });

    Ok(ReceiverStream::new(rx))
}

pub async fn read_file_by_chunks<'a>(
    file_path: &'a str,
    _chunk_size: usize,
) -> impl Stream<Item = io::Result<Box<str>>> {
    let input_file_path = PathBuf::from(OsString::from(file_path));
    let input_file = std::fs::File::open(input_file_path).expect("opening input file path failed");

    let file_iter = FileIter::try_from(input_file).expect("create input file iterator failed");

    return read_file_from_file_iterator(file_iter).await;
}


pub async fn read_file_by_chunks_from_to_locations_tokio(
    file_path: &str,
    chunk_size: usize,
    from_line: Option<usize>,
    to_line: Option<usize>,
) -> io::Result<impl Stream<Item = io::Result<String>>> {
    let (tx, rx) = mpsc::channel(10);
    let mut file = File::open(file_path).await?;
    let mut buffer = vec![0; chunk_size];

    if from_line.is_some() {
        file.seek(io::SeekFrom::Start(from_line.unwrap() as u64))
            .await
            .expect("Failed to seek to start position");
    }

    let mut size_read = Box::new(0);

    tokio::spawn(async move {
        loop {
            match file.read(&mut buffer).await {
                Ok(0) => break, // EOF reached
                Ok(n) => {
                    if to_line.is_some() && *size_read + n > to_line.unwrap() {
                        send_string_buffer_chunk!(buffer, to_line.unwrap() - *size_read, tx);
                        break;
                    }
                    *size_read += n;

                    send_string_buffer_chunk!(buffer, n, tx);
                }
                Err(e) => {
                    let _ = tx.send(Err(e)).await;
                    break;
                }
            }
        }
    });

    Ok(ReceiverStream::new(rx))
}

pub async fn read_file_by_chunks_from_to_locations(
    file_path: &str,
    chunk_size: usize,
    from_line: Option<usize>,
    to_line: Option<usize>,
) -> Pin<Box<dyn Stream<Item = io::Result<Box<str>>>>> {
    if from_line.is_none() && to_line.is_none() {
        return Box::pin(read_file_by_chunks(file_path, chunk_size).await);
    }

    let input_file_path = PathBuf::from(OsString::from(file_path));
    let input_file = std::fs::File::open(input_file_path).expect("opening input file path failed");

    let mut file_iter = FileIter::try_from(input_file).expect("create input file iterator failed");

    if from_line.is_some() {
        file_iter = file_iter
            .set_start_position_bytes(from_line.unwrap())
            .expect("Failed to set start position");
    }

    if to_line.is_none() {
        return Box::pin(read_file_from_file_iterator(file_iter).await);
    }

    let end = to_line.unwrap() - from_line.unwrap();

    let mut current = 0;

    return Box::pin(stream! {
        for item in file_iter.into_iter() {
            let item = item.expect("Failed to get file chunk");
            if current + item.len() > end && current < end {
                yield Ok(String::from_utf8_lossy(item[..end - current].as_ref()).to_string().into_boxed_str());
                break;
            }
            current += item.len();
            let s = String::from_utf8_lossy(item.as_ref()).to_string();
            yield Ok(s.into_boxed_str());
        }
    });
}

async fn read_file_from_file_iterator(
    file_iter: FileIter<std::fs::File>,
) -> impl Stream<Item = io::Result<Box<str>>> {
    stream! {
        for item in file_iter.into_iter() {
            let s = String::from_utf8_lossy(item.expect("Failed to get file chunk").as_ref()).to_string();
            yield Ok(s.into_boxed_str());
        }
    }
}
