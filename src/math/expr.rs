use std::collections::BTreeMap;

#[cfg(test)]
mod test;

const ONE: Expr = Expr::Num(1);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Expr {
    Num(i32),
    Sym(String),
    Add(Add),
    Mul(Mul),
    Pow(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn to_coeff_mul(self) -> (i32, Expr) {
        match self {
            Expr::Num(n) => (n, ONE),
            e @ Expr::Sym(_) | e @ Expr::Add(_) | e @ Expr::Pow(_, _) => (1, e),
            Expr::Mul(m) => (m.coeff, m.to_expr()),
        }
    }

    fn pow(self, exp: Expr) -> Expr {
        Expr::Pow(Box::new(self), Box::new(exp))
    }
}

impl std::ops::Add for Expr {
    type Output = Expr;
    fn add(self, rhs: Self) -> Self::Output {
        Add::add(self, rhs).to_expr()
    }
}

impl std::ops::Mul for Expr {
    type Output = Expr;
    fn mul(self, rhs: Self) -> Self::Output {
        Mul::mul(self, rhs).to_expr()
    }
}

#[derive(Debug)]
pub struct Add {
    args: Option<Vec<Expr>>,
    coeff: i32,
    terms: BTreeMap<Expr, i32>,
}

impl Clone for Add {
    fn clone(&self) -> Self {
        Add {
            args: None,
            coeff: self.coeff,
            terms: self.terms.clone(),
        }
    }
}

impl PartialEq for Add {
    fn eq(&self, other: &Self) -> bool {
        self.coeff == other.coeff && self.terms == other.terms
    }
}

impl Eq for Add {}

impl Ord for Add {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.terms
            .cmp(&other.terms)
            .then(self.coeff.cmp(&other.coeff))
    }
}

impl PartialOrd for Add {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Add {
    fn new() -> Add {
        Add {
            args: None,
            coeff: 0,
            terms: BTreeMap::new(),
        }
    }

    fn args(&mut self) -> &Vec<Expr> {
        let terms = &self.terms;
        self.args.get_or_insert_with(|| {
            let mut args = Vec::new();
            for (e, c) in terms {
                args.push(Expr::Num(*c) * e.clone());
            }
            args
        })
    }

    fn to_expr(self) -> Expr {
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
                self.terms.append(&mut other.terms);
            }
            Expr::Num(n) => self.coeff += n,
            e @ _ => {
                let (c, e) = e.to_coeff_mul();
                *self.terms.entry(e).or_insert(0) += c;
            }
        }
    }
}

#[derive(Debug)]
pub struct Mul {
    args: Option<Vec<Expr>>,
    coeff: i32,
    terms: BTreeMap<Expr, i32>,
}

impl Clone for Mul {
    fn clone(&self) -> Self {
        Mul {
            args: None,
            coeff: self.coeff,
            terms: self.terms.clone(),
        }
    }
}

impl PartialEq for Mul {
    fn eq(&self, other: &Self) -> bool {
        self.coeff == other.coeff && self.terms == other.terms
    }
}

impl Eq for Mul {}

impl Ord for Mul {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.terms
            .cmp(&other.terms)
            .then(self.coeff.cmp(&other.coeff))
    }
}

impl PartialOrd for Mul {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Mul {
    fn new() -> Mul {
        Mul {
            args: None,
            coeff: 1,
            terms: BTreeMap::new(),
        }
    }

    fn args(&mut self) -> &Vec<Expr> {
        let terms = &self.terms;
        self.args.get_or_insert_with(|| {
            let mut args = Vec::new();
            for (e, c) in terms {
                args.push(Expr::pow(e.clone(), Expr::Num(*c)));
            }
            args
        })
    }

    fn to_expr(self) -> Expr {
        if self.coeff == 0 {
            return Expr::Num(0);
        } else if self.terms.is_empty() {
            return Expr::Num(self.coeff);
        } else if self.coeff == 1 && self.terms.len() == 1 {
            let (e, c) = self.terms.into_iter().next().unwrap();
            return Expr::pow(e, Expr::Num(c));
        }
        Expr::Mul(self)
    }

    fn mul(left: Expr, right: Expr) -> Mul {
        let mut a = Mul::new();
        a.mul_assign(left);
        a.mul_assign(right);
        a
    }

    fn mul_assign(&mut self, other: Expr) {
        if self.coeff != 0 {
            match other {
                Expr::Mul(mut other) => {
                    self.coeff *= other.coeff;
                    self.terms.append(&mut other.terms);
                }
                Expr::Num(n) => self.coeff *= n,
                e @ _ => {
                    let (c, e) = e.to_coeff_mul();
                    *self.terms.entry(e).or_insert(0) += c;
                }
            }
        }
    }
}
