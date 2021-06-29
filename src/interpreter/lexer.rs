
use std::{str, result::Result};
use super::tokens::{TokenType, Value, Token};

// pub struct LexerError
// {
//    pub msg: String
// }

pub struct Lexer
{
    pub token_list: Vec<Token>,
    curr_index: usize,
    curr_char: char,
    raw_code: Vec<u8>,
    white_space: Vec<char>,
    numbers: Vec<char>,
    eof: bool
}

impl Lexer
{
    pub fn new() -> Lexer
    {
        Lexer { token_list: vec![], curr_index: 0, curr_char: ' ', 
                raw_code: vec![], white_space: vec![' ', '\t', '\n', '\r'], 
                numbers: vec!['0', '1', '2', '3', '4', '5', '6' ,'7' ,'8', '9', '.'],  eof: false }
    }

    // Run the Lexer on a given chunk of code as a String
    pub fn run(&mut self, input: &str) -> Result<(), String>
    {
        self.raw_code = String::from(input).into_bytes();
        self.curr_index = 0;
        self.curr_char = self.raw_code[self.curr_index] as char;
        self.token_list.clear();
        self.eof = false;

        while !self.eof
        {
            if self.white_space.contains(&self.curr_char)
            {
                self.advance();
            }
            else if self.numbers.contains(&self.curr_char)
            {
                let t = self.build_num()?;
                self.token_list.push(t);
              //  self.advance();
            }
            else if self.curr_char == '+'
            {
                let t = Token { ttype: TokenType::PLUS, csym: '+', value: Value::VOID };
                self.token_list.push(t);
                self.advance();
            }
            else if self.curr_char == '-'
            {
                let t = Token { ttype: TokenType::MINUS, csym: '-', value: Value::VOID };
                self.token_list.push(t);
                self.advance();
            }
            else if self.curr_char == '*'
            {
                let t = Token { ttype: TokenType::MULT, csym: '*', value: Value::VOID };
                self.token_list.push(t);
                self.advance();
            }
            else if self.curr_char == '/'
            {
                let t = Token { ttype: TokenType::DIV, csym: '/', value: Value::VOID };
                self.token_list.push(t);
                self.advance();
            }
            else if self.curr_char == '('
            {
                let t = Token { ttype: TokenType::LPAREN, csym: '(', value: Value::VOID };
                self.token_list.push(t);
                self.advance();
            }
            else if self.curr_char == ')'
            {
                let t = Token { ttype: TokenType::RPAREN, csym: ')', value: Value::VOID };
                self.token_list.push(t);
                self.advance();
            }
            else
            {
                return Err( format!("Illegal character: {}", self.curr_char) );
            }
        }

        // Err( "Lexer::run method not yet implemented" )

        Ok(())
    }

    // Move to the next char/byte of the raw code
    fn advance(&mut self) -> ()
    {
        self.curr_index += 1;

        if self.curr_index >= self.raw_code.len()
        {
            self.eof = true;
            self.curr_char = 0 as char;
        } 
        else
        {
            self.curr_char = self.raw_code[self.curr_index] as char;
        }
    }

    // Extract a Number Token from the code
    fn build_num(&mut self) -> Result<Token, &str>
    {
        let mut num: Vec<u8> = Vec::new();
        let mut radix_count = 0;

        while self.numbers.contains(&self.curr_char) && !self.eof
        {
            // A number can only have 1 decimal point
            // So we need to stop reading characters if we hit a second one
            if self.curr_char == '.'
            {
                radix_count += 1;
                if radix_count > 1
                {
                    break;
                }
            }

            num.push(self.curr_char as u8);
            self.advance();
        }

        // Don't try to parse the number if it's empty or just a single decimal point
        if num.len() > 0 || (num.len() == 1 && num[0] != '.' as u8)
        {
            // Convert to either INT or FLOAT, make the Token and return it
            let final_num;

            if num.contains(&('.' as u8)) 
            {
               final_num = Value::FLOAT(str::from_utf8(&num).unwrap().parse::<f32>().unwrap());
            }
            else
            {
                final_num = Value::INT(str::from_utf8(&num).unwrap().parse::<i32>().unwrap());
            }

            //let num: f32 = str::from_utf8(&num).unwrap().parse::<f32>().unwrap();
            let t = Token { ttype: TokenType::NUM, csym: '.', value: final_num };
            return Ok (t);
        }

     //  Err(LexerError { msg: String::from("Lexer::build_num - No number found")})
        Err( "Number was empty or was probably just a '.'" )
    }
}