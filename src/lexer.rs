use crate::token::Token;
use std::iter::Peekable;
use std::str::CharIndices;

pub struct Lexer<'input> {
    input: &'input str,
    chars: Peekable<CharIndices<'input>>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Lexer<'input> {
        Lexer {
            input,
            chars: input.char_indices().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        // let ch = match self.chars.next() {
        //     Some(ch) => ch,
        //     None => return Token::EOF,
        // };
        loop {
            match self.chars.next() {
                None => return None,
                Some((i, c)) => match c {
                    ' ' | '\t' => continue,
                    '+' => return Some(Token::Plus),
                    '-' => return Some(Token::Minus),
                    '*' => return Some(Token::Asterisk),
                    '/' => return Some(Token::Slash),
                    '(' => return Some(Token::Lparen),
                    ')' => return Some(Token::Rparen),
                    '0'..='9' => return Some(self.read_number(i)),
                    '\\' => return Some(self.read_identifier(i)),
                    'a'..='z' | 'A'..='Z' => {
                        return Some(Token::Ident(&self.input[i..i + 1]))
                    }
                    _ => return Some(Token::Illegal),
                },
            }
        }
    }

    fn read_number(&mut self, pos: usize) -> Token {
        loop {
            match self.chars.peek() {
                None => return Token::Num(&self.input[pos..]),
                Some((j, c)) => {
                    if *c >= '0' && *c <= '9' {
                        self.chars.next();
                    } else {
                        return Token::Num(&self.input[pos..*j]);
                    }
                }
            }
        }
    }

    fn read_identifier(&mut self, pos: usize) -> Token {
        loop {
            match self.chars.peek() {
                None => return Token::Ident(&self.input[pos..]),
                Some((j, c)) => match *c {
                    'a'..='z' | 'A'..='Z' => {
                        self.chars.next();
                    }
                    _ => {
                        return Token::Ident(&self.input[pos..*j]);
                    }
                },
            }
        }
    }
}

#[cfg(test)]
mod text_lexer {
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
                assert_eq!(tok.unwrap(), e);
            }
        }
    }
}