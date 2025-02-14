use crate::filter::parser::errors::ParseError;
use ts_rs::TS;

/// Represents a line in the loot filter block.
/// The `name` field is the line name; `params` holds all following parameters.
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, Clone, TS)]
#[ts(export)]
pub struct BlockItem {
    pub name: String,
    pub params: Vec<String>,
}

/// Parses a loot filter block line into a BlockItem struct.
/// Splits the line into tokens (handling double quotes) and maps the first token into the name.
pub fn parse_block_item(line: &str) -> Result<BlockItem, ParseError> {
    let tokens = tokenize_line(line);

    if tokens.is_empty() {
        return Err(ParseError::EmptyLine);
    }

    Ok(BlockItem {
        name: tokens[0].clone(),
        params: tokens[1..].to_vec(),
    })
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
        let block_item = parse_block_item(line).unwrap();

        assert_eq!(block_item.name, "Rarity");
        assert_eq!(block_item.params, vec!["Normal", "Magic", "Rare"]);
    }

    #[test]
    fn test_parse_block_line_basetype() {
        let line = "BaseType == \"Time-Lost Emerald\" \"Time-Lost Ruby\" \"Time-Lost Sapphire\"";
        let block_item = parse_block_item(line).unwrap();

        assert_eq!(block_item.name, "BaseType");
        assert_eq!(
            block_item.params,
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
        let block_item = parse_block_item(line).unwrap();

        assert_eq!(block_item.name, "UnknownName");
        assert_eq!(block_item.params, vec!["param1", "param2"]);
    }

    #[test]
    fn test_parse_block_line_empty() {
        let line = "";
        let result = parse_block_item(line);
        assert_eq!(result, Err(ParseError::EmptyLine));
    }

    #[test]
    fn test_block_item_serialization() {
        let block_item = BlockItem {
            name: "Rarity".to_string(),
            params: vec!["Normal".to_string(), "Magic".to_string(), "Rare".to_string()],
        };

        let json = serde_json::to_string(&block_item).unwrap();
        let decoded: BlockItem = serde_json::from_str(&json).unwrap();
        assert_eq!(block_item, decoded);
    }
}
