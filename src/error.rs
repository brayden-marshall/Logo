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
        write!(formatter, "{}", match self {
            RuntimeError::RedeclaredProcedure { name } => "",
            RuntimeError::ProcedureNotFound { name } => "",
            RuntimeError::VariableNotFound { name } => "",
            RuntimeError::ArgCountMismatch { expected } => "",
            RuntimeError::TypeMismatch { expected } => "",
            RuntimeError::Other(message) => message,
        })
    }
}

#[derive(Debug)]
pub enum ParseError {
    EOF,
    UnexpectedToken(Token),
    TypeError,
    ParseInteger(String),
    MismatchParens,
}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(formatter, "{}", match self {
            ParseError::EOF => "",
            ParseError::UnexpectedToken(tok) => "",
            ParseError::TypeError => "",
            ParseError::ParseInteger(n) => "",
            ParseError::MismatchParens => "",
        })
    }
}

#[derive(Debug)]
pub enum LexError {
    UnrecognizedToken,
}

impl fmt::Display for LexError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(formatter, "{}", match self {
            LexError::UnrecognizedToken => "",
        })
    }
}
