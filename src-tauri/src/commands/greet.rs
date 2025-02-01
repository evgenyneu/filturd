use crate::filter::parser::read_from_disk::read_filter_from_disk;

#[tauri::command]
pub async fn greet(name: &str) -> Result<String, String> {
    let filter_path =
        r"C:\Users\Evgenii\Documents\My Games\Path of Exile 2\FilterBladeFilter01.filter";

    match read_filter_from_disk(filter_path).await {
        Ok(content) => {
            let preview = content.chars().take(20).collect::<String>();
            Ok(format!(
                "Hello, {}! You've been greeted from Rust! First 20 chars of filter: {}",
                name, preview
            ))
        }
        Err(e) => Ok(format!(
            "Hello, {}! You've been greeted from Rust! (Failed to read filter: {})",
            name, e
        )),
    }
}
