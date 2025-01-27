use arboard::Clipboard;

use enigo::{
    Direction::{Click, Press},
    Enigo, Key, Keyboard, Settings,
};

#[tauri::command]
pub fn copy_item_description_under_cursor() -> Option<String> {
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
        Err(_e) => {
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
