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
        match self.chars.next() {
            None => None,
            Some((i, c)) => match c {
                '+' => Some(Token::Plus),
                '*' => Some(Token::Asterisk),
                '0'..='9' => Some(self.read_number(i)),
                _ => Some(Token::Illegal),
            },
        }
    }

    fn read_number(&mut self, i: usize) -> Token {
        loop {
            match self.chars.peek() {
                None => return Token::Num(&self.input[i..]),
                Some((j, c)) => {
                    if *c >= '0' && *c <= '9' {
                        self.chars.next();
                    } else {
                        return Token::Num(&self.input[i..*j]);
                    }
                }
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
        let input = r"10+2*393";
        let expected = [
            Token::Num("10"),
            Token::Plus,
            Token::Num("2"),
            Token::Asterisk,
            Token::Num("393"),
        ];

        let mut l = Lexer::new(input);
        for e in expected {
            let tok = l.next_token();
            assert_eq!(tok.unwrap(), e);
        }
    }
}
