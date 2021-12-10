use super::token::*;
use super::node::*;

pub fn parse(tokens: &Vec<Token>, pos: usize) -> Result<Node, String> {
    expr(tokens, 0)
}

pub fn expr(tokens: &Vec<Token>, pos: usize) -> Result<Node, String> {
    if pos > tokens.len() {
        return Err(format!("End of sequence"));
    }
    let ops = [Token::Plus, Token::Minus];
    let mut res = term(tokens, pos).unwrap();
    let current_token = tokens[pos];
    while ops.contains(&current_token) {
        res = Node::bin_op(res, current_token, term(tokens, pos+1).unwrap())
    }
    return Ok(res);
}

pub fn term(tokens: &Vec<Token>, pos: usize) -> Result<Node, String> {
    if pos > tokens.len() {
        return Err(format!("End of sequence"));
    }
    let ops = [Token::Mul, Token::Div];
    let mut res = pow(tokens, pos).unwrap();
    let current_token = tokens[pos];
    while ops.contains(&current_token) {
        res = Node::bin_op(res, current_token, pow(tokens, pos+1).unwrap())
    }
    return Ok(res);
}

pub fn pow(tokens: &Vec<Token>, pos: usize) -> Result<Node, String> {
    if pos > tokens.len() {
        return Err(format!("End of sequence"));
    }
    let ops = [Token::Pow];
    let mut res = factor(tokens, pos).unwrap();
    let current_token = tokens[pos];
    while ops.contains(&current_token) {
        res = Node::bin_op(res, current_token, factor(tokens, pos+1).unwrap())
    }
    return Ok(res);
}

pub fn factor(tokens: &Vec<Token>, pos: usize) -> Result<Node, String> {
    let current_token = tokens[pos];
    if [Token::Plus, Token::Minus].contains(&current_token) {
        return Ok(Node::un_op(current_token, factor(tokens, pos+1).unwrap()));
    } else if current_token.is_number() {
        return Ok(Node::number(current_token))
    } else if current_token == Token::Lparen {
        let res = expr(tokens, pos+1);
        // TODO: добавить правую скобку

        return res;
    }
    Err(format!("Invalid token sequence"))
}