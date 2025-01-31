// src/main.rs

mod capture;
mod db;
mod ui;

use std::{collections::HashMap, sync::{Arc, Mutex}, thread};

use crate::db::Database;

pub type TrafficData = Arc<Mutex<HashMap<String, (usize, String)>>>;

fn main() {
    // Initialize DB (requires "libsqlite3-dev" or "sqlite-devel" system library)
    let database = Database::new("mydb.sqlite").expect("Failed to open DB");
    let db_arc = Arc::new(Mutex::new(database));

    // Shared data for TUI
    let traffic_data: TrafficData = Arc::new(Mutex::new(HashMap::new()));

    // Spawn capture thread
    {
        let data_clone = Arc::clone(&traffic_data);
        let db_clone = Arc::clone(&db_arc);

        thread::spawn(move || {
            capture::capture_packets(data_clone, db_clone);
        });
    }

    // Start TUI
    ui::run_tui(traffic_data);
}
