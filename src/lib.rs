mod lexer;
mod parser;
mod error;

use std::collections::BTreeMap;
use std::ops::Index;

use error::JsonPretError;
use lexer::Lexer;
use parser::Parser;

#[derive(Debug, Clone, PartialEq)]
pub enum JsonObject {
    String(String),                  // 文字列
    Number(f64),                     // 数値
    Bool(bool),                      // 真偽値
    Null,                            // Null
    Array(Vec<JsonObject>),               // JSON Array
    Object(BTreeMap<String, JsonObject>), // JSON Object
}

/// {"key": true}
/// v["key"] => JsonObject::Bool(true)
impl Index<&str> for JsonObject {
    type Output = JsonObject;
    fn index(&self, key: &str) -> &Self::Output {
        match self {
            JsonObject::Object(map) => map
                .get(key)
                .unwrap_or_else(|| panic!("A key is not found: {}", key)),
            _ => {
                panic!("A JsonObject is not object");
            }
        }
    }
}

/// [null, false, 3]
/// v[3] => JsonObject::Number(3f64)
impl Index<usize> for JsonObject {
    type Output = JsonObject;
    fn index(&self, idx: usize) -> &Self::Output {
        match self {
            JsonObject::Array(array) => &array[idx],
            _ => {
                panic!("A JsonObject is not array");
            }
        }
    }
}


/// JSON文字列を受け取り、JsonObjectを返す。
pub fn parse(input: &str) -> Result<JsonObject, JsonPretError> {
    let mut lexer: Lexer<'_> =  Lexer::new(input);
    let tokens: Vec<lexer::Token> = match lexer.lexical_analyze() {
        Ok(t) => t,
        Err(e) => return Err(e)
    };

    let mut parser: Parser = Parser::new(tokens);
    parser.parse()
}