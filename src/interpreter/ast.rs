use std::fmt;

#[derive(PartialEq)]
pub enum Expr<'input> {
    Num(&'input str),
    Ident(&'input str),
    Unary(UnOp, Box<Expr<'input>>),
    Binary(BinOp, Box<Expr<'input>>, Box<Expr<'input>>),
}

#[derive(PartialEq)]
pub enum UnOp {
    Neg,
}

#[derive(PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    ImpliedMul,
    Pow,
}

impl fmt::Debug for Expr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &Expr::Num(s) | &Expr::Ident(s) => write!(f, "{}", s),
            Expr::Unary(op, e) => write!(f, "({:?} {:?})", op, **e),
            Expr::Binary(op, l, r) => {
                write!(f, "({:?} {:?} {:?})", **l, op, **r)
            }
        }
    }
}

impl fmt::Debug for UnOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &UnOp::Neg => write!(f, "-"),
        }
    }
}

impl fmt::Debug for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            BinOp::Add => write!(f, "+"),
            BinOp::Sub => write!(f, "-"),
            BinOp::Mul => write!(f, "*"),
            BinOp::Div => write!(f, "/"),
            BinOp::ImpliedMul => write!(f, "im"),
            BinOp::Pow => write!(f, "^"),
        }
    }
}
