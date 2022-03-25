use super::*;

#[test]
fn test_parse_binary_expr() {
    let tests = [
        ("12+3-10", "((12 + 3) - 10)"),
        ("12 + 3*9", "(12 + (3 * 9))"),
        ("12/3 - 9", "((12 / 3) - 9)"),
        ("a/\\pi - d", "((a / \\pi) - d)"),
        ("10^3*4", "((10 ^ 3) * 4)"),
        ("10^3^4", "(10 ^ (3 ^ 4))"),
    ];

    for (input, expected) in tests {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let expr = p.parse_stmt();
        assert_eq!(format!("{:?}", expr), expected);
    }
}

#[test]
fn test_parse_unary_expr() {
    let tests = [("-10", "(- 10)"), ("-b", "(- b)"), ("--10", "(- (- 10))")];

    for (input, expected) in tests {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let expr = p.parse_stmt();
        assert_eq!(format!("{:?}", expr), expected);
    }
}

#[test]
fn test_parse_grouped_expr() {
    let tests = [("(10)", "10"), ("(10 + 7) * 100", "((10 + 7) * 100)")];

    for (input, expected) in tests {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let expr = p.parse_stmt();
        assert_eq!(format!("{:?}", expr), expected);
    }
}

#[test]
fn test_parse_implied_mul_expr() {
    let tests = [
        ("abc", "((a im b) im c)"),
        ("3\\pi a/2b", "(((3 im \\pi) im a) / (2 im b))"),
    ];

    for (input, expected) in tests {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let expr = p.parse_stmt();
        assert_eq!(format!("{:?}", expr), expected);
    }
}

#[test]
fn test_parse_expr() {
    let tests = [
        ("10 + -7", "(10 + (- 7))"),
        ("--7 * 10", "((- (- 7)) * 10)"),
        ("-2x", "(- (2 im x))"),
        ("-x^2", "(- (x ^ 2))"),
        ("x^2y", "((x ^ 2) im y)"),
    ];

    for (input, expected) in tests {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let expr = p.parse_stmt();
        assert_eq!(format!("{:?}", expr), expected);
    }
}
