use std::collections::BTreeMap;
use std::fmt;

use super::num::{self, Num};

#[cfg(test)]
mod test;

pub const ZERO: Expr = Expr::Num(num::ZERO);
pub const ONE: Expr = Expr::Num(num::ONE);
pub const NEG_ONE: Expr = Expr::Num(num::NEG_ONE);

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Expr {
    Num(Num),
    Sym(String),
    Add(Add),
    Mul(Mul),
    Pow(Box<Expr>, Box<Expr>),
    Vec(Vec<Expr>),
    Cmd(&'static str, fn(Expr) -> Expr),
    Call(Box<Expr>, Box<Expr>),
    Err(String),
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Num(n) => write!(f, "{:?}", n),
            Expr::Sym(s) => write!(f, "{}", s),
            Expr::Add(a) => fmt_expr_list(f, &a.clone().into_args(), " + "),
            Expr::Mul(m) => fmt_expr_list(f, &m.clone().into_args(), " * "),
            Expr::Pow(b, e) => write!(f, "({:?} ^ {:?})", b, e),
            Expr::Vec(v) => fmt_expr_list(f, v, ", "),
            Expr::Cmd(s, _) => write!(f, "{}", s),
            Expr::Call(g, x) => write!(f, "{:?}({:?})", g, x),
            Expr::Err(s) => write!(f, "{}", s),
        }
    }
}

fn fmt_expr_list(
    f: &mut fmt::Formatter<'_>,
    v: &[Expr],
    sp: &str,
) -> fmt::Result {
    let mut i = v.iter();
    if let Some(e) = i.next() {
        let mut r = write!(f, "({:?}", e);
        for e in i {
            r = r.and_then(|_| write!(f, "{}{:?}", sp, e));
        }
        r = r.and_then(|_| write!(f, ")"));
        r
    } else {
        Ok(())
    }
}

impl Default for Expr {
    fn default() -> Self {
        ZERO
    }
}

impl Expr {
    pub fn int(i: i32) -> Expr {
        Expr::Num(Num::int(i))
    }

    // pub fn rational(num: i32, den: i32) -> Expr {
    //     Expr::Num(Number::rational(num, den))
    // }

    pub fn err(s: impl Into<String>) -> Expr {
        Expr::Err(s.into())
    }

    fn into_coeff_mul(self) -> (Num, Expr) {
        match self {
            Expr::Num(n) => (n, ONE),
            Expr::Mul(m) => m.into_coeff_mul(),
            e => (num::ONE, e),
        }
    }

    pub fn pow(self, exp: Expr) -> Expr {
        // TODO: 0^0 = 1?
        match (self, exp) {
            (Expr::Err(mut s1), Expr::Err(s2)) => {
                s1.push('\n');
                s1.push_str(&s2);
                Expr::err(s1)
            }
            (Expr::Vec(_), _) | (_, Expr::Vec(_)) => {
                let s = String::from("unsupported operand");
                Expr::err(s)
            }
            (_, ZERO) => ONE,
            (base, ONE) => base,
            (ZERO, _) => ZERO,
            (ONE, _) => ONE,
            (Expr::Num(base), Expr::Num(Num::Int(exp))) => {
                Expr::Num(base.pow(exp))
            }
            (Expr::Mul(mul), exp @ Expr::Num(_)) => Expr::prod(
                mul.into_args()
                    .into_iter()
                    .map(|e| Expr::pow(e, exp.clone())),
            ),
            (Expr::Pow(base, exp2), exp1) => Expr::pow(*base, *exp2 * exp1),
            (base, exp) => Expr::Pow(Box::new(base), Box::new(exp)),
        }
    }

    pub fn call(callable: Expr, argument: Expr) -> Expr {
        match Expr::into_coeff_mul(callable) {
            (c, Expr::Cmd(_, f)) => Expr::Num(c) * f(argument),
            _ => Expr::err(""),
        }
    }

    pub fn unevaluated_call(callable: Expr, argument: Expr) -> Expr {
        Expr::Call(Box::new(callable), Box::new(argument))
    }

    pub fn sum<I>(iter: I) -> Expr
    where
        I: IntoIterator<Item = Expr>,
    {
        let mut a = Add::new();
        for e in iter {
            a.add_assign(e);
        }
        a.into_expr()
    }

    pub fn prod<I>(iter: I) -> Expr
    where
        I: IntoIterator<Item = Expr>,
    {
        let mut m = Mul::new();
        for e in iter {
            m.mul_assign(e);
        }
        m.into_expr()
    }
}

impl std::ops::Add for Expr {
    type Output = Expr;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Expr::Err(mut s1), Expr::Err(s2)) => {
                s1.push('\n');
                s1.push_str(&s2);
                Expr::err(s1)
            }
            (e @ Expr::Err(_), _) | (_, e @ Expr::Err(_)) => e,
            (Expr::Vec(mut v1), Expr::Vec(v2)) if v1.len() == v2.len() => {
                for (e1, e2) in v1.iter_mut().zip(v2.into_iter()) {
                    *e1 += e2;
                }
                Expr::Vec(v1)
            }
            (Expr::Vec(v1), Expr::Vec(v2)) => Expr::err(format!(
                "unsupported operand: +: \\R^{} x \\R^{} -> ?",
                v1.len(),
                v2.len()
            )),
            (Expr::Vec(_), _) | (_, Expr::Vec(_)) => {
                Expr::err("unsupported operand")
            }
            (e1, e2) => {
                let mut a = Add::new();
                a.add_assign(e1);
                a.add_assign(e2);
                a.into_expr()
            }
        }
    }
}

impl std::ops::AddAssign for Expr {
    fn add_assign(&mut self, rhs: Self) {
        *self = std::mem::take(self) + rhs;
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
        match (self, rhs) {
            (Expr::Err(mut s1), Expr::Err(s2)) => {
                s1.push('\n');
                s1.push_str(&s2);
                Expr::err(s1)
            }
            (Expr::Vec(_), Expr::Vec(_)) => Expr::err("unsupported operand"),
            (Expr::Vec(mut v), c) | (c, Expr::Vec(mut v)) => {
                for e in &mut v {
                    *e *= c.clone();
                }
                Expr::Vec(v)
            }
            (e1, e2) => {
                let mut m = Mul::new();
                m.mul_assign(e1);
                m.mul_assign(e2);
                m.into_expr()
            }
        }
    }
}

impl std::ops::MulAssign for Expr {
    fn mul_assign(&mut self, rhs: Self) {
        *self = std::mem::take(self) * rhs;
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
    terms: BTreeMap<Expr, Num>,
    coeff: Num,
}

impl Default for Add {
    fn default() -> Self {
        Self::new()
    }
}

impl Add {
    fn new() -> Add {
        Add {
            coeff: num::ZERO,
            terms: BTreeMap::new(),
        }
    }

    pub fn into_args(self) -> Vec<Expr> {
        let mut args = Vec::new();
        for (e, c) in self.terms {
            args.push(Expr::Num(c) * e)
        }
        if self.coeff != num::ZERO {
            args.push(Expr::Num(self.coeff));
        }
        args
    }

    fn into_expr(mut self) -> Expr {
        self.terms.retain(|_, c| *c != num::ZERO);
        if self.terms.is_empty() {
            return Expr::Num(self.coeff);
        } else if self.coeff == num::ZERO && self.terms.len() == 1 {
            let (e, c) = self.terms.into_iter().next().unwrap();
            return Expr::Num(c) * e;
        }
        Expr::Add(self)
    }

    fn add_assign(&mut self, other: Expr) {
        match other {
            Expr::Add(mut other) => {
                self.coeff += other.coeff;
                if self.terms.is_empty() {
                    std::mem::swap(&mut self.terms, &mut other.terms);
                } else {
                    for (e, c) in other.terms {
                        *self.terms.entry(e).or_insert(num::ZERO) += c;
                    }
                }
            }
            Expr::Num(n) => self.coeff += n,
            e => {
                let (c, e) = e.into_coeff_mul();
                *self.terms.entry(e).or_insert(num::ZERO) += c;
            }
        }
    }

    fn is_zero(&self) -> bool {
        self.coeff == num::ZERO && self.terms.is_empty()
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mul {
    terms: BTreeMap<Expr, Add>,
    coeff: Num,
}

impl Mul {
    fn new() -> Mul {
        Mul {
            coeff: num::ONE,
            terms: BTreeMap::new(),
        }
    }

    pub fn into_args(self) -> Vec<Expr> {
        let mut args = Vec::new();
        if self.coeff != num::ONE {
            args.push(Expr::Num(self.coeff));
        }
        for (e, c) in self.terms {
            args.push(Expr::pow(e, c.into_expr()));
        }
        args
    }

    fn into_expr(mut self) -> Expr {
        self.terms.retain(|_, c| !c.is_zero());
        if self.coeff == num::ZERO {
            return ZERO;
        } else if self.terms.is_empty() {
            return Expr::Num(self.coeff);
        } else if self.coeff == num::ONE && self.terms.len() == 1 {
            let (e, c) = self.terms.into_iter().next().unwrap();
            return Expr::pow(e, c.into_expr());
        }
        Expr::Mul(self)
    }

    fn mul_assign(&mut self, other: Expr) {
        if self.coeff != num::ZERO {
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

    fn into_coeff_mul(mut self) -> (Num, Expr) {
        let c = self.coeff;
        self.coeff = num::ONE;
        (c, self.into_expr())
    }
}
