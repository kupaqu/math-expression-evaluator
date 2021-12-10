use super::token::*;
use super::node::*;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize
}

impl Parser {

    pub fn new (my_tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: my_tokens,
            pos: 0
        }
    }

}

pub fn parse(mut parser: Parser) -> Node {
    expr(parser)
}

pub fn expr(mut parser: Parser) -> Node {
    let ops = [Token::Plus, Token::Minus];
    let res = term(parser);
    let mut current_token = parser.tokens[parser.pos].clone();
    while ops.contains(&current_token) {
        parser.pos += 1;
        res = Node::bin_op(res, current_token, term(parser))
    }
    return res;
}

pub fn term(mut parser: Parser) -> Node {
    let ops = [Token::Mul, Token::Div];
    let res = pow(parser);
    let mut current_token = parser.tokens[parser.pos].clone();
    while ops.contains(&current_token) {
        parser.pos += 1;
        res = Node::bin_op(res, current_token, pow(parser))
    }
    return res;
}

pub fn pow(mut parser: Parser) -> Node {
    let ops = [Token::Pow];
    let res = factor(parser);
    let mut current_token = parser.tokens[parser.pos].clone();
    while ops.contains(&current_token) {
        parser.pos += 1;
        res = Node::bin_op(res, current_token, factor(parser))
    }
    return res;
}

pub fn factor(mut parser: Parser) -> Node {
    let mut current_token = parser.tokens[parser.pos].clone();

    if vec![Token::Plus, Token::Minus].contains(&current_token) {
        parser.pos += 1;
        return Node::un_op(current_token, factor(parser));
    }
    if vec![Token::Lparen].contains(&current_token) {
        parser.pos += 1;
        let res = expr(parser);
        current_token = parser.tokens[parser.pos].clone();
        if vec![Token::Rparen].contains(&current_token) {
            parser.pos += 1;
        }
        return Node::un_op(current_token, factor(parser));
    }
    if let Token::Number(n) = current_token {
        parser.pos += 1;
        return Node::new(current_token);
    }
    return Node::new(Token::Eos);
}