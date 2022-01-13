use super::token::*;
use super::node::*;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    pos: usize
}

impl<'a> Parser<'a> {
    pub fn new(my_tokens: &Vec<Token>) -> Parser {
        Parser {
            tokens: my_tokens,
            pos: 0
        }
    }
    fn current_token(&mut self) -> Token {
        println!("pos: {}", self.pos);
        return self.tokens[self.pos];
    }

    // присваивание переменной
    pub fn assign(&mut self) -> Result<Node, String> {
        let cur = self.current_token();
        if self.current_token().is_var() {
            self.pos += 1;
            if self.current_token() != Token::Equation {
                return Err(format!("Expected equation"));
            } else {
                self.pos += 1;
            }
            let result = self.expr();
            if self.current_token() == Token::Semicolon {
                self.pos += 1;
                return Ok(Node::var_assign(cur, result));
            } else {
                return Err(format!("Missing semicolon!"));
            }
        }
        return Err(format!("Expected variable equation"));
    }
    pub fn expr(&mut self) -> Node {
        let ops = [Token::Plus, Token::Minus];
        let mut result = self.term();
        let cur = self.current_token();
        while ops.contains(&self.current_token()) {
            self.pos += 1;
            result = Node::bin_op(result, cur, self.term());
        }
        return result;
    }
    pub fn term(&mut self) -> Node {
        let ops = [Token::Mul, Token::Div];
        let mut result = self.pow();
        let cur = self.current_token();
        while ops.contains(&self.current_token()) {
            self.pos += 1;
            result = Node::bin_op(result, cur, self.pow());
        }
        return result;
    }
    pub fn pow(&mut self) -> Node {
        let ops = [Token::Pow];
        let mut result = self.factor().unwrap();
        let cur = self.current_token();
        while ops.contains(&self.current_token()) {
            self.pos += 1;
            result = Node::bin_op(result, cur, self.factor().unwrap())
        }
        return result;
    }
    pub fn factor(&mut self) -> Result<Node, String> {
        let cur = self.current_token();
        if self.current_token().is_var() {
            self.pos += 1;
            return Ok(Node::var(cur))
        }
        if [Token::Plus, Token::Minus].contains(&self.current_token()) {
            self.pos += 1;
            return Ok(Node::un_op(cur, self.factor().unwrap()));
        }
        if self.current_token().is_number() {
            self.pos += 1;
            return Ok(Node::number(cur));
        }
        if self.current_token() == Token::Lparen {
            self.pos += 1;
            let result = self.expr();
            if self.current_token() == Token::Rparen {
                self.pos += 1;
                return Ok(result);
            } else {
                return Err(format!("Expected Token::Rparen"));
            }
        }
        return Err(format!("Unexpected token {:?}", self.current_token()));
    }
}