#[derive(Debug)]
pub enum DriveType {
    HDD,
    SSD,
    Unknown,
}

#[cfg(target_os = "linux")]
pub fn get_drives() -> Vec<(String, DriveType)> {
    let mut drives = Vec::new();

    if let Ok(entries) = std::fs::read_dir("/sys/block") {
        for entry in entries.flatten() {
            let dev_name = entry.file_name().into_string().unwrap_or_default();
            let rotational_path = format!("/sys/block/{}/queue/rotational", dev_name);

            let drive_type = match std::fs::read_to_string(rotational_path) {
                Ok(val) if val.trim() == "0" => DriveType::SSD,
                Ok(val) if val.trim() == "1" => DriveType::HDD,
                _ => DriveType::Unknown,
            };

            drives.push((dev_name, drive_type));
        }
    }

    drives
}

#[cfg(target_os = "windows")]
pub fn get_drives() -> Vec<(String, DriveType)> {
    use std::process::Command;
    let mut drives = Vec::new();

    // Call PowerShell Get-PhysicalDisk
    let output = Command::new("powershell")
        .args(&["-Command", "Get-PhysicalDisk | Select-Object -Property FriendlyName,MediaType"])
        .output()
        .expect("failed to run PowerShell");

    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines().skip(3) { // skip header lines
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }

        let dev_name = parts[0].to_string();
        let media_type_str = parts.last().unwrap_or(&"Unknown");

        let drive_type = match *media_type_str {
            "SSD" => DriveType::SSD,
            "HDD" => DriveType::HDD,
            _ => DriveType::Unknown,
        };

        drives.push((dev_name, drive_type));
    }

    drives
}

#[cfg(target_os = "android")]
pub fn get_drives() -> Vec<(String, DriveType)> {
    // Placeholder: treat internal as SSD, SD card as HDD
    vec![
        ("/storage/emulated/0".to_string(), DriveType::SSD),
        ("/storage/0123-4567".to_string(), DriveType::HDD),
    ]
}