use super::token::*;

pub fn tokenize(input_string: &str) -> Result<Vec<Token>, String> {
    let mut res = Vec::new(); // конечный вектор токенизированной строки
    let mut chars = input_string.chars().peekable();
    /* когда chars закончится будет возвращаться None и цикл завершится */
    while let Some(&c) = chars.peek() {
        match c {
            'A'..='Z' => {
                let mut word: String = String::new();
                while let Some(ch) = chars.next_if(|&x| x.is_alphabetic()) {
                    word.push(ch);
                }   
                if String::from("BEGIN") == word {
                    res.push(Token::Begin);
                }
                else if String::from("END") == word {
                    res.push(Token::End);
                }
                else {
                    return Err(format!("Unknown word {}", word));
                }
            }
            'a'..='z' => {
                res.push(Token::Variable(c));
                chars.next();
            }
            ':' => {
                chars.next();
                if let Some(&i) = chars.peek() {
                    if i == '=' {
                        res.push(Token::Equation);
                        chars.next();
                    }
                    else {
                        return Err(format!("Expected equation!"));
                    }
                }
            }
            '.' => {
                res.push(Token::Dot);
                chars.next();
            }
            ';' => {
                res.push(Token::Semicolon);
                chars.next();
            }
            '0'..='9' => {
                let mut n: f64 = 0.;
                while let Some(&i) = chars.peek() {
                    if i.is_digit(10) {
                        n = n * 10. + i.to_digit(10).unwrap() as f64;
                        chars.next();
                    }
                    else {
                        break;
                    }
                }
                res.push(Token::Number(n));
            }
            '+' => {
                res.push(Token::Plus);
                chars.next();
            }
            '-' => {
                res.push(Token::Minus);
                chars.next();
            }
            '*' => {
                res.push(Token::Mul);
                chars.next();
            }
            '/' => {
                res.push(Token::Div);
                chars.next();
            }
            '^' => {
                res.push(Token::Pow);
                chars.next();
            }
            '(' => {
                res.push(Token::Lparen);
                chars.next();
            }
            ')' => {
                res.push(Token::Rparen);
                chars.next();
            }
            c if c.is_whitespace() => {
                chars.next();
            }
            _ => {
                return Err(format!("Bad token {}", c));
            }
        }
    }
    res.push(Token::Eos);
    // println!("{:?}", res);
    return Ok(res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let correct_tokens = Ok(vec![Token::Begin, Token::Variable('y'), Token::Equation, Token::Number(4.0), Token::Mul,
        Token::Variable('x'), Token::Plus, Token::Number(10.0), Token::Semicolon, Token::End, Token::Dot, Token::Eos]);
        let tokens = tokenize("BEGIN y := 4*x+10; END.");
        assert_eq!(correct_tokens, tokens);
    }
}