use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};

use clap::{App, Arg};
use turtle::Turtle;

mod lexer;
mod parser;

use lexer::{Command, Lexer, Token};
use parser::{Statement, Expression, AST};

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

    // if a script argument was passed, run the script
    if let Some(file) = matches.value_of("SCRIPT") {
        run_program(&mut t, &fs::read_to_string(file).unwrap(), debug, &mut vars);
    }

    // running interactive shell
    loop {
        let input = get_input();

        run_program(&mut t, &input, debug, &mut vars);
    }
}

fn run_program(t: &mut Turtle, input: &str, debug: bool, vars: &mut HashMap<String, Expression>) {
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
        println!("{:?}", tokens);
    }

    // building the AST out of the tokens and running the program
    // based off of the AST
    match AST::build(&tokens) {
        Ok(ast) => {
            if debug {
                println!("{:?}", ast);
            }
            run_ast(t, &ast, vars);
        }
        Err(e) => eprintln!("{:?}", e),
    }
}

fn run_ast(t: &mut Turtle, ast: &AST, vars: &mut HashMap<String, Expression>) {
    for stmt in ast.statements.iter() {
        if let Err(e) = run_statement(t, stmt, vars) {
            eprintln!("{:?}", e);
        }
    }
}

fn run_statement(t: &mut Turtle, stmt: &Statement, 
                      vars: &mut HashMap<String, Expression>) 
-> Result<(), String> {
    // currently does not handle varying argument types,
    // only accept LOGO number values as command arguments
    match stmt {
        Statement::VariableDeclaration { name, val } => {
            let _val = (**val).clone();
            vars.insert(name.to_string(), match _val {
                Expression::Number {val: _} => _val,
                Expression::Variable { name } => match vars.get(&name) {
                    Some(e) => match e {
                        Expression::Number { val } => Expression::Number { 
                            val: val.clone()
                        },
                        _ => return Err(String::from("Expected number argument")),
                    },
                    None => return Err(format!("Variable {} does not exist", name)),
                },
                _ => return Err(String::from("Expected number argument")),
            });
            println!("Vars: {:?}", vars);
            ()
        }

        Statement::Command { command, args: _args } => {
            let mut args: Vec<f64> = Vec::new();
            for arg in _args.iter() {
                match arg {
                    Expression::Number { val } => args.push(*val as f64),
                    Expression::Variable { name } => match vars.get(name) {
                        Some(e) => match e {
                            Expression::Number { val } => args.push(*val as f64),
                            _ => return Err(String::from("Expected number argument")),
                        },
                        None => return Err(format!("Variable {} does not exist", name)),
                    },
                    _ => return Err(String::from("Expected number argument")),
                }
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

        Statement::Repeat { count, body } => {
            for _ in 0..*count {
                for body_statement in body.iter() {
                    if let Err(e) = run_statement(t, body_statement, vars) {
                        eprintln!("{:?}", e);
                    }
                }
            }
        }
    }
    Ok(())
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
