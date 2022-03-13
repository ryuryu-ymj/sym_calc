use std::fmt;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
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

pub fn add(left: Expr, right: Expr) -> Expr {
    match (left, right) {
        (Expr::Add(mut v1), Expr::Add(mut v2)) => {
            v1.append(&mut v2);
            Expr::Add(v1)
        }
        (Expr::Add(mut v), e) => {
            v.push(e);
            Expr::Add(v)
        }
        (e, Expr::Add(mut v)) => {
            v.insert(0, e);
            Expr::Add(v)
        }
        (l, r) => Expr::Add(vec![l, r]),
    }
}

pub fn mul(left: Expr, right: Expr) -> Expr {
    match (left, right) {
        (Expr::Mul(mut v1), Expr::Mul(mut v2)) => {
            v1.append(&mut v2);
            Expr::Mul(v1)
        }
        (Expr::Mul(mut v), e) => {
            v.push(e);
            Expr::Mul(v)
        }
        (e, Expr::Mul(mut v)) => {
            v.insert(0, e);
            Expr::Mul(v)
        }
        (l, r) => Expr::Mul(vec![l, r]),
    }
}

pub fn pow(base: Expr, power: Expr) -> Expr {
    Expr::Pow(Box::new(base), Box::new(power))
}
