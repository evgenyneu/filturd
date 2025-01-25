// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use enigo::{
    Direction::{Click, Press},
    Enigo, Key, Keyboard, Settings,
};

use arboard::Clipboard;

use std::io::BufReader;
use std::path::Path;

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

#[tauri::command]
fn copy_item_description_under_cursor() -> Option<String> {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let _ = enigo.key(Key::Control, Press);
    let _ = enigo.key(Key::Unicode('c'), Click);
    std::thread::sleep(std::time::Duration::from_millis(100));

    let mut clipboard = match Clipboard::new() {
        Ok(clipboard) => clipboard,
        Err(e) => {
            eprintln!("Failed to access clipboard: {}", e);
            return None;
        }
    };

    let text = match clipboard.get_text() {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Failed to get clipboard text: {}", e);
            return None;
        }
    };

    if !text.starts_with("Item Class:") {
        // Not an item description
        return None;
    }

    match clipboard.clear() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to clear clipboard: {}", e);
            return Some(text);
        }
    };

    Some(text)
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
