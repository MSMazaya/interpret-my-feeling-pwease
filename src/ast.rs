use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> Token;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}

pub struct Program {
    // TODO: this should be an array instead, should we just
    // use Statement as a struct?
    pub statements: dyn Statement,
}

impl Node for Program {
    fn token_literal(&self) -> Token {
        todo!()
    }
}
