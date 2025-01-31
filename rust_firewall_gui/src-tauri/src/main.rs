// src-tauri/src/main.rs

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod db;
mod capture; // optional if you have a separate capture logic

use db::Database;
use std::sync::{Arc, Mutex};
use tauri::Manager;

// A shared app state
struct AppState {
    db: Arc<Mutex<Database>>,
}

// A Tauri command to fetch logs from DB
#[tauri::command]
fn fetch_logs(state: tauri::State<AppState>) -> Vec<(i64, String, String, String, i64)> {
    let db_lock = state.db.lock().unwrap();
    db_lock.get_all_logs().unwrap_or_default()
}

// Another command: start capture
#[tauri::command]
fn start_capture_cmd(state: tauri::State<AppState>) -> bool {
    // spawn a thread for capturing
    let db_clone = state.db.clone();
    std::thread::spawn(move || {
        capture::start_capture(db_clone);
    });
    true
}

fn main() {
    // Initialize our Database
    let database = Database::new("mydb.sqlite").expect("Failed to open database");
    let db_arc = Arc::new(Mutex::new(database));

    // (Optional) automatically start capturing
    // std::thread::spawn({
    //     let db_clone = db_arc.clone();
    //     move || { capture::start_capture(db_clone); }
    // });

    tauri::Builder::default()
        // Provide state to Tauri
        .manage(AppState { db: db_arc })
        // Register commands that frontend can call with invoke("commandName")
        .invoke_handler(tauri::generate_handler![
            fetch_logs,
            start_capture_cmd
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
