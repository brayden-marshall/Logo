use regex::Regex;

#[derive(Debug, Clone)]
pub enum Token {
    Number(f64),
    Builtin(Builtin),
}

#[derive(Debug, Clone)]
pub enum Builtin {
    Forward,
    Backward,
    Left,
    Right,
}

struct TokenDefinition {
    token: Token,
    regex: Regex,
}

const NUMBER_REGEX: &str = r"^([0-9]+\.[0-9]+|[0-9]+)";
const FORWARD_REGEX: &str = r"^(fd|forward)";
const BACKWARD_REGEX: &str = r"^(bk|backward)";
const LEFT_REGEX: &str = r"^(lt|left)";
const RIGHT_REGEX: &str = r"^(rt|right)";

fn get_token_definitions() -> Vec<TokenDefinition> {
    vec![
        TokenDefinition { 
            token: Token::Number(0.0), 
            regex: Regex::new(NUMBER_REGEX).unwrap(),
        },
        TokenDefinition { 
            token: Token::Builtin(Builtin::Forward),
            regex: Regex::new(FORWARD_REGEX).unwrap(),
        },
        TokenDefinition { 
            token: Token::Builtin(Builtin::Backward),
            regex: Regex::new(BACKWARD_REGEX).unwrap(),
        },
        TokenDefinition { 
            token: Token::Builtin(Builtin::Left),
            regex: Regex::new(LEFT_REGEX).unwrap(),
        },
        TokenDefinition { 
            token: Token::Builtin(Builtin::Right),
            regex: Regex::new(RIGHT_REGEX).unwrap(),
        },
    ]
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut index = 0;
    let token_definitions = get_token_definitions();

    while index < input.len() {
        let mut found_match = false;
        // check every token definition for a match
        for def in token_definitions.iter() {

            // if we find a match for the current token
            if let Some(m) = def.regex.find(&input[index..]) {
                // special case for number because it has a value field
                if let Token::Number(_) = def.token {
                    tokens.push(Token::Number(
                        input[index..index+m.end()].parse()
                            .expect("Error parsing numeral")
                    ));
                } else {
                    tokens.push(def.token.clone());
                }
                // advance past the current token
                index += m.end();
                found_match = true;
                break;
            }
        }

        if !found_match {
            index += 1;
        }
    }

    tokens
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
}
