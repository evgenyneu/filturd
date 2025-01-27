use std::io::BufReader;
use std::path::Path;

mod commands {
    pub mod copy_item_desc;
}

use commands::copy_item_desc::copy_item_description_under_cursor;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn play_sound(file: String) -> Result<(), ()> {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    let path = Path::new("resources/sounds/").join(file);
    let file = std::fs::File::open(path).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    sink.append(source);
    sink.sleep_until_end();
    Ok(())
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
