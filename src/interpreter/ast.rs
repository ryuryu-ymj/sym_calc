#[derive(Debug, PartialEq)]
pub enum Expr<'input> {
    Num(&'input str),
    Ident(&'input str),
    Unary(UnOp, Box<Expr<'input>>),
    Binary(BinOp, Box<Expr<'input>>, Box<Expr<'input>>),
}

#[derive(Debug, PartialEq)]
pub enum UnOp {
    Neg,
}

#[derive(Debug, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    ImpliedMul,
}
