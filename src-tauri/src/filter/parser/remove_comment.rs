/// Removes comments from a line. Comments start with # symbol.
pub fn remove_comment(line: &str) -> String {
    match line.split_once('#') {
        Some((content, _)) => content.to_string(),
        None => line.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_with_comment() {
        assert_eq!(remove_comment("Show # This is a comment"), "Show ");
        assert_eq!(remove_comment("Hide    # Another comment"), "Hide    ");
    }

    #[test]
    fn test_line_without_comment() {
        assert_eq!(remove_comment("Show"), "Show");
        assert_eq!(remove_comment("BaseType \"Mirror\""), "BaseType \"Mirror\"");
    }

    #[test]
    fn test_only_comment() {
        assert_eq!(remove_comment("# Pure comment"), "");
    }

    #[test]
    fn test_multiple_hash_symbols() {
        assert_eq!(
            remove_comment("SetFontSize 45 # First comment # Not a comment"),
            "SetFontSize 45 "
        );
    }

    #[test]
    fn test_empty_line() {
        assert_eq!(remove_comment(""), "");
    }
}
