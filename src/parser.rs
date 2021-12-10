use super::token::*;
use super::node::*;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize
}

impl Parser {

    fn new (my_tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: my_tokens,
            pos: 0
        }
    }

    fn check_token(mut self, token: Token) -> Result<(), String> {
        if self.tokens[self.pos] == token && self.pos > self.tokens.len() {
            self.pos += 1;
            return Ok(());
        }
        return Err(format!("Invalid token order"));
    }

    fn expr (mut self) -> Node {
        let ops = [Token::Plus, Token::Minus];
        let mut result = self.term();
        while ops.contains(&self.tokens[self.pos]) {
            let token = self.tokens[self.pos];
            if token == Token::Plus {
                self.check_token(Token::Plus);
            } else {
                self.check_token(Token::Minus);
            }
            result = Node::bin_op(result, token, self.term());
        }
        return result;
    }

    fn term (mut self) -> Node {
        let ops = [Token::Mul, Token::Div];
        let mut result = self.pow();
        while ops.contains(&self.tokens[self.pos]) {
            let token = self.tokens[self.pos];
            if token == Token::Mul {
                self.check_token(Token::Mul);
            } else {
                self.check_token(Token::Div);
            }
            result = Node::bin_op(result, token, self.pow());
        }
        return result;
    }

    fn pow (mut self) -> Node {
        let ops = [Token::Pow];
        let mut result = self.factor().unwrap();
        while ops.contains(&self.tokens[self.pos]) {
            let token = self.tokens[self.pos];
            if token == Token::Pow {
                self.check_token(Token::Pow);
            }
            result = Node::bin_op(result, token, self.factor().unwrap());
        }
        return result;
    }

    fn factor (mut self) -> Result<Node, String> {
        let token = self.tokens[self.pos];
        if token == Token::Plus {
            self.check_token(Token::Plus);
            return Ok(Node::un_op(token, self.factor()?));
        } else {
            return Err(format!("Invalid token order!"));
        }
    }
}