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
    println!("Hello, world!");
    let tokens = tokenize("1+2+3+4");

    if let Ok(tree) = tokenize("1+2*3") {
        let mut parser = Parser::new(&tree);
        println!("{:?}", parser.expr());
    }

    // let mut parser = Parser::new(&tokens.unwrap());
}
