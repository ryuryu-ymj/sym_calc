use crate::token::Token;
use std::iter::Peekable;
use std::str::CharIndices;

#[cfg(test)]
mod test_lexer;

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
