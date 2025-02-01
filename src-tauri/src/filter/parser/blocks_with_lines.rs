#[derive(Debug, PartialEq)]
pub enum BlockName {
    Show,
    Hide,
}

const BLOCK_NAMES: [&str; 2] = ["Show", "Hide"];

impl BlockName {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "Show" => Some(BlockName::Show),
            "Hide" => Some(BlockName::Hide),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BlockWithLines {
    pub name: BlockName,
    pub lines: Vec<String>,
}

fn is_block_start(line: &str) -> Option<BlockName> {
    BLOCK_NAMES
        .iter()
        .find(|&&name| line.starts_with(name))
        .and_then(|&name| BlockName::from_str(name))
}

fn try_add_block_if_exists(
    blocks: &mut Vec<BlockWithLines>,
    lines: &[String],
    start: Option<(usize, BlockName)>,
    end: usize,
) {
    let Some((start, block_name)) = start else {
        return;
    };

    let line_count = end.saturating_sub(start);

    if line_count == 0 {
        // empty block
        return;
    }

    blocks.push(BlockWithLines {
        name: block_name,
        lines: lines[start + 1..=end].to_vec(),
    });
}

pub fn parse_lines(lines: &[String]) -> Vec<BlockWithLines> {
    let mut blocks = Vec::new();
    let mut current_block_start = None;

    for (i, line) in lines.iter().enumerate() {
        let Some(block_name) = is_block_start(line) else {
            continue;
        };

        try_add_block_if_exists(&mut blocks, lines, current_block_start, i.saturating_sub(1));
        current_block_start = Some((i, block_name));
    }

    try_add_block_if_exists(
        &mut blocks,
        lines,
        current_block_start,
        lines.len().saturating_sub(1),
    );

    blocks
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

        let blocks = parse_lines(&content);

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

        let blocks = parse_lines(&content);

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

        let blocks = parse_lines(&content);

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

        let blocks = parse_lines(&content);

        assert_eq!(blocks.len(), 0);
    }

    #[test]
    fn test_empty_content() {
        let blocks = parse_lines(&[]);

        assert_eq!(blocks.len(), 0);
    }
}
