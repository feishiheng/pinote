// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::sync::Mutex;
use tauri::State;

// 全局状态存储便签内容
type NoteStorage = Mutex<HashMap<String, String>>;

#[tauri::command]
async fn save_note_content(file_name: String, content: String, storage: State<'_, NoteStorage>) -> Result<(), String> {
    let mut notes = storage.lock().map_err(|e| format!("Failed to lock storage: {}", e))?;
    notes.insert(file_name, content);
    Ok(())
}

#[tauri::command]
async fn load_note_content(file_name: String, storage: State<'_, NoteStorage>) -> Result<String, String> {
    let notes = storage.lock().map_err(|e| format!("Failed to lock storage: {}", e))?;
    Ok(notes.get(&file_name).cloned().unwrap_or_default())
}

fn main() {
  tauri::Builder::default()
    .manage(NoteStorage::default())
    .invoke_handler(tauri::generate_handler![save_note_content, load_note_content])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
