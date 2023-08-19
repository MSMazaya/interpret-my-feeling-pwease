pub mod tests;

#[derive(PartialEq, Debug)]
pub enum Token {
    ILLEGAL,
    EOF,

    // identifiers + literals
    IDENT(String),
    INT(i32),

    // operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    EQ,
    NOTEQ,

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
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

pub struct Lexer {
    pub input: String,
    pub position: usize,
    pub read_position: usize,
    pub ch: u8,
}

pub fn is_letter(ch: u8) -> bool {
    ch.is_ascii_alphabetic() || ch == b'_'
}

pub fn is_digit(ch: u8) -> bool {
    ch.is_ascii_digit()
}

pub fn lookup_ident(ident: String) -> Token {
    match ident.as_str() {
        "pungsi" => Token::FUNCTION,
        "ieu" => Token::LET,
        "leres" => Token::TRUE,
        "lepat" => Token::FALSE,
        "lamun" => Token::IF,
        "salianna" => Token::ELSE,
        "pasihan" => Token::RETURN,
        _ => Token::IDENT(ident),
    }
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

    pub fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input.as_bytes()[self.read_position]
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.eat_whitespace();
        let tok = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::EQ
                } else {
                    Token::ASSIGN
                }
            }
            b'+' => Token::PLUS,
            b';' => Token::SEMICOLON,
            b'(' => Token::LPAREN,
            b')' => Token::RPAREN,
            b',' => Token::COMMA,
            b'{' => Token::LBRACE,
            b'}' => Token::RBRACE,
            b'-' => Token::MINUS,
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::NOTEQ
                } else {
                    Token::BANG
                }
            }
            b'*' => Token::ASTERISK,
            b'/' => Token::SLASH,
            b'<' => Token::LT,
            b'>' => Token::GT,
            0 => Token::EOF,
            _ => {
                if is_letter(self.ch) {
                    return lookup_ident(self.read_identifier());
                } else if is_digit(self.ch) {
                    return Token::INT(self.read_number());
                } else {
                    Token::ILLEGAL
                }
            }
        };
        self.read_char();
        tok
    }

    pub fn eat_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char();
        }
    }

    pub fn read_identifier(&mut self) -> String {
        let first_word_index = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        self.input[first_word_index..self.position].to_string()
    }

    pub fn read_number(&mut self) -> i32 {
        let first_word_index = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }
        self.input[first_word_index..self.position].parse().unwrap()
    }

    pub fn from(input: String) -> Self {
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
