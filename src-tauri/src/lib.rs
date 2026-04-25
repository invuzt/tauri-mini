use tauri::command;
use std::fs;
use base64::{engine::general_purpose, Engine as _};

#[command]
fn save_photo(base64_data: String) -> String {
    // 1. Bersihkan prefix base64 (data:image/jpeg;base64,...)
    let clean_data = base64_data.split(',').last().unwrap_or("");
    
    // 2. Decode string ke binary
    let bytes = general_purpose::STANDARD.decode(clean_data).unwrap();
    
    // 3. Tentukan nama file unik (pake timestamp)
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let file_path = format!("/sdcard/Download/Vuzt_{}.jpg", timestamp);
    
    // 4. Tulis ke storage
    match fs::write(&file_path, bytes) {
        Ok(_) => format!("Foto tersimpan di: {}", file_path),
        Err(e) => format!("Gagal simpan: {}", e),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_photo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
