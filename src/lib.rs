
mod interpreter;
use interpreter::{lexer::Lexer, parser::Parser, interpreter::Interpreter};

pub fn evaluate(input: &str) -> ()
{
    let lexer = interpreter::lexer::Lexer::new();
}