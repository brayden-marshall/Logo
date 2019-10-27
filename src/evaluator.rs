use std::collections::HashMap;

use crate::command::Command;
use crate::error::RuntimeError;
use crate::lexer::Operator;
use crate::parser::{Expression, Statement, AST};

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub command: Command,
    pub args: Vec<isize>,
}

struct Procedure {
    ast: AST,
    params: Vec<String>,
}

pub struct Evaluator {
    globals: HashMap<String, Expression>,
    // stack of local scopes
    locals: Vec<HashMap<String, Expression>>,
    procedures: HashMap<String, Procedure>,
}

impl Evaluator {
    /// Creates a new Evaluator object, including the memory (as HashMaps) to store
    /// variables and procedures.
    pub fn new() -> Self {
        Evaluator {
            globals: HashMap::new(),
            locals: Vec::new(),
            procedures: HashMap::new(),
        }
    }

    pub fn evaluate_ast(&mut self, ast: &AST) -> Result<Vec<Instruction>, RuntimeError> {
        let mut instructions = Vec::new();
        for stmt in ast.statements.iter() {
            self.evaluate_statement(stmt, &mut instructions)?;
        }

        Ok(instructions)
    }

    fn evaluate_statement(
        &mut self,
        stmt: &Statement,
        instructions: &mut Vec<Instruction>,
    ) -> Result<(), RuntimeError> {
        // currently does not handle varying argument types,
        // only accept LOGO number values as command arguments
        match stmt {
            Statement::ProcedureDeclaration { name, body, params } => {
                if let Some(_) = self.procedures.get(name) {
                    return Err(RuntimeError::RedeclaredProcedure {
                        name: name.to_string(),
                    });
                }

                self.procedures.insert(
                    name.to_string(),
                    Procedure {
                        ast: body.clone(),
                        params: params.clone(),
                    },
                );
            }

            Statement::ProcedureCall { name, args } => {
                if let Some(command) = Command::from_string(name) {
                    if command.arity() != args.len() {
                        return Err(RuntimeError::ArgCountMismatch {
                            expected: command.arity(),
                        });
                    }

                    let mut _args: Vec<isize> = Vec::new();
                    for i in 0..args.len() {
                        _args.push(self.evaluate_expression(&args[i])?);
                    }

                    instructions.push(Instruction {
                        command,
                        args: _args,
                    });
                } else {
                    let procedure = match self.procedures.get(name) {
                        Some(p) => p,
                        None => {
                            return Err(RuntimeError::ProcedureNotFound {
                                name: name.to_string(),
                            })
                        }
                    };

                    if args.len() != procedure.params.len() {
                        return Err(RuntimeError::ArgCountMismatch {
                            expected: procedure.params.len(),
                        });
                    }

                    let ast = procedure.ast.clone();

                    let mut local_vars = HashMap::<String, Expression>::new();
                    for i in 0..args.len() {
                        local_vars.insert(procedure.params[i].to_string(), args[i].clone());
                    }

                    // begin procedure scope
                    self.locals.push(local_vars);

                    // evaluate the ast and append the result to 'instructions'
                    instructions.extend(self.evaluate_ast(&ast)?);

                    // end procedure scope
                    self.locals.pop();
                }
            }

            Statement::VariableDeclaration { name, val } => {
                let _val = (**val).clone();
                let expr = Expression::Number {
                    val: self.evaluate_expression(&_val)?,
                };

                // check for whether the variable is local or global
                let scope_depth = self.locals.len();
                if scope_depth > 0 {
                    self.locals[scope_depth - 1].insert(name.to_string(), expr);
                } else {
                    self.globals.insert(name.to_string(), expr);
                }
            }

            Statement::Repeat { count, body } => {
                let _count = self.evaluate_expression(count)?;
                for _ in 0.._count {
                    instructions.extend(self.evaluate_ast(body)?);
                }
            }
        }

        Ok(())
    }

    fn evaluate_expression(&self, expr: &Expression) -> Result<isize, RuntimeError> {
        match expr {
            Expression::Number { val } => Ok(*val),
            Expression::Variable { name } => {
                // check for variable in local scope first
                for i in 0..self.locals.len() {
                    match self.locals[i].get(name) {
                        Some(e) => return self.evaluate_expression(e),
                        None => (),
                    }
                }

                // check in global scope if variable wasn't found
                match self.globals.get(name) {
                    Some(e) => self.evaluate_expression(e),
                    None => Err(RuntimeError::VariableNotFound {
                        name: name.to_string(),
                    }),
                }
            }
            Expression::ArithmeticExpression { postfix } => Ok(self.evaluate_postfix(postfix)?),

            // this case should not be reached under normal circumstances
            Expression::Operator { op } => Err(RuntimeError::Other(format!(
                "Encountered unexpected operator {:?}",
                op
            ))),
        }
    }

    /// Evaluates an arithmetic expression in postfix notation. The arithmetic expression is
    /// represented as a Vec of Expressions. Returns a Result of either the resulting number
    /// or any encountered RuntimeErrors.
    fn evaluate_postfix(&self, postfix: &Vec<Expression>) -> Result<isize, RuntimeError> {
        let mut stack: Vec<isize> = Vec::new();
        for expr in postfix.iter() {
            match expr {
                Expression::Number { val: _ } | Expression::Variable { name: _ } => {
                    stack.push(self.evaluate_expression(expr)?)
                }
                Expression::Operator { op } => {
                    let operand_2 = stack.pop().unwrap();
                    let operand_1 = stack.pop().unwrap();

                    let result = match op {
                        Operator::Addition => operand_1 + operand_2,
                        Operator::Subtraction => operand_1 - operand_2,
                        Operator::Multiplication => operand_1 * operand_2,
                        Operator::Division => operand_1 / operand_2,
                    };
                    stack.push(result);
                }
                _ => {
                    return Err(RuntimeError::Other(
                        "reverse polish notation should only contain numbers,
                        variables and operators"
                            .to_string(),
                    ))
                }
            }
        }
        Ok(stack[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_movement_commands_test() {
        let mut evaluator = Evaluator::new();

        let ast = AST {
            statements: vec![
                Statement::ProcedureCall {
                    name: "fd".to_string(),
                    args: vec![Expression::Number { val: 10 }],
                },
                Statement::ProcedureCall {
                    name: "backward".to_string(),
                    args: vec![Expression::Number { val: 4321 }],
                },
                Statement::ProcedureCall {
                    name: "right".to_string(),
                    args: vec![Expression::Number { val: 100 }],
                },
                Statement::ProcedureCall {
                    name: "left".to_string(),
                    args: vec![Expression::Number { val: -100 }],
                },
            ],
        };

        let instructions = match evaluator.evaluate_ast(&ast) {
            Ok(i) => i,
            Err(e) => panic!(e),
        };

        assert_eq!(
            instructions,
            vec![
                Instruction {
                    command: Command::Forward,
                    args: vec![10],
                },
                Instruction {
                    command: Command::Backward,
                    args: vec![4321],
                },
                Instruction {
                    command: Command::Right,
                    args: vec![100],
                },
                Instruction {
                    command: Command::Left,
                    args: vec![-100],
                },
            ],
        );
    }

    #[test]
    fn evaluate_repeat_test() {
        let mut evaluator = Evaluator::new();

        let ast = AST {
            statements: vec![
                Statement::Repeat {
                    count: Expression::Number {
                        val: 3,
                    },
                    body: AST {
                        statements: vec![
                            Statement::ProcedureCall {
                                name: "forward".to_string(),
                                args: vec![
                                    Expression::Number {
                                        val: 10
                                    }
                                ],
                            }
                        ]
                    }
                }
            ]
        };

        let instructions = evaluator.evaluate_ast(&ast).unwrap();
        assert_eq!(
            instructions,
            (0..3).map(|_| {
                Instruction {
                    command: Command::Forward,
                    args: vec![10],
                }
            }).collect::<Vec<_>>()
        );
    }

    #[test]
    fn evaluate_postfix_test() {
        let mut evaluator = Evaluator::new();
        //let mut vars: HashMap<String, Expression> = HashMap::new();
        evaluator
            .globals
            .insert("count".to_string(), Expression::Number { val: 10 });
        evaluator
            .globals
            .insert("size".to_string(), Expression::Number { val: 50 });

        // 10 5 /
        let postfix = vec![
            Expression::Number { val: 10 },
            Expression::Number { val: 5 },
            Expression::Operator {
                op: Operator::Division,
            },
        ];

        assert_eq!(evaluator.evaluate_postfix(&postfix).unwrap(), 2);

        // evaluating 10 * :count + :size / 10
        // in postfix: '10 :count * :size 10 / +'
        let postfix = vec![
            Expression::Number { val: 10 },
            Expression::Variable {
                name: "count".to_string(),
            },
            Expression::Operator {
                op: Operator::Multiplication,
            },
            Expression::Variable {
                name: "size".to_string(),
            },
            Expression::Number { val: 10 },
            Expression::Operator {
                op: Operator::Division,
            },
            Expression::Operator {
                op: Operator::Addition,
            },
        ];

        assert_eq!(evaluator.evaluate_postfix(&postfix).unwrap(), 105);

        // 10 7 8 * + 2 -
        let postfix = vec![
            Expression::Number { val: 10 },
            Expression::Number { val: 7 },
            Expression::Number { val: 8 },
            Expression::Operator {
                op: Operator::Multiplication,
            },
            Expression::Operator {
                op: Operator::Addition,
            },
            Expression::Number { val: 2 },
            Expression::Operator {
                op: Operator::Subtraction,
            },
        ];

        assert_eq!(evaluator.evaluate_postfix(&postfix).unwrap(), 64);
    }
}
