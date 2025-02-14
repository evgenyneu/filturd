use crate::filter::parser::parse_file::parse_file;
use crate::filter::parser::blocks::Block;
use std::path::Path;

#[tauri::command]
pub async fn open_file(path: &str) -> Result<Vec<Block>, ()> {
    let start = std::time::Instant::now();
    let result = parse_file(Path::new(path)).await;
    let duration = start.elapsed();
    println!("parse_file() took {:?}", duration);
    assert!(result.is_ok());
    let blocks = result.unwrap();
    Ok(blocks)
}
