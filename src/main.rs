use std::fs;

use clap::{App, Arg};
use rustyline::error::ReadlineError;
use rustyline::Editor;

mod commands;
mod error;
mod lexer;
mod parser;
mod evaluator;

use evaluator::{Evaluator, EvaluatorConfig};

/// Simple function to print to either stdout or stderr based on
/// a given Result object.
fn print_program_output(program_result: Result<String, String>) {
    match program_result {
        Ok(output) => print!("{}", output),
        Err(output) => eprint!("{}", output),
    };
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

    // create the Evaluator object
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

    // run interactive shell using the rustyline crate
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
