use crate::{token::Token, term::Term};

pub fn parse(tokens: &[Token]) -> Result<Term, String> {
    let (term, _) = parse_term(tokens, 0)?;
    Ok(term)
}

fn parse_term(tokens: &[Token], mut pos: usize) -> Result<(Term, usize), String> {
    match tokens.get(pos) {
        Some(Token::Lambda) => {
            pos += 1; // Skip Lambda
            match tokens.get(pos) {
                Some(Token::Var(var)) => {
                    pos += 1; // Skip variable
                    if let Some(Token::Dot) = tokens.get(pos) {
                        pos += 1; // Skip Dot
                        let (body, new_pos) = parse_term(tokens, pos)?;
                        pos = new_pos;
                        Ok((Term::Abs(var.clone(), Box::new(body)), pos))
                    } else {
                        Err("Expected '.' after variable in lambda abstraction".to_string())
                    }
                },
                _ => Err("Expected variable after lambda".to_string()),
            }
        },
        Some(Token::Var(var)) => {
            let mut term = Term::Var(var.clone());
            pos += 1; // Skip variable
            while let Some(Token::Var(v)) = tokens.get(pos) {
                term = Term::App(Box::new(term), Box::new(Term::Var(v.clone())));
                pos += 1; // Skip to next token
            }
            Ok((term, pos))
        },
        Some(Token::LParen) => {
            pos += 1; // Skip LParen
            let (term, new_pos) = parse_term(tokens, pos)?;
            pos = new_pos;
            println!("pos: {}", pos);
            println!("tokens: {}", tokens.get(pos).unwrap());
            match tokens.get(pos) {
                Some(Token::RParen) => {
                    pos += 1; // Skip RParen
                    Ok((term, pos))
                },
                Some(Token::LParen) => {
                    let (term2, new_pos) = parse_term(tokens, pos)?;
                    pos = new_pos;
                    match tokens.get(pos) {
                        Some(Token::RParen) => {
                            pos += 1; // Skip RParen
                            Ok((Term::App(Box::new(term), Box::new(term2)), pos))
                        },
                        _ => Err("Expected ')'".to_string()),
                    }
                },
                None => Err("Expected ')'".to_string()),
                
            }
          
        },
        _ => Err("Unexpected token".to_string()),
    }
}
