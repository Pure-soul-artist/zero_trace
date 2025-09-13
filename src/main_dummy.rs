use std::fs::OpenOptions;
use std::io::{Seek, Write, SeekFrom};

pub fn wipe_dummy_file(path: &str) -> std::io::Result<()> {
    let mut f = OpenOptions::new()
        .write(true)
        .open(path)?;

    let metadata = f.metadata()?;
    let size = metadata.len();

    // Overwrite with zeros
    let buf = vec![0u8; 1024 * 1024]; // 1MB buffer
    let mut written = 0;

    while written < size {
        f.write_all(&buf)?;
        written += buf.len() as u64;
    }

    f.flush()?;
    f.seek(SeekFrom::Start(0))?;
    Ok(())
}

fn main() {
    let test_path = "D:/dummy.img"; // use your dummy file here
    match wipe_dummy_file(test_path) {
        Ok(_) => println!("Dummy file wiped successfully."),
        Err(e) => eprintln!("Error wiping: {}", e),
    }
}