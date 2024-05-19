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
