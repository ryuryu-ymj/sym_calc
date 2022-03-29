use super::super::{lexer::Lexer, parser::Parser};
use super::*;

#[test]
fn test_eval_let_stmt() {
    let tests = [
        (r"\let x = 3", r"\let x = 3"),
        ("x", "3"),
        ("x + 2", "5"),
        ("x^x + x/x", "28"),
        (r"\let x = x*x", r"\let x = 9"),
        ("x", "9"),
        (r"\let y = -1/x", r"\let y = -1/9"),
        ("x + y", "80/9"),
    ];

    let mut env = Environment::new();
    for (input, expected) in tests {
        let l = Lexer::new(&input);
        let mut p = Parser::new(l);
        let stmt = p.parse_stmt();
        assert_eq!(eval_stmt(stmt, &mut env), expected);
    }
}
