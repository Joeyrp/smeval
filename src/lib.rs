
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
    // println!("TOKENS: \n{:#?}", lexer.token_list);

    let mut parser = Parser::new(lexer.token_list);

    match parser.run()
    {
        Err(msg) => println!("Parser Error: {}", msg),
        _ => ()
    }

    // DEBUG: Print the parsed node tree for verification
    println!("NODE TREE: \n{:#?}", parser.node_tree);
}