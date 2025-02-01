use std::path::Path;

use crate::filter::parser::blocks_with_lines::parse_blocks_with_lines;
use crate::filter::parser::blocks_with_lines::BlockWithLines;
use crate::filter::parser::lines::content_to_lines;
use crate::filter::parser::read_from_disk::read_filter_from_disk;

/// Parses a loot filter file.
///
/// This function reads the file from disk, processes its raw content into lines,
/// and then parses the remaining lines into blocks.
///
/// Returns a vector of parsed BlockItem on success or an error if processing fails.
pub async fn parse_file(path: &Path) -> Result<Vec<BlockWithLines>, Box<dyn std::error::Error>> {
    let content = read_filter_from_disk(path).await?;
    let lines = content_to_lines(&content);
    let blocks_with_lines = parse_blocks_with_lines(&lines);

    Ok(blocks_with_lines)
}
