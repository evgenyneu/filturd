use std::error::Error;
use strum::{AsRefStr, EnumIter, IntoEnumIterator};

#[derive(Debug, PartialEq, EnumIter, AsRefStr)]
pub enum BlockName {
    Show,
    Hide,
}

impl BlockName {
    fn from_str(s: &str) -> Option<Self> {
        BlockName::iter().find(|name| name.as_ref() == s)
    }
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub name: BlockName,
    pub lines: Vec<String>,
}

fn create_block(lines: &[String], start: usize, end: usize) -> Block {
    let name = BlockName::from_str(lines[start].as_str()).expect("Invalid block name"); // We already validated this in is_block_start

    Block {
        name,
        lines: lines[start + 1..=end].to_vec(),
    }
}

fn is_block_start(line: &str) -> bool {
    BlockName::iter().any(|name| line.starts_with(name.as_ref()))
}

fn try_add_block_if_exists(
    blocks: &mut Vec<Block>,
    lines: &[String],
    start: Option<usize>,
    end: usize,
) {
    if let Some(start) = start {
        let line_count = end.saturating_sub(start);

        if line_count > 0 {
            blocks.push(create_block(lines, start, end));
        }
    }
}

pub fn parse_blocks(lines: &[String]) -> Result<Vec<Block>, Box<dyn Error>> {
    let mut blocks = Vec::new();
    let mut current_block_start = None;

    for (i, line) in lines.iter().enumerate() {
        if is_block_start(line) {
            try_add_block_if_exists(&mut blocks, lines, current_block_start, i.saturating_sub(1));
            current_block_start = Some(i);
        }
    }

    try_add_block_if_exists(
        &mut blocks,
        lines,
        current_block_start,
        lines.len().saturating_sub(1),
    );

    Ok(blocks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_blocks() {
        let content: Vec<String> = vec![
            "Show",
            "BaseType == \"Mirror of Kalandra\"",
            "SetFontSize 45",
            "Hide",
            "BaseType == \"Scroll of Wisdom\"",
            "SetFontSize 18",
            "Show",
            "Class \"Currency\"",
            "SetFontSize 40",
        ]
        .into_iter()
        .map(Into::into)
        .collect();

        let blocks = parse_blocks(&content).unwrap();

        assert_eq!(blocks.len(), 3);

        // First block
        assert_eq!(blocks[0].name, BlockName::Show);
        assert_eq!(
            blocks[0].lines,
            vec!["BaseType == \"Mirror of Kalandra\"", "SetFontSize 45"]
        );

        // Second block
        assert_eq!(blocks[1].name, BlockName::Hide);
        assert_eq!(
            blocks[1].lines,
            vec!["BaseType == \"Scroll of Wisdom\"", "SetFontSize 18"]
        );

        // Third block
        assert_eq!(blocks[2].name, BlockName::Show);
        assert_eq!(
            blocks[2].lines,
            vec!["Class \"Currency\"", "SetFontSize 40"]
        );
    }

    #[test]
    fn test_single_block() {
        let content: Vec<String> = vec!["Show", "BaseType == \"Mirror\""]
            .into_iter()
            .map(Into::into)
            .collect();

        let blocks = parse_blocks(&content).unwrap();

        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].name, BlockName::Show);
        assert_eq!(blocks[0].lines, vec!["BaseType == \"Mirror\""]);
    }

    #[test]
    fn test_ignore_block_without_content() {
        let content: Vec<String> = vec!["Show", "Hide", "BaseType == \"Mirror\""]
            .into_iter()
            .map(Into::into)
            .collect();

        let blocks = parse_blocks(&content).unwrap();

        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].name, BlockName::Hide);
        assert_eq!(blocks[0].lines, vec!["BaseType == \"Mirror\""]);
    }

    #[test]
    fn test_no_blocks() {
        let content: Vec<String> = vec!["nothing to see here", "another one"]
            .into_iter()
            .map(Into::into)
            .collect();

        let blocks = parse_blocks(&content).unwrap();

        assert_eq!(blocks.len(), 0);
    }

    #[test]
    fn test_empty_content() {
        let blocks = parse_blocks(&[]).unwrap();

        assert_eq!(blocks.len(), 0);
    }
}
