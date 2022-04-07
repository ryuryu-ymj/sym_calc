use super::expr::{Expr, ONE, ZERO};

#[cfg(test)]
mod test;

pub const CMD_DIFF: Expr = Expr::Cmd("\\diff", lib_diff);

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
                let expr = Expr::Pow(base, exp);
                Expr::Call(
                    Box::new(CMD_DIFF),
                    Box::new(Expr::Vec(vec![expr, Expr::Sym(sym.to_string())])),
                )
            }
        }
        _ => Expr::Call(
            Box::new(CMD_DIFF),
            Box::new(Expr::Vec(vec![expr, Expr::Sym(sym.to_string())])),
        ),
    }
}

pub fn lib_diff(expr: Expr) -> Expr {
    match expr {
        Expr::Vec(mut v) if v.len() == 2 => {
            if let (Expr::Sym(s), e) = (v.pop().unwrap(), v.pop().unwrap()) {
                return diff(e, &s);
            }
        }
        _ => {}
    }
    Expr::err("arguments error")
}
