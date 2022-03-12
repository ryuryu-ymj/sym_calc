use super::*;
use lexer::Lexer;
use parser::Parser;

#[test]
fn test_eval_expr() {
    let tests = [
        ("1", "1"),
        ("x", "x"),
        ("x + y + z", "(x + y + z)"),
        ("x * y * z", "(x * y * z)"),
        ("xyz", "(x * y * z)"),
        ("12+3-10", "(12 + 3 + (-1 * 10))"),
        ("12+3/10", "(12 + (3 * (10 ^ -1)))"),
    ];

    for (input, expected) in tests {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let expr = p.parse_expr_stmt();
        let expr = eval_expr(expr);
        assert_eq!(format!("{:?}", expr), expected);
    }
}
