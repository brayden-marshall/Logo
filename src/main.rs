use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};

use clap::{App, Arg};
use turtle::Turtle;

mod lexer;
mod parser;
mod commands;

use lexer::{Lexer, Operator, Token};
use parser::{Expression, Parser, Statement, AST};
use commands::{TurtleCommand, get_turtle_commands};

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
        .get_matches();

    let mut t = Turtle::new();
    let debug: bool = matches.is_present("debug");
    let mut vars: HashMap<String, Expression> = HashMap::new();
    let mut procedures: HashMap<String, AST> = HashMap::new();
    let turtle_commands = get_turtle_commands();

    // if a script argument was passed, run the script
    if let Some(file) = matches.value_of("SCRIPT") {
        run_program(&mut t, &fs::read_to_string(file).unwrap(), debug, &mut vars, &mut procedures, &turtle_commands);
    }

    // running interactive shell
    loop {
        let input = get_input();

        run_program(&mut t, &input, debug, &mut vars, &mut procedures, &turtle_commands);
    }
}

fn run_program(
    t: &mut Turtle,
    input: &str,
    debug: bool,
    vars: &mut HashMap<String, Expression>,
    procedures: &mut HashMap<String, AST>,
    turtle_commands: &HashMap<String, TurtleCommand>
) {
    // lexing input and returning vector of tokens
    let mut lexer = Lexer::new(&input);
    let mut tokens: Vec<Token> = Vec::new();
    while let Some(lex_result) = lexer.next() {
        match lex_result {
            Ok(tok) => tokens.push(tok),
            Err(e) => {
                eprintln!("{:?}", e);
                return;
            }
        }
    }

    if debug {
        println!("Lexing phase completed without error");
        println!("{:?}", tokens);
    }

    let mut parser = Parser::new(&tokens);

    // building the AST out of the tokens and running the program
    // based off of the AST
    match parser.build_ast() {
        Ok(ast) => {
            if debug {
                println!("Parsing phase completed without error");
                println!("{:?}", ast);
            }
            run_ast(t, &ast, vars, procedures, turtle_commands);
        }
        Err(e) => eprintln!("{:?}", e),
    }
}

fn run_ast(
    t: &mut Turtle,
    ast: &AST,
    vars: &mut HashMap<String, Expression>,
    procedures: &mut HashMap<String, AST>,
    turtle_commands: &HashMap<String, TurtleCommand>
) {
    for stmt in ast.statements.iter() {
        if let Err(e) = run_statement(t, stmt, vars, procedures, turtle_commands) {
            eprintln!("{:?}", e);
        }
    }
}

fn run_statement(
    t: &mut Turtle,
    stmt: &Statement,
    vars: &mut HashMap<String, Expression>,
    procedures: &mut HashMap<String, AST>,
    turtle_commands: &HashMap<String, TurtleCommand>
) -> Result<(), String> {
    // currently does not handle varying argument types,
    // only accept LOGO number values as command arguments
    match stmt {
        Statement::ProcedureDeclaration { name, body } => {
            if let Some(_) = procedures.get(name) {
                return Err(format!("Procedure with name {} already exists.", name));
            }

            procedures.insert(name.to_string(), body.clone());
        }

        Statement::ProcedureCall { name, args } => {
            if let Some(func) = turtle_commands.get(name) {
                let mut _args: Vec<isize> = Vec::new();
                for i in 0..args.len() {
                    _args.push(evaluate_expression(&args[i], vars)?);
                }
                (*func)(t, &_args); 
            } else {
                let ast = match procedures.get(name) {
                    Some(ast) => ast.clone(),
                    None => return Err("Undeclared procedure name.".to_string()),
                };
                run_ast(t, &ast, vars, procedures, turtle_commands);
            }
        },

        Statement::VariableDeclaration { name, val } => {
            let _val = (**val).clone();
            let expr = Expression::Number {
                val: evaluate_expression(&_val, vars)?,
            };

            vars.insert(name.to_string(), expr);
        }

        /*
        Statement::Command {
            command,
            args: _args,
        } => (),
            let mut args: Vec<f64> = Vec::new();
            for arg in _args.iter() {
                args.push(evaluate_expression(arg, vars)? as f64);
            }

            match command {
                // 0 arity
                Command::PenUp => t.pen_up(),
                Command::PenDown => t.pen_down(),
                Command::HideTurtle => t.hide(),
                Command::ShowTurtle => t.show(),
                Command::Home => t.home(),
                Command::ClearScreen => {
                    t.clear();
                    t.home()
                }
                Command::Clean => t.clear(),
                Command::Fill => return Err(String::from("Fill not yet implemented")),
                Command::Exit => std::process::exit(0),

                // 1 arity
                Command::Forward => t.forward(args[0]),
                Command::Backward => t.backward(args[0]),
                Command::Left => t.left(args[0]),
                Command::Right => t.right(args[0]),
                Command::SetPenSize => t.set_pen_size(args[0]),
                Command::SetHeading => t.set_heading(args[0]),
                Command::Show => println!("{}", args[0]),

                // 2 arity
                Command::SetXY => t.go_to([args[0], args[1]]),

                // 3 arity
                Command::SetPenColor => t.set_pen_color([args[0], args[1], args[2]]),
                Command::SetFillColor => t.set_fill_color([args[0], args[1], args[2]]),
                Command::SetScreenColor => t
                    .drawing_mut()
                    .set_background_color([args[0], args[1], args[2]]),
            }
        }
        */

        Statement::Repeat { count, body } => {
            for _ in 0..*count {
                run_ast(t, body, vars, procedures, turtle_commands);
            }
        }
    }

    Ok(())
}

fn evaluate_expression(
    expr: &Expression,
    vars: &mut HashMap<String, Expression>,
) -> Result<isize, String> {
    match expr {
        Expression::Number { val } => Ok(*val),
        Expression::Variable { name } => match vars.get(name) {
            Some(e) => match e {
                Expression::Number { val } => Ok(*val),
                _ => Err(String::from("Expected number argument")),
            },
            None => Err(format!("Variable {} does not exist", name)),
        },
        Expression::ArithmeticExpression { postfix } => Ok(evaluate_postfix(postfix, vars)?),
        _ => Err(String::from("There was an errorrrror")),
    }
}

fn evaluate_postfix(
    postfix: &Vec<Expression>,
    vars: &HashMap<String, Expression>,
) -> Result<isize, String> {
    let mut stack: Vec<isize> = Vec::new();
    for expr in postfix.iter() {
        match expr {
            Expression::Number { val } => stack.push(*val),
            Expression::Variable { name } => stack.push(match vars.get(name) {
                Some(e) => match e {
                    Expression::Number { val } => *val,
                    _ => return Err("Expected number argument".to_string()),
                },
                None => return Err("Error: variable does not exist".to_string()),
            }),
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
                return Err("reverse polish notation should only contain
                         numbers, variables and operators"
                    .to_string())
            }
        }
    }
    Ok(stack[0])
}


fn get_input() -> String {
    print!(">> ");
    match io::stdout().flush() {
        Err(e) => panic!(e),
        Ok(_) => (),
    }

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");

    input.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_postfix_test() {
        let mut vars: HashMap<String, Expression> = HashMap::new();
        vars.insert("count".to_string(), Expression::Number { val: 10 });
        vars.insert("size".to_string(), Expression::Number { val: 50 });

        // 10 5 /
        let postfix = vec![
            Expression::Number { val: 10 },
            Expression::Number { val: 5 },
            Expression::Operator {
                op: Operator::Division,
            },
        ];

        assert_eq!(evaluate_postfix(&postfix, &vars).unwrap(), 2,);

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

        assert_eq!(evaluate_postfix(&postfix, &vars).unwrap(), 105,);

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

        assert_eq!(evaluate_postfix(&postfix, &vars).unwrap(), 64,);
    }
}
