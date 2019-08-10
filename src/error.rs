use crate::lexer::Token;
use std::fmt;

#[derive(Debug)]
pub enum RuntimeError {
    RedeclaredProcedure { name: String },
    ProcedureNotFound { name: String },
    VariableNotFound { name: String },
    ArgCountMismatch { expected: usize },
    TypeMismatch { expected: String },
    Other(String),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            formatter,
            "{}",
            match self {
                RuntimeError::RedeclaredProcedure { name } => {
                    format!("Procedure '{}' has already been declared", name)
                }
                RuntimeError::ProcedureNotFound { name } => {
                    format!("Procedure '{}' does not exist", name)
                }
                RuntimeError::VariableNotFound { name } => {
                    format!("Variable :{} has not been declared", name)
                }
                RuntimeError::ArgCountMismatch { expected } => {
                    format!("Wrong number of arguments, expected {}", expected)
                }
                RuntimeError::TypeMismatch { expected } => {
                    format!("Unexpected type found, expected {}", expected)
                }
                RuntimeError::Other(message) => message.to_string(),
            }
        )
    }
}

#[derive(Debug)]
pub enum ParseError {
    TypeMismatch { expected: String },
    EOF,
    UnexpectedToken(Token),
    ParseInteger(String),
    UnbalancedParens,
}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            formatter,
            "{}",
            match self {
                ParseError::EOF => String::from("Reached EOF (End of file) while parsing"),
                ParseError::UnexpectedToken(tok) => format!("Unexpected token: {}", tok),
                ParseError::TypeMismatch { expected } => {
                    format!("Found unexpected type while parsing, expected {}", expected)
                }
                ParseError::ParseInteger(n) => format!("Error while parsing integer: {}", n),
                ParseError::UnbalancedParens => {
                    String::from("Found unbalanced parentheses while parsing")
                }
            }
        )
    }
}

#[derive(Debug)]
pub enum LexError {
    UnrecognizedToken,
}

impl fmt::Display for LexError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            formatter,
            "{}",
            match self {
                LexError::UnrecognizedToken => {
                    String::from("Found unexpected token during lexing phase")
                }
            }
        )
    }
}
