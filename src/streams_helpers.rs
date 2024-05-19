use async_stream::stream;
use tokio::io;
use tokio_stream::Stream;

/**
Usage:

```rust
let s = compose_streams!(
    zero_to_three,
    double,
    str
);
pin_mut!(s); // needed for iteration

while let Some(value) = s.next().await {
    println!("got {}", value);
}
```
 */

#[macro_export] macro_rules! compose_streams {
    // match rule which matches multiple expressions in an argument
    ($readable:expr, $($transform:expr),*) => {
        (|| {
            let output = $readable();
            $(
                let input = $transform(output);
                let output = input;
            )*
            return output;
        })()
    };
}


/**
Each function must be async and return a stream
Usage:

```rust
let s = compose_async_steams!(
    // callback so the function can be called with args
     | | read_file_by_chunks("example.txt", 1024),
        unwrap_items,
        split_to_lines
    ).await;
pin_mut!(s); // needed for iteration

while let Some(value) = s.next().await {
    println!("got {}", value);
}
```
 */
#[macro_export] macro_rules! compose_async_steams {
    ($readable:expr, $($transform:expr),*) => {
        (|| async {
            let output = $readable();
            $(
                let input = $transform(output.await);
                let output = input;
            )*
            return output;
        })().await

    };
}


pub async fn unwrap_items<I>(input: io::Result<impl Stream<Item = io::Result<I>>>)
                         -> impl Stream<Item=I>
{
    stream! {
        for await value in input.unwrap() {
            yield value.unwrap();
        }
    }
}

