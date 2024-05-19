use std::char::decode_utf16;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncSeekExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::Stream;

pub fn read_string(slice: &[u8], size: usize) -> Option<String> {
    assert!(2*size <= slice.len());
    let iter = (0..size)
        .map(|i| u16::from_be_bytes([slice[2*i], slice[2*i+1]]));

    decode_utf16(iter).collect::<Result<String, _>>().ok()
}

pub async fn read_file_by_chunks(
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
                    let s = read_string(&buffer[..n], n / 2).expect("Failed to convert buffer to string");

                    if tx.send(Ok(s)).await.is_err() {
                        break;
                    }
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
                        let s = read_string(&buffer[..n], n / 2).expect("Failed to convert buffer to string");

                        if tx.send(Ok(s)).await.is_err() {
                            break;
                        }
                        break;
                    }
                    *size_read += n;
                    
                    let s = read_string(&buffer[..n], n / 2).expect("Failed to convert buffer to string");

                    if tx.send(Ok(s)).await.is_err() {
                        break;
                    }
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
