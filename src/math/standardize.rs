use crate::interpret::object::{mul, pow, Expr};
use std::collections::BTreeMap;

#[cfg(test)]
mod test;

pub fn standardize(e: Expr) -> Expr {
    match e {
        Expr::Add(v) => {
            let mut v = standardize_add(v);
            match v.len() {
                0 => Expr::Num(0),
                1 => v.pop().unwrap(),
                _ => Expr::Add(v),
            }
        }
        Expr::Mul(v) => {
            let (c, mut v) = standardize_mul(v);
            if v.is_empty() {
                Expr::Num(c)
            } else if c == 0 {
                Expr::Num(0)
            } else {
                if c != 1 {
                    v.insert(0, Expr::Num(c));
                }
                if v.len() == 1 {
                    v.pop().unwrap()
                } else {
                    Expr::Mul(v)
                }
            }
        }
        _ => e,
    }
}

fn standardize_add(v: Vec<Expr>) -> Vec<Expr> {
    let mut terms = BTreeMap::new();
    let mut coeff = 0;

    for e in v {
        match e {
            Expr::Num(n) => {
                coeff += n;
            }
            Expr::Mul(v) => {
                let (c, mut v) = standardize_mul(v);
                if c != 0 {
                    match v.len() {
                        0 => {
                            coeff += c;
                        }
                        1 => {
                            *terms.entry(v.pop().unwrap()).or_insert(0) += c;
                        }
                        _ => {
                            *terms.entry(Expr::Mul(v)).or_insert(0) += c;
                        }
                    }
                }
            }
            _ => {
                *terms.entry(e).or_insert(0) += 1;
            }
        }
    }

    let mut new_v = Vec::new();
    for (e, c) in terms {
        match c {
            0 => {}
            1 => {
                new_v.push(e);
            }
            _ => {
                new_v.push(mul(Expr::Num(c), e));
            }
        }
    }
    if coeff != 0 {
        new_v.push(Expr::Num(coeff));
    }
    new_v
}

fn standardize_mul(v: Vec<Expr>) -> (i32, Vec<Expr>) {
    let mut terms = BTreeMap::new();
    let mut coeff = 1;

    for e in v {
        match e {
            Expr::Num(n) => {
                coeff *= n;
            }
            _ => {
                *terms.entry(e).or_insert(0) += 1;
            }
        }
    }

    let mut new_v = Vec::new();
    if coeff != 0 {
        for (e, c) in terms {
            match c {
                0 => {}
                1 => {
                    new_v.push(e);
                }
                _ => {
                    new_v.push(pow(e, Expr::Num(c)));
                }
            }
        }
    }
    (coeff, new_v)
}
