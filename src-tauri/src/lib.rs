mod commands;
pub mod filter;

use commands::copy_item_desc::copy_item_description_under_cursor;
use commands::greet::greet;
use commands::play_sound::play_sound;
use commands::open_file::open;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            copy_item_description_under_cursor,
            play_sound,
            open
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
