#[path = "./syntax.rs"]
mod syntax;
#[path = "./lexer.rs"]
mod lexer;
#[path = "./parser.rs"]
mod parser;
use syntax::*;

fn main() {
    let nonterminal = Symbol::Nonterminal('S');
    let terminal = Symbol::Terminal('f');
    
    let ab_disjunct = Disjunct::Sequences(
    vec![
        Symbol::Terminal('a'),
        Symbol::Terminal('b'),
        Symbol::Nonterminal('S'),
        Symbol::Terminal('b'),
        Symbol::Terminal('a')
    ]);
    
    let foobar_disjunct = Disjunct::Sequences(vec![terminal.clone()]);
    let production_rule = ProductionRule::new(
        'S',
        &vec![
            ab_disjunct.clone(),
            foobar_disjunct.clone()
        ]
    );

    println!("Nonterminal: {}", nonterminal);
    println!("Terminal: {}", terminal);
    println!("ab Disjunct: {}", ab_disjunct);
    println!("foobar Disjunct: {}", foobar_disjunct);
    println!("Production Rule: {}", production_rule);

    let mut tokens = lexer::lex("abSba | A").expect("this lexer example should be valid");
    let token_strings: Vec<String> = tokens.iter().map(|token| token.to_string()).collect();
    println!("Lexer tokens: {}", token_strings.join(" "));
    tokens.insert(0, Token::Symbol(Symbol::Nonterminal('S')));

    let mut tokens2 = lexer::lex("aAa | a").expect("this lexer example should be valid");
    let token_strings2: Vec<String> = tokens2.iter().map(|token| token.to_string()).collect();
    println!("Lexer tokens 2: {}", token_strings2.join(" "));
    tokens2.insert(0, Token::Symbol(Symbol::Nonterminal('A')));

    tokens.push(Token::NewLine);
    tokens.append(&mut tokens2);
    match parser::parse(&tokens) {
        Ok(langauge) => println!("{}", langauge),
        Err(error) => println!("{}", error)
    };
}
