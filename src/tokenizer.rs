use crate::token::Token;

// make simply typed lambda calculus tokenizer
pub fn tokenizer(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '\\' => tokens.push(Token::Lambda),
            '.' => tokens.push(Token::Dot),
            ' ' => (),
            '\n' => (),
            _ => {
                if c.is_alphabetic() {
                    let mut var = String::new();
                    var.push(c);
                    while let Some(&c) = chars.peek() {
                        if c.is_alphabetic() {
                            var.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Var(var));
                } else {
                    panic!("unexpected character: {}", c);
                }
            }
        }
    }
    tokens
}


// exmple input: \x.x