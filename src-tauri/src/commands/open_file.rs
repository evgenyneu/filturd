use crate::filter::parser::parse_file::parse_file;
use crate::filter::parser::blocks::Block;
use std::path::Path;
use std::fs::File;
use std::io::Write;

#[tauri::command]
pub async fn open_file(path: &str) -> Result<Vec<Block>, ()> {
    let start = std::time::Instant::now();
    let result = parse_file(Path::new(path)).await;
    let duration = start.elapsed();
    println!("parse_file() took {:?}", duration);
    assert!(result.is_ok());
    let blocks = result.unwrap();
    let json: String = serde_json::to_string(&blocks).unwrap();
    // Save to file
    let mut file = File::create("blocks.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();
    Ok(blocks)
}
