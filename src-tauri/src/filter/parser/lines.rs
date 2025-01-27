use super::remove_comment::remove_comment;
use std::error::Error;

/// Parses content of a loot filter file into a vector of lines.
pub fn content_to_lines(content: &str) -> Result<Vec<String>, Box<dyn Error>> {
    Ok(content
        .lines()
        .map(|line| remove_comment(line).trim().to_string())
        .filter(|line| !line.is_empty())
        .collect())
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

        let lines = content_to_lines(content).unwrap();

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
        let lines = content_to_lines(content).unwrap();
        assert!(lines.is_empty());
    }

    #[test]
    fn test_tabs_and_spaces() {
        let content = "\t  Show  \t\n\t\tBaseType \"Mirror\"  \t";
        let lines = content_to_lines(content).unwrap();
        assert_eq!(lines, vec!["Show", "BaseType \"Mirror\""]);
    }

    #[test]
    fn test_empty_lines() {
        let content = r#"

        Show

  BaseType "Mirror"

"#;

        let lines = content_to_lines(content).unwrap();
        assert_eq!(lines, vec!["Show", "BaseType \"Mirror\""]);
    }
}
