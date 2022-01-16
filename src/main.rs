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
    if let Ok(tokens) = tokenize("
    BEGIN
        y := 2;
        BEGIN
            a := 3;
            a := a;
            b := 10 + a + 10 * y / 4;
            c := a - b;
        END;
        x := 11;
    END.") {
        let mut parser = Parser::new(&tokens);
        let mut tree = parser.prog().unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(tree);
        println!("{:?}", interpreter.variables);

    }
}