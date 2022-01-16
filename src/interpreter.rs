use super::node::*;
use super::token::*;

use std::collections::LinkedList;
use std::collections::HashMap;

/* модификация цепочки таблиц (A. Aхо - "Компиляторы", глава 2.7), идущая сверху вниз (занимает много памяти?) */
#[derive(Clone)]
pub struct Visibility {
    seen: HashMap<char, f64>,
    mine: HashMap<char, f64>
}

impl Visibility {
    pub fn new() -> Visibility {
        Visibility {
            seen: HashMap::new(),
            mine: HashMap::new()
        }
    }
}

pub struct Interpreter{ 
    pub variables: LinkedList<HashMap<char, f64>>
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            variables: LinkedList::new()
        }
    }

    pub fn interpret(&mut self, root: ListElement) {
        let mut my_visibility = Visibility::new();
        self.visit(root, &mut my_visibility);
        return;
    }

    pub fn visit(&mut self, tree_node: ListElement, visibility: &mut Visibility) {
        match tree_node {
            ListElement::Node(node) => {
                let char = node.token.get_char().unwrap();
                if visibility.mine.contains_key(&char) {
                    visibility.mine.remove(&char);
                }
                let res = self.visit_node(&node.children[0], visibility);
                visibility.mine.insert(char, res);
                visibility.seen.insert(char, res);
                return;
            },
            ListElement::Composite(block) => {
                let mut my_visibility = Visibility::new();
                my_visibility.seen = visibility.seen.clone();
                for component in block {
                    self.visit(component, &mut my_visibility);
                }
                self.variables.push_back(my_visibility.mine);
                return;
            }
            _ => {
                return;
            }
        }
    }
    pub fn child_number(&mut self, node: &Node) -> usize {
        return node.children.len();
    }
    pub fn visit_node(&mut self, node: &Node, visibility: &mut Visibility) -> f64 {
        let c = self.child_number(&node);
        match c {
            0 => { return self.visit_number(&node, visibility).unwrap(); },
            1 => { return self.visit_unary(&node, visibility); },
            2 => { return self.visit_binary(&node, visibility).unwrap(); }
            _ => { return 0. }
        }
    }
    pub fn visit_number(&mut self, node: &Node, visibility: &mut Visibility) -> Result<f64, String> {
        if node.token.is_var() {
            let c = node.token.get_char().unwrap();
            if visibility.seen.contains_key(&c) {
                return Ok(visibility.seen[&c]);
            }
            return Err(format!("Variable is not declared!"));
        } else {
            return Ok(node.token.get_num().unwrap());
        }
    }
    pub fn visit_unary(&mut self, node: &Node, visibility: &mut Visibility) -> f64 {
        return self.visit_node(node, visibility);
    }
    pub fn visit_binary(&mut self, node: &Node, visibility: &mut Visibility) -> Result<f64, String> {
        if node.token == Token::Plus {
            return Ok(self.visit_node(&node.children[0], visibility) + self.visit_node(&node.children[1], visibility));
        } else if node.token == Token::Minus {
            return Ok(self.visit_node(&node.children[0], visibility) - self.visit_node(&node.children[1], visibility));
        } else if node.token == Token::Mul {
            return Ok(self.visit_node(&node.children[0], visibility) * self.visit_node(&node.children[1], visibility));
        } else if node.token == Token::Div {
            // if let denom = self.visit_node(&node.children[1], visibility) == 0. {
            //     return Ok(f64::INFINITY);
            // }
            return Ok(self.visit_node(&node.children[0], visibility) / self.visit_node(&node.children[1], visibility));
        } else if node.token == Token::Pow {
            return Ok(self.visit_node(&node.children[0], visibility).powf(self.visit_node(&node.children[1], visibility)));
        } else {
            return Err(format!("Unexpected node token type!"));
        }
    }
}