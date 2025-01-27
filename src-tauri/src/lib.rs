use std::io::BufReader;
use std::path::Path;

mod commands {
    pub mod copy_item_desc;
    pub mod play_sound;
}

use commands::copy_item_desc::copy_item_description_under_cursor;
use commands::play_sound::play_sound;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            copy_item_description_under_cursor,
            play_sound
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
