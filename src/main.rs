mod interpret;
mod math;

use interpret::{eval, lexer::Lexer, parser::Parser};
use math::standardize;

fn main() {
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        let l = Lexer::new(&input[..]);
        let mut p = Parser::new(l);
        let expr = p.parse_expr_stmt();
        let mut expr = eval::eval_expr(expr);
        expr = standardize::standardize(expr);
        println!("{:?}", expr);
    }
}
