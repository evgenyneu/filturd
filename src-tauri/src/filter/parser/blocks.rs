use crate::filter::parser::block_item::{parse_block_item, BlockItem};
use crate::filter::parser::blocks_with_lines::{BlockName, BlockWithLines};
use crate::filter::parser::errors::ParseError;
use ts_rs::TS;

/// Represents a parsed Block which holds a block name and its parsed block items.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, TS)]
#[ts(export)]
pub struct Block {
    // Position of the block in the file, starting from 1
    pub order: u16,
    pub name: BlockName,
    pub items: Vec<BlockItem>,
}

/// Parses an array of `BlockWithLines` into an array of `Block` by parsing each line as a block item.
///
/// Returns an error if any line fails to parse.
///
/// # Arguments
///
/// * `blocks_with_lines` - A slice of `BlockWithLines` to be parsed into `Block` structures.
///
/// # Errors
///
/// Returns a [`ParseError`] if any block line cannot be parsed into a block item.
pub fn parse_block_with_lines(
    blocks_with_lines: &[BlockWithLines],
) -> Result<Vec<Block>, ParseError> {
    let mut blocks = Vec::new();

    for block in blocks_with_lines {
        let mut items = Vec::new();

        for line in &block.lines {
            // Propagates error if parse_block_item fails.
            let item = parse_block_item(line)?;

            items.push(item);
        }

        blocks.push(Block {
            order: block.order,
            name: block.name.clone(),
            items,
        });
    }

    Ok(blocks)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filter::parser::blocks_with_lines::{BlockName, BlockWithLines};

    #[test]
    fn test_parse_blocks_success() {
        let blocks_with_lines = vec![
            BlockWithLines {
                order: 1,
                name: BlockName::Show,
                lines: vec![
                    "BaseType == \"Mirror of Kalandra\"".to_string(),
                    "SetFontSize 45".to_string(),
                ],
            },
            BlockWithLines {
                order: 2,
                name: BlockName::Hide,
                lines: vec![
                    "Class \"Currency\"".to_string(),
                    "SetFontSize 40".to_string(),
                ],
            },
        ];

        let result = parse_block_with_lines(&blocks_with_lines);
        assert!(result.is_ok());

        let blocks = result.unwrap();

        assert_eq!(blocks.len(), 2);

        // First block
        assert_eq!(blocks[0].order, 1);
        assert_eq!(blocks[0].name, BlockName::Show);
        assert_eq!(blocks[0].items.len(), 2);

        // Second block
        assert_eq!(blocks[1].order, 2);
        assert_eq!(blocks[1].name, BlockName::Hide);
        assert_eq!(blocks[1].items.len(), 2);
    }

    #[test]
    fn test_block_serde() {
        let block = Block {
            order: 1,
            name: BlockName::Show,
            items: vec![
                BlockItem {
                    name: "BaseType".to_string(),
                    params: vec!["==".to_string(), "Mirror of Kalandra".to_string()],
                },
                BlockItem {
                    name: "SetFontSize".to_string(),
                    params: vec!["45".to_string()],
                },
            ],
        };

        let json = serde_json::to_string(&block).unwrap();
        let decoded: Block = serde_json::from_str(&json).unwrap();

        assert_eq!(block, decoded);
        assert_eq!(decoded.order, 1);
    }
}
