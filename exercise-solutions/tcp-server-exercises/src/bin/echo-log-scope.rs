use std::{
    io::{self, BufRead as _, BufReader, BufWriter, Write as _},
    net::{TcpListener, TcpStream},
    sync::{Mutex, MutexGuard},
    thread,
};

fn handle_client(stream: TcpStream, log: &Mutex<Vec<usize>>) -> Result<(), io::Error> {
    let reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);
    for line in reader.lines() {
        let line = line?;
        {
            let mut log: MutexGuard<'_, _> = log.lock().unwrap();
            log.push(line.len());
        }
        writer.write_all(line.as_bytes())?;
        writer.write_all(b"\n")?;
        writer.flush()?;
    }
    Ok(())
}

fn main() -> Result<(), io::Error> {
    let log = Mutex::new(vec![]);

    let listener = TcpListener::bind("127.0.0.1:7878")?;
    thread::scope(|scope| {
        for stream in listener.incoming() {
            let stream = stream?;

            scope.spawn(|| {
                handle_client(stream, &log).unwrap();
            });
        }
        Ok(())
    })
}
