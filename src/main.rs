mod os_detect;
mod drive_detect;
mod wipe;
mod cert;

use std::env;
use std::fs;
use cert::{WipeCertificate, generate_keypair, sign_certificate, verify_certificate, export_pdf};
fn main() {
    // 1. Detect the OS
    let os_name = os_detect::detect_os();
    println!("🔎 Detected OS: {}", os_name);

    // 2. Detect drives (just printing info for now)
    let drives = drive_detect::get_drives();
    println!("📀 Detected drives:");
    for (name, dtype) in &drives {
        println!(" - {} ({:?})", name, dtype);
    }

    // 3. Get path argument (dummy file or real disk path)
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("❌ Usage: zerotrace <path_to_device_or_dummy_file>");
        return;
    }
    let target_path = &args[1];
    println!("📝 Target for wipe: {}", target_path);

    // 4. Wipe the target using wipe.rs
    match wipe::wipe_drive(target_path, drive_detect::DriveType::HDD) {
        Ok(_) => println!("✅ Wipe completed successfully for {}", target_path),
        Err(e) => eprintln!("❌ Wipe failed: {}", e),
    }
    let keypair = generate_keypair();
    let cert = WipeCertificate::new(&os_name, "Null overwriting");
    let signed = sign_certificate(cert, &keypair);

    let json_filename = format!("wipe_cert_{}.json", signed.certificate_id);
    let pdf_filename = format!("wipe_cert_{}.pdf", signed.certificate_id);

    // Save JSON
    let json = serde_json::to_string_pretty(&signed).unwrap();
    fs::write(&json_filename, json).unwrap();

    // Save PDF
    export_pdf(&signed, &pdf_filename);


    println!("✅ Certificates created:\n- {}\n- {}", json_filename, pdf_filename);
}
