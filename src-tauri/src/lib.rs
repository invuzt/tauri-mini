use tauri::State;
use std::sync::Mutex;
use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
struct Item {
    id: i32,
    name: String,
}

struct AppData(Mutex<Vec<Item>>);

const FILE_PATH: &str = "/sdcard/Download/vuzt_data.md";

fn save_to_file(items: &Vec<Item>) {
    let mut content = String::from("# Vuzt AI Data Storage\n\n");
    for item in items {
        content.push_str(&format!("- [ID: {}] {}\n", item.id, item.name));
    }
    let _ = fs::write(FILE_PATH, content);
}

fn load_from_file() -> Vec<Item> {
    if !Path::new(FILE_PATH).exists() {
        return Vec::new();
    }
    let content = fs::read_to_string(FILE_PATH).unwrap_or_default();
    let mut items = Vec::new();
    for line in content.lines() {
        if line.starts_with("- [ID: ") {
            let parts: Vec<&str> = line.split("] ").collect();
            if parts.len() == 2 {
                let id_part = parts[0].replace("- [ID: ", "");
                if let Ok(id) = id_part.parse::<i32>() {
                    items.push(Item { id, name: parts[1].to_string() });
                }
            }
        }
    }
    items
}

#[tauri::command]
fn create_item(state: State<AppData>, name: String) -> Vec<Item> {
    let mut db = state.0.lock().unwrap();
    let new_id = db.last().map(|i| i.id + 1).unwrap_or(1);
    db.push(Item { id: new_id, name });
    save_to_file(&db);
    db.clone()
}

#[tauri::command]
fn read_items(state: State<AppData>) -> Vec<Item> {
    let mut db = state.0.lock().unwrap();
    *db = load_from_file();
    db.clone()
}

#[tauri::command]
fn delete_item(state: State<AppData>, id: i32) -> Vec<Item> {
    let mut db = state.0.lock().unwrap();
    db.retain(|i| i.id != id);
    save_to_file(&db);
    db.clone()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppData(Mutex::new(load_from_file())))
        .invoke_handler(tauri::generate_handler![create_item, read_items, delete_item])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
