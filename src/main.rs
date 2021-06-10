
use std::{io, io::Write};

//use smeval::{interpreter::Interpreter};

fn main() 
{
    let mut input: String;
    let mut history = String::new();
   // let interp = Interpreter::new();

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

        smeval::evaluate(&input);
    }
}
