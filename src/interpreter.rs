use super::token::*;
use super::node::*;

use std::collections::LinkedList;
use std::collections::HashMap;

pub type Variables = LinkedList<HashMap<char, f64>>;

#[derive(Debug)]
pub struct Interpreter {
    variables: Variables
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            variables: LinkedList::new()
        }
    }
    pub fn interpret(&mut self, tree: ListElement) -> Result<Variables, String> {
        let global_hash = HashMap::new();
        self.visit(tree, global_hash);
        return Ok(self.variables);
    }
    fn visit(&mut self, tree_node: ListElement, vars: HashMap<char, f64>) {
        match tree_node {
            ListElement::Node(node) => {
                if node.token.is_var() {
                    let varchar = node.token.get_char()?;
                    let val = self.visit_node(node)?;
                    vars.insert(node.token.get_char(), self.visit_node(node));
                    return;
                }
            },
            ListElement::Composite(block) => {
                
            },
        }
    }
    fn visit_node(&mut self, node: Node) -> Result<f64, String> {
        Ok(0.0)
    }
}