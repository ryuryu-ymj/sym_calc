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

impl BinOp {
    pub fn precedence(&self) -> u32 {
        match &self {
            BinOp::Mul | BinOp::Div => 2,
            BinOp::Add | BinOp::Sub => 1,
        }
    }
}
