use crate::lexer::{Command, Token};
use std::slice;

#[derive(Debug, PartialEq)]
pub struct AST {
    pub statements: Vec<Statement>,
}

impl AST {
    pub fn new() -> Self {
        AST {
            statements: vec![],
        }
    }
}

/// Statements are any logo 'sentence' that does not evaluate to a value
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Command {
        command: Command,
        args: Vec<Expression>,
    },
    Repeat {
        count: usize,
        body: Vec<Statement>,
    },
    VariableDeclaration {
        name: String,
        val: Box<Expression>,
    },
}

/// Expressions are any logo 'sentence' that evaluates to a value
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Number {
        val: isize,
    },
    Word {
        literal: String,
    },
    Variable {
        name: String,
    },
}

#[derive(Debug)]
pub enum ParseError {
    EOF,
    UnexpectedToken,
    TypeError,
    ParseInteger,
}

impl AST {
    // main parsing logic. currently does not handle varying argument types
    pub fn build(tokens: &Vec<Token>) -> Result<AST, ParseError> {
        let mut ast = AST::new();

        let mut token_iter = tokens.iter();
        while let Some(tok) = token_iter.next() {
            let expr = match tok {
                Token::Command(command) => 
                    AST::parse_command(command.clone(), &mut token_iter),

                Token::Repeat => AST::parse_repeat(&mut token_iter),

                Token::Make => AST::parse_variable_declaration(&mut token_iter),
                _ => Err(ParseError::UnexpectedToken),
            };

            match expr {
                Ok(e) => ast.statements.push(e),
                Err(err) => return Err(err),
            }
        }
        Ok(ast)
    }

    // takes a command
    fn parse_command(
        command: Command,
        tokens: &mut slice::Iter<'_, Token>,
    ) -> Result<Statement, ParseError> {
        let mut args: Vec<Expression> = Vec::new();
        // consuming the next tokens as arguments according to how many
        // the arguments the command takes as input
        for _ in 0..command.arity() {
            match tokens.next() {
                Some(e) => match e {
                    Token::Number { literal } => args.push(Expression::Number {
                        val: literal.parse().unwrap(),
                    }),
                    Token::Variable { name } => args.push(Expression::Variable {
                        name: name.to_string(),
                    }),
                    _ => return Err(ParseError::TypeError),
                },
                None => return Err(ParseError::EOF),
            }
        }

        Ok(Statement::Command { command, args })
    }

    fn parse_repeat(
        tokens: &mut slice::Iter<'_, Token>,
    ) -> Result<Statement, ParseError> {
        let mut body: Vec<Statement> = Vec::new();

        // check that the next number is a number, and parse it
        let count: Result<usize, ParseError> = match tokens.next() {
            Some(tok) => match tok {
                Token::Number { literal } => match literal.parse() {
                    Ok(n) => Ok(n),
                    Err(_) => Err(ParseError::ParseInteger),
                },
                _ => Err(ParseError::TypeError),
            },
            None => Err(ParseError::TypeError),
        };

        // handle the possible integer parsing error
        let count: usize = match count {
            Ok(n) => n,
            Err(e) => return Err(e),
        };

        // check for a left bracket to start the body of repeat command
        match tokens.next() {
            Some(tok) => match tok {
                Token::LBracket => (),
                _ => return Err(ParseError::UnexpectedToken),
            },
            None => return Err(ParseError::UnexpectedToken),
        }

        // parse expressions of repeat body until we find a closing bracket
        loop {
            let statement = match tokens.next() {
                Some(tok) => match tok {
                    Token::RBracket => break,

                    Token::Command(command) => AST::parse_command(command.clone(), tokens),
                    Token::Repeat => AST::parse_repeat(tokens),
                    _ => Err(ParseError::UnexpectedToken),
                },
                None => Err(ParseError::EOF),
            };

            match statement {
                Ok(s) => body.push(s),
                Err(e) => return Err(e),
            }
        }

        Ok(Statement::Repeat { count, body })
    }

    fn parse_variable_declaration(
        tokens: &mut slice::Iter<'_, Token>,
    ) -> Result<Statement, ParseError> {
        let name = match tokens.next() {
            Some(tok) => match tok {
                Token::Word { literal } => literal.to_string(),
                _ => return Err(ParseError::UnexpectedToken),
            },
            None => return Err(ParseError::EOF),
        };

        let val: Box<Expression> = match tokens.next() {
            Some(tok) => match tok {
                Token::Word { literal } => Box::new(Expression::Word {
                    literal: literal.to_string(),
                }),
                Token::Number { literal } => Box::new(Expression::Number {
                    val: literal.parse().unwrap(),
                }),
                Token::Variable { name } => Box::new(Expression::Variable {
                    name: name.to_string(),
                }),
                _ => return Err(ParseError::UnexpectedToken),
            },
            None => return Err(ParseError::EOF),
        };

        Ok(Statement::VariableDeclaration { name, val })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_test(input: Vec<Token>, expected: AST) {
        let ast = AST::build(&input).unwrap();
        assert_eq!(ast, expected);
    }

    #[test]
    fn parse_short_command_test() {
        parse_test(
            vec![
                Token::Command(Command::Forward),
                Token::Number {
                    literal: String::from("70"),
                },
            ],
            AST {
                statements: vec![
                    Statement::Command {
                        command: Command::Forward,
                        args: vec![Expression::Number { val: 70 }],
                    },
                ],
            },
        );
    }

    #[test]
    #[should_panic]
    fn parse_not_enough_arguments_test() {
        AST::build(&vec![Token::Command(Command::Forward)]).unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_too_many_arguments_test() {
        // too many arguments for command forward "fd 100 101"
        AST::build(&vec![
            Token::Command(Command::Forward),
            Token::Number {
                literal: String::from("100"),
            },
            Token::Number {
                literal: String::from("101"),
            },
        ])
        .unwrap();
    }

    #[test]
    fn parse_two_argument_command_test() {
        parse_test(
            vec![
                Token::Command(Command::SetXY),
                Token::Number {
                    literal: String::from("-60"),
                },
                Token::Number {
                    literal: String::from("60"),
                },
            ],
            AST {
                statements: vec![
                    Statement::Command {
                        command: Command::SetXY,
                        args: vec![
                            Expression::Number { val: -60 },
                            Expression::Number { val: 60 },
                        ],
                    },
                ],
            },
        );
    }

    #[test]
    fn parse_variable_argument_command_test() {
        parse_test(
            vec![
                Token::Command(Command::SetXY),
                Token::Variable { name: String::from("x") },
                Token::Variable { name: String::from("Y") },
            ],
            AST {
                statements: vec![
                    Statement::Command {
                        command: Command::SetXY,
                        args: vec![
                            Expression::Variable { name: String::from("x") },
                            Expression::Variable { name: String::from("Y") },
                        ],
                    }
                ]
            }
        );
    }

    #[test]
    fn parse_repeat_test() {
        // source: repeat 10 [ forward 50 ]
        parse_test(
            vec![
                Token::Repeat,
                Token::Number {
                    literal: String::from("10"),
                },
                Token::LBracket,
                Token::Command(Command::Forward),
                Token::Number {
                    literal: String::from("50"),
                },
                Token::RBracket,
            ],
            AST {
                statements: vec![
                    Statement::Repeat {
                        count: 10,
                        body: vec![Statement::Command {
                            command: Command::Forward,
                            args: vec![Expression::Number { val: 50 }],
                        }],
                    },
                ],
            },
        );
    }

    #[test]
    fn parse_nested_repeat_test() {
        // source: repeat 10 [ forward 50 repeat 45 [ rt 1 ] ]
        parse_test(
            vec![
                Token::Repeat,
                Token::Number {
                    literal: String::from("10"),
                },
                Token::LBracket,
                Token::Command(Command::Forward),
                Token::Number {
                    literal: String::from("50"),
                },
                Token::Repeat,
                Token::Number {
                    literal: String::from("45"),
                },
                Token::LBracket,
                Token::Command(Command::Right),
                Token::Number {
                    literal: String::from("1"),
                },
                Token::RBracket,
                Token::RBracket,
            ],
            AST {
                statements: vec![
                    Statement::Repeat {
                        count: 10,
                        body: vec![
                            Statement::Command {
                                command: Command::Forward,
                                args: vec![Expression::Number { val: 50 }],
                            },
                            Statement::Repeat {
                                count: 45,
                                body: vec![Statement::Command {
                                    command: Command::Right,
                                    args: vec![Expression::Number { val: 1 }],
                                }],
                            },
                        ],
                    },
                ],
            },
        );
    }
}
