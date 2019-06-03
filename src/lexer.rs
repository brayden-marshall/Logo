use regex::Regex;
#[allow(unused_imports)]
use std::iter::FromIterator;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number{literal: String},
    Command(Command),

    Repeat,

    LBracket,
    RBracket,
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
    Exit,

    // 1 arity
    Forward,
    Backward,
    Left,
    Right,
    SetPenSize,

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
            Exit | ClearScreen | Clean | PenUp | PenDown |
            HideTurtle | ShowTurtle | Home => 0,

            Forward | Backward | Left | Right | 
            SetPenSize  => 1,

            SetXY => 2,

            SetPenColor | SetScreenColor | SetFillColor => 3,
        }
    }
}

// currently takes a reference to str as it's input source, in future it
// should ideally be changed to take an Iterator over chars, to be more
// flexible toward input source type
pub struct Lexer<'a> {
    source: &'a str,
    index: usize,
    token_definitions: Vec<TokenDefinition>,
}

#[derive(Debug)]
pub enum LexError {
    NoMatchingToken,
}

type LexResult = Result<Token, LexError>;

struct TokenDefinition {
    token: Token,
    regex: Regex,
}

const NUMBER_REGEX: &str = r"^-?[0-9]+";

fn regex(input: &str) -> Regex {
    Regex::new(input).unwrap()
}

// returns a vector of the definition of every language token
// a token definition consists of it's enumerated type and
// it's regular expression used for parsing
fn get_token_definitions() -> Vec<TokenDefinition> {
    vec![
        TokenDefinition { 
            token: Token::Number{literal: String::from("")}, 
            regex: regex(NUMBER_REGEX),
        },
        TokenDefinition { 
            token: Token::Command(Command::Forward),
            regex: regex(r"^(fd|forward)"),
        },
        TokenDefinition { 
            token: Token::Command(Command::Backward),
            regex: regex(r"^(bk|backward)"),
        },
        TokenDefinition { 
            token: Token::Command(Command::Left),
            regex: regex(r"^(lt|left)"),
        },
        TokenDefinition { 
            token: Token::Command(Command::Right),
            regex: regex(r"^(rt|right)"),
        },
        TokenDefinition {
            token: Token::Command(Command::Exit),
            regex: regex(r"^exit"),
        },
        TokenDefinition {
            token: Token::Command(Command::ClearScreen),
            regex: regex(r"^(cs|clearscreen)"),
        },
        TokenDefinition {
            token: Token::Command(Command::Clean),
            regex: regex(r"^clean"),
        },
        TokenDefinition {
            token: Token::Command(Command::PenUp),
            regex: regex(r"^(pu|penup)"),
        },
        TokenDefinition {
            token: Token::Command(Command::PenDown),
            regex: regex(r"^(pd|pendown)"),
        },
        TokenDefinition {
            token: Token::Command(Command::HideTurtle),
            regex: regex(r"^(ht|hideturtle)"),
        },
        TokenDefinition {
            token: Token::Command(Command::ShowTurtle),
            regex: regex(r"^(st|showturtle)"),
        },
        TokenDefinition {
            token: Token::Command(Command::Home),
            regex: regex(r"^home"),
        },
        TokenDefinition {
            token: Token::Command(Command::SetXY),
            regex: regex(r"^setxy"),
        },
        TokenDefinition {
            token: Token::Command(Command::SetPenSize),
            regex: regex(r"^(setps|setpensize)"),
        },
        TokenDefinition {
            token: Token::Command(Command::SetPenColor),
            regex: regex(r"^(setpc|setpencolor)"),
        },
        TokenDefinition {
            token: Token::Command(Command::SetFillColor),
            regex: regex(r"^(setfc|setfillcolor)"),
        },
        TokenDefinition {
            token: Token::Command(Command::SetScreenColor),
            // order of two strings between | must stay this way
            // if switched, full name will not be properly detected
            regex: regex(r"^(setscreencolor|setsc)"),
        },
        TokenDefinition {
            token: Token::Repeat,
            regex: regex(r"^repeat"),
        },
        TokenDefinition {
            token: Token::LBracket,
            regex: regex(r"^\["),
        },
        TokenDefinition {
            token: Token::RBracket,
            regex: regex(r"^\]"),
        },
    ]
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            index: 0,
            token_definitions: get_token_definitions(),
        }
    }

    // increasing internal index to the first non-whitespace character
    fn skip_whitespace(&mut self) {
        let whitespace_chars = ["\t", "\n", "\x20"];
        let mut whitespace_found = true;
        while whitespace_found {
            whitespace_found = false;
            for chr in whitespace_chars.iter() {
                if self.source[self.index..].starts_with(chr) {
                    whitespace_found = true;
                    self.index += 1;
                    break;
                }
            }
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

                // special case for number because it has a value field
                let token = if let Token::Number{literal: _} = def.token {
                    Token::Number{
                        literal: String::from(
                                 &self.source[self.index..self.index+m.end()])
                    }
                } else {
                    def.token.clone()
                };

                // increasing internal index counter by the number of characters
                // the token consumed
                self.index += m.end();
                return Some(Ok(token));
            }
        }

        Some(Err(LexError::NoMatchingToken))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_regex_test() {
        let number_regex = Regex::new(NUMBER_REGEX).unwrap();
        let test_strings = vec!["1", "123456789", "-567", "-2943090"];
        let test_positions = vec![1, 9, 4, 8];

        for i in 0..test_strings.len() {
            if let Some(m) = number_regex.find(test_strings[i]) {
                assert_eq!(m.start(), 0);
                assert_eq!(m.end(), test_positions[i]);
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
                Number{literal: String::from("0")}, Number{literal: String::from("100")}, 
                Number{literal: String::from("-79")}, 
            ]
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
            setsc setscreencolor
            ",
            commands!(
                PenUp, PenUp, PenDown, PenDown, HideTurtle, HideTurtle,
                ShowTurtle, ShowTurtle, ClearScreen, ClearScreen, Home,
                Exit, Forward, Forward, Backward, Backward, Left, Left,
                Right, Right, SetXY, Clean, SetPenSize, SetPenSize,
                SetPenColor, SetPenColor, SetFillColor, SetFillColor,
                SetScreenColor, SetScreenColor
            ),
        );
    }

    #[test]
    fn lex_repeat_test() {
        lex_test(
            "repeat 7 [ forward 100 ]",
            vec![
                Token::Repeat, Token::Number{literal: String::from("7")}, Token::LBracket,
                Token::Command(Command::Forward), Token::Number{literal: String::from("100")},
                Token::RBracket
            ]
        );
    }

    // this test was added because there was a bug with setscreencolor command
    #[test]
    fn lex_set_screen_color_test() {
        lex_test(
            "setscreencolor 100 100 100",
            vec![
                Token::Command(Command::SetScreenColor),
                Token::Number{literal: String::from("100")},
                Token::Number{literal: String::from("100")},
                Token::Number{literal: String::from("100")},
            ]
        );
    }
}
