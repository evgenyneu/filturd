use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum SectionType {
    Show,
    Hide,
}

#[derive(Debug, PartialEq)]
pub struct SectionPosition {
    pub start_line: usize,
    pub end_line: usize,
}

#[derive(Debug, PartialEq)]
pub struct FileSection {
    pub index: usize,
    pub type_: SectionType,
    pub position: SectionPosition,
    pub text: String,
}

pub fn parse_filter_file(content: &str) -> Result<Vec<FileSection>, Box<dyn Error>> {
    let lines: Vec<&str> = content.lines().collect();
    let mut sections = Vec::new();
    let mut current_section_start = None;
    let mut section_index = 0;

    // Helper to create section from line range
    let create_section = |start: usize, end: usize, index: usize| -> FileSection {
        let first_line = lines[start].trim();
        let section_type = if first_line.starts_with("Show") {
            SectionType::Show
        } else {
            SectionType::Hide
        };

        FileSection {
            index,
            type_: section_type,
            position: SectionPosition {
                start_line: start,
                end_line: end,
            },
            text: lines[start..=end].join("\n"),
        }
    };

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with("Show") || trimmed.starts_with("Hide") {
            // If we already have a section in progress, finish it
            if let Some(start) = current_section_start {
                sections.push(create_section(start, i - 1, section_index));
                section_index += 1;
            }
            current_section_start = Some(i);
        }
    }

    // Handle the last section
    if let Some(start) = current_section_start {
        sections.push(create_section(start, lines.len() - 1, section_index));
    }

    Ok(sections)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_sections() {
        let content = r#"Show
    BaseType == "Mirror of Kalandra"
    SetFontSize 45

Hide
    BaseType == "Scroll of Wisdom"
    SetFontSize 18

Show
    Class "Currency"
    SetFontSize 40"#;

        let sections = parse_filter_file(content).unwrap();

        assert_eq!(sections.len(), 3);

        // First section
        assert_eq!(sections[0].index, 0);
        assert_eq!(sections[0].type_, SectionType::Show);
        assert_eq!(sections[0].position.start_line, 0);
        assert_eq!(sections[0].position.end_line, 2);

        // Second section
        assert_eq!(sections[1].index, 1);
        assert_eq!(sections[1].type_, SectionType::Hide);
        assert_eq!(sections[1].position.start_line, 4);
        assert_eq!(sections[1].position.end_line, 6);

        // Third section
        assert_eq!(sections[2].index, 2);
        assert_eq!(sections[2].type_, SectionType::Show);
        assert_eq!(sections[2].position.start_line, 8);
        assert_eq!(sections[2].position.end_line, 10);
    }

    #[test]
    fn test_parse_with_comments() {
        let content = r#"# Comment at start
Show # Comment after Show
    BaseType == "Mirror" # Comment after rule

# Comment between sections
Hide
    BaseType == "Wisdom""#;

        let sections = parse_filter_file(content).unwrap();

        assert_eq!(sections.len(), 2);
        assert_eq!(sections[0].type_, SectionType::Show);
        assert_eq!(sections[1].type_, SectionType::Hide);
        assert!(sections[0].text.contains("Mirror"));
        assert!(sections[1].text.contains("Wisdom"));
    }

    #[test]
    fn test_parse_with_empty_lines() {
        let content = r#"

Show
    BaseType == "Mirror"

Hide
    BaseType == "Wisdom"

"#;

        let sections = parse_filter_file(content).unwrap();

        assert_eq!(sections.len(), 2);
        assert_eq!(sections[0].type_, SectionType::Show);
        assert_eq!(sections[1].type_, SectionType::Hide);
    }

    #[test]
    fn test_single_section() {
        let content = "Show\n    BaseType == \"Mirror\"";

        let sections = parse_filter_file(content).unwrap();

        assert_eq!(sections.len(), 1);
        assert_eq!(sections[0].type_, SectionType::Show);
        assert_eq!(sections[0].index, 0);
        assert_eq!(sections[0].position.start_line, 0);
        assert_eq!(sections[0].position.end_line, 1);
    }
}
