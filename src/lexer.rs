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
                c if c.is_whitespace() || *c == '\n' => Ok(Some(self.get_token(Token::WhiteSpace))),
                c if is_number(*c, true) => Ok(Some(self.parse_number().unwrap())),
                '{' => Ok(Some(self.get_token(Token::LeftBrace))),
                '}' => Ok(Some(self.get_token(Token::RightBrace))),
                '[' => Ok(Some(self.get_token(Token::LeftBracket))),
                ']' => Ok(Some(self.get_token(Token::RightBracket))),
                ',' => Ok(Some(self.get_token(Token::Comma))),
                ':' => Ok(Some(self.get_token(Token::Colon))),
                't' => Ok(Some(self.parse_boolean(true).unwrap())),
                'f' => Ok(Some(self.parse_boolean(false).unwrap())),
                'n' => Ok(Some(self.parse_null().unwrap())),
                _ => Err(JsonPretError::LexerError(
                    LexerError::new(&format!("an unexpected char {}", c))
                )),
            }, 
            None => Ok(None)
        }
    }

    fn get_token(&mut self, token: Token) -> Token {
        self.chars.next();
        token
    } 

    fn parse_number(&mut self) -> Result<Token, JsonPretError>{
        let mut number_str: String = String::new();
        while let Some(&c) = self.chars.peek() {
            if is_number(c, false) {
                self.chars.next();
                number_str.push(c);
            } else {
                break;
            }
        }

        match number_str.parse::<f64>() {
            Ok(number) => Ok(Token::Number(number)),
            Err(e) => Err(JsonPretError::LexerError(
                LexerError::new(&e.to_string()),
            ))
        }
    }

    fn parse_boolean(&mut self, b: bool) -> Result<Token, JsonPretError> {
        // true の場合は4文字、falseの場合は5文字取得
        let string: String =  match b {
            true => self.get_string(4),
            false => self.get_string(5),
        };

        if &string == "true" || &string == "false" {
            Ok(Token::Bool(b))
        } else {
            Err(JsonPretError::LexerError(
                LexerError::new(&format!("'{string}' is syntactically incorrect."))
            ))
        }
    }

    fn parse_null(&mut self) -> Result<Token, JsonPretError> {
        // 4文字取得
        let string: String = self.get_string(4);
        
        // 読み込んだ文字が "null" の場合、Token を返す。
        if &string == "null" {
            Ok(Token::Null)
        } else {
            Err(JsonPretError::LexerError(
                LexerError::new(&format!("'{string}' is syntactically incorrect."))
            ))
        }
    }

    /// 指定した文字数を取得する
    fn get_string(&mut self, length: usize) -> String {
        let mut str: String = String::new();
        for _ in 0..length {
            match self.chars.next() {
                Some(c) => str.push(c),
                None => {}
            }
        }
        str
    }
}

/// Numberで使用される文字([0-9], +, -, .)かどうかを返す。  
fn is_number(c: char, is_prefix: bool) -> bool {
    if is_prefix {
        c.is_numeric() || matches!(c, '+' | '-' | '.')
    } else {
        c.is_numeric() || matches!(c, '+' | '-' | 'e' | 'E' | '.')

    }
}

// --- テストコード ---

#[cfg(test)]
mod tests {
    use crate::{error::{JsonPretError, LexerError}, lexer::{Lexer, Token, is_number}};

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

    // #[test]
    // fn test_next_token() {
    //     let expect = Token::LeftBrace;
    // }

    #[test]
    fn test_parse_number() {
        let expect = Token::Number(1.0);
        let mut lexer = Lexer::new("1.0");
        let actual = lexer.parse_number().unwrap();

        assert_eq!(actual, expect)
    }
 
    #[test]
    fn test_parse_boolean() {
        // true のケース
        let expect_true = Token::Bool(true);
        let mut lexer_true = Lexer::new("true");
        let actual_true = lexer_true.parse_boolean(true).unwrap();
        assert_eq!(actual_true, expect_true);

        // false のケース
        let expect_false = Token::Bool(false);
        let mut lexer_false = Lexer::new("false");
        let actual_false = lexer_false.parse_boolean(false).unwrap();
        assert_eq!(actual_false, expect_false);

        // t で true 以外の文字のケース(エラー)
        let err_str_t = "test";
        let expect_err_t = JsonPretError::LexerError(
            LexerError::new(&format!("'{err_str_t}' is syntactically incorrect."))
        );
        let mut lexer_err_t = Lexer::new(&err_str_t);
        let actual_err_t = lexer_err_t.parse_boolean(true).unwrap_err();
        assert_eq!(actual_err_t, expect_err_t);

        // f で false 以外の文字のケース(エラー)
        let err_str_f = "fight";
        let expect_err_f = JsonPretError::LexerError(
            LexerError::new(&format!("'{err_str_f}' is syntactically incorrect."))
        );
        let mut lexer_err_f = Lexer::new(&err_str_f);
        let actual_err_f = lexer_err_f.parse_boolean(false).unwrap_err();
        assert_eq!(actual_err_f, expect_err_f);
    }

    #[test]
    fn test_parse_null() {
        let expect = Token::Null;
        let mut lexer = Lexer::new("null");        
        let actual = lexer.parse_null().unwrap();

        assert_eq!(actual, expect);
    }

    #[test]
    fn test_get_string() {
        let expect = String::from("test");
        let mut lexer = Lexer::new("test");
        let actual = lexer.get_string(4);
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_is_number() {
        assert_eq!(is_number('1', true), true);
        assert_eq!(is_number('+', true), true);
        assert_eq!(is_number('e', true), false);
        assert_eq!(is_number('e', false), true);
        assert_eq!(is_number('a', false), false);
    }
}