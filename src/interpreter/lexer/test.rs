use super::super::token::Token;
use super::*;

#[test]
fn test_next_token() {
    let tests = [
        (
            "10+2*393\n",
            vec![
                Token::Num("10"),
                Token::Plus,
                Token::Num("2"),
                Token::Star,
                Token::Num("393"),
                Token::LF,
            ],
        ),
        (
            "\t10  +  2 * 393\n",
            vec![
                Token::Num("10"),
                Token::Plus,
                Token::Num("2"),
                Token::Star,
                Token::Num("393"),
                Token::LF,
            ],
        ),
        (
            "(10 - 2) / 393\n",
            vec![
                Token::Lparen,
                Token::Num("10"),
                Token::Minus,
                Token::Num("2"),
                Token::Rparen,
                Token::Slash,
                Token::Num("393"),
                Token::LF,
            ],
        ),
        (
            "ab c*de\n",
            vec![
                Token::Ident("a"),
                Token::Ident("b"),
                Token::Ident("c"),
                Token::Star,
                Token::Ident("d"),
                Token::Ident("e"),
                Token::LF,
            ],
        ),
        (
            r"\sin(3\pi a)",
            vec![
                Token::Ident(r"\sin"),
                Token::Lparen,
                Token::Num("3"),
                Token::Ident(r"\pi"),
                Token::Ident("a"),
                Token::Rparen,
            ],
        ),
        (
            "x^2+y^2",
            vec![
                Token::Ident("x"),
                Token::Caret,
                Token::Num("2"),
                Token::Plus,
                Token::Ident("y"),
                Token::Caret,
                Token::Num("2"),
            ],
        ),
        (
            r"\let x = 3",
            vec![Token::Let, Token::Ident("x"), Token::Eq, Token::Num("3")],
        ),
    ];

    for (input, expected) in tests {
        let mut l = Lexer::new(input);
        let mut e = expected.into_iter();
        loop {
            let tok = l.next_token();
            if tok == Token::Eof {
                break;
            }
            assert_eq!(Some(tok), e.next());
        }
    }
}
