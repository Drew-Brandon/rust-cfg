use std::{collections::HashMap, fmt};
use crate::syntax::{ContextFreeLanguage, Disjunct, ProductionRule, Symbol::{self, Nonterminal}, Token};

pub fn parse(input: &Vec<Token>) -> Result<ContextFreeLanguage, ParserError> {
    let mut parser = Parser {
        tokens: input.clone(),
        position: 0
    };

    parser.parse_langauge()
}

pub struct ParserError {
    line: u32,
    token: Token,
    position: usize
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: Unexpected token {} at position {}.",
            self.line, self.token, self.position
        )
    }
}

struct Parser {
    tokens: Vec<Token>,
    position: usize
}

impl Parser {
    fn peek(&self) -> &Token {
        if self.position >= self.tokens.len() {
            &Token::End
        } else {
            &self.tokens[self.position]
        }
    }

    fn advance(&mut self) -> Token {
        let token = self.peek().clone();

        if self.position < self.tokens.len() {
            self.position += 1;
        }

        token
    }

    fn expect(&mut self, expected: &Token) -> Result<(), ParserError> {
        if self.peek() == expected {
            self.advance();
            Ok(())
        } else {
            Err(self.throw(line!()))
        }
    }

    fn throw(&self, line: u32) -> ParserError {
        ParserError {
            line: line,
            token: self.peek().clone(),
            position: self.position
        }
    }
    
    fn parse_langauge(&mut self) -> Result<ContextFreeLanguage, ParserError> {
        let mut production_rules_map: HashMap<char, ProductionRule> = HashMap::new();

        loop {
            let production_rule = self.parse_production_rule()?;
            production_rules_map.insert(production_rule.nonterminal, production_rule);
            
            if self.peek() == &Token::End {
                return Ok(ContextFreeLanguage::new(&production_rules_map));
            }

            self.expect(&Token::NewLine)?;
        }
    }

    fn parse_production_rule(&mut self) -> Result<ProductionRule, ParserError> {
        let peek_result = match self.peek() {
            Token::Symbol(symbol) => match symbol {
                Nonterminal(ch) => Some(*ch),
                _ => None
            },
            _ => None
        };
        
        match peek_result {
            Some(ch) => {
                self.advance();
                Ok(ProductionRule::new(
                    ch,
                    &self.parse_disjuncts()?
                ))
            },
            None => Err(self.throw(line!()))
        }
    }

    fn parse_disjuncts(&mut self) -> Result<Vec<Disjunct>, ParserError> {
        let mut disjuncts: Vec<Disjunct> = Vec::new();
        
        loop {
            disjuncts.push(self.parse_disjunct()?);
            
            if matches!(self.peek(), Token::NewLine | Token::End) {
                return Ok(disjuncts);
            }

            self.expect(&Token::Disjunction)?;
        }
    }

    fn parse_disjunct(&mut self) -> Result<Disjunct, ParserError> {
        if self.peek() == &Token::Empty {
            return Ok(Disjunct::Empty);
        }

        let mut symbols: Vec<Symbol> = Vec::new();
        
        loop {
            symbols.push(self.parse_symbol()?);

            if matches!(self.peek(), Token::Disjunction | Token::NewLine | Token::End) {
                return Ok(Disjunct::Sequences(symbols));
            }
        }
    }

    fn parse_symbol(&mut self) -> Result<Symbol, ParserError> {
        let peek_result = match self.peek() {
            Token::Symbol(symbol) => Some(symbol.clone()),
            _ => None
        };

        match peek_result {
            Some(symbol) => {
                self.advance();
                Ok(symbol)
            },
            None => Err(self.throw(line!()))
        }
    }
}
