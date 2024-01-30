use std::{
    io::{self, BufRead as _, BufReader, BufWriter, Write as _},
    net::{TcpListener, TcpStream},
    thread,
};

/// There are multiple ways to implement handle client function.
/// You can use `try_clone` to make multiple variable referring to the same
/// underlying TCP stream.
/// Or you can use the fact that `Read` *and* `Write` traits are both
/// implemented for `&TcpStream` like we do above.
///
/// Using `BufWriter` can be convenient because we can call `write` or
/// `write_all` multiple times, and the writes to underlying TCP stream will
/// only be done when the internal buffer is full (or when we call `flush`).
/// Without a buffered writer we would construct our output explicitly before
/// performing a write.
///
/// Here's another alternative implementation:
///
/// ```rust ignore
/// fn handle_client(stream: TcpStream) -> Result<(), io::Error> {
///     let mut writer = stream.try_clone()?;
///     let reader = BufReader::new(stream);
///     for line in reader.lines() {
///         let line = line?;
///         let line = format!("{}\n", line);
///         writer.write_all(line.as_bytes())?;
///     }
///     Ok(())
/// }
/// ```
fn handle_client(stream: TcpStream) -> Result<(), io::Error> {
    let reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);
    for line in reader.lines() {
        let line = line?;
        writer.write_all(line.as_bytes())?;
        writer.write_all(b"\n")?;
        writer.flush()?;
    }
    Ok(())
}

fn main() -> Result<(), io::Error> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    for stream in listener.incoming() {
        let stream = stream?;
        thread::spawn(|| {
            handle_client(stream).unwrap();
        });
    }
    Ok(())
}
