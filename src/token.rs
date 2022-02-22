#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Num(&'a str),
    Plus,
    Minus,
    Asterisk,
    Slash,
    Lparen,
    Rparen,
    Illegal,
}
