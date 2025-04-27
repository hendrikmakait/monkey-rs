use std::io;

use crate::{lexer::Lexer, token::Token};

pub fn start<R: ?Sized, W: ?Sized>(reader: &mut R, writer: &mut W)
where
    R: io::BufRead,
    W: io::Write,
{
    let mut buffer = String::new();
    loop {
        write!(writer, ">> ").expect("Failed to prompt.");
        match reader.read_line(&mut buffer) {
            Ok(0) => return,
            Ok(_) => {
                let mut lexer = Lexer::new(buffer.as_str());
                let mut token = lexer.next_token();
                while token != Token::EOF {
                    writeln!(writer, "{:?}", token).expect("Failed to write token.");
                    token = lexer.next_token();
                }
                buffer.clear();
            }
            Err(err) => eprintln!("Encountered error while reading input: {}", err),
        }
    }
}
