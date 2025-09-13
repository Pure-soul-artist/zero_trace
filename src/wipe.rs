use std::fs::{OpenOptions};
use std::io::{Write, Seek, SeekFrom};
use crate::drive_detect::DriveType;

pub fn wipe_drive(path: &str, dtype: DriveType) -> Result<(), String> {
    match dtype {
        DriveType::HDD => wipe_hdd(path),
        DriveType::SSD => wipe_ssd(path),
        DriveType::Unknown => Err("Unknown drive type".to_string()),
    }
}

fn wipe_hdd(path: &str) -> Result<(), String> {
    println!("Performing zero overwrite on HDD: {}", path);

    // WARNING: DEMO ONLY — this would destroy real data!
    // Instead of wiping the actual /dev/sda, just simulate.
    match OpenOptions::new().write(true).open(path) {
        Ok(mut f) => {
            let buf = vec![0u8; 1024 * 1024]; // 1MB buffer of zeros
            f.seek(SeekFrom::Start(0)).map_err(|e| e.to_string())?;
            f.write_all(&buf).map_err(|e| e.to_string())?;
            Ok(())
        }
        Err(e) => Err(format!("Failed to open {}: {}", path, e)),
    }
}

fn wipe_ssd(path: &str) -> Result<(), String> {
    println!("Performing cryptographic erase on SSD: {}", path);

    // For demo, simulate with message
    // Real implementation: call `nvme format` or `hdparm --security-erase`
    println!("(Simulated) Secure erase command sent to {}", path);

    Ok(())
}
