use super::*;
use crate::ast;

#[test]
fn test_parser() {
    let tests = [(
        "12+3",
        ast::Expr::Binary(
            ast::BinOp::Add,
            Box::new(ast::Expr::Num("12")),
            Box::new(ast::Expr::Num("3")),
        ),
    )];

    for (input, expected) in tests {
        let l = Lexer::new(input);
        let p = Parser::new(l);
        assert_eq!(p.parse_expr(), expected);
    }
}
