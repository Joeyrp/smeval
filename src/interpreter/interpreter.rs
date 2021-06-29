

use super::nodes::{Node};
use super::tokens::Value;

pub struct Interpreter
{
    tree_root: Node,
    pub result: Value
}

pub struct InterpreterError
{
   pub msg: String
}

impl Interpreter
{
    pub fn new() -> Interpreter
    {
        Interpreter { tree_root: Node::Nil, result: Value::VOID }
    }

    pub fn run(&mut self, root: Node) -> Result<(), String>
    {
       self.tree_root = root;
       self.result = self.visit(self.tree_root.clone())?;

        Ok(())
    }

    fn visit(&mut self, node: Node) -> Result<Value, String>
    {
        let result;
        // print!("NODE TYPE: {:#?}", node);
        match node 
        {
            Node::Number(x) => { return Ok (x)},
            Node::Multiply(l, r) => { result = self.visit_mult(*l, *r)?; },
            Node::Divide(l, r) => { result = self.visit_div(*l, *r)?; },
            Node::Modulus(l, r) => { result = self.visit_mod(*l, *r)?; },
            Node::Add(l, r) => { result = self.visit_add(*l, *r)?; },
            Node::Subtract(l, r) => { result = self.visit_sub(*l, *r)?; },
            Node::Plus(x) => { result = self.visit_plus(*x)?; }
            Node::Minus(x) => { result = self.visit_minus(*x)?; }
            _ => { return Err ( format!("Interpreter::visit method not yet implemented for this node type: {:#?}", node) ); }
        };

        Ok(self.visit(result)?)        
    }

    fn visit_mult(&mut self, left: Node, right: Node) -> Result<Node, String>
    {
        let lvalue = self.visit(left)?;
        let rvalue = self.visit(right)?;

        // Need to do type checking so that the math operation can be performed.
        // Hopefully I can find a better way to do this.
        // This isn't so bad now but if I wanted to add unsigned values
        // and/or value of different sizes this would become huge and messy.
        // I'm thinking about overriding the math traits for the Value type which
        // I think will allow me to hide this type checking code.
        // Probably won't do it for this project though because I think I'll stick
        // with just INT and FLOAT types
        match lvalue
        {
            Value::INT(x) =>
            {
                match rvalue
                {
                    Value::INT(y) => return Ok(Node::Number(Value::INT(x * y))),
                    Value::FLOAT(y) => return Ok(Node::Number(Value::FLOAT((x as f32) * y))),
                    _ =>  return Err(format!("Invalid type on right value {:#?}", rvalue ))
                }
            },

            Value::FLOAT(x) =>
            {
                match rvalue
                {
                    Value::INT(y) => return Ok(Node::Number(Value::FLOAT(x * (y as f32)))),
                    Value::FLOAT(y) => return Ok(Node::Number(Value::FLOAT(x * y))),
                    _ =>  return Err(format!("Invalid type on right value{:#?}", rvalue ))
                }
            },

            _ => { return Err(format!("Invalid type on left value {:#?}", lvalue)); }
        }


       // Ok(Node::Number(lresult * rresult))
    }

    fn visit_div(&mut self, left: Node, right: Node) -> Result<Node, String>
    {
        let lvalue = self.visit(left)?;
        let rvalue = self.visit(right)?;

        
        // Catch a divide by zero error
        if rvalue.is_zero()
        {
            return Err( String::from("Attempt to divide by zero!") );
        }

        match lvalue
        {
            Value::INT(x) =>
            {
                match rvalue
                {
                    Value::INT(y) => return Ok(Node::Number(Value::INT(x / y))),
                    Value::FLOAT(y) => return Ok(Node::Number(Value::FLOAT((x as f32) / y))),
                    _ =>  return Err(format!("Invalid type on right value {:#?}", rvalue ))
                }
            },

            Value::FLOAT(x) =>
            {
                match rvalue
                {
                    Value::INT(y) => return Ok(Node::Number(Value::FLOAT(x / (y as f32)))),
                    Value::FLOAT(y) => return Ok(Node::Number(Value::FLOAT(x / y))),
                    _ =>  return Err(format!("Invalid type on right value{:#?}", rvalue ))
                }
            },

            _ => { return Err(format!("Invalid type on left value {:#?}", lvalue)); }
        }


       // Ok(Node::Number(lresult / rresult))
    }

    fn visit_mod(&mut self, left: Node, right: Node) -> Result<Node, String>
    {
        let lvalue = self.visit(left)?;
        let rvalue = self.visit(right)?;

        
        // Catch a divide by zero error
        if rvalue.is_zero()
        {
            return Err( String::from("Attempt to divide by zero!") );
        }

        match lvalue
        {
            Value::INT(x) =>
            {
                match rvalue
                {
                    Value::INT(y) => return Ok(Node::Number(Value::INT(x % y))),
                    _ =>  return Err(format!("Invalid type on right value {:#?}", rvalue ))
                }
            },
            _ => { return Err(format!("Invalid type on left value {:#?}", lvalue)); }
        }


       // Ok(Node::Number(lresult / rresult))
    }

    fn visit_add(&mut self, left: Node, right: Node) -> Result<Node, String>
    {
        let lvalue = self.visit(left)?;
        let rvalue = self.visit(right)?;

        match lvalue
        {
            Value::INT(x) =>
            {
                match rvalue
                {
                    Value::INT(y) => return Ok(Node::Number(Value::INT(x + y))),
                    Value::FLOAT(y) => return Ok(Node::Number(Value::FLOAT((x as f32) + y))),
                    _ =>  return Err(format!("Invalid type on right value {:#?}", rvalue ))
                }
            },

            Value::FLOAT(x) =>
            {
                match rvalue
                {
                    Value::INT(y) => return Ok(Node::Number(Value::FLOAT(x + (y as f32)))),
                    Value::FLOAT(y) => return Ok(Node::Number(Value::FLOAT(x + y))),
                    _ =>  return Err(format!("Invalid type on right value{:#?}", rvalue ))
                }
            },

            _ => { return Err(format!("Invalid type on left value {:#?}", lvalue)); }
        }

        // Ok(Node::Number(lresult + rresult))
    }

    fn visit_sub(&mut self, left: Node, right: Node) -> Result<Node, String>
    {
        let lvalue = self.visit(left)?;
        let rvalue = self.visit(right)?;

        match lvalue
        {
            Value::INT(x) =>
            {
                match rvalue
                {
                    Value::INT(y) => return Ok(Node::Number(Value::INT(x - y))),
                    Value::FLOAT(y) => return Ok(Node::Number(Value::FLOAT((x as f32) - y))),
                    _ =>  return Err(format!("Invalid type on right value {:#?}", rvalue ))
                }
            },

            Value::FLOAT(x) =>
            {
                match rvalue
                {
                    Value::INT(y) => return Ok(Node::Number(Value::FLOAT(x - (y as f32)))),
                    Value::FLOAT(y) => return Ok(Node::Number(Value::FLOAT(x - y))),
                    _ =>  return Err(format!("Invalid type on right value{:#?}", rvalue ))
                }
            },

            _ => { return Err(format!("Invalid type on left value {:#?}", lvalue)); }
        }

       // Ok(Node::Number(lresult - rresult))
    }

    fn visit_plus(&mut self, node: Node) -> Result<Node, String>
    {
        let result = self.visit(node)?;

        match result
        {
            Value::INT(x) => Ok (Node::Number(Value::INT(x * 1))),
            Value::FLOAT(x) => Ok (Node::Number(Value::FLOAT(x * 1.0))),
            _ => Err (format!("Invalid type {:#?}", result))
        }

       // Ok(Node::Number(result * 1.0))
    }

    fn visit_minus(&mut self, node: Node) -> Result<Node, String>
    {
        let result = self.visit(node)?;

        match result
        {
            Value::INT(x) => Ok (Node::Number(Value::INT(x * -1))),
            Value::FLOAT(x) => Ok (Node::Number(Value::FLOAT(x * -1.0))),
            _ => Err (format!("Invalid type {:#?}", result))
        }
       
        // Ok(Node::Number(result * -1.0))
    }
}