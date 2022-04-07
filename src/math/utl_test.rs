use super::expr::Expr;
use crate::interpreter::{
    ast, environment::Environment, lexer::Lexer, parser::Parser,
};
use evaluator::eval_expr;

pub fn parse_expr(input: &str) -> Expr {
    let mut e = Environment::default();
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let expr = p.parse_stmt();
    if let ast::Stmt::Expr(expr) = expr {
        eval_expr(expr, &mut e)
    } else {
        panic!()
    }
}
