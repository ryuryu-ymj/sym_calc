use super::*;

fn num(n: i32) -> Expr {
    Expr::Num(n)
}

fn sym(s: &str) -> Expr {
    Expr::Sym(s.to_string())
}

#[test]
fn test_expr_add() {
    let tests = [
        (num(1) + num(2), num(3)),
        (sym("y") + sym("x"), sym("x") + sym("y")),
        (sym("x") + sym("x"), num(2) * sym("x")),
    ];

    for (input, expected) in tests {
        assert_eq!(input, expected);
    }
}

#[test]
fn test_expr_mul() {
    let tests = [
        (num(3) * num(2), num(6)),
        (sym("y") * sym("x"), sym("x") * sym("y")),
        (sym("x") * sym("x"), Expr::pow(sym("x"), num(2))),
    ];

    for (input, expected) in tests {
        assert_eq!(input, expected);
    }
}
