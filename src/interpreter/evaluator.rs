use super::{ast, environment::Environment};
use crate::math::expr::{Expr, NEG_ONE};

#[cfg(test)]
mod test;

pub fn eval_stmt(stmt: ast::Stmt, env: &mut Environment) -> String {
    match stmt {
        ast::Stmt::Empty => String::new(),
        ast::Stmt::Expr(e) => {
            let e = eval_expr(e, Some(env));
            format!("{:?}", e)
        }
        ast::Stmt::Let(l, r) => {
            let r = eval_expr(r, Some(env));
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

fn eval_expr(e: ast::Expr, env: Option<&Environment>) -> Expr {
    match e {
        ast::Expr::Num(s) => {
            let n: i32 = s.parse().unwrap();
            Expr::int(n)
        }
        ast::Expr::Ident(s) => match env {
            Some(env) => env.get(s),
            None => Expr::Sym(s.to_string()),
        },
        ast::Expr::Unary(op, expr) => match op {
            ast::UnOp::Neg => NEG_ONE * eval_expr(*expr, env),
        },
        ast::Expr::Binary(op, left, right) => match op {
            ast::BinOp::Add => eval_expr(*left, env) + eval_expr(*right, env),
            ast::BinOp::Sub => eval_expr(*left, env) - eval_expr(*right, env),
            ast::BinOp::Mul | ast::BinOp::ImpliedMul => {
                eval_expr(*left, env) * eval_expr(*right, env)
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

// fn eval_expr_with_env(
//     expr: ast::Expr,
//     env: Option<Environment>,
// ) -> (Expr, Option<Environment>) {
//     match expr {
//         ast::Expr::Num(s) => {
//             let n: i32 = s.parse().unwrap();
//             (Expr::int(n), env)
//         }
//         ast::Expr::Ident(s) => match env {
//             Some(env) => (env.get(s), Some(env)),
//             None => (Expr::Sym(s.to_string()), env),
//         },
//         ast::Expr::Unary(op, expr) => match op {
//             ast::UnOp::Neg => {
//                 let (expr, env) = eval_expr_with_env(*expr, env);
//                 (NEG_ONE * expr, env)
//             }
//         },
//         ast::Expr::Binary(op, left, right) => match op {
//             ast::BinOp::Add => {
//                 let (left, env) = eval_expr_with_env(*left, env);
//                 let (right, env) = eval_expr_with_env(*right, env);
//                 (left + right, env)
//             }
//             ast::BinOp::Sub => {
//                 let (left, env) = eval_expr_with_env(*left, env);
//                 let (right, env) = eval_expr_with_env(*right, env);
//                 (left - right, env)
//             }
//             ast::BinOp::Mul | ast::BinOp::ImpliedMul => {
//                 let (left, env) = eval_expr_with_env(*left, env);
//                 let (right, env) = eval_expr_with_env(*right, env);
//                 (left * right, env)
//             }
//             ast::BinOp::Div => {
//                 let (left, env) = eval_expr_with_env(*left, env);
//                 let (right, env) = eval_expr_with_env(*right, env);
//                 (left / right, env)
//             }
//             ast::BinOp::Pow => {
//                 let (left, env) = eval_expr_with_env(*left, env);
//                 let (right, env) = eval_expr_with_env(*right, env);
//                 (Expr::pow(left, right), env)
//             }
//         },
//     }
// }
