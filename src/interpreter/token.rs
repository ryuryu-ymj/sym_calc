#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Num(&'a str),   // 16
    Ident(&'a str), // x, \pi
    Plus,           // +
    Minus,          // -
    Star,           // *
    Slash,          // /
    Caret,          // ^
    Lparen,         // (
    Rparen,         // )
    Illegal,
    Eof,
}
