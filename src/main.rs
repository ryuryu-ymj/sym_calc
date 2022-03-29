mod interpreter;
mod math;

use interpreter::{
    environment::Environment, evaluator, lexer::Lexer, parser::Parser,
};

fn main() {
    let mut env = Environment::new();
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        let l = Lexer::new(&input);
        let mut p = Parser::new(l);
        let stmt = p.parse_stmt();
        println!("{}", evaluator::eval_stmt(stmt, &mut env));
    }
}
