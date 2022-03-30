mod interpreter;
mod math;

use interpreter::{
    environment::Environment, evaluator, lexer::Lexer, parser::Parser,
};
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut env = Environment::new();
    loop {
        print!(">> ");
        std::io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if input == "" {
            println!();
            break Ok(());
        }
        let l = Lexer::new(&input);
        let mut p = Parser::new(l);
        let stmt = p.parse_stmt();
        let out = evaluator::eval_stmt(stmt, &mut env);
        if out != "" {
            println!("{}", out);
        }
    }
}
