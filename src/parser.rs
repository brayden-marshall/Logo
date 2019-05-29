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
    Command { func: Command, args: Vec<Expression> },
    Number { val: f64 },
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

                Token::Number(_) => 
                    Err("Expected command, found number literal"),
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
        let mut args: Vec<Expression> = vec![];
        // consuming the next tokens as arguments according to how many
        // the arguments the command takes as input
        for _ in 0..command.arity() {
            match tokens.next() {
                Some(e) => match e {
                    Token::Number(n) => args.push(
                        Expression::Number {
                            val: *n,
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
                Token::Command(Command::Forward), Token::Number(70.0),
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
            Token::Command(Command::Forward), Token::Number(100.0),
            Token::Number(101.0),
        ]).unwrap();
    }
}
