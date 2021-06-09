
use std::{io, io::Write};

fn main() 
{
    let mut input: String;
    let mut history = String::new();
    loop
    {
        input = String::new();
        print!("smeval> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        history += &input;

        // Debugging
        // println!("input: {}", input);
        // println!("history: {}", history);

        if input.trim() == "quit"
        {
            break;
        }
    }
}
