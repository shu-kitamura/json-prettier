use std::fmt::{self, Display};

/// Json Prettier で発生するエラーを扱う enum
#[derive(Debug, PartialEq)]
pub enum JsonPretError {
    LexerError(LexerError),
    ParserError(ParserError),
}

impl Display for JsonPretError {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonPretError::LexerError(e) => write!(f, "LexerError: {}", e.message),
            JsonPretError::ParserError(e) => write!(f, "ParserError: {}", e.message)
        }
    }
}


/// 字句解析中に発生したエラー
#[derive(Debug, PartialEq)]
pub struct LexerError {
    /// エラーメッセージ
    pub message: String,
}

impl LexerError {
    pub fn new(msg: &str) -> LexerError {
        LexerError {
            message: msg.to_string(),
        }
    }
}

/// パース中に発生したエラー
#[derive(Debug, PartialEq)]
pub struct ParserError {
    /// エラーメッセージ
    pub message: String,
}

impl ParserError {
    pub fn new(msg: &str) -> ParserError {
        ParserError {
            message: msg.to_string(),
        }
    }
}


// --- テストコード ---

#[cfg(test)]
mod tests {
    use crate::error::*;
    #[test]
    fn test_lexer_error_new() {
        let expect: LexerError = LexerError {
            message: "Error message".to_string()
        };
        let actual: LexerError = LexerError::new("Error message");

        assert_eq!(actual, expect);
    }

    #[test]
    fn test_parser_error_new() {
        let expect: ParserError = ParserError {
            message: "Error message".to_string()
        };
        let actual: ParserError = ParserError::new("Error message");

        assert_eq!(actual, expect);
    }
}