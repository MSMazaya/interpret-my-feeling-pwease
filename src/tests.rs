#[cfg(test)]
#[test]
fn test_delimeter_tokens() {
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

    let mut l = Lexer::from(input);

    for expected_token in tokens {
        let received_token = l.next_token();

        assert_eq!(expected_token, received_token);
    }
}

#[test]
fn test_complex_token_1() {
    use crate::{Lexer, Token};
    let input = r#"
    ieu lima = 5;
    ieu sapuluh = 10;

    ieu nambah = pungsi(eks,ye) {
        eks + ye;
    }

    ieu hasilna = nambah(lima, sapuluh);
    "#
    .to_string();

    let tokens = vec![
        Token::LET,
        Token::IDENT("lima".to_string()),
        Token::ASSIGN,
        Token::INT(5),
        Token::SEMICOLON,
        Token::LET,
        Token::IDENT("sapuluh".to_string()),
        Token::ASSIGN,
        Token::INT(10),
        Token::SEMICOLON,
        Token::LET,
        Token::IDENT("nambah".to_string()),
        Token::ASSIGN,
        Token::FUNCTION,
        Token::LPAREN,
        Token::IDENT("eks".to_string()),
        Token::COMMA,
        Token::IDENT("ye".to_string()),
        Token::RPAREN,
        Token::LBRACE,
        Token::IDENT("eks".to_string()),
        Token::PLUS,
        Token::IDENT("ye".to_string()),
        Token::SEMICOLON,
        Token::RBRACE,
        Token::LET,
        Token::IDENT("hasilna".to_string()),
        Token::ASSIGN,
        Token::IDENT("nambah".to_string()),
        Token::LPAREN,
        Token::IDENT("lima".to_string()),
        Token::COMMA,
        Token::IDENT("sapuluh".to_string()),
        Token::RPAREN,
        Token::SEMICOLON,
        Token::EOF,
    ];

    let mut l = Lexer::from(input);

    for expected_token in tokens {
        let received_token = l.next_token();

        assert_eq!(expected_token, received_token);
    }
}
