use std::path::Path;
use anyhow::Result;

use crate::filter::parser::blocks::{Block, parse_block_with_lines};
use crate::filter::parser::blocks_with_lines::parse_lines;
use crate::filter::parser::lines::content_to_lines;
use crate::filter::parser::read_from_disk::read_filter_from_disk;

/// Parses a loot filter file.
///
/// This function reads the file from disk, processes its raw content into lines,
/// and then parses the remaining lines into blocks.
///
/// Returns a vector of parsed Block on success or an error if processing fails.
pub async fn parse_file(path: &Path) -> Result<Vec<Block>> {
    let content = read_filter_from_disk(path).await?;
    let lines = content_to_lines(&content);
    let blocks_with_lines = parse_lines(&lines);
    let blocks = parse_block_with_lines(&blocks_with_lines)?;

    Ok(blocks)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn integration_test_parse_file_with_example_filter() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("filter_examples/example_001.filter");

        let result = parse_file(&path).await;

        assert!(result.is_ok());

        let blocks = result.unwrap();
        assert_eq!(blocks.len(), 305);
    }
}
