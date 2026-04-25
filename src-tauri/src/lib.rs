use tauri::State;
use std::sync::Mutex;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
struct Item {
    id: i32,
    name: String,
}

struct AppData(Mutex<Vec<Item>>);

#[tauri::command]
fn create_item(state: State<AppData>, name: String) -> Vec<Item> {
    // BUKTI RUST BEKERJA: Cetak ke Logcat Android
    println!("RUST LOG: Membuat item baru dengan nama: {}", name);
    
    let mut db = state.0.lock().unwrap();
    let new_id = db.last().map(|i| i.id + 1).unwrap_or(1);
    db.push(Item { id: new_id, name });
    db.clone()
}

#[tauri::command]
fn get_rust_info() -> String {
    // Mengambil info versi Rust atau OS dari sisi Native
    format!("Diproses oleh Rust versi 1.70+ di arsitektur: {}", std::env::consts::ARCH)
}

#[tauri::command]
fn read_items(state: State<AppData>) -> Vec<Item> {
    println!("RUST LOG: Membaca semua data dari RAM");
    state.0.lock().unwrap().clone()
}

#[tauri::command]
fn delete_item(state: State<AppData>, id: i32) -> Vec<Item> {
    println!("RUST LOG: Menghapus item ID: {}", id);
    let mut db = state.0.lock().unwrap();
    db.retain(|i| i.id != id);
    db.clone()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppData(Mutex::new(Vec::new())))
        .invoke_handler(tauri::generate_handler![create_item, read_items, delete_item, get_rust_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
