#[cfg(test)]
#[test]
fn tokenizer() {
    use crate::{Lexer, Token};

    let input = "=+(){},;".to_string();

    let tokens = vec![
        Token::ASSIGN,
        Token::PLUS,
        Token::LPAREN,
        Token::RPAREN,
        Token::LBRACE,
        Token::RBRACE,
        Token::COMMA,
        Token::SEMICOLON,
        Token::EOF,
    ];

    let mut l = Lexer::New(input);

    for expected_token in tokens {
        let received_token = l.next_token();

        assert_eq!(expected_token, received_token);
    }
}
