use crate::*;

#[derive(Debug)]
pub enum Token {
    LParen, RParen,
    Equals, // =
    Comma, // ,
    Dot, // .
    Var(String),
    Fun(String),
}

pub fn parse_file(s: &str) -> Vec<Equation> {
    let mut tokens = tokenize(s);
    todo!()
}

fn tokenize(s: &str) -> Option<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut current: Option<String> = None;
    for c in s.chars() {
        if c.is_alphanumeric() {
            if let Some(s) = &mut current {
                s.push(c);
            } else {
                current = Some(c.to_string());
            }
            continue;
        }

        if let Some(s) = current.take() {
            let ch = s.chars().next()?;
            let is_var = ('A'..='Z').contains(&ch);

            let tok = if is_var { Token::Var(s) } else { Token::Fun(s) };
            tokens.push(tok);
        }

        if c == '=' {
            tokens.push(Token::Equals);
        } else if c == '(' {
            tokens.push(Token::LParen);
        } else if c == ')' {
            tokens.push(Token::RParen);
        } else if c == ',' {
            tokens.push(Token::Comma);
        } else if !c.is_whitespace() {
            return None;
        }
    }

    if let Some(s) = current.take() {
        let ch = s.chars().next()?;
        let is_var = ('A'..='Z').contains(&ch);

        let tok = if is_var { Token::Var(s) } else { Token::Fun(s) };
        tokens.push(tok);
    }

    Some(tokens)
}
