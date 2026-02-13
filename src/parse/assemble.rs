use crate::parse::*;

pub struct AST {
    pub f: String,
    pub children: Vec<AST>,
}

fn assemble_ast(toks: &mut &[Token]) -> AST {
    let tok = take_tok(toks).clone();
    let Token::Str(f) = tok else { panic!() };
    let f = f.to_string();
    let Some(Token::LParen) = toks.get(0) else {
        return AST { f, children: Vec::new() };
    };
    take_tok(toks); // (

    let mut children = Vec::new();
    loop {
        let t = assemble_ast(toks);
        children.push(t);

        let Some(Token::Comma) = toks.get(0) else { break; };
        take_tok(toks); // ,
    }
    assert_eq!(take_tok(toks), Token::RParen);

    AST { f, children }
}

fn assemble_token(t: Token, toks: &mut &[Token]) {
    let t2 = take_tok(toks);
    assert_eq!(t, t2);
}

fn assemble_str(s: &str, toks: &mut &[Token]) {
    let Token::Str(ref s2) = take_tok(toks) else { panic!() };
    assert_eq!(s, s2);
}

pub fn assemble_eq(toks: &mut &[Token]) -> (AST, /*true = '='*/bool, AST) {
    // cnf(a,axiom,
    assemble_str("cnf", toks);
    assemble_token(Token::LParen, toks);
    assemble_str("a", toks);
    assemble_token(Token::Comma, toks);
    assemble_str("axiom", toks);
    assemble_token(Token::Comma, toks);

    let lhs = assemble_ast(toks);

    let mut op = true;
    if let Token::Excl = toks[0] {
        op = false;
        assemble_token(Token::Excl, toks);
    }
    assemble_token(Token::Equals, toks);
    let rhs = assemble_ast(toks);
    assemble_token(Token::RParen, toks);
    assemble_token(Token::Dot, toks);

    (lhs, op, rhs)
}

fn take_tok(toks: &mut &[Token]) -> Token {
    let out = toks[0].clone();
    *toks = &toks[1..];
    out
}
