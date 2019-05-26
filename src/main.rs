use std::io::{self, Write};
use turtle::Turtle;

mod lexer;
mod parser;

use lexer::{tokenize, Builtin};
use parser::{AST, Expr};

fn main() {
    let mut input: String;
    let mut t = Turtle::new();

    loop {
        input = get_input();

        let tokens = tokenize(&input);
        println!("{:?}", tokens);
        let ast = AST::build(&tokens);
        println!("{:?}", ast);

        run(&mut t, &ast);
    }
}

fn run(t: &mut Turtle, ast: &AST) {
    for expr in ast.expressions.iter() {
        if let Expr::ProgramStart = expr {
            continue;
        }

        if let Expr::FunctionCall{func, args} = expr {
            if let Expr::Number{val} = args[0] {
                match func {
                    Builtin::Forward => t.forward(val),
                    Builtin::Backward => t.backward(val),
                    Builtin::Left => t.left(val),
                    Builtin::Right => t.right(val),
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
