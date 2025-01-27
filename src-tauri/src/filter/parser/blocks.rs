use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum BlockName {
    Show,
    Hide,
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub name: BlockName,
    pub lines: Vec<String>,
}

fn create_block(lines: &[String], start: usize, end: usize) -> Block {
    let name = match lines[start].as_str() {
        "Show" => BlockName::Show,
        "Hide" => BlockName::Hide,
        _ => panic!("Invalid block name"), // We might want to handle this more gracefully later
    };

    Block {
        name,
        lines: lines[start + 1..=end].to_vec(),
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
