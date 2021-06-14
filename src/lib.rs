
mod interpreter;
use interpreter::{lexer::Lexer, parser::Parser, interpreter::Interpreter};

pub fn evaluate(input: &str) -> ()
{
    let mut lexer = Lexer::new();
    
    match lexer.run(input)
    {
        Err(msg) => println!("Lexer Error: {}", msg),
        _ => ()
    } 

    // DEBUG: Print the token list for verification
    println!("TOKENS: \n{:#?}", lexer.token_list);
}