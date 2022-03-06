use crate::{ast, lexer::Lexer, token::Token};
use std::mem;

#[cfg(test)]
mod test;

pub struct Parser<'input> {
    lexer: Lexer<'input>,
    token: Token<'input>,
    next_token: Token<'input>,
}

impl<'input> Parser<'input> {
    pub fn new(lexer: Lexer<'input>) -> Parser<'input> {
        let mut p = Parser {
            lexer,
            token: Token::Eof,
            next_token: Token::Eof,
        };

        p.bump();
        p.bump();
        p
    }

    fn bump(&mut self) {
        self.token =
            mem::replace(&mut self.next_token, self.lexer.next_token());
    }

    pub fn parse_expr(&mut self, precedence: u32) -> ast::Expr<'input> {
        let mut left = match self.token {
            Token::Num(s) => ast::Expr::Num(s),
            _ => panic!(),
        };
        self.bump();

        loop {
            match self.parse_binary_op() {
                Some(op) => {
                    let op_prec = op.precedence();
                    if precedence >= op_prec {
                        break;
                    }
                    left = self.parse_binary_expr(op, left, op_prec);
                }
                None => return left,
            }
        }
        left
    }

    fn parse_binary_op(&self) -> Option<ast::BinOp> {
        match self.token {
            Token::Plus => Some(ast::BinOp::Add),
            Token::Minus => Some(ast::BinOp::Sub),
            Token::Asterisk => Some(ast::BinOp::Mul),
            Token::Slash => Some(ast::BinOp::Div),
            _ => None,
        }
    }

    fn parse_binary_expr(
        &mut self,
        op: ast::BinOp,
        left: ast::Expr<'input>,
        precedence: u32,
    ) -> ast::Expr<'input> {
        self.bump();
        let right = self.parse_expr(precedence);
        ast::Expr::Binary(op, Box::new(left), Box::new(right))
    }
}
