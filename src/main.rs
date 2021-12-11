mod token;
use token::*;

mod node;
use node::*;

mod lexer;
use lexer::*;

mod parser;
use parser::*;

mod interpreter;
use interpreter::*;

fn main() {
    if let Ok(tokens) = tokenize("1*2^3+4+5") {
        println!("{:?}", tokens);
        let mut parser = Parser::new(&tokens);
        println!("{:?}", parser.expr());
    }

}
