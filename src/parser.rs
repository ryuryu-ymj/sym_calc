use crate::{ast, lexer::Lexer, token::Token};

#[cfg(test)]
mod test;

pub struct Parser<'input> {
    lexer: Lexer<'input>,
    token: Token<'input>,
    prev_token: Token<'input>,
}

impl<'input> Parser<'input> {
    pub fn new(lexer: Lexer<'input>) -> Parser<'input> {
        Parser {
            lexer,
            token: Token::Dummy,
            prev_token: Token::Dummy,
        }
    }

    pub fn parse_expr(self) -> ast::Expr<'input> {
        ast::Expr::Num("1")
    }
}
