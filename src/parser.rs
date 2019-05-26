use super::lexer::{Token, Builtin};

#[derive(Debug)]
pub struct AST {
    pub expressions: Vec<Expr>,
}

#[derive(Debug)]
pub enum Expr {
    ProgramStart,
    FunctionCall { func: Builtin, args: Vec<Expr> },
    Number { val: f64 },
}

impl AST {
    pub fn build(tokens: &Vec<Token>) -> AST {
        let mut ast = AST {
            expressions: vec![Expr::ProgramStart]
        };

        let mut index = 0;
        while index < tokens.len() {
            let mut expr = Expr::ProgramStart;
            match &tokens[index] {
                Token::Builtin(func) => {
                    if index + 1 >= tokens.len() {
                        return ast;
                    }

                    match tokens[index+1] {
                        Token::Number(n) => {
                            expr = Expr::FunctionCall {
                                func: func.clone(),
                                args: vec![Expr::Number {
                                    val: n,
                                }],
                            }
                        },
                        _ => return ast,
                    }
                    index += 2;
                },
                _ => index += 1,
            }
            ast.expressions.push(expr);
        }

        ast
    }

}
