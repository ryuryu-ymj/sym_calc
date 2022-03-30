use super::token::Token;
use std::iter::Peekable;
use std::str::CharIndices;

#[cfg(test)]
mod test;

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

    pub fn next_token(&mut self) -> Token<'input> {
        loop {
            match self.chars.next() {
                None => return Token::Eof,
                Some((i, c)) => match c {
                    ' ' | '\t' => continue,
                    '\n' => return Token::LF,
                    '+' => return Token::Plus,
                    '-' => return Token::Minus,
                    '*' => return Token::Star,
                    '/' => return Token::Slash,
                    '^' => return Token::Caret,
                    '(' => return Token::Lparen,
                    ')' => return Token::Rparen,
                    '=' => return Token::Eq,
                    '0'..='9' => return self.read_number(i),
                    '\\' => return self.read_word(i),
                    'a'..='z' | 'A'..='Z' => {
                        return Token::Ident(&self.input[i..i + 1])
                    }
                    _ => return Token::Illegal,
                },
            }
        }
    }

    fn read_number(&mut self, pos: usize) -> Token<'input> {
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

    fn read_word(&mut self, pos: usize) -> Token<'input> {
        loop {
            match self.chars.peek() {
                None => return Lexer::word_to_token(&self.input[pos..]),
                Some((j, c)) => match *c {
                    'a'..='z' | 'A'..='Z' => {
                        self.chars.next();
                    }
                    _ => {
                        return Lexer::word_to_token(&self.input[pos..*j]);
                    }
                },
            }
        }
    }

    fn word_to_token(s: &'input str) -> Token<'input> {
        if s == "\\let" {
            Token::Let
        } else {
            Token::Ident(s)
        }
    }
}
