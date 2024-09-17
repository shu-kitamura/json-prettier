use crate::lexer::Token;

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