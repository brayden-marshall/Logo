use crate::lexer::{Token, Command};

#[derive(Debug)]
pub struct AST {
    pub expressions: Vec<Expression>,
}

#[derive(Debug)]
pub enum Expression {
    ProgramStart,
    FunctionCall { func: Command, args: Vec<Expression> },
    Number { val: f64 },
}

impl AST {
    // main parsing logic. currently does not handle varying argument types
    pub fn build(tokens: &Vec<Token>) -> Result<AST, &'static str> {
        let mut ast = AST {
            expressions: vec![Expression::ProgramStart]
        };

        let mut token_iter = tokens.iter();
        while let Some(tok) = token_iter.next() {
            // the expression we will be adding to ast
            let mut expr: Option<Expression> = None;
            if let Token::Command(func) = tok {
                let mut args: Vec<Expression> = vec![];

                for _ in 0..func.arity() {
                    match token_iter.next() {
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

                expr = Some(Expression::FunctionCall {
                    func: func.clone(),
                    args,
                });
            }

            if let Some(e) = expr {
                ast.expressions.push(e);
            } else {
                return Err("Expected command");
            }
        }
        Ok(ast)
    }
}
