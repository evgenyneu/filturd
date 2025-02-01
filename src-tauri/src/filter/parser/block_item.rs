use std::fmt;
use std::str::FromStr;

/// This enum holds only the known block line names.
/// The `strum_macros::EnumVariantNames` derive makes available the list
/// of known variants (using the same name as in the source file).
#[derive(Debug, PartialEq, Eq, strum_macros::EnumString)]
#[strum(serialize_all = "PascalCase")]
pub enum KnownBlockItemName {
    AreaLevel,
    BaseArmour,
    BaseEnergyShield,
    BaseEvasion,
    BaseType,
    Class,
    Continue,
    Corrupted,
    CustomAlertSound,
    DisableDropSound,
    DropLevel,
    Height,
    Identified,
    ItemLevel,
    MinimapIcon,
    Mirrored,
    PlayAlertSound,
    PlayEffect,
    Quality,
    Rarity,
    SetBackgroundColor,
    SetBorderColor,
    SetFontSize,
    SetTextColor,
    Sockets,
    StackSize,
    Width,
    WaystoneTier,
}

/// BlockItemName wraps known names (using the enum above) and falls back to Unknown.
#[derive(Debug, PartialEq, Eq)]
pub enum BlockItemName {
    Known(KnownBlockItemName),
    Unknown(String),
}

impl FromStr for BlockItemName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Try parsing into a KnownBlockItemName with strum.
        if let Ok(known) = s.parse::<KnownBlockItemName>() {
            Ok(BlockItemName::Known(known))
        } else {
            Ok(BlockItemName::Unknown(s.to_owned()))
        }
    }
}

/// Represents a line in the loot filter block.
/// The `name` field is the parsed enum variant; `params` holds all following parameters.
#[derive(Debug, PartialEq, Eq)]
pub struct BlockItem {
    pub name: BlockItemName,
    pub params: Vec<String>,
}

/// Parses a loot filter block line into a BlockItem struct.
/// Splits the line into tokens (handling double quotes) and maps the first token into a BlockItemName.
pub fn parse_block_item(line: &str) -> Result<BlockItem, ParseError> {
    let tokens = tokenize_line(line);

    if tokens.is_empty() {
        return Err(ParseError::EmptyLine);
    }

    let name = tokens[0]
        .parse()
        .unwrap_or(BlockItemName::Unknown(tokens[0].clone()));

    // All tokens after the item name are parsed as parameters.
    let params = tokens[1..].to_vec();
    Ok(BlockItem { name, params })
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

// New error type for parsing errors.
#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    EmptyLine,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EmptyLine => write!(f, "The line is empty"),
        }
    }
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

        assert_eq!(
            block_item.name,
            BlockItemName::Known(KnownBlockItemName::Rarity)
        );

        assert_eq!(block_item.params, vec!["Normal", "Magic", "Rare"]);
    }

    #[test]
    fn test_parse_block_line_basetype() {
        let line = "BaseType == \"Time-Lost Emerald\" \"Time-Lost Ruby\" \"Time-Lost Sapphire\"";
        let block_item = parse_block_item(line).unwrap();

        assert_eq!(
            block_item.name,
            BlockItemName::Known(KnownBlockItemName::BaseType)
        );

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

        assert_eq!(
            block_item.name,
            BlockItemName::Unknown("UnknownName".to_string())
        );

        assert_eq!(block_item.params, vec!["param1", "param2"]);
    }

    #[test]
    fn test_parse_block_line_empty() {
        let line = "";
        let result = parse_block_item(line);
        assert_eq!(result, Err(ParseError::EmptyLine));
    }
}
