use super::token::*;

pub fn tokenize(input_string: &str) -> Result<Vec<Token>, String> {
    let mut res = Vec::new(); // конечный вектор токенизированной строки
    let mut chars = input_string.chars().peekable();
    /* когда chars закончится будет возвращаться None и цикл завершится */
    while let Some(&c) = chars.peek() {
        match c {
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
            ' ' => {
                chars.next();
            }
            _ => {
                return Err(format!("Bad token {}", c));
            }
        }
    }
    res.push(Token::Eos);
    return Ok(res);
}