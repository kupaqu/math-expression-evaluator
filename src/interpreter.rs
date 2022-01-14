use super::token::*;
use super::node::*;

use std::collections::LinkedList;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Interpreter {
    variables: LinkedList<HashMap<char, f64>>
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            variables: LinkedList::new()
        }
    }
}

pub fn interpret(tree: &Node) -> f64 {
    return visit(tree);
}

fn child_number(node: &Node) -> usize {
    return node.children.len();
}

fn visit(node: &Node) -> f64 {
    let c = child_number(&node);
    match c {
        0 => { return visit_number(&node); },
        1 => { return visit_unary(&node); },
        2 => { return visit_binary(&node).unwrap(); },
        _ => { return 0. }
    }
}

fn visit_number(node: &Node) -> f64 {
    return node.token.get_num().unwrap();
}

fn visit_unary(node: &Node) -> f64 {
    return visit(&node.children[0]);
}

fn visit_binary(node: &Node) -> Result<f64, String> {
    if node.token == Token::Plus {
        return Ok(visit(&node.children[0]) + visit(&node.children[1]));
    } else if node.token == Token::Minus {
        return Ok(visit(&node.children[0]) - visit(&node.children[1]));
    } else if node.token == Token::Mul {
        return Ok(visit(&node.children[0]) * visit(&node.children[1]));
    } else if node.token == Token::Div {
        if let denom = visit(&node.children[1]) == 0. {
            return Ok(f64::INFINITY);
        }
        return Ok(visit(&node.children[0]) / visit(&node.children[1]));
    } else if node.token == Token::Pow {
        return Ok(visit(&node.children[0]).powf(visit(&node.children[1])));
    } else {
        return Err(format!("Unexpected node token type!"));
    }
}