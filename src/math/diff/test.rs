use super::super::utl_test::*;

#[test]
fn test_diff() {
    let tests = [
        (r"\diff(3, x)", "0"),
        (r"\diff(x, x)", "1"),
        (r"\diff(x^2, x)", "(2 * x)"),
        (r"\diff(2x^3, x)", "(6 * (x ^ 2))"),
        (r"\diff(x, y)", r"\diff((x, y))"),
        (
            r"\diff(x^2 + xy, y)",
            r"(x + (2 * x * \diff((x, y))) + (y * \diff((x, y))))",
        ),
    ];

    for (input, expected) in tests {
        let expr = parse_expr(input);
        assert_eq!(format!("{:?}", expr), expected);
    }
}
