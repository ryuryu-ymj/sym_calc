use std::{fmt, mem};

#[cfg(test)]
mod test;

fn gcd(n: i32, m: i32) -> i32 {
    if m == 0 {
        n
    } else {
        gcd(m, n % m)
    }
}

pub const ZERO: Number = Number::Int(0);
pub const ONE: Number = Number::Int(1);
pub const NEG_ONE: Number = Number::Int(-1);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Number {
    Int(i32),
    Rat(i32, i32),
}

impl Number {
    pub fn int(i: i32) -> Number {
        Number::Int(i)
    }

    pub fn rational(num: i32, den: i32) -> Number {
        let p = num * den;
        let g = gcd(num, den);
        let num = (num / g).abs() * if p >= 0 { 1 } else { -1 };
        let den = (den / g).abs();
        if den == 1 {
            Number::int(num)
        } else {
            Number::Rat(num, den)
        }
    }

    pub fn is_positive(&self) -> bool {
        match self {
            Number::Int(i) => i > &0,
            Number::Rat(n, _) => n > &0,
        }
    }

    pub fn pow(self, exp: i32) -> Number {
        match (self, exp) {
            (Number::Int(i), exp @ 0..) => {
                Number::int(i.pow(exp.unsigned_abs()))
            }
            (Number::Int(i), exp) => {
                Number::rational(1, i.pow(exp.unsigned_abs()))
            }
            (Number::Rat(n, d), exp @ 0..) => {
                let exp = exp.unsigned_abs();
                Number::Rat(n.pow(exp), d.pow(exp))
            }
            (Number::Rat(n, d), exp) => {
                let exp = exp.unsigned_abs();
                Number::Rat(d.pow(exp), n.pow(exp))
            }
        }
    }
}

impl Default for Number {
    fn default() -> Self {
        ZERO
    }
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Int(i) => write!(f, "{}", i),
            Number::Rat(n, d) => write!(f, "{}/{}", n, d),
        }
    }
}

impl std::ops::Add for Number {
    type Output = Number;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Int(i), Number::Int(j)) => Number::int(i + j),
            (Number::Int(i), Number::Rat(n, d))
            | (Number::Rat(n, d), Number::Int(i)) => {
                Number::rational(n + d * i, d)
            }
            (Number::Rat(n1, d1), Number::Rat(n2, d2)) => {
                Number::rational(n1 * d2 + n2 * d1, d1 * d2)
            }
        }
    }
}

impl std::ops::AddAssign for Number {
    fn add_assign(&mut self, rhs: Self) {
        *self = mem::take(self) + rhs;
    }
}

// impl std::ops::AddAssign<i32> for Number {
//     fn add_assign(&mut self, rhs: i32) {
//         *self = mem::take(self) + Number::int(rhs);
//     }
// }

// impl std::ops::Sub for Number {
//     type Output = Number;
//     fn sub(self, rhs: Self) -> Self::Output {
//         match (self, rhs) {
//             (Number::Int(i), Number::Int(j)) => Number::int(i - j),
//             (Number::Int(i), Number::Rat(n, d)) => {
//                 Number::rational(d * i - n, d)
//             }
//             (Number::Rat(n, d), Number::Int(i)) => {
//                 Number::rational(n - d * i, d)
//             }
//             (Number::Rat(n1, d1), Number::Rat(n2, d2)) => {
//                 Number::rational(n1 * d2 - n2 * d1, d1 * d2)
//             }
//         }
//     }
// }

impl std::ops::Mul for Number {
    type Output = Number;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Int(i), Number::Int(j)) => Number::int(i * j),
            (Number::Int(i), Number::Rat(n, d))
            | (Number::Rat(n, d), Number::Int(i)) => Number::rational(n * i, d),
            (Number::Rat(n1, d1), Number::Rat(n2, d2)) => {
                Number::rational(n1 * n2, d1 * d2)
            }
        }
    }
}

impl std::ops::MulAssign for Number {
    fn mul_assign(&mut self, rhs: Self) {
        *self = mem::take(self) * rhs;
    }
}
