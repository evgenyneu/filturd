use crate::filter::parser::parse_file::parse_file;
use crate::filter::parser::blocks::Block;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use rand::seq::SliceRandom;
use std::path::PathBuf;

fn save_sample_json(blocks: Vec<Block>) -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::rng();
    let mut random_blocks = blocks.clone();
    random_blocks.shuffle(&mut rng);
    let random_blocks: Vec<Block> = random_blocks.into_iter().take(10).collect();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("filter_examples/example_001.json");
    let mut file = File::create(path)?;

    let json = serde_json::to_string_pretty(&random_blocks)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

#[tauri::command]
pub async fn open_file(path: &str) -> Result<Vec<Block>, ()> {
    let start = std::time::Instant::now();
    let result = parse_file(Path::new(path)).await;
    let duration = start.elapsed();
    println!("parse_file() took {:?}", duration);
    assert!(result.is_ok());
    let blocks = result.unwrap();

    if let Err(e) = save_sample_json(blocks.clone()) {
        eprintln!("Failed to save sample JSON: {}", e);
    }

    Ok(blocks)
}
