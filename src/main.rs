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
    // if let Ok(tokens) = tokenize("1*2^3+4+5") {
    //     println!("{:?}", tokens);
    //     let mut parser = Parser::new(&tokens);
    //     let tree = parser.expr();
    //     println!("{}", interpret(&tree));
    // }
    // if let Ok(tokens) = tokenize("
    //     BEGIN
    //         BEGIN
    //             a := 1;
    //         END;
    //         BEGIN
    //             a := a + 1 + b + 2;
    //             c := 1 + 2 + 3 + d;
    //         END;
    //     END."
    // )
    if let Ok(tokens) = tokenize("BEGIN a := 1 + b END.") {
        println!("{:?}", tokens);
        let mut parser = Parser::new(&tokens);
        let mut tree = parser.prog().unwrap();
        println!("{:?}", tree);
        let mut interpreter = Interpreter::new();
        interpreter.interpret(tree);
        println!("{:?}", interpreter.variables);

    }
}