use std::io::{self, Write};
use turtle::Turtle;

mod lexer;
mod parser;

use lexer::{Lexer, Command};
use parser::{AST, Expression};

fn main() {
    let mut input: String;
    let mut t = Turtle::new();

    loop {
        input = get_input();

        // lexing input and returning vector of tokens
        let lexer = Lexer::new(&input);
        let tokens = lexer.collect();
        println!("{:?}", tokens);

        // building the AST out of the tokens and running the program
        // based off of the AST
        match AST::build(&tokens) {
            Ok(ast) => {
                println!("{:?}", ast);
                run(&mut t, &ast);
            }
            Err(e) => println!("{}", e),
        }
    }
}

fn run(t: &mut Turtle, ast: &AST) {
    for expr in ast.expressions.iter() {
        if let Expression::ProgramStart = expr {
            continue;
        }

        run_expression(t, expr);
    } 
}

fn run_expression(t: &mut Turtle, expr: &Expression) {
    // currently a hack to get the interpreter running
    // does not properly handle varying number of command arguments
    // or varying argument types
    match expr {
        Expression::Command{func, args} => {
            match func.arity() {
                0 => match func {
                    Command::PenUp => t.pen_down(),
                    Command::PenDown => t.pen_down(),
                    Command::HideTurtle => t.hide(),
                    Command::ShowTurtle => t.show(),
                    Command::Home => t.home(),
                    Command::ClearScreen => { t.clear(); t.home() },
                    Command::Clean => t.clear(),

                    Command::Exit => std::process::exit(0),
                    _ => (),
                },
                1 => if let Expression::Number{val} = args[0] {
                    match func {
                        Command::Forward => t.forward(val),
                        Command::Backward => t.backward(val),
                        Command::Left => t.left(val),
                        Command::Right => t.right(val),
                        _ => (),
                    }
                }
                2 => 
                    if let Expression::Number{val: arg1} = args[0] {
                        if let Expression::Number{val: arg2} = args[1] {
                            match func {
                                Command::SetXY => t.go_to([arg1, arg2]),
                                _ => (),
                            }
                        }
                    }
                _ => (),
            }
        }

        Expression::Repeat{count, body} => {
            for _ in 0..*count {
                for body_expr in body.iter() {
                    run_expression(t, body_expr);
                }
            }
        }

        _ => (),
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
