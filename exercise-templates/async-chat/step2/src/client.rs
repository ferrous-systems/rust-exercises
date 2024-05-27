use tokio::{
    io::{stdin, AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpStream, ToSocketAddrs},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
pub(crate) async fn main() -> Result<()> {
    try_main("127.0.0.1:8080").await
}

async fn try_main(addr: impl ToSocketAddrs) -> Result<()> {
    let stream = TcpStream::connect(addr).await?;
    let (reader, mut writer) = tokio::io::split(stream);
    let reader = BufReader::new(reader);
    let mut lines_from_server = reader.lines();

    let stdin = BufReader::new(stdin());
    let mut lines_from_stdin = stdin.lines();
    loop {
        tokio::select! {
            line = lines_from_server.next_line() => match line {
                Ok(Some(line)) => {
                    println!("{}", line);
                },
                Ok(None) => break,
                Err(e) => eprintln!("Error {:?}:", e),
            },
            line = lines_from_stdin.next_line() => match line {
                Ok(Some(line)) => {
                    writer.write_all(line.as_bytes()).await?;
                    writer.write_all(b"\n").await?;
                },
                Ok(None) => break,
                Err(e) => eprintln!("Error {:?}:", e),
            }
        }
    }
    Ok(())
}
