use std::fs::File;
use std::io::Write;

struct DurableFile {
    file: File,
    needs_sync: bool,
}

impl DurableFile {
    pub fn new(file: File) -> DurableFile {
        DurableFile {
            file,
            needs_sync: false,
        }
    }
}

impl Write for DurableFile {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let written_bytes = self.file.write(buf)?;
        self.needs_sync = true;
        Ok(written_bytes)
    }

    fn flush(&mut self) -> std::io::Result<()> {
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
