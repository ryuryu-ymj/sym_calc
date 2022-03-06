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
        assert_eq!(p.parse_expr_stmt(), expected);
    }
}

#[test]
fn test_parse_unary_expr() {
    let tests = [
        (
            "-10",
            ast::Expr::Unary(ast::UnOp::Neg, Box::new(ast::Expr::Num("10"))),
        ),
        (
            "--10",
            ast::Expr::Unary(
                ast::UnOp::Neg,
                Box::new(ast::Expr::Unary(
                    ast::UnOp::Neg,
                    Box::new(ast::Expr::Num("10")),
                )),
            ),
        ),
    ];

    for (input, expected) in tests {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        assert_eq!(p.parse_expr_stmt(), expected);
    }
}

#[test]
fn test_parse_grouped_expr() {
    let tests = [
        ("(10)", ast::Expr::Num("10")),
        (
            "(10 + 7) * 100",
            ast::Expr::Binary(
                ast::BinOp::Mul,
                Box::new(ast::Expr::Binary(
                    ast::BinOp::Add,
                    Box::new(ast::Expr::Num("10")),
                    Box::new(ast::Expr::Num("7")),
                )),
                Box::new(ast::Expr::Num("100")),
            ),
        ),
    ];

    for (input, expected) in tests {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        assert_eq!(p.parse_expr_stmt(), expected);
    }
}

#[test]
fn test_parse_arithmetic_expr() {
    let tests = [
        (
            "10 + -7",
            ast::Expr::Binary(
                ast::BinOp::Add,
                Box::new(ast::Expr::Num("10")),
                Box::new(ast::Expr::Unary(
                    ast::UnOp::Neg,
                    Box::new(ast::Expr::Num("7")),
                )),
            ),
        ),
        (
            "--7 * 10",
            ast::Expr::Binary(
                ast::BinOp::Mul,
                Box::new(ast::Expr::Unary(
                    ast::UnOp::Neg,
                    Box::new(ast::Expr::Unary(
                        ast::UnOp::Neg,
                        Box::new(ast::Expr::Num("7")),
                    )),
                )),
                Box::new(ast::Expr::Num("10")),
            ),
        ),
    ];

    for (input, expected) in tests {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        assert_eq!(p.parse_expr_stmt(), expected);
    }
}
