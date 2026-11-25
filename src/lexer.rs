use std::fmt;

use crate::syntax::{Symbol, Token};

/// Converts production's rhs into tokens.

/// Uppercase letters are nonterminals, lowercase letters are terminals, and
/// '|' separates disjuncts. Whitespace is ignored.
pub fn lex(input: &str) -> Result<Vec<Token>, LexerError> {
    let mut tokens = Vec::new();

    //Loop through each character in the input string and convert it to a token.
    for (index, ch) in input.chars().enumerate() {
        let token = if ch.is_whitespace() {
            continue;
        } 
        
        else if ch == '|' {
            Token::Disjunction
        } else if ch.is_uppercase() {
            Token::Symbol(Symbol::Nonterminal(ch))
        } else if ch.is_lowercase() {
            Token::Symbol(Symbol::Terminal(ch))
        } else {
            return Err(LexerError { //return on first invalid char 
                character: ch,
                position: index + 1, //1 based indexing
            });
        };

        tokens.push(token);
    }

    Ok(tokens)
}

/// Describes a character that is not valid in expression.
#[derive(Debug, PartialEq, Eq)]
pub struct LexerError {
    character: char,
    position: usize,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "unexpected character {:?} at position {}.",
            self.character, self.position
        )
    }
}

impl std::error::Error for LexerError {}

//some test cases
#[cfg(test)]
mod tests {
    use super::lex;
    use crate::syntax::{Symbol, Token};

    #[test]
    fn tokenizes_symbols_and_disjunctions() {
        let tokens = lex("aSb | f").unwrap();

        assert!(matches!(
            tokens.as_slice(),
            [
                Token::Symbol(Symbol::Terminal('a')),
                Token::Symbol(Symbol::Nonterminal('S')),
                Token::Symbol(Symbol::Terminal('b')),
                Token::Disjunction,
                Token::Symbol(Symbol::Terminal('f'))
            ]
        ));
    }

    #[test]
    fn ignores_whitespace() {
        let tokens = lex(" a \n S\tb ").unwrap();

        assert!(matches!(
            tokens.as_slice(),
            [
                Token::Symbol(Symbol::Terminal('a')),
                Token::Symbol(Symbol::Nonterminal('S')),
                Token::Symbol(Symbol::Terminal('b'))
            ]
        ));
    }

    #[test]
    fn reports_invalid_characters_with_position() {
        let error = match lex("ab -> c") {
            Err(error) => error,
            Ok(_) => panic!("expected invalid input to fail"),
        };

        assert_eq!(error.character, '-');
        assert_eq!(error.position, 4);
    }
}