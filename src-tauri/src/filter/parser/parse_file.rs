use std::path::Path;

use crate::filter::parser::block_item::BlockItem;
use crate::filter::parser::blocks_with_lines::parse_blocks_with_lines;
use crate::filter::parser::lines::content_to_lines;
use crate::filter::parser::read_from_disk::read_filter_from_disk;
use crate::filter::parser::remove_comment::remove_comment;

/// Parses a loot filter file.
///
/// This function reads the file from disk, processes its raw content into lines,
/// removes commented lines, and then parses the remaining lines into blocks.
///
/// # Arguments
///
/// * `path` - A reference to the path of the loot filter file.
///
/// # Returns
///
/// Returns a vector of parsed BlockItem on success or an error if processing fails.
pub async fn parse_file(path: &Path) -> Result<Vec<BlockItem>, Box<dyn std::error::Error>> {
    let content = read_filter_from_disk(path).await;
    let lines = content_to_lines(&content);
    let cleaned_lines = remove_comment(lines);
    let blocks = parse_blocks_with_lines(cleaned_lines)?;

    Ok(blocks)
}
