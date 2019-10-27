mod command;
mod error;
mod evaluator;
mod lexer;
mod parser;

use error::LogoError;
use evaluator::Evaluator;
use lexer::Lexer;
use parser::Parser;

// re-exports
pub use evaluator::Instruction;
pub use command::Command;

/// Exposed type that acts as the interface to the library.
pub struct Interpreter {
    evaluator: Evaluator,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            evaluator: Evaluator::new(),
        }
    }

    /// # Args
    /// - self
    /// - source: program source code to be run
    ///
    /// Goes through all phases of the interpreter (lexing, parsing, and evaluation) and
    /// returns a set instructions to be run by the frontend.
    ///
    /// # Return
    /// Returns a Vec of Instruction objects if the program runs successfully. The
    /// instructions being returned correspond to the turtle commands that will be
    /// run by the frontend. This includes things such as
    /// - Movement commands (forward, left, setxy ...)
    /// - Console output (show)
    /// - Misc. turtle commands (penup, hideturtle, setscreencolor ...)
    /// - Exit command
    ///
    /// Returns a LogoError if an error is encountered during execution.
    ///
    /// # Side effects
    /// Not all valid programs will return a set of instructions. Some programs will
    /// modify the state of the evaluator without having to send instructions to the
    /// frontend. Some examples include the following:
    /// - Declaring variables
    /// - Modifying variables
    /// - Declaring procedures
    ///
    /// If one of these programs runs successfully, it will return a Vec of length 0
    /// as the instructions set. If it fails, it will return an error as usual.
    pub fn run_program(&mut self, source: &str) -> Result<Vec<Instruction>, LogoError> {
        // lexing phase
        let mut lexer = Lexer::new(&source);
        let tokens = match lexer.collect_tokens() {
            Ok(t) => Ok(t),
            Err(e) => Err(LogoError::Lex(e)),
        }?;

        // parsing phase
        let mut parser = Parser::new(&tokens);
        let ast = match parser.build_ast() {
            Ok(ast) => Ok(ast),
            Err(e) => Err(LogoError::Parse(e)),
        }?;

        // evaluation phase
        match self.evaluator.evaluate_ast(&ast) {
            Ok(instructions) => Ok(instructions),
            Err(e) => Err(LogoError::Runtime(e)),
        }
    }
}
