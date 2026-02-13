mod token;
use token::*;

mod assemble;
use assemble::*;

mod flatten;
use flatten::*;

use crate::*;

pub fn parse_file(s: &str) -> (Vec<Equation>, Vec<Goal>) {
    let tokens: Vec<Token> = tokenize(s);

    let toks: &mut &[Token] = &mut &*tokens;

    let mut eqs = Vec::new();
    while toks.len() > 0 {
        let eq = assemble_eq(toks);
        eqs.push(eq);
    }

    let (eqs, goals, map) = flatten(eqs);

    init_symbol_map(map);

    (eqs, goals)
}


