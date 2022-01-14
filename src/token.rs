#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Mul,
    Div,
    Pow,
    Lparen,
    Rparen,
    Begin,
    End,
    Variable(char),
    Equation,
    Semicolon,
    Dot,
    Eos
}

impl Token {
    /* вспомогательные функции для работы с числами */
    pub fn is_number(&self) -> bool {
        match self {
            Token::Number(_) => true,
            _ => false
        }
    }
    pub fn get_num(&self) -> Result<f64, String> {
        if let Token::Number(f) = self {
            return Ok(*f);
        }
        return Err(format!("Calling get_num on non-numeric token"));
    }

    /* вспомогательные функции для работы с идентификаторами переменной */
    pub fn is_var(&self) -> bool {
        match self {
            Token::Variable(_) => true,
            _ => false
        }
    }
    pub fn get_char(&self) -> Result<char, String> {
        if let Token::Variable(c) = self {
            return Ok(*c);
        }
        return Err(format!("Calling get_char on non-character token"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_token() {
        if let Token::Number(num) = Token::Number(100.) {
            assert_eq!(num, 100.);
        }
    }
}