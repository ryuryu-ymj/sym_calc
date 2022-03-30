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

pub const ZERO: Num = Num::Int(0);
pub const ONE: Num = Num::Int(1);
pub const NEG_ONE: Num = Num::Int(-1);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Num {
    Int(i32),
    Rat(i32, i32),
}

impl Num {
    pub fn int(i: i32) -> Num {
        Num::Int(i)
    }

    pub fn rational(num: i32, den: i32) -> Num {
        let p = num * den;
        let g = gcd(num, den);
        let num = (num / g).abs() * if p >= 0 { 1 } else { -1 };
        let den = (den / g).abs();
        if den == 1 {
            Num::int(num)
        } else {
            Num::Rat(num, den)
        }
    }

    // pub fn is_positive(&self) -> bool {
    //     match self {
    //         Number::Int(i) => i > &0,
    //         Number::Rat(n, _) => n > &0,
    //     }
    // }

    pub fn pow(self, exp: i32) -> Num {
        match (self, exp) {
            (Num::Int(i), exp @ 0..) => Num::int(i.pow(exp.unsigned_abs())),
            (Num::Int(i), exp) => Num::rational(1, i.pow(exp.unsigned_abs())),
            (Num::Rat(n, d), exp @ 0..) => {
                let exp = exp.unsigned_abs();
                Num::Rat(n.pow(exp), d.pow(exp))
            }
            (Num::Rat(n, d), exp) => {
                let exp = exp.unsigned_abs();
                Num::Rat(d.pow(exp), n.pow(exp))
            }
        }
    }
}

impl Default for Num {
    fn default() -> Self {
        ZERO
    }
}

impl fmt::Debug for Num {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Num::Int(i) => write!(f, "{}", i),
            Num::Rat(n, d) => write!(f, "{}/{}", n, d),
        }
    }
}

impl std::ops::Add for Num {
    type Output = Num;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Num::Int(i), Num::Int(j)) => Num::int(i + j),
            (Num::Int(i), Num::Rat(n, d)) | (Num::Rat(n, d), Num::Int(i)) => {
                Num::rational(n + d * i, d)
            }
            (Num::Rat(n1, d1), Num::Rat(n2, d2)) => {
                Num::rational(n1 * d2 + n2 * d1, d1 * d2)
            }
        }
    }
}

impl std::ops::AddAssign for Num {
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

impl std::ops::Mul for Num {
    type Output = Num;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Num::Int(i), Num::Int(j)) => Num::int(i * j),
            (Num::Int(i), Num::Rat(n, d)) | (Num::Rat(n, d), Num::Int(i)) => {
                Num::rational(n * i, d)
            }
            (Num::Rat(n1, d1), Num::Rat(n2, d2)) => {
                Num::rational(n1 * n2, d1 * d2)
            }
        }
    }
}

impl std::ops::MulAssign for Num {
    fn mul_assign(&mut self, rhs: Self) {
        *self = mem::take(self) * rhs;
    }
}
