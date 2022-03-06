use super::*;
use crate::ast;

#[test]
fn test_parse_binary_expr() {
    let tests = [
        (
            "12+3-10",
            ast::Expr::Binary(
                ast::BinOp::Sub,
                Box::new(ast::Expr::Binary(
                    ast::BinOp::Add,
                    Box::new(ast::Expr::Num("12")),
                    Box::new(ast::Expr::Num("3")),
                )),
                Box::new(ast::Expr::Num("10")),
            ),
        ),
        (
            "12 + 3*9",
            ast::Expr::Binary(
                ast::BinOp::Add,
                Box::new(ast::Expr::Num("12")),
                Box::new(ast::Expr::Binary(
                    ast::BinOp::Mul,
                    Box::new(ast::Expr::Num("3")),
                    Box::new(ast::Expr::Num("9")),
                )),
            ),
        ),
        (
            "12/3 - 9",
            ast::Expr::Binary(
                ast::BinOp::Sub,
                Box::new(ast::Expr::Binary(
                    ast::BinOp::Div,
                    Box::new(ast::Expr::Num("12")),
                    Box::new(ast::Expr::Num("3")),
                )),
                Box::new(ast::Expr::Num("9")),
            ),
        ),
    ];

    for (input, expected) in tests {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        assert_eq!(p.parse_expr(0), expected);
    }
}
