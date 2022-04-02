use super::expr::{Expr, ONE, ZERO};

#[cfg(test)]
mod test;
pub fn diff(expr: Expr, sym: &str) -> Expr {
    match expr {
        Expr::Num(_) => ZERO,
        Expr::Sym(s) if s == sym => ONE,
        // e @ Expr::Sym(_) => None,
        Expr::Add(a) => {
            Expr::sum(a.into_args().into_iter().map(|e| diff(e, sym)))
        }
        Expr::Mul(m) => {
            let args = m.into_args();
            Expr::sum((0..args.len()).into_iter().map(|i| {
                let mut args = args.clone();
                args[i] = diff(std::mem::take(&mut args[i]), sym);
                Expr::prod(args)
            }))
        }
        Expr::Pow(base, exp) => {
            if let exp @ Expr::Num(_) = *exp {
                exp.clone()
                    * Expr::pow(*base.clone(), exp - ONE)
                    * diff(*base, sym)
            } else {
                todo!()
            }
        }
        _ => todo!(),
    }
}
