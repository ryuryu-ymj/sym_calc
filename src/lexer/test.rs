use super::*;
use crate::token::Token;

#[test]
fn test_next_token() {
    let tests = [
        (
            "10+2*393",
            vec![
                Token::Num("10"),
                Token::Plus,
                Token::Num("2"),
                Token::Asterisk,
                Token::Num("393"),
            ],
        ),
        (
            "\t10  +  2 * 393",
            vec![
                Token::Num("10"),
                Token::Plus,
                Token::Num("2"),
                Token::Asterisk,
                Token::Num("393"),
            ],
        ),
        (
            "(10 - 2) / 393",
            vec![
                Token::Lparen,
                Token::Num("10"),
                Token::Minus,
                Token::Num("2"),
                Token::Rparen,
                Token::Slash,
                Token::Num("393"),
            ],
        ),
        (
            "ab c*de",
            vec![
                Token::Ident("a"),
                Token::Ident("b"),
                Token::Ident("c"),
                Token::Asterisk,
                Token::Ident("d"),
                Token::Ident("e"),
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
    ];

    for (input, expected) in tests {
        let mut l = Lexer::new(input);
        for e in expected {
            let tok = l.next_token();
            assert_eq!(tok, e);
        }
    }
}
