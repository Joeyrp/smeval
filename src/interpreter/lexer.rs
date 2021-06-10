
use std::result::Result;
use super::tokens::{TokenType, Token};

pub struct LexerError
{
   pub msg: String
}

pub struct Lexer
{
    token_list: Vec<Token>
}

impl Lexer
{
    pub fn new() -> Lexer
    {
        Lexer { token_list: vec![] }
    }

    pub fn run(&mut self, input: &String) -> Result<(), LexerError>
    {
        Ok(())
    }
}