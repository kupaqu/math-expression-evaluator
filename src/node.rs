use super::token::*;

pub struct Node {
    /* использую вектор для избежания использования std::boxed::Box
       и сопутствующих сложностей его использования */
    pub children: Vec<Node>,
    pub token: Token
}

impl Node {
    /* конструктор */
    pub fn new(my_token: Token) -> Node {
        Node {
            children: Vec::new(),
            token: my_token
        }
    }
}