use super::node::*;

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
    variables: LinkedList<HashMap<char, f64>>
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            variables: LinkedList::new()
        }
    }

    pub fn interpret(&mut self, root: ListElement) {
        let my_visibility = Visibility::new();
        self.visit(root, my_visibility);
        return;
    }

    pub fn visit(&mut self, tree_node: ListElement, mut visibility: Visibility) {
        match tree_node {
            ListElement::Node(node) => {
                let char = node.token.get_char().unwrap();
                if visibility.mine.contains_key(&char) {
                    visibility.mine.remove(&char);
                }
                let res = 0.0;
                visibility.mine.insert(char, res);
                visibility.seen.insert(char, res);
                return;
            }
            ListElement::Composite(block) => {
                let mut my_visibility = Visibility::new();
                my_visibility.seen = visibility.seen.clone();
                for component in block {
                    self.visit(component, my_visibility);
                }
                self.variables.push_back(my_visibility.mine.clone());
                return;
            }
            _ => {
                return;
            }
        }
    }

    // pub fn visit_node(&mut self, node: Node) -> f64 {
    //     return 0.0;
    // }

}