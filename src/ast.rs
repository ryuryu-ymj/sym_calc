#[derive(Debug, PartialEq)]
pub enum Expr<'input> {
    Num(&'input str),
    Binary(BinOp, Box<Expr<'input>>, Box<Expr<'input>>),
}

#[derive(Debug, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}
