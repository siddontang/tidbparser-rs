use std::boxed::Box;
use std::fmt;

use sqlparser::ast::Statement as SQLStatement;

use crate::admin::AdminStatement;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Statement(Box<SQLStatement>),
    Admin(AdminStatement),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Statement(stmt) => write!(f, "{}", stmt),
            Statement::Admin(stmt) => write!(f, "{}", stmt),
        }
    }
}
