use super::{ast, environment::Environment};
use crate::math::expr::{Expr, NEG_ONE};

#[cfg(test)]
mod test;

pub fn eval_stmt(stmt: ast::Stmt, env: &mut Environment) -> String {
    match stmt {
        ast::Stmt::Empty => String::new(),
        ast::Stmt::Expr(e) => {
            let e = eval_expr(e, env);
            format!("{:?}", e)
        }
        ast::Stmt::Let(l, r) => {
            let r = eval_expr(r, env);
            let ret = format!("\\let {:?} = {:?}", l, r);
            match l {
                ast::Expr::Ident(s) => env.set(s, r),
                // ast::Expr::Binary(ast::BinOp::ImpliedMul, l, r) => {
                //     if let (ast::Expr::Ident(f), ast::Expr::Ident(x)) = (*l, *r)
                //     {
                //         todo!()
                //     }
                // }
                _ => {}
            }
            ret
        }
    }
}

pub fn eval_expr(e: ast::Expr, env: &Environment) -> Expr {
    match e {
        ast::Expr::Num(s) => {
            let n: i32 = s.parse().unwrap();
            Expr::int(n)
        }
        ast::Expr::Ident(s) => env.get(s),
        ast::Expr::Unary(op, expr) => match op {
            ast::UnOp::Neg => NEG_ONE * eval_expr(*expr, env),
        },
        ast::Expr::Binary(op, left, right) => match op {
            ast::BinOp::Add => eval_expr(*left, env) + eval_expr(*right, env),
            ast::BinOp::Sub => eval_expr(*left, env) - eval_expr(*right, env),
            ast::BinOp::Mul => eval_expr(*left, env) * eval_expr(*right, env),
            ast::BinOp::ImpliedMul => {
                let l = eval_expr(*left, env);
                let r = eval_expr(*right, env);
                match l {
                    f @ Expr::Cmd(..) => Expr::call(f, r),
                    _ => l * r,
                }
            }
            ast::BinOp::Div => eval_expr(*left, env) / eval_expr(*right, env),
            ast::BinOp::Pow => {
                Expr::pow(eval_expr(*left, env), eval_expr(*right, env))
            }
        },
        ast::Expr::List(v) => {
            let v = v.into_iter().map(|e| eval_expr(e, env)).collect();
            Expr::Vec(v)
        }
    }
}
