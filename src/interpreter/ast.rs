use std::fmt;

pub enum Stmt<'input> {
    Empty,
    Expr(Expr<'input>),
    Let(Expr<'input>, Expr<'input>),
}

pub enum Expr<'input> {
    Num(&'input str),
    Ident(&'input str),
    Unary(UnOp, Box<Expr<'input>>),
    Binary(BinOp, Box<Expr<'input>>, Box<Expr<'input>>),
    List(Vec<Expr<'input>>),
}

pub enum UnOp {
    Neg,
}

pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    ImpliedMul,
    Pow,
}

impl fmt::Debug for Stmt<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Empty => Ok(()),
            Stmt::Expr(e) => write!(f, "{:?}", e),
            Stmt::Let(l, r) => write!(f, "\\let ({:?}) = ({:?})", l, r),
        }
    }
}

impl fmt::Debug for Expr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &Expr::Num(s) | &Expr::Ident(s) => write!(f, "{}", s),
            Expr::Unary(op, e) => write!(f, "({:?} {:?})", op, **e),
            Expr::Binary(op, l, r) => {
                write!(f, "({:?} {:?} {:?})", **l, op, **r)
            }
            Expr::List(v) => {
                let mut i = v.iter();
                if let Some(e) = i.next() {
                    let mut r = write!(f, "({:?}", e);
                    for e in i {
                        r = r.and_then(|_| write!(f, ", {:?}", e));
                    }
                    r = r.and_then(|_| write!(f, ")"));
                    r
                } else {
                    Ok(())
                }
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
