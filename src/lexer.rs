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

    fn lexical_analyze(&mut self) -> Result<Vec<Token>, JsonPretError> {
        let mut tokens: Vec<Token> = vec![];
        while let Some(token) = self.next_token().unwrap() {
            match token {
                Token::WhiteSpace => {}
                _ => tokens.push(token),
            }
        }
        Ok(tokens)
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
                '"' => Ok(Some(self.parse_string().unwrap())),
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

    fn parse_string(&mut self) -> Result<Token, JsonPretError>{
        self.chars.next(); // 最初の " の分を進める。

        let mut utf16: Vec<u16> = vec![];
        let mut string: String = String::new();

        while let Some(c) = self.chars.next() {
            match c {
                '\\' => {
                    let escaped_c = self.chars.next().unwrap();
                    println!("{escaped_c}");
                    match escaped_c {
                        '"' | '\\' | '/' | 'b' | 'f' | 'n' | 'r' | 't' => {
                            // エスケープ文字の時の処理
                            match self.push_utf16(&mut string, &mut utf16) {
                                Ok(()) => string.push_str(&format!("\\{escaped_c}")),
                                Err(e) => return Err(e)
                            }
                        }
                        'u' => {
                            // utf16の時の処理
                            let code_point = match self.get_code_point() {
                                Ok(point) => point,
                                Err(e) => return Err(e),
                            };
                            utf16.push(code_point);
                        }
                        _ => return Err(JsonPretError::LexerError(
                            LexerError::new(&format!("an unexpected escaped char {escaped_c}"))
                        ))
                    }
                }
                '\"' => {
                    // 文字列パースの終了時の処理
                    match self.push_utf16(&mut string, &mut utf16) {
                        Ok(_) => break,
                        Err(e) => return Err(e)
                    }
                },
                _ => {
                    // 普通の文字の時の処理
                    match self.push_utf16(&mut string, &mut utf16) {
                        Ok(_) => string.push(c),
                        Err(e) => return Err(e)
                    }

                }
            }
        }
        Ok(Token::String(string))
    }

    /// 指定した文字数を取得する
    fn get_string(&mut self, length: usize) -> String {
        let mut string: String = String::new();
        for _ in 0..length {
            match self.chars.next() {
                Some(c) => string.push(c),
                None => {}
            }
        }
        string
    }

    /// utf16のコードポイントを取得する
    fn get_code_point(&mut self) -> Result<u16, JsonPretError> {
        let hexs = (0..4).filter_map(|_| {
            let c: char = self.chars.next().unwrap();
            if c.is_ascii_hexdigit() {
                Some(c)
            } else {
                None
            }
        });

        // 読み込んだ文字列を16新数に変換して、utf16のバッファにpushする
        match u16::from_str_radix(&hexs.collect::<String>(), 16) {
            Ok(code_point) => Ok(code_point),
            Err(e) => Err(JsonPretError::LexerError(
                LexerError::new(&e.to_string())
            ))
        }
    }
    /// utf16のバッファを文字列に結合する
    fn push_utf16(&mut self, string: &mut String, utf16: &mut Vec<u16>) -> Result<(), JsonPretError>{
        if utf16.is_empty() {
            return Ok(());
        }

        match String::from_utf16(utf16) {
            Ok(utf16_str) => {
                string.push_str(&utf16_str);
                utf16.clear();
                Ok(())
            }
            Err(e) => return Err(JsonPretError::LexerError(
                LexerError::new(&e.to_string())
            ))
        }
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
    fn test_parse_string() {
        let s = "\"hogehoge12345\"";
        let token = Lexer::new(s).parse_string().unwrap();
        assert_eq!(token, Token::String("hogehoge12345".to_string()));

        let s = "\"あいうえお\"";
        let token = Lexer::new(s).parse_string().unwrap();
        assert_eq!(token, Token::String("あいうえお".to_string()));

        let s = r#""\u3042\u3044\u3046abc""#; //あいうabc
        let token = Lexer::new(s).parse_string().unwrap();
        assert_eq!(token, Token::String("あいうabc".to_string()));

        let s = format!(r#"\\b\f\n\r\t\/\""#);
        let token = Lexer::new(&s).parse_string().unwrap();
        assert_eq!(
            token,
            Token::String(r#"\b\f\n\r\t\/\""#.to_string())
        );

        let s = r#""\uD83D\uDE04\uD83D\uDE07\uD83D\uDC7A""#;
        let token = Lexer::new(&s).parse_string().unwrap();
        assert_eq!(token, Token::String(r#"😄😇👺"#.to_string()));
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

    #[test]
    fn test_lexical_analyze() {
        let obj = r#"
        {
            "number": 123,
            "boolean": true,
            "string": "togatoga",
            "object": {
               "number": 2E10
            }
         }
         "#;
        // object
        let tokens = Lexer::new(obj).lexical_analyze().unwrap();
        let result_tokens = [
            // start {
            Token::LeftBrace,
            // begin: "number": 123,
            Token::String("number".to_string()),
            Token::Colon,
            Token::Number(123f64),
            Token::Comma,
            // end

            // begin: "boolean": true,
            Token::String("boolean".to_string()),
            Token::Colon,
            Token::Bool(true),
            Token::Comma,
            // end

            // begin: "string": "togatoga",
            Token::String("string".to_string()),
            Token::Colon,
            Token::String("togatoga".to_string()),
            Token::Comma,
            // end

            // begin: "object": {
            Token::String("object".to_string()),
            Token::Colon,
            Token::LeftBrace,
            // begin: "number": 2E10,
            Token::String("number".to_string()),
            Token::Colon,
            Token::Number(20000000000f64),
            // end
            Token::RightBrace,
            // end
            Token::RightBrace,
            // end
        ];
        tokens
            .iter()
            .zip(result_tokens.iter())
            .enumerate()
            .for_each(|(i, (x, y))| {
                assert_eq!(x, y, "index: {}", i);
            });

        // array
        let a = "[true, {\"キー\": null}]";
        let tokens = Lexer::new(a).lexical_analyze().unwrap();
        let result_tokens = vec![
            Token::LeftBracket,
            Token::Bool(true),
            Token::Comma,
            Token::LeftBrace,
            Token::String("キー".to_string()),
            Token::Colon,
            Token::Null,
            Token::RightBrace,
            Token::RightBracket,
        ];
        tokens
            .iter()
            .zip(result_tokens.iter())
            .for_each(|(x, y)| assert_eq!(x, y));
    }
}