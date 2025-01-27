use std::error::Error;

/// Parses content of a Path of Exile loot filter file into a vector of lines.
pub fn content_to_lines(content: &str) -> Result<Vec<String>, Box<dyn Error>> {
    Ok(content.lines().map(String::from).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_parsing() {
        let content = "Show\nBaseType \"Mirror of Kalandra\"\nSetFontSize 45\nHide\nBaseType \"Scroll of Wisdom\"";

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
}
