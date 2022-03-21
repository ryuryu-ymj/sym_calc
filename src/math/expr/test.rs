use crate::interpreter::{eval, lexer::Lexer, parser::Parser};

#[test]
fn test_add_expr() {
    let tests = [
        ("1 + 2 + 10", "13"),
        ("1 - 2", "-1"),
        ("x + x", "(2 * x)"),
        ("x + x + x", "(3 * x)"),
        ("x - x", "0"),
    ];

    for (input, expected) in tests {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let expr = p.parse_expr_stmt();
        let expr = eval::eval_expr(expr);
        assert_eq!(format!("{:?}", expr), expected);
    }
}

#[test]
fn test_mul_expr() {
    let tests = [
        ("1 * 2 * 3", "6"),
        ("10 * 3 * 0 * 6", "0"),
        ("x * x", "(x ^ 2)"),
        ("x * x * x", "(x ^ 3)"),
        ("zyx3", "(3 * x * y * z)"),
        ("zyx0", "0"),
        ("x / x", "1"),
        ("xy / yx", "1"),
    ];

    for (input, expected) in tests {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let expr = p.parse_expr_stmt();
        let expr = eval::eval_expr(expr);
        assert_eq!(format!("{:?}", expr), expected);
    }
}

#[test]
fn test_expr() {
    let tests = [
        ("xx + yyy", "((x ^ 2) + (y ^ 3))"),
        ("xy + yx", "(2 * x * y)"),
        ("yz + 3 + z + y - x", "((-1 * x) + y + z + (y * z) + 3)"),
    ];

    for (input, expected) in tests {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let expr = p.parse_expr_stmt();
        let expr = eval::eval_expr(expr);
        assert_eq!(format!("{:?}", expr), expected);
    }
}
