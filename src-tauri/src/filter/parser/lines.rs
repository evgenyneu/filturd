use super::remove_comment::remove_comment;

/// Parses content of a loot filter file into a vector of lines.
pub fn content_to_lines(content: &str) -> Vec<String> {
    let mut lines = Vec::new();

    for line in content.lines() {
        let line = remove_comment(line);
        let line = line.trim();

        if !line.is_empty() {
            lines.push(line.to_string());
        }
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_parsing() {
        let content = r#"Show
BaseType "Mirror of Kalandra" # Very rare
SetFontSize 45 # Big size
# Hiding
Hide # Not interesting
BaseType "Scroll of Wisdom""#;

        let lines = content_to_lines(content);

        assert_eq!(
            lines,
            vec![
                "Show",
                "BaseType \"Mirror of Kalandra\"",
                "SetFontSize 45",
                "Hide",
                "BaseType \"Scroll of Wisdom\""
            ]
        );
    }

    #[test]
    fn test_empty_content() {
        let content = "";
        let lines = content_to_lines(content);
        assert!(lines.is_empty());
    }

    #[test]
    fn test_tabs_and_spaces() {
        let content = "\t  Show  \t\n\t\tBaseType \"Mirror\"  \t";
        let lines = content_to_lines(content);
        assert_eq!(lines, vec!["Show", "BaseType \"Mirror\""]);
    }

    #[test]
    fn test_empty_lines() {
        let content = r#"

        Show

  BaseType "Mirror"

"#;

        let lines = content_to_lines(content);
        assert_eq!(lines, vec!["Show", "BaseType \"Mirror\""]);
    }
}
