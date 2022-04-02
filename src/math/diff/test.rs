use super::super::utl_test::*;
use super::*;

#[test]
fn test_diff() {
    let tests = [
        ("3", "0"),
        ("x", "1"),
        ("x^2", "(2 * x)"),
        ("2x^3", "(6 * (x ^ 2))"),
    ];

    for (input, expected) in tests {
        let mut expr = parse_expr(input);
        expr = diff(expr, "x");
        assert_eq!(format!("{:?}", expr), expected);
    }
}
