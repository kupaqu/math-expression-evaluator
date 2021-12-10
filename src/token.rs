#[derive(Debug, PartialEq, Clone, Copy)]
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

impl Token {
    // pub fn get_number(&self) -> u32 {
    //     let Token::Number(n) = self;
    //     return Ok(*n);
    // }
    pub fn is_number(&self) -> bool {
        match self {
            Token::Number(_) => true,
            _ => false
        }
    }
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