use regex::Regex;
#[allow(unused_imports)]
use std::iter::FromIterator;
use std::default::Default;

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl Operator {
    pub fn precedence(&self) -> usize {
        use Operator::*;

        match self {
            Multiplication | Division => 2,
            Addition | Subtraction => 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    // 0 arity
    PenUp,
    PenDown,
    HideTurtle,
    ShowTurtle,
    Home,
    ClearScreen,
    Clean,
    Fill,
    Exit,

    // 1 arity
    Forward,
    Backward,
    Left,
    Right,
    SetPenSize,
    SetHeading,
    Show,

    // 2 arity
    SetXY,

    // 3 arity
    SetPenColor,
    SetScreenColor,
    SetFillColor,
}

impl Command {
    pub fn arity(&self) -> usize {
        use Command::*;

        match self {
            Exit | ClearScreen | Clean | PenUp | PenDown | HideTurtle
            | ShowTurtle | Home | Fill => 0,

            Forward | Backward | Left | Right | SetPenSize | Show | SetHeading => 1,

            SetXY => 2,

            SetPenColor | SetScreenColor | SetFillColor => 3,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Command(Command),
    Operator(Operator),

    Number { literal: String },
    Word { literal: String },
    Variable { name: String },
    Identifier { literal: String },

    Repeat,
    Make,
    To,
    End,

    LBracket,
    RBracket,
    LParen,
    RParen,
}

struct TokenDef {
    token: Token,
    regex: Regex,
}

impl TokenDef {
    fn new(token: Token, _regex: &str) -> Self {
        TokenDef {
            token,
            regex: regex(_regex),
        }
    }
}

const NUMBER_REGEX: &str = r"^-?[0-9]+";
const WORD_REGEX: &str = r#"^"[a-zA-Z_][0-9a-zA-Z_]*"#;
const VARIABLE_REGEX: &str = r"^:[a-zA-Z_][0-9a-zA-Z_]*";

fn regex(input: &str) -> Regex {
    Regex::new(input).unwrap()
}

// returns a vector of the definition of every language token
// a token definition consists of it's enumerated type and
// it's regular expression used for parsing
fn get_token_definitions() -> Vec<TokenDef> {
    vec![
        TokenDef::new(Token::Number { literal: Default::default() }, NUMBER_REGEX),
        TokenDef::new(Token::Word { literal: Default::default() }, WORD_REGEX),
        TokenDef::new(Token::Variable { name: Default::default() }, VARIABLE_REGEX),
        TokenDef::new(Token::Command(Command::Forward), r"^(fd|forward)"),
        TokenDef::new(Token::Command(Command::Backward), r"^(bk|backward)"),
        TokenDef::new(Token::Command(Command::Left), r"^(lt|left)"),
        TokenDef::new(Token::Command(Command::Right), r"^(rt|right)"),
        TokenDef::new(Token::Command(Command::Exit), r"^exit"),
        TokenDef::new(Token::Command(Command::ClearScreen), r"^(cs|clearscreen)"),
        TokenDef::new(Token::Command(Command::Clean), r"^clean"),
        TokenDef::new(Token::Command(Command::PenUp), r"^(pu|penup)"),
        TokenDef::new(Token::Command(Command::PenDown), r"^(pd|pendown)"),
        TokenDef::new(Token::Command(Command::HideTurtle), r"^(ht|hideturtle)"),
        TokenDef::new(Token::Command(Command::ShowTurtle), r"^(st|showturtle)"),
        TokenDef::new(Token::Command(Command::Home), r"^home"),
        TokenDef::new(Token::Command(Command::SetXY), r"^setxy"),
        TokenDef::new(Token::Command(Command::SetPenSize), r"^(setps|setpensize)"),
        TokenDef::new(Token::Command(Command::SetPenColor), r"^(setpc|setpencolor)"),
        TokenDef::new(Token::Command(Command::SetFillColor), r"^(setfc|setfillcolor)"),
        // for setscreencolor: order of two strings between | must stay this way
        // if switched, full name will not be properly detected
        TokenDef::new(Token::Command(Command::SetScreenColor), r"^(setscreencolor|setsc)"),
        TokenDef::new(Token::Command(Command::SetHeading), r"^(setheading|seth)"),
        TokenDef::new(Token::Command(Command::Fill), r"^fill"),
        TokenDef::new(Token::Command(Command::Show), r"^show"),
        TokenDef::new(Token::Repeat, r"^repeat"),
        TokenDef::new(Token::Make, r"^make"),
        TokenDef::new(Token::To, r"^to"),
        TokenDef::new(Token::End, r"^end"),
        TokenDef::new(Token::LBracket, r"^\["),
        TokenDef::new(Token::RBracket, r"^\]"),
        TokenDef::new(Token::LParen, r"^\("),
        TokenDef::new(Token::RParen, r"^\)"),
        TokenDef::new(Token::Operator(Operator::Addition), r"^\+"),
        TokenDef::new(Token::Operator(Operator::Subtraction), r"^-"),
        TokenDef::new(Token::Operator(Operator::Multiplication), r"^\*"),
        TokenDef::new(Token::Operator(Operator::Division), r"^/"),
        TokenDef::new(Token::Identifier { literal: "".to_string() }, r"^[a-zA-Z_][0-9a-zA-Z_]*"),
    ]
}

#[derive(Debug)]
pub enum LexError {
    UnrecognizedToken,
}

type LexResult = Result<Token, LexError>;

// currently takes a reference to str as it's input source, in future it
// should ideally be changed to take an Iterator over chars, to be more
// flexible toward input source type
pub struct Lexer<'a> {
    source: &'a str,
    index: usize,
    token_definitions: Vec<TokenDef>,
    whitespace_regex: Regex,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            index: 0,
            token_definitions: get_token_definitions(),
            whitespace_regex: regex(r"^[\n\t\x20]*"),
        }
    }

    // increasing internal index to the first non-whitespace character
    fn skip_whitespace(&mut self) {
        if let Some(m) = self.whitespace_regex.find(&self.source[self.index..]) {
            self.index += m.end();
        }
    }
}

// the main functionality of the Lexer being implemented as an Iterator
impl<'a> Iterator for Lexer<'a> {
    type Item = LexResult;
    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        // if we have reached the end of source, return None
        if self.index == self.source.len() {
            return None;
        }

        for def in self.token_definitions.iter() {
            // if we find a match for the current token
            if let Some(m) = def.regex.find(&self.source[self.index..]) {
                let token = match def.token {
                    Token::Number { literal: _ } => Token::Number {
                        literal: String::from(&self.source[self.index..self.index + m.end()]),
                    },
                    Token::Word { literal: _ } => Token::Word {
                        literal: String::from(
                            // index+1 to ignore the leading " character
                            &self.source[self.index + 1..self.index + m.end()],
                        ),
                    },
                    Token::Variable { name: _ } => Token::Variable {
                        name: String::from(
                            // index+1 to ignore the leading : character
                            &self.source[self.index + 1..self.index + m.end()],
                        ),
                    },
                    Token::Identifier { literal: _ } => Token::Identifier {
                        literal: String::from(
                            &self.source[self.index..self.index + m.end()],
                        ),
                    },
                    _ => def.token.clone(),
                };

                // increasing internal index counter by the number of characters
                // the token consumed
                self.index += m.end();
                return Some(Ok(token));
            }
        }

        Some(Err(LexError::UnrecognizedToken))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_regex_test() {
        let number_regex = Regex::new(NUMBER_REGEX).unwrap();
        let test_strings = vec!["1", "123456789", "-567", "-2943090"];
        //let test_positions = vec![1, 9, 4, 8];

        for input in test_strings.iter() {
            if let Some(m) = number_regex.find(input) {
                assert_eq!(m.start(), 0);
                assert_eq!(m.end(), input.len());
            } else {
                panic!("Match not found");
            }
        }
    }

    #[test]
    fn word_regex_test() {
        let word_regex = Regex::new(WORD_REGEX).unwrap();
        let test_strings = vec!["\"size"];

        for input in test_strings.iter() {
            if let Some(m) = word_regex.find(input) {
                assert_eq!(m.start(), 0);
                assert_eq!(m.end(), input.len());
            } else {
                panic!("Match not found");
            }
        }
    }

    fn lex_test(input: &str, expected: Vec<Token>) {
        let lexer = Lexer::new(input);
        let output_vec = Vec::from_iter(lexer.map(|tok| tok.unwrap()));
        assert_eq!(output_vec, expected);
    }

    #[test]
    fn lex_number_test() {
        use Token::Number;
        lex_test(
            "0 100 -79 ",
            vec![
                Number {
                    literal: String::from("0"),
                },
                Number {
                    literal: String::from("100"),
                },
                Number {
                    literal: String::from("-79"),
                },
            ],
        );
    }

    #[test]
    fn lex_word_test() {
        lex_test(
            "\"size \"COUNT \"under_SCORE \"H5H6H7",
            vec![
                Token::Word {
                    literal: String::from("size"),
                },
                Token::Word {
                    literal: String::from("COUNT"),
                },
                Token::Word {
                    literal: String::from("under_SCORE"),
                },
                Token::Word {
                    literal: String::from("H5H6H7"),
                },
            ],
        );
    }

    #[test]
    fn lex_variable_test() {
        lex_test(
            ":angle :SIZE :mixed_LETTERS :variable_123",
            vec![
                Token::Variable {
                    name: String::from("angle"),
                },
                Token::Variable {
                    name: String::from("SIZE"),
                },
                Token::Variable {
                    name: String::from("mixed_LETTERS"),
                },
                Token::Variable {
                    name: String::from("variable_123"),
                },
            ],
        );
    }

    macro_rules! commands {
        ($($i:ident),+) => {
            vec![$(Token::Command(Command::$i)),+]
        }
    }

    #[test]
    fn lex_command_test() {
        lex_test(
            "pu      penup pd pendown ht hideturtle st showturtle
            cs clearscreen home exit
            fd forward bk backward lt left rt right setxy clean
            setps setpensize setpc setpencolor setfc setfillcolor
            setsc setscreencolor fill show seth setheading
            ",
            commands!(
                PenUp,
                PenUp,
                PenDown,
                PenDown,
                HideTurtle,
                HideTurtle,
                ShowTurtle,
                ShowTurtle,
                ClearScreen,
                ClearScreen,
                Home,
                Exit,
                Forward,
                Forward,
                Backward,
                Backward,
                Left,
                Left,
                Right,
                Right,
                SetXY,
                Clean,
                SetPenSize,
                SetPenSize,
                SetPenColor,
                SetPenColor,
                SetFillColor,
                SetFillColor,
                SetScreenColor,
                SetScreenColor,
                Fill,
                Show,
                SetHeading,
                SetHeading
            ),
        );
    }

    #[test]
    fn lex_operator_test() {
        lex_test(
            "+ - * /",
            vec![
                Token::Operator(Operator::Addition),
                Token::Operator(Operator::Subtraction),
                Token::Operator(Operator::Multiplication),
                Token::Operator(Operator::Division),
            ],
        );
    }

    #[test]
    fn lex_bracket_paren_test() {
        lex_test(
            "[ ] ( )",
            vec![
                Token::LBracket,
                Token::RBracket,
                Token::LParen,
                Token::RParen,
            ],
        );
    }

    #[test]
    fn lex_repeat_test() {
        lex_test(
            "repeat 7 [ forward 100 ]",
            vec![
                Token::Repeat,
                Token::Number {
                    literal: String::from("7"),
                },
                Token::LBracket,
                Token::Command(Command::Forward),
                Token::Number {
                    literal: String::from("100"),
                },
                Token::RBracket,
            ],
        );
    }

    // this test was added because there was a bug with setscreencolor command
    #[test]
    fn lex_set_screen_color_test() {
        lex_test(
            "setscreencolor 100 100 100",
            vec![
                Token::Command(Command::SetScreenColor),
                Token::Number {
                    literal: String::from("100"),
                },
                Token::Number {
                    literal: String::from("100"),
                },
                Token::Number {
                    literal: String::from("100"),
                },
            ],
        );
    }

    #[test]
    fn lex_variable_declaration_test() {
        lex_test(
            "make \"size 130",
            vec![
                Token::Make,
                Token::Word {
                    literal: String::from("size"),
                },
                Token::Number {
                    literal: String::from("130"),
                },
            ],
        );
    }

    #[test]
    fn lex_procedure_test() {
        lex_test(
            "to my_procedure forward 100 end",
            vec![
                Token::To,
                Token::Identifier { literal: "my_procedure".to_string() },
                Token::Command(Command::Forward),
                Token::Number { literal: "100".to_string() },
                Token::End,
            ],
        );
    }
}
