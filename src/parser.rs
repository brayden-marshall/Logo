use crate::lexer::{Command, Operator, Token};
use std::iter::Peekable;
use std::slice;

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
    ArithmeticExpression { postfix: Vec<Expression> },
    Operator { op: Operator },
    Number { val: isize },
    //Word {
    //    literal: String,
    //},
    Variable { name: String },
}

#[derive(Debug)]
pub enum ParseError {
    EOF,
    UnexpectedToken(Token),
    TypeError,
    ParseInteger(String),
    MismatchParens,
}

#[derive(Debug, PartialEq)]
pub struct AST {
    pub statements: Vec<Statement>,
}

impl AST {
    pub fn new() -> Self {
        AST { statements: vec![] }
    }
}

pub struct Parser<'a> {
    tokens: Peekable<slice::Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Parser {
            tokens: tokens.iter().peekable(),
        }
    }

    pub fn build_ast(&mut self) -> Result<AST, ParseError> {
        let mut ast = AST::new();

        while let Some(tok) = self.tokens.next() {
            ast.statements.push(self.parse_statement(tok)?);
        }
        Ok(ast)
    }

    fn parse_statement(&mut self, token: &Token) -> Result<Statement, ParseError> {
        match token {
            Token::Command(command) => self.parse_command(command.clone()),

            Token::Repeat => self.parse_repeat(),

            Token::Make => self.parse_variable_declaration(),

            _ => Err(ParseError::UnexpectedToken(token.clone())),
        }
    }

    // takes a command
    fn parse_command(&mut self, command: Command) -> Result<Statement, ParseError> {
        let mut args: Vec<Expression> = Vec::new();

        // consuming the next tokens as arguments according to how many
        // the arguments the command takes as input
        for _ in 0..command.arity() {
            args.push(self.parse_expression()?);
        }

        Ok(Statement::Command { command, args })
    }

    fn parse_repeat(&mut self) -> Result<Statement, ParseError> {
        let mut body: Vec<Statement> = Vec::new();

        // check that the next number is a number, and parse it
        let count: usize = match self.tokens.next() {
            Some(tok) => match tok {
                Token::Number { literal } => match literal.parse() {
                    Ok(n) => Ok(n),
                    Err(_) => Err(ParseError::ParseInteger(literal.to_string())),
                },
                _ => Err(ParseError::TypeError),
            },
            None => Err(ParseError::EOF),
        }?;

        // check for a left bracket to start the body of repeat command
        match self.tokens.next() {
            Some(tok) => match tok {
                Token::LBracket => (),
                _ => return Err(ParseError::UnexpectedToken(tok.clone())),
            },
            None => return Err(ParseError::EOF),
        }

        // parse expressions of repeat body until we find a closing bracket
        loop {
            body.push(match self.tokens.next() {
                Some(tok) => match tok {
                    Token::RBracket => break,
                    _ => self.parse_statement(tok),
                },
                None => Err(ParseError::EOF),
            }?);
        }

        Ok(Statement::Repeat { count, body })
    }

    fn parse_variable_declaration(&mut self) -> Result<Statement, ParseError> {
        let name = match self.tokens.next() {
            Some(tok) => match tok {
                Token::Word { literal } => literal.to_string(),
                _ => return Err(ParseError::UnexpectedToken(tok.clone())),
            },
            None => return Err(ParseError::EOF),
        };

        let val = Box::new(self.parse_expression()?);

        Ok(Statement::VariableDeclaration { name, val })
    }

    fn parse_arithmetic_expression<T>(
        tokens: &mut Peekable<T>,
        first: Option<Expression>,
    ) -> Result<Expression, ParseError>
    where
        T: Iterator<Item = &'a Token>,
    {
        let mut operator_stack: Vec<Token> = Vec::new();
        //let mut output: Vec<Expression> = vec![first];
        let mut output: Vec<Expression> = match first {
            Some(expr) => vec![expr],
            None => vec![],
        };
        loop {
            // check that the next token is either a number or an operator
            match tokens.peek() {
                Some(tok) => match tok {
                    Token::Number { literal: _ } => (),
                    Token::Variable { name: _ } => (),
                    Token::Operator(_) => (),
                    Token::LParen | Token::RParen => (),
                    _ => break,
                },
                None => break,
            }

            if let Some(tok) = tokens.next() {
                match tok {
                    Token::Number { literal } => {
                        output.push(Parser::parse_number(literal.to_string())?)
                    }
                    Token::Variable { name } => output.push(Expression::Variable {
                        name: name.to_string(),
                    }),
                    Token::Operator(op) => {
                        while !operator_stack.is_empty()
                            && op.precedence() <= match &operator_stack[operator_stack.len()-1] {
                                Token::Operator(op) => op.precedence(),
                                _ => 0,
                            }
                        {
                            if let Some(popped) = operator_stack.pop() {
                                match popped {
                                    Token::Operator(op) => output.push(Expression::Operator { op }),
                                    _ => (),
                                }
                            }
                        }
                        operator_stack.push(tok.clone());
                    }

                    Token::LParen => operator_stack.push(Token::LParen),

                    Token::RParen => {
                        loop {
                            if operator_stack.is_empty() {
                                return Err(ParseError::MismatchParens);
                            }

                            match operator_stack[operator_stack.len() - 1] {
                                Token::LParen => break,
                                _ => match operator_stack.pop() {
                                    Some(tok) => output.push(match tok {
                                        Token::Number { literal } =>
                                            Parser::parse_number(literal.to_string())?,

                                        Token::Variable { name } =>
                                            Expression::Variable { name: name.to_string() },

                                        Token::Operator(op) =>
                                            Expression::Operator { op },
                                        
                                        _ => return Err(ParseError::TypeError),
                                    }),
                                    _ => (),
                                }
                            }

                            /*
                            if !operator_stack.is_empty()
                                && operator_stack[operator_stack.len()-1]
                                == Token::LParen {
                                operator_stack.pop();
                            }
                            */
                        }
                    },
                    _ => (),
                }
            }
        }

        while !operator_stack.is_empty() {
            if let Some(popped) = operator_stack.pop() {
                if let Token::Operator(op) = popped {
                    output.push(Expression::Operator { op });
                }
            }
        }

        Ok(Expression::ArithmeticExpression { postfix: output })
    }

    fn parse_number(literal: String) -> Result<Expression, ParseError> {
        match literal.parse() {
            Ok(n) => Ok(Expression::Number { val: n }),
            Err(_) => Err(ParseError::ParseInteger(literal.to_string())),
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        
        // if the next token is a left paren, parse an arithmetic expression
        if let Some(tok) = self.tokens.peek() {
            if let Token::LParen = tok {
                return Parser::parse_arithmetic_expression(&mut self.tokens, None);
            }
        }

        let mut expr = match self.tokens.next() {
            Some(tok) => match tok {
                Token::Number { literal } => Parser::parse_number(literal.to_string()),
                Token::Variable { name } => Ok(Expression::Variable {
                    name: name.to_string(),
                }),
                //Token::LParen => Parser::parse_arithmetic_expression(&mut self.tokens, None),
                _ => Err(ParseError::TypeError),
            },
            None => Err(ParseError::EOF),
        }?;

        // look ahead one token to check for an operator
        if let Some(tok) = self.tokens.peek() {
            if let Token::Operator(_) = tok {
                expr = Parser::parse_arithmetic_expression(&mut self.tokens, Some(expr))?;
            }
        }

        Ok(expr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_test(input: Vec<Token>, expected: AST) {
        let ast = Parser::new(&input).build_ast().unwrap();
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
                statements: vec![Statement::Command {
                    command: Command::Forward,
                    args: vec![Expression::Number { val: 70 }],
                }],
            },
        );
    }

    #[test]
    #[should_panic]
    fn parse_not_enough_arguments_test() {
        Parser::new(&vec![Token::Command(Command::Forward)])
            .build_ast()
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_too_many_arguments_test() {
        // too many arguments for command forward "fd 100 101"
        Parser::new(&vec![
            Token::Command(Command::Forward),
            Token::Number {
                literal: String::from("100"),
            },
            Token::Number {
                literal: String::from("101"),
            },
        ])
        .build_ast()
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
                statements: vec![Statement::Command {
                    command: Command::SetXY,
                    args: vec![
                        Expression::Number { val: -60 },
                        Expression::Number { val: 60 },
                    ],
                }],
            },
        );
    }

    #[test]
    fn parse_variable_argument_command_test() {
        parse_test(
            vec![
                Token::Command(Command::SetXY),
                Token::Variable {
                    name: String::from("x"),
                },
                Token::Variable {
                    name: String::from("Y"),
                },
            ],
            AST {
                statements: vec![Statement::Command {
                    command: Command::SetXY,
                    args: vec![
                        Expression::Variable {
                            name: String::from("x"),
                        },
                        Expression::Variable {
                            name: String::from("Y"),
                        },
                    ],
                }],
            },
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
                statements: vec![Statement::Repeat {
                    count: 10,
                    body: vec![Statement::Command {
                        command: Command::Forward,
                        args: vec![Expression::Number { val: 50 }],
                    }],
                }],
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
                statements: vec![Statement::Repeat {
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
                }],
            },
        );
    }

    #[test]
    fn parse_arithmetic_expression_test() {
        // 10 + 7
        let input = vec![
            Token::Number {
                literal: "10".to_string(),
            },
            Token::Operator(Operator::Addition),
            Token::Number {
                literal: "7".to_string(),
            },
        ];

        assert_eq!(
            Parser::parse_arithmetic_expression(&mut input.iter().peekable(), None).unwrap(),
            Expression::ArithmeticExpression {
                //postfix: "10 7 +".to_string(),
                postfix: vec![
                    Expression::Number { val: 10 },
                    Expression::Number { val: 7 },
                    Expression::Operator {
                        op: Operator::Addition
                    },
                ],
            },
        );

        // 10 + 7 * 8 - 2
        let input = vec![
            Token::Number {
                literal: "10".to_string(),
            },
            Token::Operator(Operator::Addition),
            Token::Number {
                literal: "7".to_string(),
            },
            Token::Operator(Operator::Multiplication),
            Token::Number {
                literal: "8".to_string(),
            },
            Token::Operator(Operator::Subtraction),
            Token::Number {
                literal: "2".to_string(),
            },
        ];

        assert_eq!(
            Parser::parse_arithmetic_expression(&mut input.iter().peekable(), None,).unwrap(),
            Expression::ArithmeticExpression {
                //postfix: "10 7 8 * + 2 -".to_string()
                postfix: vec![
                    Expression::Number { val: 10 },
                    Expression::Number { val: 7 },
                    Expression::Number { val: 8 },
                    Expression::Operator {
                        op: Operator::Multiplication
                    },
                    Expression::Operator {
                        op: Operator::Addition
                    },
                    Expression::Number { val: 2 },
                    Expression::Operator {
                        op: Operator::Subtraction
                    },
                ],
            },
        );

        // :size + :count * :length
        let input = vec![
            Token::Variable {
                name: "size".to_string(),
            },
            Token::Operator(Operator::Addition),
            Token::Variable {
                name: "count".to_string(),
            },
            Token::Operator(Operator::Multiplication),
            Token::Variable {
                name: "length".to_string(),
            },
        ];

        assert_eq!(
            Parser::parse_arithmetic_expression(&mut input.iter().peekable(), None,).unwrap(),
            Expression::ArithmeticExpression {
                // :size :count :length * +
                postfix: vec![
                    Expression::Variable {
                        name: "size".to_string()
                    },
                    Expression::Variable {
                        name: "count".to_string()
                    },
                    Expression::Variable {
                        name: "length".to_string()
                    },
                    Expression::Operator {
                        op: Operator::Multiplication
                    },
                    Expression::Operator {
                        op: Operator::Addition
                    },
                ]
            }
        );
    }

    #[test]
    fn parse_arithmetic_with_paren_test() {
        // (10 + 2) * 20
        let input = vec![
            Token::LParen,
            Token::Number { literal: "10".to_string() },
            Token::Operator(Operator::Addition),
            Token::Number { literal: "2".to_string() },
            Token::RParen,
            Token::Operator(Operator::Multiplication),
            Token::Number { literal: "20".to_string() },
        ];

        // expect: 10 2 + 20 *
        assert_eq!(
            Parser::parse_arithmetic_expression(&mut input.iter().peekable(), None).unwrap(),
            Expression::ArithmeticExpression {
                postfix: vec![
                    Expression::Number { val: 10 },
                    Expression::Number { val: 2 },
                    Expression::Operator { op: Operator::Addition },
                    Expression::Number { val: 20 },
                    Expression::Operator { op: Operator::Multiplication },
                ],
            }
        );

        // ((2 + 7) * (5 * (3 / 1)))
        let input = vec![
            Token::LParen,
            Token::LParen,
            Token::Number { literal: "2".to_string() },
            Token::Operator(Operator::Addition),
            Token::Number { literal: "7".to_string() },
            Token::RParen,
            Token::Operator(Operator::Multiplication),
            Token::LParen,
            Token::Number { literal: "5".to_string() },
            Token::Operator(Operator::Multiplication),
            Token::LParen,
            Token::Number { literal: "3".to_string() },
            Token::Operator(Operator::Division),
            Token::Number { literal: "1".to_string() },
            Token::RParen,
            Token::RParen,
            Token::RParen,
        ];

        // expect: 2 7 + 5 3 1 / * *
        assert_eq!(
            Parser::parse_arithmetic_expression(&mut input.iter().peekable(), None).unwrap(),
            Expression::ArithmeticExpression {
                postfix: vec![
                    Expression::Number { val: 2 },
                    Expression::Number { val: 7 },
                    Expression::Operator { op: Operator::Addition },
                    Expression::Number { val: 5 },
                    Expression::Number { val: 3 },
                    Expression::Number { val: 1 },
                    Expression::Operator { op: Operator::Division },
                    Expression::Operator { op: Operator::Multiplication },
                    Expression::Operator { op: Operator::Multiplication },
                ],
            }
        );
    }
}
