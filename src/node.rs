use super::token::*;
use std::collections::LinkedList;

#[derive(Debug)]
pub struct Node {
    /* использую вектор для избежания использования std::boxed::Box
    и сопутствующих сложностей его использования */
    pub children: Vec<Node>,
    pub token: Token
}

// https://stackoverflow.com/questions/34953711/unwrap-inner-type-when-enum-variant-is-known
#[derive(Debug)]
pub enum ListElement {
    Composite(Composite),
    Node(Node)
}

pub type Composite = LinkedList<ListElement>;

impl ListElement {
    pub fn composite(self) -> Composite {
        if let ListElement::Composite(item) = self { item } else { panic!("Not a composite") }
    }
    pub fn node(self) -> Node {
        if let ListElement::Node(item) = self { item } else { panic!("Not a node") }
    }
}

impl Node {
    /* конструкторы */
    pub fn var(my_token: Token) -> Node {
        println!("var {:?}", my_token);
        Node {
            children: Vec::new(),
            token: my_token
        }
    }
    pub fn var_assign(my_token: Token, my_child: Node) -> Node {
        println!("var assign {:?}", my_token);
        Node::un_op(my_token, my_child)
    }

    pub fn bin_op(left: Node, my_token: Token, right: Node) -> Node {
        println!("binop {:?}", my_token);
        Node {
            children: vec![left, right],
            token: my_token
        }
    }
    pub fn un_op(my_token: Token, my_child: Node) -> Node {
        println!("unop {:?}", my_token);
        Node {
            children: vec![my_child],
            token: my_token
        }
    }
    pub fn number(my_token: Token) -> Node {
        println!("number {:?}", my_token);
        Node {
            children: Vec::new(),
            token: my_token
        }
    }
}