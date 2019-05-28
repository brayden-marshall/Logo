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

        let lexer = Lexer::new(&input);
        let tokens = lexer.collect();
        println!("{:?}", tokens);

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

        if let Expression::FunctionCall{func, args} = expr {
            if let Expression::Number{val} = args[0] {
                match func {
                    Command::Forward => t.forward(val),
                    Command::Backward => t.backward(val),
                    Command::Left => t.left(val),
                    Command::Right => t.right(val),
                    Command::Exit => std::process::exit(0),
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
