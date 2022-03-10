#[derive(Debug)]
pub enum Expr {
    Add(Vec<Expr>),
    Mul(Vec<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Ident(String),
    Num(i32),
}
