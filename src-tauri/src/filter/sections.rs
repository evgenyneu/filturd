use std::error::Error;

#[derive(Debug, PartialEq)]
pub struct SectionPosition {
    pub start_line: usize,
    pub end_line: usize,
}

#[derive(Debug, PartialEq)]
pub struct Section {
    pub is_show: bool,
    pub position: SectionPosition,
    pub text: String,
}

fn create_section(lines: &[&str], start: usize, end: usize) -> Section {
    let first_line = lines[start].trim();
    let is_show = first_line.starts_with("Show");

    Section {
        is_show,
        position: SectionPosition {
            start_line: start,
            end_line: end,
        },
        text: lines[start..=end].join("\n"),
    }
}

pub fn parse_sections(content: &str) -> Result<Vec<Section>, Box<dyn Error>> {
    let lines: Vec<&str> = content.lines().collect();
    let mut sections = Vec::new();
    let mut current_section_start = None;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with("Show") || trimmed.starts_with("Hide") {
            // If we already have a section in progress, finish it
            if let Some(start) = current_section_start {
                sections.push(create_section(&lines, start, i - 1));
            }
            current_section_start = Some(i);
        }
    }

    // Handle the last section
    if let Some(start) = current_section_start {
        sections.push(create_section(&lines, start, lines.len() - 1));
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

        let sections = parse_sections(content).unwrap();

        assert_eq!(sections.len(), 3);

        // First section
        assert!(sections[0].is_show);
        assert_eq!(sections[0].position.start_line, 0);
        assert_eq!(sections[0].position.end_line, 3);

        let expected_text = r#"Show
    BaseType == "Mirror of Kalandra"
    SetFontSize 45
"#;

        assert_eq!(sections[0].text, expected_text);

        // Second section
        assert!(!sections[1].is_show);
        assert_eq!(sections[1].position.start_line, 4);
        assert_eq!(sections[1].position.end_line, 6);

        let expected_text = r#" Hide
BaseType == "Scroll of Wisdom"
      SetFontSize 18"#;

        assert_eq!(sections[1].text, expected_text);

        // Third section
        assert!(sections[2].is_show);
        assert_eq!(sections[2].position.start_line, 7);
        assert_eq!(sections[2].position.end_line, 9);

        let expected_text = r#"Show
    Class "Currency"
    SetFontSize 40"#;

        assert_eq!(sections[2].text, expected_text);
    }

    #[test]
    fn test_parse_with_comments() {
        let content = r#"# Comment at start
Show # Comment after Show
    BaseType == "Mirror" # Comment after rule

# Comment between sections
Hide
    BaseType == "Wisdom""#;

        let sections = parse_sections(content).unwrap();

        assert_eq!(sections.len(), 2);

        // First section
        assert!(sections[0].is_show);
        assert_eq!(sections[0].position.start_line, 1);
        assert_eq!(sections[0].position.end_line, 4);

        let expected_text = r#"Show # Comment after Show
    BaseType == "Mirror" # Comment after rule

# Comment between sections"#;

        assert_eq!(sections[0].text, expected_text);

        // Second section
        assert!(!sections[1].is_show);
        assert_eq!(sections[1].position.start_line, 5);
        assert_eq!(sections[1].position.end_line, 6);

        let expected_text = r#"Hide
    BaseType == "Wisdom""#;

        assert_eq!(sections[1].text, expected_text);
    }

    #[test]
    fn test_single_section() {
        let content = r#"Show
    BaseType == "Mirror""#;

        let sections = parse_sections(content).unwrap();

        assert_eq!(sections.len(), 1);
        assert!(sections[0].is_show);
        assert_eq!(sections[0].position.start_line, 0);
        assert_eq!(sections[0].position.end_line, 1);

        let expected_text = r#"Show
    BaseType == "Mirror""#;

        assert_eq!(sections[0].text, expected_text);
    }

    #[test]
    fn test_tabs() {
        let content = "\n\t\t Hide\n\t\tBaseType == \"Mirror\"";

        let sections = parse_sections(content).unwrap();

        assert_eq!(sections.len(), 1);
        assert!(!sections[0].is_show);
        assert_eq!(sections[0].position.start_line, 1);
        assert_eq!(sections[0].position.end_line, 2);

        let expected_text = "\t\t Hide\n\t\tBaseType == \"Mirror\"";
        assert_eq!(sections[0].text, expected_text);
    }

    #[test]
    fn test_no_sections() {
        let content = r#"
    nothing to see here
    "#;

        let sections = parse_sections(content).unwrap();

        assert_eq!(sections.len(), 0);
    }
}
