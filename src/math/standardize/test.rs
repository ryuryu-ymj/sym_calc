use super::*;
use crate::interpreter::{eval, lexer::Lexer, parser::Parser};

#[test]
fn test_standardize() {
    let tests = [
        ("1 + 2", "3"),
        ("1 - 2", "-1"),
        ("x + x", "(2 * x)"),
        ("x - x", "0"),
        ("yz + z + y - x", "((-1 * x) + y + z + (y * z))"),
        ("x * x", "(x ^ 2)"),
        ("zyx3", "(3 * x * y * z)"),
        ("xx + yyy", "((x ^ 2) + (y ^ 3))"),
        ("xy + yx", "(2 * x * y)"),
        ("x/x", "1"),
        ("xy/yx", "1"),
    ];

    for (input, expected) in tests {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let expr = p.parse_expr_stmt();
        let mut expr = eval::eval_expr(expr);
        expr = standardize(expr);
        assert_eq!(format!("{:?}", expr), expected);
    }
}
