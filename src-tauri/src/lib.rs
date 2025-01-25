// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use enigo::{
    Direction::{Click, Press},
    Enigo, Key, Keyboard, Settings,
};

use std::io::BufReader;
use std::path::Path;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn play_sound(file: &str) {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    let path = Path::new("resources/sounds/").join(file);
    let file = std::fs::File::open(path).unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
    sink.sleep_until_end();
}

#[tauri::command]
fn simulate_copy() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let _ = enigo.key(Key::Control, Press);
    let _ = enigo.key(Key::Unicode('c'), Click);
    std::thread::sleep(std::time::Duration::from_millis(100));
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, simulate_copy, play_sound])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
