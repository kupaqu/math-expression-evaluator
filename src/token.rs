#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(u32),
    Plus,
    Minus,
    Mul,
    Div,
    Pow,
    Lparen,
    Rparen,
    Eos
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_token() {
        if let Token::Number(num) = Token::Number(100) {
            assert_eq!(num, 100);
        }
    }
}