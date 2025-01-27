use std::error::Error;

#[derive(Debug, PartialEq)]
pub struct BlockPosition {
    pub start_line: usize,
    pub end_line: usize,
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub is_show: bool,
    pub position: BlockPosition,
    pub lines: Vec<String>,
}

fn create_block(lines: &[String], start: usize, end: usize) -> Block {
    let is_show = lines[start].starts_with("Show");

    Block {
        is_show,
        position: BlockPosition {
            start_line: start,
            end_line: end,
        },
        lines: lines[start..=end].to_vec(),
    }
}

pub fn parse_blocks(lines: &[String]) -> Result<Vec<Block>, Box<dyn Error>> {
    let mut blocks = Vec::new();
    let mut current_block_start = None;

    for (i, line) in lines.iter().enumerate() {
        if line.starts_with("Show") || line.starts_with("Hide") {
            if let Some(start) = current_block_start {
                blocks.push(create_block(lines, start, i - 1));
            }
            current_block_start = Some(i);
        }
    }

    if let Some(start) = current_block_start {
        blocks.push(create_block(lines, start, lines.len() - 1));
    }

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
        assert!(blocks[0].is_show);
        assert_eq!(blocks[0].position.start_line, 0);
        assert_eq!(blocks[0].position.end_line, 2);
        assert_eq!(
            blocks[0].lines,
            vec![
                "Show",
                "BaseType == \"Mirror of Kalandra\"",
                "SetFontSize 45"
            ]
        );

        // Second block
        assert!(!blocks[1].is_show);
        assert_eq!(blocks[1].position.start_line, 3);
        assert_eq!(blocks[1].position.end_line, 5);
        assert_eq!(
            blocks[1].lines,
            vec!["Hide", "BaseType == \"Scroll of Wisdom\"", "SetFontSize 18"]
        );

        // Third block
        assert!(blocks[2].is_show);
        assert_eq!(blocks[2].position.start_line, 6);
        assert_eq!(blocks[2].position.end_line, 8);
        assert_eq!(
            blocks[2].lines,
            vec!["Show", "Class \"Currency\"", "SetFontSize 40"]
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
        assert!(blocks[0].is_show);
        assert_eq!(blocks[0].position.start_line, 0);
        assert_eq!(blocks[0].position.end_line, 1);
        assert_eq!(blocks[0].lines, vec!["Show", "BaseType == \"Mirror\""]);
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
