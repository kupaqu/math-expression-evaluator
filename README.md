# naive-pascal
Naive pascal interpreter on Rust language

## Example:
```
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
```

## Console output:
Ok([{'a': 3.0, 'c': -15.0, 'b': 18.0}, {'y': 2.0, 'x': 11.0}])

## Line coverage:
86.47 %
