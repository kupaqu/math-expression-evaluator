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
    if let Ok(tokens) = tokenize("
    BEGIN
        y := 2;
        BEGIN
            a := 3;
            a := a;
            b := 10 + a + 10 * y / 4;
            c := a - b
        END;
        x := 11;
    END.
    ") {
        let mut parser = Parser::new(&tokens);
        let mut tree = parser.prog().unwrap();
        let mut interpreter = Interpreter::new();
        let mut variables = interpreter.interpret(tree);
        println!("{:?}", variables);
    }
}