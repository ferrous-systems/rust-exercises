use std::fs::File;
use std::io::{Result, Write};

#[derive(Debug)]
pub struct DurableFile {
    file: File,
    needs_sync: bool,
}

#[derive(Debug)]
pub struct CloseError {
    file: DurableFile,
    error: std::io::Error,
}

impl Write for DurableFile {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let amt = self.file.write(buf)?;
        self.needs_sync = true;
        Ok(amt)
    }
    fn flush(&mut self) -> Result<()> {
        
        self.file.sync_all()?;
        self.needs_sync = false;
        Ok(())
    }
}

impl Drop for DurableFile {
    fn drop(&mut self) {
        // Any edge cases?
        if self.needs_sync {
            panic!("You forgot to sync!");
        }
    }
}

impl DurableFile {
    pub fn new(file: File) -> DurableFile {
        DurableFile {
            file,
            needs_sync: false,
        }
    }

    pub fn close(mut self) -> std::result::Result<(), CloseError> {
        match self.flush() {
            Ok(()) => Ok(()),
            Err(e) => Err(CloseError {
                file: self,
                error: e,
            }),
        }
    }
}