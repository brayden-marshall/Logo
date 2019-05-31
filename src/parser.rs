use crate::lexer::{Token, Command};
use std::slice;

#[derive(Debug, PartialEq)]
pub struct AST {
    pub expressions: Vec<Expression>,
}

impl AST {
    pub fn new() -> Self {
        AST {
            expressions: vec![Expression::ProgramStart],
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    ProgramStart,
    Number { val: f64 },
    Command { func: Command, args: Vec<Expression> },
    Repeat { count: usize, body: Vec<Expression> },
}

impl AST {
    // main parsing logic. currently does not handle varying argument types
    pub fn build(tokens: &Vec<Token>) -> Result<AST, &'static str> {
        let mut ast = AST::new();

        let mut token_iter = tokens.iter();
        while let Some(tok) = token_iter.next() {
            // the expression we will be adding to ast
            //let mut expr: Option<Expression>;
            let expr = match tok {
                Token::Command(command) =>
                    AST::parse_command(command.clone(), &mut token_iter),

                Token::Repeat =>
                    AST::parse_repeat(&mut token_iter),
                    
                _ =>
                    Err("Error: found unexpected token"),

            };

            match expr {
                Ok(e) => ast.expressions.push(e),
                Err(err) => return Err(err),
            }
        }
        Ok(ast)
    }

    // takes a command 
    fn parse_command(
        command: Command, 
        tokens: &mut slice::Iter<'_, Token>
    ) -> Result<Expression, &'static str> {
        let mut args: Vec<Expression> = Vec::new();
        // consuming the next tokens as arguments according to how many
        // the arguments the command takes as input
        for _ in 0..command.arity() {
            match tokens.next() {
                Some(e) => match e {
                    Token::Number{literal} => args.push(
                        Expression::Number {
                            val: literal.parse().unwrap(),
                        }
                    ),
                    _ => return Err("Expected number argument"),
                }
                None => return Err("Not enough arguments"),
            }
        }

        Ok(Expression::Command {
            func: command,
            args,
        })
    }

    fn parse_repeat(
        mut tokens: &mut slice::Iter<'_, Token>
    ) -> Result<Expression, &'static str> {
        let count: usize;
        let mut body: Vec<Expression> = Vec::new();

        // check for number as next token, and assign it to 'count' if found
        match tokens.next() {
            Some(tok) => 
                match tok {
                    Token::Number{literal} => count = literal.parse().unwrap(),
                    _ => return Err("Expected number argument after keyword 'repeat'"),
                }
            None => return Err("Expected number argument after keyword 'repeat'"),
        }

        // check for a left bracket to start the body of repeat command
        match tokens.next() {
            Some(tok) => 
                match tok {
                    Token::LBracket => (),
                    _ => return Err("Expected opening bracket '[' to start repeat body"),
                }
            None => return Err("Expected opening bracket '[' to start repeat body"),
        }

        // parse expressions of repeat body until we find a closing bracket
        loop {
            let expr = 
                match tokens.next() {
                    Some(tok) =>
                        match tok {
                            Token::RBracket => break,

                            Token::Command(command) =>
                                AST::parse_command(command.clone(), &mut tokens),
                            Token::Repeat => Err("Error: nested repeats are not currently supported"),
                            _ => Err("Error: found unexpected token"),
                        }
                    None => Err("Error: invalid repeat body"),
                };

            match expr {
                Ok(e) => body.push(e),
                Err(e) => return Err(e),
            }
        }

        Ok(Expression::Repeat{
            count,
            body,
        })
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
                Token::Number{literal: String::from("70.0")},
            ],
            AST {
                expressions: vec! [
                    Expression::ProgramStart,
                    Expression::Command {
                        func: Command::Forward,
                        args: vec![Expression::Number{val: 70.0}],
                    },
                ],
            }
        );
    }

    #[test]
    #[should_panic]
    fn parse_not_enough_arguments_test() {
        AST::build(&vec![
            Token::Command(Command::Forward),
        ]).unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_too_many_arguments_test() {
        // too many arguments for command forward "fd 100 101"
        AST::build(&vec![
            Token::Command(Command::Forward), Token::Number{literal: String::from("100.0")},
            Token::Number{literal: String::from("101.0")},
        ]).unwrap();
    }

    #[test]
    fn parse_two_argument_command_test() {
        parse_test(
            vec![
                Token::Command(Command::SetXY),
                Token::Number{literal: String::from("-60")},
                Token::Number{literal: String::from("60")},
            ],
            AST {
                expressions: vec![
                    Expression::ProgramStart,
                    Expression::Command {
                        func: Command::SetXY,
                        args: vec![
                            Expression::Number{val: -60.0},
                            Expression::Number{val: 60.0},
                        ],
                    },
                ],
            }
        );
    }
}
