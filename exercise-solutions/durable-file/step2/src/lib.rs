use std::fs::File;
struct DurableFile {
    file: File,
    needs_sync: bool,
}

impl DurableFile {
    pub fn new(file: std::fs::File) -> DurableFile {
        DurableFile {
            file,
            needs_sync: false,
        }
    }
}
