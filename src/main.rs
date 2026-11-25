#[path = "./syntax.rs"]
mod syntax;
use syntax::*;

fn main()
{
    let nonterminal = Symbol::Nonterminal('S');
    let terminal = Symbol::Terminal('f');
    
    let ab_disjunct = Disjunct::new(
    &[
        Symbol::Terminal('a'),
        Symbol::Terminal('b'),
        Symbol::Nonterminal('S'),
        Symbol::Terminal('b'),
        Symbol::Terminal('a')
    ]);
    
    let foobar_disjunct = Disjunct::new(&[terminal.clone()]);
    let production_rule = ProductionRule::new(
        'S',
        &[
            ab_disjunct.clone(),
            foobar_disjunct.clone()
        ]
    );

    println!("Nonterminal: {}", nonterminal);
    println!("Terminal: {}", terminal);
    println!("ab Disjunct: {}", ab_disjunct);
    println!("foobar Disjunct: {}", foobar_disjunct);
    println!("Production Rule: {}", production_rule);
}
