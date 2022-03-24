use super::ast;
use crate::math::expr::{Expr, NEG_ONE};

pub fn eval_expr(e: ast::Expr) -> Expr {
    match e {
        ast::Expr::Num(s) => {
            let n: i32 = s.parse().unwrap();
            Expr::int(n)
        }
        ast::Expr::Ident(s) => Expr::Sym(s.to_string()),
        ast::Expr::Unary(op, expr) => match op {
            ast::UnOp::Neg => NEG_ONE * eval_expr(*expr),
        },
        ast::Expr::Binary(op, left, right) => match op {
            ast::BinOp::Add => eval_expr(*left) + eval_expr(*right),
            ast::BinOp::Sub => eval_expr(*left) - eval_expr(*right),
            ast::BinOp::Mul | ast::BinOp::ImpliedMul => {
                eval_expr(*left) * eval_expr(*right)
            }
            ast::BinOp::Div => eval_expr(*left) / eval_expr(*right),
        },
    }
}
