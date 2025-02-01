use crate::filter::parser::block_item::{parse_block_item, BlockItem};
use crate::filter::parser::blocks_with_lines::{BlockName, BlockWithLines};
use crate::filter::parser::errors::ParseError;

/// Represents a parsed Block which holds a block name and its parsed block items.
#[derive(Debug, PartialEq)]
pub struct Block {
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
            name: block.name.clone(),
            items,
        });
    }

    Ok(blocks)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filter::parser::block_item::{BlockItemName, KnownBlockItemName};
    use crate::filter::parser::blocks_with_lines::{BlockName, BlockWithLines};

    #[test]
    fn test_parse_blocks_success() {
        let blocks_with_lines = vec![
            BlockWithLines {
                name: BlockName::Show,
                lines: vec![
                    "BaseType == \"Mirror of Kalandra\"".to_string(),
                    "SetFontSize 45".to_string(),
                ],
            },
            BlockWithLines {
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
        // --------------

        assert_eq!(blocks[0].name, BlockName::Show);
        assert_eq!(blocks[0].items.len(), 2);

        // First item
        // ------------

        assert_eq!(
            blocks[0].items[0].name,
            BlockItemName::Known(KnownBlockItemName::BaseType)
        );

        assert_eq!(blocks[0].items[0].params, vec!["==", "Mirror of Kalandra"]);

        // Second item
        // ------------

        assert_eq!(
            blocks[0].items[1].name,
            BlockItemName::Known(KnownBlockItemName::SetFontSize)
        );

        assert_eq!(blocks[0].items[1].params, vec!["45"]);

        // Second block
        // --------------

        assert_eq!(blocks[1].name, BlockName::Hide);
        assert_eq!(blocks[1].items.len(), 2);

        // First item
        // ------------

        assert_eq!(
            blocks[1].items[0].name,
            BlockItemName::Known(KnownBlockItemName::Class)
        );

        assert_eq!(blocks[1].items[0].params, vec!["Currency"]);

        // Second item
        // ------------

        assert_eq!(
            blocks[1].items[1].name,
            BlockItemName::Known(KnownBlockItemName::SetFontSize)
        );

        assert_eq!(blocks[1].items[1].params, vec!["40"]);
    }
}
