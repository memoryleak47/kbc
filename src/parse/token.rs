#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    LParen, RParen,
    Equals, // =
    Comma, // ,
    Dot, // .
    Str(String),
}

pub fn tokenize(s: &str) -> Vec<Token> {
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
            let tok = Token::Str(s);
            tokens.push(tok);
        }

        if c == '=' {
            tokens.push(Token::Equals);
        } else if c == '.' {
            tokens.push(Token::Dot);
        } else if c == '(' {
            tokens.push(Token::LParen);
        } else if c == ')' {
            tokens.push(Token::RParen);
        } else if c == ',' {
            tokens.push(Token::Comma);
        } else if !c.is_whitespace() {
            panic!();
        }
    }

    if let Some(s) = current.take() {
        let tok = Token::Str(s);
        tokens.push(tok);
    }

    tokens
}

