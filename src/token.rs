#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Num(&'a str),
    Plus,
    Asterisk,
    Illegal,
}
