use std::fmt;
use std::clone;

/// Represents a single piece of some disjunct.
/// Note that tokens cannot be used to directly represent languages.
/// Instead, they are created by the lexer and then used by the parser to create languages.
pub enum Token
{
    Symbol(Symbol),
    Disjunction
}

impl clone::Clone for Token
{
    fn clone(&self) -> Self
    {
        match self
        {
            Token::Symbol(symbol) => Token::Symbol(symbol.clone()),
            Token::Disjunction => Token::Disjunction
        }
    }
}

impl fmt::Display for Token
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            Token::Symbol(symbol) => write!(f, "{}", symbol),
            Token::Disjunction => write!(f, "|")
        }
    }
}

/// Represents a nonterminal or terminal symbol.
pub enum Symbol
{
    Nonterminal(char),
    Terminal(char)
}

impl clone::Clone for Symbol
{
    fn clone(&self) -> Self
    {
        match self
        {
            Symbol::Nonterminal(ch) => Symbol::Nonterminal(*ch),
            Symbol::Terminal(ch) => Symbol::Terminal(*ch)
        }
    }
}

impl fmt::Display for Symbol
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            Symbol::Nonterminal(ch) => write!(f, "{}", ch),
            Symbol::Terminal(st) => write!(f, "{}", st)
        }
    }
}

/// Represents a disjunct of a production rule.
/// This is mainly used to store a series of symbols.
pub struct Disjunct
{
    sequences: Vec<Symbol>
}

impl Disjunct
{
    pub fn new(sequences: &[Symbol]) -> Self
    {
        Disjunct        {
            sequences: sequences.to_vec()
        }
    }
}

impl clone::Clone for Disjunct
{
    fn clone(&self) -> Self
    {
        Disjunct        {
            sequences: self.sequences.clone()
        }
    }
}

impl fmt::Display for Disjunct
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let mut display_str = String::from("");

        for symbol in self.sequences.iter()
        {
            display_str.push_str(&format!("{}", symbol));
        }

        write!(f, "{}", display_str)
    }
}

/// Represents a production rule of a nonterminal symbol.
pub struct ProductionRule
{
    /// The nonterminal symbol that points to this production rule.
    nonterminal: char,
    /// The disjuncts that make up the production rule.
    disjuncts: Vec<Disjunct>
}

impl ProductionRule
{
    pub fn new(nonterminal: char, disjuncts: &[Disjunct]) -> Self
    {
        ProductionRule
        {
            nonterminal: nonterminal,
            disjuncts: disjuncts.to_vec()
        }
    }
}

impl clone::Clone for ProductionRule
{
    fn clone(&self) -> Self
    {
        ProductionRule
        {
            nonterminal: self.nonterminal,
            disjuncts: self.disjuncts.clone()
        }
    }
}

impl fmt::Display for ProductionRule
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let mut display_str = String::from("");
        display_str.push_str(&format!("{}->", self.nonterminal));
        
        for disjunct in &self.disjuncts.iter().as_slice()[0..self.disjuncts.len() - 1]
        {
            display_str.push_str(&format!("{}", disjunct));
            display_str.push('|');
        }
        
        display_str.push_str(&match self.disjuncts.last()
        {
            Some(value) => format!("{}", value),
            None => String::from("NONE")
        });

        write!(f, "{}", display_str)
    }
}
