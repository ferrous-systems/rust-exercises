use std::{
    io::{self, BufRead as _, BufReader, BufWriter, Write as _},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

fn handle_client(stream: TcpStream, log: &Mutex<Vec<usize>>) -> Result<(), io::Error> {
    let reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);
    for line in reader.lines() {
        let line = line?;
        // the code block here forces the MutexGuard drop to unlock the mutex
        {
            let mut log = log.lock().unwrap();
            log.push(line.len());
        }
        writer.write_all(line.as_bytes())?;
        writer.write_all(b"\n")?;
        writer.flush()?;
    }
    Ok(())
}

fn main() -> Result<(), io::Error> {
    let log = Arc::new(Mutex::new(vec![]));

    let listener = TcpListener::bind("127.0.0.1:7878")?;
    for stream in listener.incoming() {
        let Ok(stream) = stream else {
            eprintln!("Bad connection");
            continue;
        };

        let log = Arc::clone(&log);
        thread::spawn(move || {
            handle_client(stream, &log).unwrap();
        });
    }
    Ok(())
}
