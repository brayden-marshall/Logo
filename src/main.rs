use std::fs;

use clap::{App, Arg};
use rustyline::error::ReadlineError;
use rustyline::Editor;

mod commands;
mod error;
mod evaluator;
mod lexer;
mod parser;

use evaluator::Evaluator;
use lexer::Lexer;
use parser::Parser;

/// Simple function to print to either stdout or stderr based on
/// a given Result object.
fn print_program_output(program_result: Result<String, String>) {
    match program_result {
        Ok(output) => print!("{}", output),
        Err(output) => eprint!("{}", output),
    };
}

fn run_program(source: &str, evaluator: &mut Evaluator, debug: bool) -> Result<String, String> {
    let mut program_output = String::new();

    // lexing phase
    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.collect_tokens() {
        Ok(t) => Ok(t),
        Err(e) => Err(format!("{}Error: {}\n", program_output, e)),
    }?;

    if debug {
        // append lexing debug info onto output
        program_output = format!(
            "{}Lexing phase completed without error\n{:?}\n",
            program_output, tokens,
        );
    }

    // parsing phase
    let mut parser = Parser::new(&tokens);
    let ast = match parser.build_ast() {
        Ok(ast) => Ok(ast),
        Err(e) => Err(format!("{}{}", program_output, e)),
    }?;

    if debug {
        // append parsing debug info onto output
        program_output = format!(
            "{}Parsing phase completed without error\n{:?}\n",
            program_output, ast,
        );
    }

    // evaluate and return the output
    match evaluator.run_ast(&ast) {
        Ok(_) => Ok(program_output),
        Err(output) => Err(format!("{}{}", program_output, output)),
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
    let mut evaluator = Evaluator::new(!matches.is_present("no-turtle"));

    let debug = matches.is_present("debug");

    // if a script argument was passed, run the script
    if let Some(file) = matches.value_of("SCRIPT") {
        print_program_output(run_program(
            match &fs::read_to_string(file) {
                Ok(input) => input,
                Err(e) => {
                    eprint!("Error reading file: {}\n", e);
                    std::process::exit(1);
                }
            },
            &mut evaluator,
            debug,
        ));
    }

    // run interactive shell using the rustyline crate
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                print_program_output(run_program(&line, &mut evaluator, debug));
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
