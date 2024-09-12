use std::{
    iter::Peekable,
    str::Chars
};
use crate::error::{JsonPretError, LexerError};

#[derive(Debug, PartialEq, Clone)]
enum Token {
    String(String), // 文字列
    Number(f64),    // 数値
    Bool(bool),     // 真偽値
    Null,           // Null
    WhiteSpace,     // 空白
    LeftBrace,      // {　JSON object 開始文字
    RightBrace,     // }　JSON object 終了文字
    LeftBracket,    // [　JSON array  開始文字
    RightBracket,   // ]　JSON array  終了文字
    Comma,          // ,　JSON value  区切り文字
    Colon,          // :　"key":value 区切り文字
}

#[derive(Debug)]
struct Lexer<'a> {
    chars: Peekable<Chars<'a>>
}

impl<'a> Lexer<'a> {
    fn new(raw_str: &str) -> Lexer {
        Lexer {
            chars: raw_str.chars().peekable()
        }
    }

    /// 文字列を読み込み、マッチしたTokenを返す
    fn next_token(&mut self) -> Result<Option<Token>, JsonPretError> {
        match self.chars.peek() {
            Some(c) => match c {
                c if c.is_whitespace() || *c == '\n' => {
                    Ok(Some(Token::WhiteSpace))
                },
                'n' => Ok(self.parse_null()),
                _ => Err(JsonPretError::LexerError(
                    LexerError::new(&format!("an unexpected char {}", c))
                )),
            }, 
            None => Ok(None)
        }
    }

    fn parse_null(&mut self) -> Option<Token> {
        Some(Token::Null)
    }
}

// --- テストコード ---

#[cfg(test)]
mod tests {
    use crate::lexer::{Lexer, Token};

    #[test]
    fn test_lexer_new() {
        let expect = Lexer {
            chars: r##"{"key" : "value}"##.chars().peekable()
        };

        let actual = Lexer::new(r##"{"key" : "value}"##);
        for (ac, ec) in actual.chars.zip(expect.chars) {
            assert_eq!(ac, ec);
        }
    }

    #[test]
    fn test_next_token() {
        let expect = Token::LeftBrace;
    }
}