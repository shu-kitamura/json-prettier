use std::collections::BTreeMap;

use crate::{error::{JsonPretError, ParserError}, lexer::Token};

pub struct Parser {
    /// `Lexer`で`tokenize`した`Token`一覧
    tokens: Vec<Token>,
    /// `tokens`の先頭
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, index: 0 }
    }

    pub fn parse(&mut self) -> Result<(), JsonPretError>{
        let peeked = self.peek();
        if let Some(token) = peeked {
            match token {
                Token::LeftBrace => self.parse_object(),  // { の時の処理
                Token::LeftBracket => self.parse_array(), // [ の時の処理
                _ => Ok(())
            }
        } else {
            Ok(())
        }
    }

    fn parse_array(&mut self) -> Result<(), JsonPretError>{
        todo!()
    }

    fn parse_object(&mut self) -> Result<(), JsonPretError>{
        let token = match self.next_expect() {
            Ok(t) => t.clone(),
            Err(e) => return Err(e),
        };

        if token != Token::LeftBrace {
            return Err(JsonPretError::ParserError(
                ParserError::new(&format!("JSON object must start {{ but start {:?}", token))
            ))
        }

        let mut obj: BTreeMap<String, String> = BTreeMap::new();

        loop {
            let t1: Token  = match self.next_expect() {
                Ok(t) => t.clone(),
                Err(e) => return Err(e)
            };

            if t1 == Token::RightBrace {
                break;
            }

            let t2: Token  = match self.next_expect() {
                Ok(t) => t.clone(),
                Err(e) => return Err(e)
            };

            match (t1, t2) {
                (Token::String(key), Token::Colon) => obj.insert(key, "test".to_string()),
                _ => return Err(JsonPretError::ParserError(
                    ParserError::new("a pair 'String(key)' and ':' is expected.")
                ))
            };
        }

        Ok(())
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    fn peek_expect(&mut self) -> Result<&Token, JsonPretError> {
        match self.peek() {
            Some(t) => Ok(t),
            None => Err(JsonPretError::ParserError(
                ParserError::new("a token isn't peekable")
            ))
        }
    }

    fn next(&mut self) -> Option<&Token> {
        self.index += 1;
        self.tokens.get(self.index-1)
    }

    fn next_expect(&mut self) -> Result<&Token, JsonPretError> {
        match self.next() {
            Some(t) => Ok(t),
            None => Err(JsonPretError::ParserError(
                ParserError::new("a token isn't peekable")
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use crate::lexer::{Token};
    use super::Parser;

    #[test]
    fn test_parser_new() {
        let expect: Vec<Token> = vec![
            Token::LeftBrace,
            Token::String("is_test".to_string()),
            Token::Bool(true),
            Token::RightBrace
        ];

        let parser: Parser = Parser::new(vec![
            Token::LeftBrace,
            Token::String("is_test".to_string()),
            Token::Bool(true),
            Token::RightBrace
        ]);
        assert_eq!(parser.tokens, expect);
        assert_eq!(parser.index, 0);
    }

    #[test]
    fn test_parse_object() {
    }

    #[test]
    fn test_parse_array() {
    }

    #[test]
    fn test_parse() {
    }
}