use super::token::*;
use super::node::*;

use std::collections::LinkedList;

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
        // println!("pos: {}", self.pos);
        return self.tokens[self.pos];
    }
    pub fn prog(&mut self) -> Result<ListElement, String> {
        let nodes_list = self.block()?;
        if self.current_token() != Token::Dot {
            return Err(format!("Expected Token::Dot"));
        }
        return Ok(nodes_list);
    }
    pub fn block(&mut self) -> Result<ListElement, String> {
        if self.current_token() != Token::Begin {
            return Err(format!("Expected Token::Begin"));
        }
        self.pos += 1;
        let nodes = self.iterate()?;
        if self.current_token() != Token::End {
            return Err(format!("Expected Token::End"));
        }
        self.pos += 1;
        return Ok(nodes);
    }
    pub fn iterate(&mut self) -> Result<ListElement, String> {
        let mut nodes = Composite::from([self.assign()?]); // список из одного, первого элемента
        while self.current_token() == Token::Semicolon {
            nodes.push_back(self.assign()?);
            self.pos += 1;
        }
        return Ok(ListElement::Composite(nodes));
    }
    
    pub fn assign(&mut self) -> Result<ListElement, String> {
        let cur = self.current_token();
        if self.current_token() == Token::Begin {
            let res = self.block()?.composite();
            self.pos += 1;
            return Ok(ListElement::Composite(res));
        } else if self.current_token().is_var() {
            self.pos += 1;
            if self.current_token() == Token::Equation {
                self.pos += 1;
                let res = self.expr();
                let node = Node::var_assign(cur, res);
                return Ok(ListElement::Node(node));
            } else {
                let res = self.expr();
                return Ok(ListElement::Node(res));
            }
        } else {
            return Err(format!("Unexpected token {:?}", self.current_token()));
        }
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