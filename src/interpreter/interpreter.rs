
// TODO: Add the parsed tree root as a member
pub struct Interpreter
{
    
}

pub struct InterpreterError
{
   pub msg: String
}

impl Interpreter
{
    pub fn new() -> Interpreter
    {
        Interpreter { }
    }

    pub fn run(&mut self) -> Result<(), InterpreterError>
    {
        Err(InterpreterError { msg: String::from("Lexer::run method not yet implemented")})

        // Ok(())
    }
}