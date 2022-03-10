use crate::{ast, object};

const NEG_ONE: object::Expr = object::Expr::Num(-1);

macro_rules! add {
    ( $( $e:expr ),* ) => ( object::Expr::Add(vec![ $( $e ),* ]) )
}

macro_rules! mul {
    ( $( $e:expr ),* ) => ( object::Expr::Mul(vec![ $( $e ),* ]) )
}

fn pow(base: object::Expr, power: object::Expr) -> object::Expr {
    object::Expr::Pow(Box::new(base), Box::new(power))
}

pub fn eval_expr(e: ast::Expr) -> object::Expr {
    match e {
        ast::Expr::Num(s) => {
            let n: i32 = s.parse().unwrap();
            object::Expr::Num(n)
        }
        ast::Expr::Ident(s) => object::Expr::Ident(s.to_string()),
        ast::Expr::Unary(op, expr) => match op {
            ast::UnOp::Neg => {
                mul!(NEG_ONE, eval_expr(*expr))
            }
        },
        ast::Expr::Binary(op, left, right) => match op {
            ast::BinOp::Add => {
                add!(eval_expr(*left), eval_expr(*right))
            }
            ast::BinOp::Sub => {
                add!(eval_expr(*left), mul!(NEG_ONE, eval_expr(*right)))
            }
            ast::BinOp::Mul | ast::BinOp::ImpliedMul => {
                mul!(eval_expr(*left), eval_expr(*right))
            }
            ast::BinOp::Div => {
                mul!(eval_expr(*left), pow(eval_expr(*right), NEG_ONE))
            }
        },
    }
}
