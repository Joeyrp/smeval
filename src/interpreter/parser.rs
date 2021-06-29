
// LOW PRECEDENCE
// ---------------
// Expression -  Term [ADD/SUB] Term
// Term - Factor [MULT/DIV] Factor
// Factor - Left Parentheses, Number
//----------------
// HIGH PRECEDENCE



use std::{result::Result};
use super::tokens::{TokenType, Token};
use super::nodes::{Node};

pub struct Parser
{
    token_list: Vec<Token>,
    curr_token_idx: usize,
    curr_token: Token,
    pub node_tree: Box<Node>,
    eof: bool
}

impl Parser
{
    pub fn new(tokens: Vec<Token>) -> Parser
    {
        Parser { token_list: tokens.clone(), curr_token_idx: 0, curr_token: tokens[0], node_tree: Box::new(Node::Nil), eof: false }
    }

    pub fn run(&mut self) -> Result<(), String>
    {
        if self.token_list.len() < 1
        {
            return Err(String::from("Parser: No tokens to parse"));
        }

        self.node_tree = self.expr()?;

        Ok(())
      //  Err(String::from("Parser::run method not implemented yet"))
    }

    fn advance(&mut self)
    {
        self.curr_token_idx += 1;
        if self.curr_token_idx >= self.token_list.len()
        {
            self.eof = true;
        }
        else
        {
            self.curr_token = self.token_list[self.curr_token_idx];
        }
    }

    // This method checks for a factor
    // A factor could be a number or a left parenthesis
    // It can also be the unary plus/minus operators
    fn factor(&mut self) -> Result<Box<Node>, String>
    {
        
        // println!("Looking for FACTOR");
        match self.curr_token.ttype
        {
            TokenType::LPAREN =>
            {
               // println!("Found LPAREN");
                self.advance();
                let result = self.expr()?;

                if self.curr_token.ttype != TokenType::RPAREN
                {
                    return Err(format!("Syntax Error: Expected ')' but found {}", self.curr_token.csym));
                }

               // println!("Found RPAREN");
                self.advance();
                return Ok(result);
            }

            TokenType::NUM =>
            {
                let v = self.curr_token.value;
                self.advance();
                return Ok(Box::new(Node::Number(v)));
            }

            TokenType::PLUS => 
            {
                
               // println!("Creating PLUS node");
                self.advance(); 
                let value = self.factor()?;
                return Ok(Box::new(Node::Plus(Box::new(*value)))); 
            }

            TokenType::MINUS => 
            {
               // println!("Creating MINUS node");
                self.advance(); 
                let value = self.factor()?;
                return Ok(Box::new(Node::Minus(Box::new(*value)))); 
            }

            _ => { return Err(format!("Syntax Error: '{}'", self.curr_token.csym)); }
        }


        // Err(String::from("Parser::factor method not implemented yet"))
    }


    // This method looks for a term
    // A term is a factor multiplied or divided by another factor
    fn term (&mut self) -> Result<Box<Node>, String>
    {
        // println!("Looking for TERM");

        // Get the first factor of the term
        let mut result = self.factor()?;
        let mult_div = vec![TokenType::MULT, TokenType::DIV];

        while mult_div.contains(&self.curr_token.ttype)
        {        
            match self.curr_token.ttype
            {
                TokenType::MULT =>
                {
                   // println!("Creating MULTIPLY node");
                    self.advance();
                    result = Box::new(Node::Multiply(result, self.factor()?));
                }

                TokenType::DIV =>
                {
                    self.advance();
                    result = Box::new(Node::Divide(result, self.factor()?));
                }
    
                _ => { return Err(format!("Syntax Error: '{}'", self.curr_token.csym)); }
            }
        }

        return Ok(result);
        
      //  Err(String::from("Parser::term method not implemented yet"))
    }

    // This method looks for an expression
    // and expression is a term added or subtracted from another term
    fn expr(&mut self) -> Result<Box<Node>, String>
    {
        // println!("Looking for EXPR");

        // Get the first term of the expression
        let mut result = self.term()?;
        let add_sub = vec![TokenType::PLUS, TokenType::MINUS];

        while add_sub.contains(&self.curr_token.ttype)
        {        
            match self.curr_token.ttype
            {
                TokenType::PLUS =>
                {
                  //  println!("Creating ADD node");
                    self.advance();
                    result = Box::new(Node::Add(result, self.term()?));
                }

                TokenType::MINUS =>
                {
                    self.advance();
                    result = Box::new(Node::Subtract(result, self.term()?));
                }
    
                _ => { return Err(format!("Syntax Error: '{}'", self.curr_token.csym)); }
            }
        }

        return Ok(result);
    }
}