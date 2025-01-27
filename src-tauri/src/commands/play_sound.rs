use std::io::BufReader;
use std::path::Path;

#[tauri::command]
pub async fn play_sound(file: String) -> Result<(), ()> {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    let path = Path::new("resources/sounds/").join(file);
    let file = std::fs::File::open(path).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    sink.append(source);
    sink.sleep_until_end();
    Ok(())
}
