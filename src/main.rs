use std::io::{self, Write};
use std::fs;

use turtle::Turtle;
use clap::{App, Arg};

mod lexer;
mod parser;

use lexer::{Lexer, Command, Token};
use parser::{AST, Expression};

fn main() {
    let matches = App::new("Logo")
                          .version("0.1.0")
                          .author("Brayden Marshall <bmarsh579@gmail.com>")
                          .about("A Logo interpreter written in Rust")
                          .arg(Arg::with_name("SCRIPT")
                                .help("Program read from script file")
                                .required(false)
                                .index(1))
                          .arg(Arg::with_name("debug")
                                .short("d")
                                .long("debug")
                                .help("Print debug information")
                                .takes_value(false))
                          .get_matches();

    let mut t = Turtle::new();
    let debug: bool = matches.is_present("debug");
    // if a script argument was passed, run the script
    if let Some(file) = matches.value_of("SCRIPT") {
        run_program(&mut t, &fs::read_to_string(file).unwrap(), debug);
    }

    // running interactive shell
    loop {
        let input = get_input();

        run_program(&mut t, &input, debug);
    }
}

fn run_program(mut t: &mut Turtle, input: &str, debug: bool) {
    // lexing input and returning vector of tokens
    let mut lexer = Lexer::new(&input);
    let mut tokens: Vec<Token> = Vec::new();
    while let Some(lex_result) = lexer.next() {
        match lex_result {
            Ok(tok) => tokens.push(tok),
            Err(e) => {
                println!("{:?}", e);
                return;
            },
        }
    }
    if debug { println!("{:?}", tokens); }

    // building the AST out of the tokens and running the program
    // based off of the AST
    match AST::build(&tokens) {
        Ok(ast) => {
            if debug { println!("{:?}", ast); }
            run_ast(&mut t, &ast);
        }
        Err(e) => println!("{:?}", e),
    }
}

fn run_ast(t: &mut Turtle, ast: &AST) {
    for expr in ast.expressions.iter() {
        if let Expression::ProgramStart = expr {
            continue;
        }

        run_expression(t, expr);
    } 
}

fn run_expression(t: &mut Turtle, expr: &Expression) {
    // currently does not handle varying argument types,
    // only accept LOGO number values as arguments
    match expr {
        Expression::ProgramStart => (),
        Expression::Number{val: _} => (),
        Expression::Word{literal: _} => (),
        Expression::VariableDeclaration{name: _, val: _} => (),

        Expression::Command{command, args} => {
            let args: Vec<f64> = args.iter().map(
                |arg| {
                    if let Expression::Number{val} = arg {
                        *val as f64
                    } else {
                        panic!("Expected number argument");
                    }
                }
            ).collect();

            match command {
                // 0 arity 
                Command::PenUp => t.pen_up(),
                Command::PenDown => t.pen_down(),
                Command::HideTurtle => t.hide(),
                Command::ShowTurtle => t.show(),
                Command::Home => t.home(),
                Command::ClearScreen => { t.clear(); t.home() },
                Command::Clean => t.clear(),
                Command::Fill => println!("Fill not yet implemented"),
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
                Command::SetPenColor => 
                    t.set_pen_color([args[0], args[1], args[2]]),
                Command::SetFillColor => 
                    t.set_fill_color([args[0], args[1], args[2]]),
                Command::SetScreenColor => 
                    t.drawing_mut().set_background_color(
                        [args[0], args[1], args[2]],
                    ),
            }
        }

        Expression::Repeat{count, body} => {
            for _ in 0..*count {
                for body_expr in body.iter() {
                    run_expression(t, body_expr);
                }
            }
        }

    }
}

fn get_input() -> String {
    print!(">> ");
    match io::stdout().flush() {
        Err(e) => panic!(e),
        Ok(_) => (),
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read user input");

    input.trim().to_string()
}
