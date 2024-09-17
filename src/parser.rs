use std::collections::BTreeMap;

use crate::{
    error::{JsonPretError, ParserError},
    lexer::Token,
    JsonObject
};

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

    pub fn parse(&mut self) -> Result<JsonObject, JsonPretError>{
        let peeked_token = match self.peek() {
            Ok(t) => t.clone(),
            Err(e) => return Err(e),
        };

        match peeked_token {
            Token::LeftBrace => self.parse_object(),
            Token::LeftBracket => self.parse_array(),
            Token::Bool(b) => {
                match self.next() {
                    Ok(_) => Ok(JsonObject::Bool(b)),
                    Err(e) => return Err(e)
                }
            }
            Token::Null => {
                match self.next() {
                    Ok(_) => Ok(JsonObject::Null),
                    Err(e) => return Err(e)
                }                
            }
            Token::Number(n) => {
                match self.next() {
                    Ok(_) => Ok(JsonObject::Number(n)),
                    Err(e) => return Err(e)
                }
            }
            Token::String(s) => {
                match self.next(){
                    Ok(_) => Ok(JsonObject::String(s)),
                    Err(e) => return Err(e)
                }
            },
            _ => return Err(JsonPretError::ParserError(
                ParserError::new(&format!(
                    "token must start {{ or [ or String or Number or Bool or Null, but start '{:?}'",
                    peeked_token
                ))
            ))
        }
    }

    fn parse_array(&mut self) -> Result<JsonObject, JsonPretError>{
        let token = match self.next() {
            Ok(t) => t.clone(),
            Err(e) => return Err(e)
        };

        if token != Token::LeftBracket {
            return Err(JsonPretError::ParserError(
                ParserError::new(&format!("JSON Array must start [ but start {:?}", token))
            ))
        }

        let mut array: Vec<JsonObject> = vec![];

        loop {
            match self.parse() {
                Ok(v) => array.push(v),
                Err(e) => return Err(e)
            };

            let token = match self.next() {
                Ok(t) => t,
                Err(e) => return Err(e),
            };

            match token {
                Token::RightBracket => break,
                Token::Comma => continue,
                _ => return Err(JsonPretError::ParserError(
                    ParserError::new(&format!("a ']' or ',' is expected, but '{:?}' is inputed", token))
                ))
            }
        }

        Ok(JsonObject::Array(array))
    }

    fn parse_object(&mut self) -> Result<JsonObject, JsonPretError>{
        let token = match self.next() {
            Ok(t) => t.clone(),
            Err(e) => return Err(e),
        };

        if token != Token::LeftBrace {
            return Err(JsonPretError::ParserError(
                ParserError::new(&format!("JSON object must start {{ but start {:?}", token))
            ))
        }

        let mut obj: BTreeMap<String, JsonObject> = BTreeMap::new();

        loop {
            let t1: Token  = match self.next() {
                Ok(t) => {
                    if *t == Token::RightBrace {
                        break;
                    } else {
                        t.clone()
                    }
                },
                Err(e) => return Err(e)
            };

            let t2: Token  = match self.next() {
                Ok(t) => t.clone(),
                Err(e) => return Err(e)
            };

            match (t1, t2) {
                (Token::String(key), Token::Colon) => obj.insert(key, self.parse().unwrap()),
                _ => return Err(JsonPretError::ParserError(
                    ParserError::new("a pair 'String(key)' and ':' is expected.")
                ))
            };

            match self.next() {
                Ok(t) => {
                    match *t {
                        Token::RightBrace => break,
                        Token::Comma => continue,
                        _ => return Err(JsonPretError::ParserError(
                            ParserError::new(&format!(
                                "{{ or , is expected, but {:?} is inputed",
                                t
                            ))
                        ))
                    }
                }
                Err(e) => return Err(e)
            }
        }

        Ok(JsonObject::Object(obj))
    }

    fn peek(&mut self) -> Result<&Token, JsonPretError> {
        match self.tokens.get(self.index) {
            Some(t) => Ok(t),
            None => Err(JsonPretError::ParserError(
                ParserError::new("a token isn't peekable")
            ))
        }
    }

    fn next(&mut self) -> Result<&Token, JsonPretError> {
        self.index += 1;
        match self.tokens.get(self.index-1) {
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
    use crate::{lexer::{Lexer, Token}, JsonObject};
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
        let mut obj = BTreeMap::new();
        obj.insert(
            "key".to_string(),
            JsonObject::String("JsonObject".to_string())
        );
        let expect = JsonObject::Object(obj);

        let mut lexer = Lexer::new(r#"{"key" : "JsonObject"}"#);
        let tokens = lexer.lexical_analyze().unwrap();
        let mut parser = Parser::new(tokens);
        let actual = parser.parse_object().unwrap();

        assert_eq!(actual, expect);
    }

    #[test]
    fn test_parse_array() {
        let expect: JsonObject = JsonObject::Array(vec![
            JsonObject::Null,
            JsonObject::Number(1.0),
            JsonObject::Bool(true),
            JsonObject::String("test".to_string()),
        ]);

        let mut lexer = Lexer::new(r#"[null, 1, true, "test"]"#);
        let tokens = lexer.lexical_analyze().unwrap();
        let mut parser = Parser::new(tokens);
        let actual = parser.parse_array().unwrap();

        assert_eq!(actual, expect)
    }

    #[test]
    fn test_parse() {
        let json = r#"{"key" : [1, "JsonObject"]}"#;
        let json_obj = Parser::new(Lexer::new(json).lexical_analyze().unwrap())
            .parse()
            .unwrap();
        let mut object = BTreeMap::new();
        object.insert(
            "key".to_string(),
            JsonObject::Array(vec![JsonObject::Number(1.0), JsonObject::String("JsonObject".to_string())]),
        );
        assert_eq!(json_obj, JsonObject::Object(object));

        let json = r#"[{"key": "JsonObject"}]"#;
        let json_obj = Parser::new(Lexer::new(json).lexical_analyze().unwrap())
            .parse()
            .unwrap();
        let mut object = BTreeMap::new();
        object.insert("key".to_string(), JsonObject::String("JsonObject".to_string()));

        let array = JsonObject::Array(vec![JsonObject::Object(object)]);
        assert_eq!(json_obj, array);
    }
}