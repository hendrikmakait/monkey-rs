use crate::token::Token;

trait Node {}

pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Expression {
    Identifier,
}

#[derive(Debug)]
pub enum Statement {
    Let { token: Token, name: Identifier, value: Option<Expression> },
}

pub struct Let {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<Expression>,
}

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl<'a> Program {
    pub fn new() -> Self {
        Program {
            statements: Vec::new()
        }
        
    }
}

impl Node for Program {}

impl Node for Expression {}

impl Node for Statement {}
