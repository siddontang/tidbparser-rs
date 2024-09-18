use std::boxed::Box;
use std::collections::VecDeque;

use sqlparser::dialect::keywords::Keyword;
use sqlparser::dialect::MySqlDialect;
use sqlparser::parser::Parser;
use sqlparser::parser::ParserError;
use sqlparser::tokenizer::Token;
use sqlparser::tokenizer::TokenWithLocation;
use sqlparser::tokenizer::Tokenizer;

use crate::statement::Statement;

// Use `Parser::expected` instead, if possible
macro_rules! parser_err {
    ($MSG:expr) => {
        Err(ParserError::ParserError($MSG.to_string()))
    };
}

/// Report an unexpected token
pub fn parser_expected<T>(expected: &str, found: TokenWithLocation) -> Result<T, ParserError> {
    parser_err!(format!("Expected {expected}, found: {found}"))
}

// TiDB parser
// Refer to https://github.com/apache/datafusion/blob/main/datafusion/sql/src/parser.rs
pub struct TiDBParser<'a> {
    pub parser: Parser<'a>,
}

impl<'a> TiDBParser<'a> {
    pub fn new(sql: &str) -> Result<Self, ParserError> {
        // Mostly, TiDB can use MySQL dialect directly.
        let dialect = &MySqlDialect {};
        let mut tokenizer = Tokenizer::new(dialect, sql);
        let tokens = tokenizer.tokenize()?;

        let parser = Parser::new(dialect).with_tokens(tokens);

        Ok(Self { parser: parser })
    }

    // Parse the SQL statement into a list of statements.
    pub fn parse_sql(sql: &str) -> Result<VecDeque<Statement>, ParserError> {
        let mut parser = Self::new(sql)?;
        let mut stmts = VecDeque::new();
        let mut expecting_statement_delimiter = false;
        loop {
            // ignore empty statements (between successive statement delimiters)
            while parser.parser.consume_token(&Token::SemiColon) {
                expecting_statement_delimiter = false;
            }

            if parser.parser.peek_token() == Token::EOF {
                break;
            }
            if expecting_statement_delimiter {
                return parser_expected("end of statement", parser.parser.peek_token());
            }

            let statement = parser.parse_statement()?;
            stmts.push_back(statement);
            expecting_statement_delimiter = true;
        }
        Ok(stmts)
    }

    pub fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.parser.peek_token().token {
            Token::Word(w) => match w.keyword {
                Keyword::ADMIN => {
                    self.parser.next_token();
                    self.parse_admin_statement()
                }
                _ => {
                    let statement = self.parser.parse_statement()?;
                    Ok(Statement::Statement(Box::new(statement)))
                }
            },
            _ => {
                let statement = self.parser.parse_statement()?;
                Ok(Statement::Statement(Box::new(statement)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let sqls = vec![
            "admin show ddl;",
            "admin show ddl jobs;",
            "admin show ddl jobs where id > 0;",
            "admin show ddl jobs 20 where id = 0;",
        ];
        for sql in sqls {
            let mut parser = TiDBParser::new(sql).unwrap();
            let _ = parser
                .parse_statement()
                .expect(format!("Failed to parse sql: {} ", sql).as_str());
        }
    }
}
