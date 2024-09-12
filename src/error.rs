use std::fmt::{self, Display};

/// Json Prettier で発生するエラーを扱う enum
#[derive(Debug, PartialEq)]
pub enum JsonPretError {
    LexerError(LexerError),
}

impl Display for JsonPretError {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonPretError::LexerError(e) => write!(f, "LexerError: {}", e.message)
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


// --- テストコード ---

#[cfg(test)]
mod tests {
    use crate::error::LexerError;
    #[test]
    fn test_lexer_error_new() {
        let expect = LexerError {
            message: "Error message".to_string()
        };
        let actual = LexerError::new("Error message");

        assert_eq!(actual, expect);
    }
}