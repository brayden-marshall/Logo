use crate::error::ParseError;
use crate::lexer::{Operator, Token};
use std::iter::Peekable;
use std::slice;

/// Statements are any logo 'sentence' that does not evaluate to a value
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Repeat {
        count: Expression,
        body: AST,
    },
    VariableDeclaration {
        name: String,
        val: Box<Expression>,
    },
    ProcedureDeclaration {
        name: String,
        body: AST,
        params: Vec<String>,
    },
    ProcedureCall {
        name: String,
        args: Vec<Expression>,
    },
}

/// Expressions are any logo 'sentence' that evaluates to a value
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    ArithmeticExpression { postfix: Vec<Expression> },
    Operator { op: Operator },
    Number { val: isize },
    Variable { name: String },
}

#[derive(Debug, PartialEq, Clone)]
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


    fn expect(&mut self, expected: Token) -> Result<&Token, ParseError> {
        match self.tokens.next() {
            Some(tok) => if *tok == expected {
                    Ok(tok)
            } else {
                Err(ParseError::UnexpectedToken (
                    (*tok).clone(), vec![expected.clone()]
                ))
            },
            None => Err(ParseError::EOF),
        }
    }

    fn parse_statement(&mut self, token: &Token) -> Result<Statement, ParseError> {
        use Token::*;
        match token {
            Repeat => self.parse_repeat(),

            Make => self.parse_variable_declaration(),

            To => self.parse_procedure_declaration(),

            Identifier { literal } => self.parse_procedure_call(literal),

            _ => Err(ParseError::UnexpectedToken(
                token.clone(),
                vec![
                    Repeat,
                    Make,
                    To,
                    Identifier {
                        literal: "".to_string(),
                    },
                ],
            )),
        }
    }

    fn parse_procedure_call(&mut self, name: &str) -> Result<Statement, ParseError> {
        let mut args: Vec<Expression> = Vec::new();

        while let Some(tok) = self.tokens.peek() {
            match tok {
                Token::Variable { name: _ } | Token::Number { literal: _ } | Token::LParen => {
                    args.push(self.parse_expression()?)
                }
                _ => break,
            }
        }

        Ok(Statement::ProcedureCall {
            name: name.to_string(),
            args,
        })
    }

    fn parse_repeat(&mut self) -> Result<Statement, ParseError> {

        let count: Expression = self.parse_expression()?;

        self.expect(Token::LBracket)?;

        let mut body: Vec<Statement> = Vec::new();
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

        Ok(Statement::Repeat {
            count,
            body: AST { statements: body },
        })
    }

    fn parse_procedure_declaration(&mut self) -> Result<Statement, ParseError> {
        let name = self.expect(
            Token::Identifier { literal: "".to_string() }
        )?.value().unwrap().to_string();

        // parse parameters if given
        let mut params = Vec::<String>::new();
        while let Some(tok) = self.tokens.peek() {
            match tok {
                Token::Variable { name } => {
                    params.push(name.to_string());
                    self.tokens.next();
                }
                _ => break,
            }
        }

        let mut body = AST::new();

        // parse the body of the procedure until a repeat is found
        loop {
            body.statements.push(match self.tokens.next() {
                Some(tok) => match tok {
                    Token::End => break,
                    _ => self.parse_statement(tok),
                },
                None => Err(ParseError::EOF),
            }?);
        }

        Ok(Statement::ProcedureDeclaration { name, body, params })
    }

    fn parse_variable_declaration(&mut self) -> Result<Statement, ParseError> {
        let name = self.expect(
            Token::Word { literal: "".to_string() }
        )?.value().unwrap().to_string();

        let val = Box::new(self.parse_expression()?);

        Ok(Statement::VariableDeclaration { name, val })
    }

    /// Uses the shunting-yard algorithm for parsing arithmetic expressions.
    /// Parses the expression into postfix notation and returns an
    /// Expression::ArithmeticExpression
    fn parse_arithmetic_expression<T>(
        tokens: &mut Peekable<T>,
        first: Option<Expression>,
    ) -> Result<Expression, ParseError>
    where
        T: Iterator<Item = &'a Token>,
    {
        let mut operator_stack: Vec<Token> = Vec::new();
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
                        output.push(Parser::parse_number(literal)?)
                    }
                    Token::Variable { name } => output.push(Expression::Variable {
                        name: name.to_string(),
                    }),
                    Token::Operator(op) => {
                        while !operator_stack.is_empty()
                            && op.precedence()
                                <= match &operator_stack[operator_stack.len() - 1] {
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

                    Token::RParen => loop {
                        if operator_stack.is_empty() {
                            return Err(ParseError::UnbalancedParens);
                        }

                        match operator_stack[operator_stack.len() - 1] {
                            Token::LParen => {
                                operator_stack.pop();
                                break;
                            }
                            _ => match operator_stack.pop() {
                                // can't use parse_expression to cover all options here because the
                                // operator is a special case
                                Some(tok) => output.push(match tok {
                                    Token::Number { literal } => {
                                        Parser::parse_number(&literal)?
                                    }

                                    Token::Variable { name } => Expression::Variable {
                                        name: name.to_string(),
                                    },

                                    Token::Operator(op) => Expression::Operator { op },

                                    _ => {
                                        return Err(ParseError::TypeMismatch {
                                            expected: "Number, Variable, Operator".to_string(),
                                        })
                                    }
                                }),
                                _ => (),
                            },
                        }
                    },
                    _ => (),
                }
            }
        }

        while !operator_stack.is_empty() {
            if let Some(popped) = operator_stack.pop() {
                match popped {
                    Token::Operator(op) => output.push(Expression::Operator { op }),
                    Token::LParen | Token::RParen => return Err(ParseError::UnbalancedParens),
                    _ => (),
                }
            }
        }

        Ok(Expression::ArithmeticExpression { postfix: output })
    }

    fn parse_number(literal: &str) -> Result<Expression, ParseError> {
        match literal.parse() {
            Ok(n) => Ok(Expression::Number { val: n }),
            Err(_) => Err(ParseError::ParseInteger(literal.to_string())),
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        let mut expr = match self.tokens.next() {
            Some(tok) => match tok {
                Token::LParen => Parser::parse_arithmetic_expression(&mut self.tokens, None),
                Token::Number { literal } => Parser::parse_number(literal),
                Token::Variable { name } => Ok(Expression::Variable {
                    name: name.to_string(),
                }),
                _ => Err(ParseError::UnexpectedToken(
                    tok.clone(),
                    vec![
                        Token::Number {
                            literal: "".to_string(),
                        },
                        Token::Variable {
                            name: "".to_string(),
                        },
                    ],
                )),
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
                Token::Identifier {
                    literal: "forward".to_string(),
                },
                Token::Number {
                    literal: String::from("70"),
                },
            ],
            AST {
                statements: vec![Statement::ProcedureCall {
                    name: "forward".to_string(),
                    args: vec![Expression::Number { val: 70 }],
                }],
            },
        );
    }

    #[test]
    fn parse_two_argument_command_test() {
        parse_test(
            vec![
                Token::Identifier {
                    literal: "setxy".to_string(),
                },
                Token::Number {
                    literal: String::from("-60"),
                },
                Token::Number {
                    literal: String::from("60"),
                },
            ],
            AST {
                statements: vec![Statement::ProcedureCall {
                    name: "setxy".to_string(),
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
                Token::Identifier {
                    literal: "setxy".to_string(),
                },
                Token::Variable {
                    name: String::from("x"),
                },
                Token::Variable {
                    name: String::from("Y"),
                },
            ],
            AST {
                statements: vec![Statement::ProcedureCall {
                    name: "setxy".to_string(),
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
                Token::Identifier {
                    literal: "forward".to_string(),
                },
                Token::Number {
                    literal: String::from("50"),
                },
                Token::RBracket,
            ],
            AST {
                statements: vec![Statement::Repeat {
                    count: Expression::Number { val: 10 },
                    body: AST {
                        statements: vec![Statement::ProcedureCall {
                            name: "forward".to_string(),
                            args: vec![Expression::Number { val: 50 }],
                        }],
                    },
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
                Token::Identifier {
                    literal: "forward".to_string(),
                },
                Token::Number {
                    literal: String::from("50"),
                },
                Token::Repeat,
                Token::Number {
                    literal: String::from("45"),
                },
                Token::LBracket,
                Token::Identifier {
                    literal: "right".to_string(),
                },
                Token::Number {
                    literal: String::from("1"),
                },
                Token::RBracket,
                Token::RBracket,
            ],
            AST {
                statements: vec![Statement::Repeat {
                    count: Expression::Number { val: 10 },
                    body: AST {
                        statements: vec![
                            Statement::ProcedureCall {
                                name: "forward".to_string(),
                                args: vec![Expression::Number { val: 50 }],
                            },
                            Statement::Repeat {
                                count: Expression::Number { val: 45 },
                                body: AST {
                                    statements: vec![Statement::ProcedureCall {
                                        name: "right".to_string(),
                                        args: vec![Expression::Number { val: 1 }],
                                    }],
                                },
                            },
                        ],
                    },
                }],
            },
        );
    }

    #[test]
    fn parse_arithmetic_expression_test() {
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
        // ((2 + 7) * (5 * (3 / 1)))
        let input = vec![
            Token::LParen,
            Token::LParen,
            Token::Number {
                literal: "2".to_string(),
            },
            Token::Operator(Operator::Addition),
            Token::Number {
                literal: "7".to_string(),
            },
            Token::RParen,
            Token::Operator(Operator::Multiplication),
            Token::LParen,
            Token::Number {
                literal: "5".to_string(),
            },
            Token::Operator(Operator::Multiplication),
            Token::LParen,
            Token::Number {
                literal: "3".to_string(),
            },
            Token::Operator(Operator::Division),
            Token::Number {
                literal: "1".to_string(),
            },
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
                    Expression::Operator {
                        op: Operator::Addition
                    },
                    Expression::Number { val: 5 },
                    Expression::Number { val: 3 },
                    Expression::Number { val: 1 },
                    Expression::Operator {
                        op: Operator::Division
                    },
                    Expression::Operator {
                        op: Operator::Multiplication
                    },
                    Expression::Operator {
                        op: Operator::Multiplication
                    },
                ],
            }
        );
    }

    #[test]
    fn parse_procedure_test() {
        /* to my_procedure
         * forward 100
         * repeat 10 [
         *     right 45
         * ]
         * end
         *
         */
        parse_test(
            vec![
                Token::To,
                Token::Identifier {
                    literal: "my_procedure".to_string(),
                },
                Token::Identifier {
                    literal: "forward".to_string(),
                },
                Token::Number {
                    literal: "100".to_string(),
                },
                Token::Repeat,
                Token::Number {
                    literal: "10".to_string(),
                },
                Token::LBracket,
                Token::Identifier {
                    literal: "right".to_string(),
                },
                Token::Number {
                    literal: "45".to_string(),
                },
                Token::RBracket,
                Token::End,
            ],
            AST {
                statements: vec![Statement::ProcedureDeclaration {
                    name: "my_procedure".to_string(),
                    body: AST {
                        statements: vec![
                            Statement::ProcedureCall {
                                name: "forward".to_string(),
                                args: vec![Expression::Number { val: 100 }],
                            },
                            Statement::Repeat {
                                count: Expression::Number { val: 10 },
                                body: AST {
                                    statements: vec![Statement::ProcedureCall {
                                        name: "right".to_string(),
                                        args: vec![Expression::Number { val: 45 }],
                                    }],
                                },
                            },
                        ],
                    },
                    params: Vec::new(),
                }],
            },
        );
    }

    #[test]
    fn parse_paramaterized_procedure_test() {
        /* to show_me :x
         * show :x
         * end
         */
        parse_test(
            vec![
                Token::To,
                Token::Identifier {
                    literal: "show_me".to_string(),
                },
                Token::Variable {
                    name: "x".to_string(),
                },
                Token::Identifier {
                    literal: "show".to_string(),
                },
                Token::Variable {
                    name: "x".to_string(),
                },
                Token::End,
            ],
            AST {
                statements: vec![Statement::ProcedureDeclaration {
                    name: "show_me".to_string(),
                    body: AST {
                        statements: vec![Statement::ProcedureCall {
                            name: "show".to_string(),
                            args: vec![Expression::Variable {
                                name: "x".to_string(),
                            }],
                        }],
                    },
                    params: vec!["x".to_string()],
                }],
            },
        );
    }
}
