use std::str::Bytes;

pub mod tests;

#[derive(PartialEq, Debug)]
pub enum Token {
    ILLEGAL,
    EOF,

    // identifiers + literals
    IDENT,
    INT,

    // operators
    ASSIGN,
    PLUS,

    // delimeters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // keywords
    FUNCTION,
    LET,
}

pub struct Lexer {
    pub input: String,
    pub position: usize,
    pub read_position: usize,
    pub ch: u8,
}

impl Lexer {
    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let tok: Token;
        match self.ch {
            b'=' => tok = Token::ASSIGN,
            b'+' => tok = Token::PLUS,
            b';' => tok = Token::SEMICOLON,
            b'(' => tok = Token::LPAREN,
            b')' => tok = Token::RPAREN,
            b',' => tok = Token::COMMA,
            b'{' => tok = Token::LBRACE,
            b'}' => tok = Token::RBRACE,
            _ => {
                print!("ch: {:?}", self.ch);
                tok = Token::EOF
            }
        }
        self.read_char();
        tok
    }

    #[allow(non_snake_case)]
    pub fn New(input: String) -> Self {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        l.read_char();
        l
    }
}

fn main() {
    println!("Hello, world!");
}
