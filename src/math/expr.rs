use std::collections::BTreeMap;
use std::fmt;

#[cfg(test)]
mod test;

pub const ZERO: Expr = Expr::Num(0);
pub const ONE: Expr = Expr::Num(1);
pub const NEG_ONE: Expr = Expr::Num(-1);

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Expr {
    Num(i32),
    Sym(String),
    Add(Add),
    Mul(Mul),
    Pow(Box<Expr>, Box<Expr>),
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Num(n) => write!(f, "{}", n),
            Expr::Sym(s) => write!(f, "{}", s),
            Expr::Add(a) => fmt_expr_list(f, &a.clone().into_args(), '+'),
            Expr::Mul(m) => fmt_expr_list(f, &m.clone().into_args(), '*'),
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

impl Expr {
    fn into_coeff_mul(self) -> (i32, Expr) {
        match self {
            Expr::Num(n) => (n, ONE),
            e @ Expr::Sym(_) | e @ Expr::Add(_) | e @ Expr::Pow(_, _) => (1, e),
            Expr::Mul(m) => m.into_coeff_mul(),
        }
    }

    fn pow(self, exp: Expr) -> Expr {
        // TODO: 0^0 = 1?
        match (self, exp) {
            (_, ZERO) => ONE,
            (base, ONE) => base,
            (ZERO, _) => ZERO,
            (ONE, _) => ONE,
            (Expr::Num(n), Expr::Num(m)) => {
                if m > 0 {
                    Expr::Num(n.pow(m.unsigned_abs()))
                } else {
                    ONE / Expr::Num(n.pow(m.unsigned_abs()))
                }
            }
            (Expr::Mul(mul), exp @ Expr::Num(_)) => {
                let mut ret = ONE;
                for e in mul.into_args() {
                    ret = ret * (Expr::pow(e, exp.clone()));
                }
                println!("{:?}", ret);
                ret
            }
            (Expr::Pow(base, exp2), exp1) => Expr::pow(*base, *exp2 * exp1),
            (base, exp) => Expr::Pow(Box::new(base), Box::new(exp)),
        }
    }
}

impl std::ops::Add for Expr {
    type Output = Expr;
    fn add(self, rhs: Self) -> Self::Output {
        Add::add(self, rhs).into_expr()
    }
}

impl std::ops::Sub for Expr {
    type Output = Expr;
    fn sub(self, rhs: Self) -> Self::Output {
        self + rhs * NEG_ONE
    }
}

impl std::ops::Mul for Expr {
    type Output = Expr;
    fn mul(self, rhs: Self) -> Self::Output {
        Mul::mul(self, rhs).into_expr()
    }
}

impl std::ops::Div for Expr {
    type Output = Expr;
    fn div(self, rhs: Self) -> Self::Output {
        self * Expr::pow(rhs, NEG_ONE)
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Add {
    terms: BTreeMap<Expr, i32>,
    coeff: i32,
}

impl Default for Add {
    fn default() -> Self {
        Self::new()
    }
}

impl Add {
    fn new() -> Add {
        Add {
            coeff: 0,
            terms: BTreeMap::new(),
        }
    }

    fn into_args(self) -> Vec<Expr> {
        let mut args = Vec::new();
        for (e, c) in self.terms {
            args.push(Expr::Num(c) * e)
        }
        if self.coeff != 0 {
            args.push(Expr::Num(self.coeff));
        }
        args
    }

    fn into_expr(mut self) -> Expr {
        self.terms.retain(|_, c| *c != 0);
        if self.terms.is_empty() {
            return Expr::Num(self.coeff);
        } else if self.coeff == 0 && self.terms.len() == 1 {
            let (e, c) = self.terms.into_iter().next().unwrap();
            return Expr::Num(c) * e;
        }
        Expr::Add(self)
    }

    fn add(left: Expr, right: Expr) -> Add {
        let mut a = Add::new();
        a.add_assign(left);
        a.add_assign(right);
        a
    }

    fn add_assign(&mut self, other: Expr) {
        match other {
            Expr::Add(mut other) => {
                self.coeff += other.coeff;
                if self.terms.is_empty() {
                    std::mem::swap(&mut self.terms, &mut other.terms);
                } else {
                    for (e, c) in other.terms {
                        *self.terms.entry(e).or_insert(0) += c;
                    }
                }
            }
            Expr::Num(n) => self.coeff += n,
            e => {
                let (c, e) = e.into_coeff_mul();
                *self.terms.entry(e).or_insert(0) += c;
            }
        }
    }

    fn is_zero(&self) -> bool {
        self.coeff == 0 && self.terms.is_empty()
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mul {
    terms: BTreeMap<Expr, Add>,
    coeff: i32,
}

impl Mul {
    fn new() -> Mul {
        Mul {
            coeff: 1,
            terms: BTreeMap::new(),
        }
    }

    fn into_args(self) -> Vec<Expr> {
        let mut args = Vec::new();
        if self.coeff != 1 {
            args.push(Expr::Num(self.coeff));
        }
        for (e, c) in self.terms {
            args.push(Expr::pow(e, c.into_expr()));
        }
        args
    }

    fn into_expr(mut self) -> Expr {
        self.terms.retain(|_, c| !c.is_zero());
        if self.coeff == 0 {
            return ZERO;
        } else if self.terms.is_empty() {
            return Expr::Num(self.coeff);
        } else if self.coeff == 1 && self.terms.len() == 1 {
            let (e, c) = self.terms.into_iter().next().unwrap();
            return Expr::pow(e, c.into_expr());
        }
        Expr::Mul(self)
    }

    fn mul(left: Expr, right: Expr) -> Mul {
        let mut m = Mul::new();
        m.mul_assign(left);
        m.mul_assign(right);
        m
    }

    fn mul_assign(&mut self, other: Expr) {
        if self.coeff != 0 {
            match other {
                Expr::Mul(mut other) => {
                    self.coeff *= other.coeff;
                    if self.terms.is_empty() {
                        std::mem::swap(&mut self.terms, &mut other.terms);
                    } else {
                        for (e, c) in other.terms {
                            self.terms
                                .entry(e)
                                .or_default()
                                .add_assign(Expr::Add(c));
                        }
                    }
                }
                Expr::Num(n) => self.coeff *= n,
                Expr::Pow(base, exp) => {
                    self.terms.entry(*base).or_default().add_assign(*exp);
                }
                e => {
                    self.terms.entry(e).or_default().add_assign(ONE);
                }
            }
        }
    }

    fn into_coeff_mul(mut self) -> (i32, Expr) {
        let c = self.coeff;
        self.coeff = 1;
        (c, self.into_expr())
    }
}
