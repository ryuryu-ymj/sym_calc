use super::super::utl_test::*;

#[test]
fn test_expr_ord() {
    let seq = [
        "0", "1", "1/2", "1/3", "3/2", "x", "y", "z", "x + y", "x + z", "-1",
        "-x", "2x", "3x", "2y",
    ];
    let seq = seq.map(parse_expr);
    let mut seq = seq.iter();
    while let (Some(e), Some(f)) = (seq.next(), seq.next()) {
        assert!(e < f);
    }
}

#[test]
fn test_add_expr() {
    let tests = [
        ("1 + 2 + 10", "13"),
        ("1/2 + 1/2", "1"),
        ("1/3 + 5/4", "19/12"),
        ("1/3 + 5/4 + 1/12", "5/3"),
        ("x + x", "(2 * x)"),
        ("x + x + x", "(3 * x)"),
        ("z + y + x", "(x + y + z)"),
    ];

    for (input, expected) in tests {
        let expr = parse_expr(input);
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
    ];

    for (input, expected) in tests {
        let expr = parse_expr(input);
        assert_eq!(format!("{:?}", expr), expected);
    }
}

#[test]
fn test_pow_expr() {
    let tests = [
        ("2 ^ 3", "8"),
        ("3 ^ 2 ^ 2", "81"),
        ("x * x", "(x ^ 2)"),
        ("x * x * x", "(x ^ 3)"),
        ("zyx3", "(3 * x * y * z)"),
        ("zyx0", "0"),
    ];

    for (input, expected) in tests {
        let expr = parse_expr(input);
        assert_eq!(format!("{:?}", expr), expected);
    }
}

#[test]
fn test_expr() {
    let tests = [
        ("1 - 2", "-1"),
        ("1/3 - 3/4", "-5/12"),
        ("x - x", "0"),
        ("x - 2x", "(-1 * x)"),
        ("1/2*x + 3/2*x", "(2 * x)"),
        ("x + y - x - y", "0"),
        ("xx + yyy", "((x ^ 2) + (y ^ 3))"),
        ("xy + yx", "(2 * x * y)"),
        ("yz + 3 + z + y - x", "((-1 * x) + y + z + (y * z) + 3)"),
        ("15/3", "5"),
        ("20/6*7/5", "14/3"),
        ("x / x", "1"),
        ("xy / yx", "1"),
        ("1 / xx / xx", "(x ^ -4)"),
        ("1/x + 3/x", "(4 * (x ^ -1))"),
    ];

    for (input, expected) in tests {
        let expr = parse_expr(input);
        assert_eq!(format!("{:?}", expr), expected);
    }
}

#[test]
fn test_vec_expr() {
    let tests = [
        ("(1 + 2, x + x, yy)", "(3, (2 * x), (y ^ 2))"),
        ("(1, 2) + (2, 3)", "(3, 5)"),
        (
            "(1, 2) + (2, 3, 4)",
            "unsupported operand: +: \\R^2 x \\R^3 -> ?",
        ),
        ("(1, 2) + 3", "unsupported operand"),
        ("(x, y) + (x, z)", "((2 * x), (y + z))"),
        ("(1, 2) * 3", "(3, 6)"),
        ("3(x, y)", "((3 * x), (3 * y))"),
        ("(1,2)(3,y)", "unsupported operand"),
        ("0^(3,y)", "unsupported operand"),
        ("(3,y)^0", "unsupported operand"),
    ];

    for (input, expected) in tests {
        let expr = parse_expr(input);
        assert_eq!(format!("{:?}", expr), expected);
    }
}
