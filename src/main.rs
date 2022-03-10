mod ast;
mod eval;
mod lexer;
mod object;
mod parser;
mod token;

use lexer::Lexer;
use parser::Parser;

fn main() {
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        let l = Lexer::new(&input[..]);
        let mut p = Parser::new(l);
        let expr = p.parse_expr_stmt();
        let expr = eval::eval_expr(expr);
        println!("{:?}", expr);
    }
}
