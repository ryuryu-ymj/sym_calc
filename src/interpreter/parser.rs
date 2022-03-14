use super::{ast, lexer::Lexer, token::Token};

#[cfg(test)]
mod test;

#[derive(PartialEq, PartialOrd)]
enum Precedence {
    Lowest,
    Sum,
    Prod,
    Unary,
    ImpliedMul,
}

pub struct Parser<'input> {
    lexer: Lexer<'input>,
    token: Token<'input>,
}

impl<'input> Parser<'input> {
    pub fn new(lexer: Lexer<'input>) -> Parser<'input> {
        let mut p = Parser {
            lexer,
            token: Token::Eof,
        };

        p.bump();
        p
    }

    fn bump(&mut self) {
        self.token = self.lexer.next_token();
    }

    pub fn parse_expr_stmt(&mut self) -> ast::Expr<'input> {
        self.parse_expr(Precedence::Lowest)
    }

    fn parse_expr(&mut self, precedence: Precedence) -> ast::Expr<'input> {
        let mut left = match self.token {
            Token::Num(s) => {
                self.bump();
                ast::Expr::Num(s)
            }
            Token::Ident(s) => {
                self.bump();
                ast::Expr::Ident(s)
            }
            Token::Lparen => self.parse_grouped_expr(),
            _ => match self.parse_unary_op() {
                Some(op) => self.parse_unary_expr(op),
                None => panic!("parsing error"),
            },
        };

        loop {
            match self.parse_binary_op() {
                Some((op, p)) => {
                    if precedence >= p {
                        break;
                    }
                    left = self.parse_binary_expr(op, left, p);
                }
                None => match self.token {
                    Token::Num(_) | Token::Ident(_) | Token::Lparen => {
                        if precedence >= Precedence::ImpliedMul {
                            break;
                        }
                        left = self.parse_implied_mul_expr(left);
                    }
                    _ => return left,
                },
            }
        }
        left
    }

    fn parse_unary_op(&self) -> Option<ast::UnOp> {
        match self.token {
            Token::Minus => Some(ast::UnOp::Neg),
            _ => None,
        }
    }

    fn parse_unary_expr(&mut self, op: ast::UnOp) -> ast::Expr<'input> {
        self.bump();
        let e = self.parse_expr(Precedence::Unary);
        ast::Expr::Unary(op, Box::new(e))
    }

    fn parse_binary_op(&self) -> Option<(ast::BinOp, Precedence)> {
        match self.token {
            Token::Plus => Some((ast::BinOp::Add, Precedence::Sum)),
            Token::Minus => Some((ast::BinOp::Sub, Precedence::Sum)),
            Token::Asterisk => Some((ast::BinOp::Mul, Precedence::Prod)),
            Token::Slash => Some((ast::BinOp::Div, Precedence::Prod)),
            _ => None,
        }
    }

    fn parse_binary_expr(
        &mut self,
        op: ast::BinOp,
        left: ast::Expr<'input>,
        p: Precedence,
    ) -> ast::Expr<'input> {
        self.bump();
        let right = self.parse_expr(p);
        ast::Expr::Binary(op, Box::new(left), Box::new(right))
    }

    fn parse_grouped_expr(&mut self) -> ast::Expr<'input> {
        self.bump();
        let e = self.parse_expr(Precedence::Lowest);
        if self.token != Token::Rparen {
            panic!("No corresponding right parentheses.");
        }
        self.bump();
        e
    }

    fn parse_implied_mul_expr(
        &mut self,
        left: ast::Expr<'input>,
    ) -> ast::Expr<'input> {
        let right = self.parse_expr(Precedence::ImpliedMul);
        ast::Expr::Binary(
            ast::BinOp::ImpliedMul,
            Box::new(left),
            Box::new(right),
        )
    }
}
