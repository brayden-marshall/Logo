use regex::Regex;

macro_rules! regex {
    ($pattern:expr) => {
        Regex::new($pattern).unwrap();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Command(Command),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Forward,
    Backward,
    Left,
    Right,
    Exit,
}

impl Command {
    pub fn arity(&self) -> usize {
        match self {
            Command::Forward | Command::Backward | 
            Command::Left | Command::Right => 1,
            Command::Exit => 0,
        }
    }
}

pub struct Lexer<'a> {
    source: &'a str,
    index: usize,
    token_definitions: Vec<TokenDefinition>,
}

struct TokenDefinition {
    token: Token,
    regex: Regex,
}

const NUMBER_REGEX: &str = r"^([0-9]+\.[0-9]+|[0-9]+)";

fn get_token_definitions() -> Vec<TokenDefinition> {
    vec![
        TokenDefinition { 
            token: Token::Number(0.0), 
            regex: regex!(NUMBER_REGEX),
        },
        TokenDefinition { 
            token: Token::Command(Command::Forward),
            regex: regex!(r"^(fd|forward)"),
        },
        TokenDefinition { 
            token: Token::Command(Command::Backward),
            regex: regex!(r"^(bk|backward)"),
        },
        TokenDefinition { 
            token: Token::Command(Command::Left),
            regex: regex!(r"^(lt|left)"),
        },
        TokenDefinition { 
            token: Token::Command(Command::Right),
            regex: regex!(r"^(rt|right)"),
        },
        TokenDefinition {
            token: Token::Command(Command::Exit),
            regex: regex!(r"^exit"),
        }
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

    fn skip_whitespace(&mut self) {
        let whitespace_regex = Regex::new("^[\t\n\x20]").unwrap();
        while whitespace_regex.is_match(&self.source[self.index..]) {
            self.index += 1;
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        for def in self.token_definitions.iter() {
            // if we find a match for the current token
            if let Some(m) = def.regex.find(&self.source[self.index..]) {
                let token: Option<Token>;

                // special case for number because it has a value field
                if let Token::Number(_) = def.token {
                    token = Some(Token::Number(
                        self.source[self.index..self.index+m.end()].parse()
                            .expect("Error parsing numeral")
                    ));
                } else {
                    token = Some(def.token.clone());
                }
                // increasing internal index counter by the number of characters
                // the token consumed
                self.index += m.end();
                return token;
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_regex_test() {
        let number_regex = Regex::new(NUMBER_REGEX).unwrap();
        let test_strings = vec!["1", "123456789", "1.0", "123456789.987654321"];
        let test_positions = vec![(0, 1), (0, 9), (0, 3), (0, 19)];

        for i in 0..test_strings.len() {
            if let Some(m) = number_regex.find(test_strings[i]) {
                assert_eq!(m.start(), test_positions[i].0);
                assert_eq!(m.end(), test_positions[i].1);
            } else {
                panic!("Match not found");
            }
        }
    }

    #[test]
    fn lex_short_command_test() {
        let input_string: &str = "forward 100 bk 683.27";
        let lexer = Lexer::new(input_string);
        let output_vec: Vec<Token> = lexer.collect();
        assert_eq!(
            output_vec,
            vec![
                Token::Command(Command::Forward), Token::Number(100.0),
                Token::Command(Command::Backward), Token::Number(683.27),
            ],
       );
    }
}
