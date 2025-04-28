use crate::{lexer::Lexer, token::Token};
use crate::ast::{Identifier, Let, Program, Statement};

struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    current_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Self {
        Self {
            current_token: lexer.next_token(),
            peek_token: lexer.next_token(),
            lexer: lexer,
        }
    }

    fn next_token(&mut self) {
        self.current_token = std::mem::replace(&mut self.peek_token, self.lexer.next_token());
    }

    fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        while self.current_token != Token::EOF {
            let statement = self.parse_statement();

            if let Some(statement) = statement {
                program.statements.push(statement);
            }
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let token = self.current_token.clone();
        
        match self.peek_token {
            Token::Ident(_) => self.next_token(),
            _ => return None
        }

        let name = match self.current_token {
            Token::Ident(ref ident) => Some(Identifier { token: self.current_token.clone(), value: ident.clone()}),
            _ => None 
        };

        if !self.expect_peek(Token::Assign) {
            return None;
        }

        while !self.current_token_is(Token::Semicolon) && !self.current_token_is(Token::EOF) {
            self.next_token();
        }

        name.map(|n| {
            Statement::Let { token: token, name: n, value: None}
        })
    }

    fn current_token_is(&self, token: Token) -> bool {
        self.current_token == token
    }

    fn peek_token_is(&self, token: Token) -> bool {
        self.peek_token == token
    }

    fn expect_peek(&mut self, token: Token) -> bool {
        let result = self.peek_token_is(token);
        if result {
            self.next_token();
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_let_statements() {
        let input = r"
let x = 5;
let y = 10;
let foobar = 838383;
";
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse_program();
        assert_eq!(program.statements.len(), 3);

        let expected_identifiers = ["x", "y", "foobar"];

        for (i, expected_identifier) in expected_identifiers.iter().enumerate() {
            assert_let_statement(&program.statements[i], expected_identifier);
        }
    }

    fn assert_let_statement(statement: &Statement, expected_name: &str) {
        match statement {
            Statement::Let { token,name, value} => {
                assert_eq!(token, &Token::Let);
                assert_eq!(&name.value, expected_name);
                match &name.token {
                    &Token::Ident(ref ident) => assert_eq!(ident, expected_name),
                    _ => panic!("Expected Token::Ident")
                    
                }
            },
            _ => panic!("Expected Statement::Let, got {:?}", statement),
        }
    }
}