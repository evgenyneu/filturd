/// Enum for known block line names.
/// For any unknown name, Unknown is used with its original String.
#[derive(Debug, PartialEq, Eq)]
pub enum BlockLineName {
    Class,
    BaseType,
    Rarity,
    SetFontSize,
    SetTextColor,
    SetBorderColor,
    SetBackgroundColor,
    PlayAlertSound,
    PlayEffect,
    MinimapIcon,
    Unknown(String),
}

/// Represents a line in the loot filter block.
/// The `name` field is the parsed enum variant; `params` holds all following parameters.
#[derive(Debug, PartialEq, Eq)]
pub struct BlockLine {
    pub name: BlockLineName,
    pub params: Vec<String>,
}

/// Maps a token to a known BlockLineName.
/// Any token that is not one of the known names is wrapped inside BlockLineName::Unknown.
fn map_block_line_name(token: &str) -> BlockLineName {
    match token {
        "Class" => BlockLineName::Class,
        "BaseType" => BlockLineName::BaseType,
        "Rarity" => BlockLineName::Rarity,
        "SetFontSize" => BlockLineName::SetFontSize,
        "SetTextColor" => BlockLineName::SetTextColor,
        "SetBorderColor" => BlockLineName::SetBorderColor,
        "SetBackgroundColor" => BlockLineName::SetBackgroundColor,
        "PlayAlertSound" => BlockLineName::PlayAlertSound,
        "PlayEffect" => BlockLineName::PlayEffect,
        "MinimapIcon" => BlockLineName::MinimapIcon,
        other => BlockLineName::Unknown(other.to_string()),
    }
}

/// Parses a loot filter block line into a BlockLine struct.
/// Splits the line into tokens (handling double quotes) and maps the first token into a BlockLineName.
pub fn parse_block_line(line: &str) -> BlockLine {
    let tokens = tokenize_line(line);

    if tokens.is_empty() {
        return BlockLine {
            name: BlockLineName::Unknown(String::new()),
            params: Vec::new(),
        };
    }

    let name = map_block_line_name(&tokens[0]);

    // All tokens after the line name are parsed as parameters.
    let params = tokens[1..].to_vec();
    BlockLine { name, params }
}

/// Tokenizes a given line into parts.
///
/// This function splits the input string by whitespace but supports tokens wrapped in double quotes
/// so that spaces inside quotes are preserved. For example:
///
///   BaseType == "Time-Lost Emerald" "Time-Lost Ruby" "Time-Lost Sapphire"
///
/// will be tokenized into:
///   ["BaseType", "==", "Time-Lost Emerald", "Time-Lost Ruby", "Time-Lost Sapphire"]
fn tokenize_line(s: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '"' => {
                in_quotes = !in_quotes;

                // When leaving a quoted section, push the current token.
                if !in_quotes && !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();

                    // Consume any spaces immediately following.
                    while let Some(' ') = chars.peek() {
                        chars.next();
                    }
                }
            }
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            }
            _ => {
                current.push(c);
            }
        }
    }

    // If anything remains, add it as a token.
    if !current.is_empty() {
        tokens.push(current);
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_line_without_quotes() {
        let line = "Rarity Normal Magic Rare";
        let tokens = tokenize_line(line);
        assert_eq!(tokens, vec!["Rarity", "Normal", "Magic", "Rare"]);
    }

    #[test]
    fn test_tokenize_line_with_quotes() {
        let line = "BaseType == \"Time-Lost Emerald\" \"Time-Lost Ruby\" \"Time-Lost Sapphire\"";
        let tokens = tokenize_line(line);
        assert_eq!(
            tokens,
            vec![
                "BaseType",
                "==",
                "Time-Lost Emerald",
                "Time-Lost Ruby",
                "Time-Lost Sapphire"
            ]
        );
    }

    #[test]
    fn test_parse_block_line_rarity() {
        let line = "Rarity Normal Magic Rare";
        let block_line = parse_block_line(line);
        assert_eq!(block_line.name, BlockLineName::Rarity);
        assert_eq!(block_line.params, vec!["Normal", "Magic", "Rare"]);
    }

    #[test]
    fn test_parse_block_line_basetype() {
        let line = "BaseType == \"Time-Lost Emerald\" \"Time-Lost Ruby\" \"Time-Lost Sapphire\"";
        let block_line = parse_block_line(line);
        assert_eq!(block_line.name, BlockLineName::BaseType);
        assert_eq!(
            block_line.params,
            vec![
                "==",
                "Time-Lost Emerald",
                "Time-Lost Ruby",
                "Time-Lost Sapphire"
            ]
        );
    }

    #[test]
    fn test_parse_block_line_unknown() {
        let line = "UnknownName param1 param2";
        let block_line = parse_block_line(line);
        assert_eq!(
            block_line.name,
            BlockLineName::Unknown("UnknownName".to_string())
        );
        assert_eq!(block_line.params, vec!["param1", "param2"]);
    }
}
