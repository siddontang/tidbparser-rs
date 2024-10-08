use std::fmt;

use crate::parser::TiDBParser;
use crate::statement::Statement;
use sqlparser::ast::Expr;
use sqlparser::ast::Value;
use sqlparser::dialect::keywords::Keyword;
use sqlparser::parser::ParserError;
use sqlparser::tokenizer::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdminStatement {
    ShowDDL,
    ShowDDLJobs {
        num: Option<Value>,
        where_clause: Option<Expr>,
    },
}

impl fmt::Display for AdminStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdminStatement::ShowDDL => write!(f, "ADMIN SHOW DDL"),
            AdminStatement::ShowDDLJobs { num, where_clause } => {
                write!(f, "ADMIN SHOW DDL JOBS")?;
                if let Some(num) = num {
                    write!(f, " {}", num)?;
                }
                if let Some(where_clause) = where_clause {
                    write!(f, " WHERE {}", where_clause)?;
                }
                Ok(())
            }
        }
    }
}

impl<'a> TiDBParser<'a> {
    pub fn parse_admin_statement(&mut self) -> Result<Statement, ParserError> {
        if let Some(keyword) = self.parser.parse_one_of_keywords(&[Keyword::SHOW]) {
            match keyword {
                Keyword::SHOW => {
                    return self.parse_admin_show();
                }
                _ => {
                    return parser_err!("Unexpected keyword after ADMIN");
                }
            }
        }
        unreachable!()
    }

    fn parse_admin_show(&mut self) -> Result<Statement, ParserError> {
        if let Some(keyword) = self.parser.parse_one_of_keywords(&[Keyword::DDL]) {
            match keyword {
                Keyword::DDL => {
                    return self.parse_admin_show_ddl();
                }
                _ => {
                    return parser_err!("Unexpected keyword after ADMIN SHOW");
                }
            }
        }
        unreachable!()
    }

    fn parse_admin_show_ddl_jobs(&mut self) -> Result<Statement, ParserError> {
        let mut num = None;
        let mut where_clause = None;

        // Parse number if present
        if let Token::Number(_, _) = self.parser.peek_token().token {
            num = Some(self.parser.parse_number_value()?);
        }

        // Parse WHERE clause if present
        if let Token::Word(w) = self.parser.peek_token().token {
            if w.keyword == Keyword::WHERE {
                self.parser.next_token();
                where_clause = Some(self.parser.parse_expr()?);
            }
        }

        Ok(Statement::Admin(AdminStatement::ShowDDLJobs {
            num,
            where_clause,
        }))
    }

    fn parse_admin_show_ddl(&mut self) -> Result<Statement, ParserError> {
        if let Some(keyword) = self.parser.parse_one_of_keywords(&[Keyword::JOBS]) {
            match keyword {
                Keyword::JOBS => {
                    return self.parse_admin_show_ddl_jobs();
                }
                _ => {
                    return parser_err!("Unexpected keyword after ADMIN SHOW DDL");
                }
            }
        } else {
            Ok(Statement::Admin(AdminStatement::ShowDDL))
        }
    }
}
