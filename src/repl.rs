use std::io;

use crate::{Lexer, Token};

const PROMPT: &'static str = ">>";

pub fn start() {
    println!(
        r#"
Sampurasun! 
Mangga dicobian kang/teh basa pemrogramannana
    "#
    );
    loop {
        let mut input = String::new();
        print!("{PROMPT} ");
        io::Write::flush(&mut io::stdout()).expect("flush failed!");
        match io::stdin().read_line(&mut input) {
            Err(e) => panic!("Failed to read_line: {e}"),
            _ => {}
        }
        let mut l = Lexer::from(input);
        let mut token = l.next_token();
        while token != Token::EOF {
            println!("{token:?}");
            token = l.next_token();
        }
    }
}
