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
        self.tokens[self.pos]
    }
    pub fn expr(&mut self) -> Node {
        let ops = [Token::Plus, Token::Minus];
        let result = self.term();
        let cur = self.current_token();
        while ops.contains(&cur) {
            self.pos += 1;
            return Node::bin_op(result, cur, self.term());
        }
        return result;
    }
    pub fn term(&mut self) -> Node {
        let ops = [Token::Mul, Token::Div];
        let result = self.factor().unwrap();
        let cur = self.current_token();
        while ops.contains(&cur) {
            self.pos += 1;
            return Node::bin_op(result, cur, self.factor().unwrap());
        }
        return result;
    }
    pub fn factor(&mut self) -> Result<Node, String> {
        let cur = self.current_token();
        if cur.is_number() {
            self.pos += 1;
            return Ok(Node::number(cur));
        } else {
            return Err(format!("Unexpected token {:?}", self.current_token()));
        }
    }
}