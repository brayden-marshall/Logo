use std::collections::HashMap;
use std::fs;

use clap::{App, Arg};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use turtle::Turtle;

mod commands;
mod error;
mod lexer;
mod parser;

use commands::{get_turtle_commands, TurtleCommand};
use error::RuntimeError;
use lexer::{Lexer, Operator};
use parser::{Expression, Parser, Statement, AST};

fn main() {
    let matches = App::new("Logo")
        .version("0.1.0")
        .author("Brayden Marshall <bmarsh579@gmail.com>")
        .about("A Logo interpreter written in Rust")
        .arg(
            Arg::with_name("SCRIPT")
                .help("Program read from script file")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .help("Print debug information")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("no-turtle")
                .short("n")
                .long("no-turtle")
                .help("do not create turtle or window or startup")
                .takes_value(false),
        )
        .get_matches();

    let mut evaluator = Evaluator::new(EvaluatorConfig {
        turtle: !matches.is_present("no-turtle"),
        debug: matches.is_present("debug"),
    });

    // if a script argument was passed, run the script
    if let Some(file) = matches.value_of("SCRIPT") {
        print_program_output(evaluator.run_program(match &fs::read_to_string(file) {
            Ok(input) => input,
            Err(e) => {
                eprint!("Error reading file: {}\n", e);
                std::process::exit(1);
            },
        }));
    }

    // running interactive shell using the rustyline crate
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                print_program_output(evaluator.run_program(&line));
            }
            Err(ReadlineError::Interrupted) => {
                eprintln!("CTRL-C");
            }
            Err(ReadlineError::Eof) => {
                eprintln!("CTRL-D");
                std::process::exit(1);
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn print_program_output(program_result: Result<String, String>) {
    match program_result {
        Ok(output) => print!("{}", output),
        Err(output) => eprint!("{}", output),
    };
}

pub struct Procedure {
    ast: AST,
    params: Vec<String>,
}

pub struct EvaluatorConfig {
    turtle: bool,
    debug: bool,
}

pub struct Evaluator {
    turtle: Option<Turtle>,
    globals: HashMap<String, Expression>,
    // stack of local scopes
    locals: Vec<HashMap<String, Expression>>,
    procedures: HashMap<String, Procedure>,
    commands: HashMap<String, TurtleCommand>,
    debug: bool,
}

impl Evaluator {
    pub fn new(config: EvaluatorConfig) -> Self {
        Evaluator {
            turtle: if config.turtle {
                Some(Turtle::new())
            } else {
                None
            },
            globals: HashMap::new(),
            locals: Vec::new(),
            procedures: HashMap::new(),
            commands: get_turtle_commands(),
            debug: config.debug,
        }
    }

    pub fn run_program(&mut self, input: &str) -> Result<String, String> {
        let mut program_output = String::new();

        let mut lexer = Lexer::new(&input);
        let tokens = match lexer.collect_tokens() {
            Ok(t) => Ok(t),
            Err(e) => Err(format!("{}Error: {}\n", program_output, e)),
        }?;

        if self.debug {
            // append lexing debug info onto output
            program_output = format!(
                "{}Lexing phase completed without error\n{:?}\n",
                program_output, tokens,
            );
        }

        let mut parser = Parser::new(&tokens);
        let ast = match parser.build_ast() {
            Ok(ast) => Ok(ast),
            Err(e) => Err(format!("{}{}", program_output, e)),
        }?;

        if self.debug {
            // append parsing debug info onto output
            program_output = format!(
                "{}Parsing phase completed without error\n{:?}\n",
                program_output, ast,
            );
        }

        match self.run_ast(&ast) {
            Ok(_) => Ok(program_output),
            Err(output) => Err(format!("{}{}", program_output, output)),
        }

        /*
        // building the AST out of the tokens and running the program
        // based off of the AST
        let mut parser = Parser::new(&tokens);

        match parser.build_ast() {
            Ok(ast) => {
                if self.debug {
                    // append parsing debug info onto output
                    program_output = format!(
                        "{}Parsing phase completed without error\n{:?}\n",
                        program_output, ast,
                    );
                }
                match self.run_ast(&ast) {
                    Ok(_) => Ok(program_output),
                    Err(output) => Err(format!("{}{}", program_output, output)),
                }
            }
            Err(e) => Err(format!("{}Error: {}\n", program_output, e)),
        }
        */
    }

    fn run_ast(&mut self, ast: &AST) -> Result<(), RuntimeError> {
        for stmt in ast.statements.iter() {
            if let Err(e) = self.run_statement(stmt) {
                return Err(e);
            }
        }
        Ok(())
    }

    fn run_statement(&mut self, stmt: &Statement) -> Result<(), RuntimeError> {
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
                if let Some(command) = self.commands.get(name) {
                    if command.arity != args.len() {
                        return Err(RuntimeError::ArgCountMismatch {
                            expected: command.arity,
                        });
                    }

                    let mut _args: Vec<isize> = Vec::new();
                    for i in 0..args.len() {
                        _args.push(self.evaluate_expression(&args[i])?);
                    }

                    if let None = self.turtle {
                        self.turtle = Some(Turtle::new());
                    }

                    if let Some(turtle) = &mut self.turtle {
                        (command.func)(turtle, &_args);
                    }
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

                    self.run_ast(&ast)?;

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
                for _ in 0..*count {
                    self.run_ast(body)?;
                }
            }
        }

        Ok(())
    }

    fn evaluate_expression(&self, expr: &Expression) -> Result<isize, RuntimeError> {
        match expr {
            Expression::Number { val } => Ok(*val),
            Expression::Variable { name } => {
                for i in (0..self.locals.len()).rev() {
                    match self.locals[i].get(name) {
                        Some(e) => {
                            return match e {
                                Expression::Number { val } => Ok(*val),
                                _ => Err(RuntimeError::TypeMismatch {
                                    expected: "Number".to_string(),
                                }),
                            }
                        }
                        None => (),
                    }
                }

                match self.globals.get(name) {
                    Some(e) => match e {
                        Expression::Number { val } => Ok(*val),
                        _ => Err(RuntimeError::TypeMismatch {
                            expected: "Number".to_string(),
                        }),
                    },
                    None => Err(RuntimeError::VariableNotFound {
                        name: name.to_string(),
                    }),
                }
            }
            Expression::ArithmeticExpression { postfix } => Ok(self.evaluate_postfix(postfix)?),
            Expression::Operator { op } => Err(RuntimeError::Other(format!(
                "Encountered unexpected operator {:?}",
                op
            ))),
        }
    }

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
    fn evaluate_postfix_test() {
        let config = EvaluatorConfig {
            turtle: false,
            debug: false,
        };
        let mut evaluator = Evaluator::new(config);
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
