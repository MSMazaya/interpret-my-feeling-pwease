pub mod lexer;
pub mod repl;
pub mod tests;
pub mod token;

fn main() {
    repl::start();
}
