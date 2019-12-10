use std::collections::HashMap;
use std::default::Default;
use std::fmt;
#[allow(unused_imports)]
use std::iter::FromIterator;

use crate::error::LexError;

use regex::Regex;

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

    pub fn literal(&self) -> &str {
        use Operator::*;
        match self {
            Addition => "+",
            Subtraction => "-",
            Multiplication => "*",
            Division => "/",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
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

impl Token {
    /// Converts a token into a string representation.
    /// Does not include additional fields such as `literal` or `name`, only
    /// the name of the type of Token.
    ///
    /// Useful for some error reporting output that only cares about the expected
    /// token, but not any extra info.
    pub fn to_string(&self) -> &str {
        use Token::*;
        match self {
            Operator(op) => op.literal(),
            Number { literal: _ } => "Number",
            Word { literal: _ } => "Word",
            Variable { name: _ } => "Variable",
            Identifier { literal: _ } => "Identifier",
            Repeat => "repeat",
            Make => "make",
            To => "to",
            End => "end",
            LBracket => "[",
            RBracket => "]",
            LParen => "(",
            RParen => ")",
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(formatter, "{}", self.to_string())
    }
}

fn regex(input: &str) -> Regex {
    Regex::new(input).unwrap()
}

fn get_keywords() -> HashMap<String, Token> {
    let mut keywords = HashMap::<String, Token>::new();

    keywords.insert("repeat".to_string(), Token::Repeat);
    keywords.insert("make".to_string(), Token::Make);
    keywords.insert("to".to_string(), Token::To);
    keywords.insert("end".to_string(), Token::End);

    keywords
}

struct TokenDef {
    token: Token,
    regex: Regex,
}

impl TokenDef {
    fn new(token: Token, token_regex: &str) -> Self {
        TokenDef {
            token,
            regex: regex(token_regex),
        }
    }
}

const NUMBER_REGEX: &str = r"^-?[0-9]+";
const WORD_REGEX: &str = r#"^"[a-zA-Z][0-9a-zA-Z_]*"#;
const VARIABLE_REGEX: &str = r"^:[a-zA-Z][0-9a-zA-Z_]*";
const IDENT_REGEX: &str = r"^[a-zA-Z][0-9a-zA-Z_]*";

// returns a vector of the definition of every language token
// a token definition consists of it's enumerated type and
// it's regular expression used for parsing
fn get_token_definitions() -> Vec<TokenDef> {
    vec![
        TokenDef::new(
            Token::Number {
                literal: Default::default(),
            },
            NUMBER_REGEX,
        ),
        TokenDef::new(
            Token::Word {
                literal: Default::default(),
            },
            WORD_REGEX,
        ),
        TokenDef::new(
            Token::Variable {
                name: Default::default(),
            },
            VARIABLE_REGEX,
        ),
        TokenDef::new(
            Token::Identifier {
                literal: "".to_string(),
            },
            IDENT_REGEX,
        ),
        // bracket characters
        TokenDef::new(Token::LBracket, r"^\["),
        TokenDef::new(Token::RBracket, r"^\]"),
        TokenDef::new(Token::LParen, r"^\("),
        TokenDef::new(Token::RParen, r"^\)"),
        // operators
        TokenDef::new(Token::Operator(Operator::Addition), r"^\+"),
        TokenDef::new(Token::Operator(Operator::Subtraction), r"^-"),
        TokenDef::new(Token::Operator(Operator::Multiplication), r"^\*"),
        TokenDef::new(Token::Operator(Operator::Division), r"^/"),
    ]
}

type LexResult = Result<Token, LexError>;

// currently takes a reference to str as it's input source, in future it
// should ideally be changed to take an Iterator over chars, to be more
// flexible toward input source type
pub struct Lexer<'a> {
    source: &'a str,
    index: usize,
    token_definitions: Vec<TokenDef>,
    keywords: HashMap<String, Token>,
    whitespace_regex: Regex,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            index: 0,
            token_definitions: get_token_definitions(),
            keywords: get_keywords(),
            whitespace_regex: regex(r"^[\n\t\x20]*"),
        }
    }

    pub fn collect_tokens(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens: Vec<Token> = Vec::new();
        while let Some(lex_result) = self.next() {
            match lex_result {
                Ok(tok) => tokens.push(tok),
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(tokens)
    }

    // increasing internal index to the first non-whitespace character
    fn skip_whitespace(&mut self) {
        if let Some(m) = self.whitespace_regex.find(&self.source[self.index..]) {
            self.index += m.end();
        }
    }

    // consumes n characters from the underlying slice, returns the consumed content
    fn consume(&mut self, n: usize) -> String {
        let content = (&self.source[self.index..self.index + n]).to_string();
        self.index += n;
        content
    }
}

// the main functionality of the Lexer being implemented as an Iterator
impl<'a> Iterator for Lexer<'a> {
    type Item = LexResult;
    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        // if we have reached the end of source, return None
        if self.index >= self.source.len() {
            return None;
        }

        for def in self.token_definitions.iter() {
            // if we find a match for the current token
            if let Some(m) = def.regex.find(&self.source[self.index..]) {
                let token = match def.token {
                    Token::Number { literal: _ } => Token::Number {
                        literal: self.consume(m.end()),
                    },
                    Token::Word { literal: _ } => {
                        // advance 1 to ignore the leading " character
                        self.index += 1;
                        Token::Word {
                            // m.end() - 1 because we already skipped one character of the match
                            literal: self.consume(m.end() - 1),
                        }
                    }
                    Token::Variable { name: _ } => {
                        // advance 1 to ignore the leading " character
                        self.index += 1;
                        Token::Variable {
                            // m.end() - 1 because we already skipped one character of the match
                            name: self.consume(m.end() - 1),
                        }
                    }
                    Token::Identifier { literal: _ } => {
                        let literal = self.consume(m.end());
                        if let Some(tok) = self.keywords.get(&literal) {
                            tok.clone()
                        } else {
                            Token::Identifier { literal: literal }
                        }
                    }
                    _ => {
                        self.index += m.end();
                        def.token.clone()
                    },
                };

                return Some(Ok(token));
            }
        }

        // no match was found for any token definition
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

    #[test]
    fn lex_identifier_test() {
        lex_test(
            "hello HELLO hello_there N_UM53r5",
            vec![
                Token::Identifier {
                    literal: String::from("hello"),
                },
                Token::Identifier {
                    literal: String::from("HELLO"),
                },
                Token::Identifier {
                    literal: String::from("hello_there"),
                },
                Token::Identifier {
                    literal: String::from("N_UM53r5"),
                },
            ],
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
                Token::Identifier {
                    literal: "forward".to_string(),
                },
                Token::Number {
                    literal: String::from("100"),
                },
                Token::RBracket,
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
                Token::Identifier {
                    literal: "my_procedure".to_string(),
                },
                Token::Identifier {
                    literal: "forward".to_string(),
                },
                Token::Number {
                    literal: "100".to_string(),
                },
                Token::End,
            ],
        );
    }
}
