use std::fs;

use clap::{App, Arg};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use turtle::Turtle;

use logo::{Interpreter, Instruction, Command};

fn run_instructions(instructions: &Vec<Instruction>, turtle: &mut Turtle) {
    for instruction in instructions.iter() {
        let args: Vec<_> = instruction.args.iter().map(|val| {
            *val as f64
        }).collect();

        use Command::*;
        match instruction.command {
            Forward => turtle.forward(args[0]),
            Backward => turtle.backward(args[0]),
            Left => turtle.left(args[0]),
            Right => turtle.right(args[0]),
            SetHeading => turtle.set_heading(args[0]),
            SetXY => turtle.go_to([args[0], args[1]]),
            Home => turtle.home(),

            // pen
            PenUp => turtle.pen_up(),
            PenDown => turtle.pen_down(),
            SetPenSize => turtle.set_pen_size(args[0]),
            SetPenColor => turtle.set_pen_color([args[0], args[1], args[2]]),

            // other
            HideTurtle => turtle.hide(),
            ShowTurtle => turtle.show(),
            ClearScreen => { turtle.clear(); turtle.home() },
            Clean => turtle.clear(),
            SetScreenColor => 
                turtle
                    .drawing_mut()
                    .set_background_color([args[0], args[1], args[2]]),
            Show => println!("{}", args[0]),
            Exit => std::process::exit(0),
        }
    }
}

fn main() {
    // clap CLI app setup
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
            Arg::with_name("no-turtle")
                .short("n")
                .long("no-turtle")
                .help("do not create turtle or window or startup")
                .takes_value(false),
        )
        .get_matches();

    // create the Interpreter
    let mut interpreter = Interpreter::new();

    // create the turtle (also creates the window)
    let mut turtle = Turtle::new();

    // if a script argument was passed, run the script
    if let Some(file) = matches.value_of("SCRIPT") {
        // read in the file
        let instructions = interpreter.run_program(
            match &fs::read_to_string(file) {
                Ok(input) => input,
                Err(e) => {
                    eprintln!("Error reading file: {}", e);
                    std::process::exit(1);
                }
            },
        );

        match instructions {
            Ok(i) => run_instructions(&i, &mut turtle),
            Err(e) => eprintln!("{}", e)
        };
    }

    // run interactive shell using the rustyline crate
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                match interpreter.run_program(&line) {
                    Ok(i) => run_instructions(&i, &mut turtle),
                    Err(e) => eprintln!("{}", e),
                }
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
