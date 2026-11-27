use std::fmt;
use std::clone;
use std::collections::HashMap;
use std::cmp;

/// Represents a single piece of some disjunct.
/// Note that tokens cannot be used to directly represent languages.
/// Instead, they are created by the lexer and then used by the parser to create languages.
pub enum Token {
    Symbol(Symbol),
    Empty,
    Disjunction,
    NewLine,
    End
}

impl cmp::PartialEq for Token {
    fn eq(&self, right: &Token) -> bool {
        match (self, right) {
            (Self::Symbol(left_symbol), Self::Symbol(right_symbol)) => left_symbol == right_symbol,
            (Self::Empty, Self::Empty) => true,
            (Self::Disjunction, Self::Disjunction) => true,
            (Self::NewLine, Self::NewLine) => true,
            (Self::End, Self::End) => true,
            (_, _) => false
        }
    }
}

impl clone::Clone for Token {
    fn clone(&self) -> Self {
        match self {
            Self::Symbol(symbol) => Self::Symbol(symbol.clone()),
            Self::Empty => Self::Empty,
            Self::Disjunction => Self::Disjunction,
            Self::NewLine => Self::NewLine,
            Self::End => Self::End
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Symbol(symbol) => write!(f, "{}", symbol),
            Self::Empty => write!(f, "Empty"),
            Self::Disjunction => write!(f, "|"),
            Self::NewLine => write!(f, "New Line"),
            Self::End => write!(f, "End")
        }
    }
}

/// Represents a nonterminal or terminal symbol.
pub enum Symbol {
    Nonterminal(char),
    Terminal(char)
}

impl cmp::PartialEq for Symbol {
    fn eq(&self, right: &Symbol) -> bool {
        match (self, right) {
            (Self::Nonterminal(left_ch), Self::Nonterminal(right_ch)) => left_ch == right_ch,
            (Self::Terminal(left_ch), Self::Terminal(right_ch)) => left_ch == right_ch,
            (_, _) => false
        }
    }
}

impl clone::Clone for Symbol {
    fn clone(&self) -> Self {
        match self {
            Self::Nonterminal(ch) => Self::Nonterminal(*ch),
            Self::Terminal(ch) => Self::Terminal(*ch)
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Nonterminal(ch) => write!(f, "{}", ch),
            Self::Terminal(st) => write!(f, "{}", st)
        }
    }
}

/// Represents a disjunct of a production rule.
/// This is mainly used to store a series of symbols.
pub enum Disjunct {
    Sequences(Vec<Symbol>),
    Empty
}

impl clone::Clone for Disjunct {
    fn clone(&self) -> Self {
        match self {
            Self::Sequences(symbols) => Self::Sequences(symbols.clone()),
            Self::Empty => Self::Empty
        }
    }
}

impl fmt::Display for Disjunct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Sequences(symbols) => {
                let mut display_str = String::from("");

                for symbol in symbols.iter() {
                    display_str.push_str(&format!("{}", symbol));
                }

                write!(f, "{}", display_str)
            },
            Self::Empty => write!(f, "Empty")
        }
    }
}

/// Represents a production rule of a nonterminal symbol.
pub struct ProductionRule {
    /// The nonterminal that points to the production rule.
    pub nonterminal: char,
    /// The disjuncts that make up the production rule.
    disjuncts: Vec<Disjunct>
}

impl ProductionRule {
    pub fn new(nonterminal: char, disjuncts: &Vec<Disjunct>) -> Self {
        ProductionRule {
            nonterminal: nonterminal,
            disjuncts: disjuncts.clone()
        }
    }
}

impl clone::Clone for ProductionRule {
    fn clone(&self) -> Self {
        ProductionRule {
            nonterminal: self.nonterminal,
            disjuncts: self.disjuncts.clone()
        }
    }
}

impl fmt::Display for ProductionRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display_str = String::from(&format!("{}->", self.nonterminal));
        
        for disjunct in &self.disjuncts.iter().as_slice()[0..self.disjuncts.len() - 1] {
            display_str.push_str(&format!("{}", disjunct));
            display_str.push('|');
        }
        
        display_str.push_str(&match self.disjuncts.last() {
            Some(value) => format!("{}", value),
            None => String::from("NONE")
        });

        write!(f, "{}", display_str)
    }
}

pub struct ContextFreeLanguage {
    production_rules_map: HashMap<char, ProductionRule>
}

impl ContextFreeLanguage {
    pub fn new(production_rules_map: &HashMap<char, ProductionRule>) -> Self {
        ContextFreeLanguage {
            production_rules_map: production_rules_map.clone()
        }
    }
}

impl clone::Clone for ContextFreeLanguage {
    fn clone(&self) -> Self {
        ContextFreeLanguage {
            production_rules_map: self.production_rules_map.clone()
        }
    }
}

impl fmt::Display for ContextFreeLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut display_string = String::from("");

        for (_, production_rule) in self.production_rules_map.iter() {
            display_string.push_str(&format!("{}\n", production_rule));
        }

        write!(f, "{}", display_string)
    }
}
