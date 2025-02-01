use crate::filter::parser::block_item::{parse_block_item, BlockItem, ParseError};
use crate::filter::parser::blocks_with_lines::{BlockName, BlockWithLines};

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
    use crate::filter::parser::block_item::{BlockItemName, KnownBlockItemName, ParseError};
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

        // First block tests
        assert_eq!(blocks[0].name, BlockName::Show);
        assert_eq!(blocks[0].items.len(), 2);

        // First block first item should be known with "BaseType"
        match &blocks[0].items[0].name {
            BlockItemName::Known(name) => {
                assert_eq!(name, &KnownBlockItemName::BaseType);
            }
            _ => panic!("Expected a Known BlockItemName"),
        }

        // Second block tests
        assert_eq!(blocks[1].name, BlockName::Hide);
        assert_eq!(blocks[1].items.len(), 2);
    }

    #[test]
    fn test_parse_blocks_error_empty_line() {
        // Create a block with an empty line that should trigger an error
        let blocks_with_lines = vec![BlockWithLines {
            name: BlockName::Show,
            lines: vec![
                "".to_string(), // This line is empty, expecting error.
                "SetFontSize 45".to_string(),
            ],
        }];

        let result = parse_block_with_lines(&blocks_with_lines);
        assert!(result.is_err());

        match result {
            Err(ParseError::EmptyLine) => {}
            _ => panic!("Expected ParseError::EmptyLine"),
        }
    }
}
