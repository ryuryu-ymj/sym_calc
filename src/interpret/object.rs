use std::fmt;

pub enum Expr {
    Num(i32),
    Ident(String),
    Add(Vec<Expr>),
    Mul(Vec<Expr>),
    Pow(Box<Expr>, Box<Expr>),
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Num(n) => write!(f, "{}", n),
            Expr::Ident(s) => write!(f, "{}", s),
            Expr::Add(v) => fmt_expr_list(f, v, '+'),
            Expr::Mul(v) => fmt_expr_list(f, v, '*'),
            Expr::Pow(b, e) => write!(f, "({:?} ^ {:?})", b, e),
        }
    }
}

fn fmt_expr_list(
    f: &mut fmt::Formatter<'_>,
    v: &[Expr],
    op: char,
) -> fmt::Result {
    let mut i = v.iter();
    if let Some(e) = i.next() {
        let mut r = write!(f, "({:?}", e);
        for e in i {
            r = r.and_then(|_| write!(f, " {} {:?}", op, e));
        }
        r = r.and_then(|_| write!(f, ")"));
        r
    } else {
        Ok(())
    }
}
