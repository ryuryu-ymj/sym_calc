#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Num(&'a str),
    Ident(&'a str),
    Plus,
    Minus,
    Asterisk,
    Slash,
    Lparen,
    Rparen,
    Illegal,
    Eof,
}
