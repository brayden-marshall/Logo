use std::io::{self, Write};
use turtle::Turtle;

mod lexer;
mod parser;

use lexer::{Lexer, Command};
use parser::{AST, Expression};

fn main() {
    //println!("Welcome to LOGO!");
    //println!("Enter commands in the prompt below to move the turtle");

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
    // currently does not handle varying argument types,
    // only accept LOGO number values as arguments
    match expr {
        Expression::ProgramStart => (),
        Expression::Number{val: _} => (),

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
