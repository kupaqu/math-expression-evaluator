mod token;
use token::*;

mod node;
use node::*;

mod lexer;
use lexer::*;

fn main() {
    println!("Hello, world!");
    println!("{:?}", tokenize("1 + 2 + 34 + 5 + 56"));
}
