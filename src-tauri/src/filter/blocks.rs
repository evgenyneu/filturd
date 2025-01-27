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
    pub text: String,
}

fn create_block(lines: &[&str], start: usize, end: usize) -> Block {
    let first_line = lines[start].trim();
    let is_show = first_line.starts_with("Show");

    Block {
        is_show,
        position: BlockPosition {
            start_line: start,
            end_line: end,
        },
        text: lines[start..=end].join("\n"),
    }
}

pub fn parse_blocks(content: &str) -> Result<Vec<Block>, Box<dyn Error>> {
    let lines: Vec<&str> = content.lines().collect();
    let mut blocks = Vec::new();
    let mut current_block_start = None;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with("Show") || trimmed.starts_with("Hide") {
            // If we already have a block in progress, finish it
            if let Some(start) = current_block_start {
                blocks.push(create_block(&lines, start, i - 1));
            }
            current_block_start = Some(i);
        }
    }

    // Handle the last block
    if let Some(start) = current_block_start {
        blocks.push(create_block(&lines, start, lines.len() - 1));
    }

    Ok(blocks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_blocks() {
        let content = r#"Show
    BaseType == "Mirror of Kalandra"
    SetFontSize 45

 Hide
BaseType == "Scroll of Wisdom"
      SetFontSize 18
Show
    Class "Currency"
    SetFontSize 40"#;

        let blocks = parse_blocks(content).unwrap();

        assert_eq!(blocks.len(), 3);

        // First block
        assert!(blocks[0].is_show);
        assert_eq!(blocks[0].position.start_line, 0);
        assert_eq!(blocks[0].position.end_line, 3);

        let expected_text = r#"Show
    BaseType == "Mirror of Kalandra"
    SetFontSize 45
"#;

        assert_eq!(blocks[0].text, expected_text);

        // Second block
        assert!(!blocks[1].is_show);
        assert_eq!(blocks[1].position.start_line, 4);
        assert_eq!(blocks[1].position.end_line, 6);

        let expected_text = r#" Hide
BaseType == "Scroll of Wisdom"
      SetFontSize 18"#;

        assert_eq!(blocks[1].text, expected_text);

        // Third block
        assert!(blocks[2].is_show);
        assert_eq!(blocks[2].position.start_line, 7);
        assert_eq!(blocks[2].position.end_line, 9);

        let expected_text = r#"Show
    Class "Currency"
    SetFontSize 40"#;

        assert_eq!(blocks[2].text, expected_text);
    }

    #[test]
    fn test_parse_with_comments() {
        let content = r#"# Comment at start
Show # Comment after Show
    BaseType == "Mirror" # Comment after rule

# Comment between blocks
Hide
    BaseType == "Wisdom""#;

        let blocks = parse_blocks(content).unwrap();

        assert_eq!(blocks.len(), 2);

        // First block
        assert!(blocks[0].is_show);
        assert_eq!(blocks[0].position.start_line, 1);
        assert_eq!(blocks[0].position.end_line, 4);

        let expected_text = r#"Show # Comment after Show
    BaseType == "Mirror" # Comment after rule

# Comment between blocks"#;

        assert_eq!(blocks[0].text, expected_text);

        // Second block
        assert!(!blocks[1].is_show);
        assert_eq!(blocks[1].position.start_line, 5);
        assert_eq!(blocks[1].position.end_line, 6);

        let expected_text = r#"Hide
    BaseType == "Wisdom""#;

        assert_eq!(blocks[1].text, expected_text);
    }

    #[test]
    fn test_single_block() {
        let content = r#"Show
    BaseType == "Mirror""#;

        let blocks = parse_blocks(content).unwrap();

        assert_eq!(blocks.len(), 1);
        assert!(blocks[0].is_show);
        assert_eq!(blocks[0].position.start_line, 0);
        assert_eq!(blocks[0].position.end_line, 1);

        let expected_text = r#"Show
    BaseType == "Mirror""#;

        assert_eq!(blocks[0].text, expected_text);
    }

    #[test]
    fn test_tabs() {
        let content = "\n\t\t Hide\n\t\tBaseType == \"Mirror\"";

        let blocks = parse_blocks(content).unwrap();

        assert_eq!(blocks.len(), 1);
        assert!(!blocks[0].is_show);
        assert_eq!(blocks[0].position.start_line, 1);
        assert_eq!(blocks[0].position.end_line, 2);

        let expected_text = "\t\t Hide\n\t\tBaseType == \"Mirror\"";
        assert_eq!(blocks[0].text, expected_text);
    }

    #[test]
    fn test_no_blocks() {
        let content = r#"
    nothing to see here
    "#;

        let blocks = parse_blocks(content).unwrap();

        assert_eq!(blocks.len(), 0);
    }
}
