

use super::nodes::{Node};

// TODO: Add the parsed tree root as a member
pub struct Interpreter
{
    tree_root: Node,
    pub result: f32
}

pub struct InterpreterError
{
   pub msg: String
}

impl Interpreter
{
    pub fn new() -> Interpreter
    {
        Interpreter { tree_root: Node::Nil, result: 0.0 }
    }

    pub fn run(&mut self, root: Node) -> Result<(), String>
    {
       self.tree_root = root;
       self.result = self.visit(self.tree_root.clone())?;

        Ok(())
    }

    fn visit(&mut self, node: Node) -> Result<f32, String>
    {
        let result;
        match node 
        {
            Node::Number(x) => { return Ok (x)},
            Node::Multiply(l, r) => { result = self.visit_mult(*l, *r)?; },
            Node::Divide(l, r) => { result = self.visit_div(*l, *r)?; },
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
        let lresult = self.visit(left)?;
        let rresult = self.visit(right)?;

        Ok(Node::Number(lresult * rresult))
    }

    fn visit_div(&mut self, left: Node, right: Node) -> Result<Node, String>
    {
        let lresult = self.visit(left)?;
        let rresult = self.visit(right)?;

        if rresult == 0.0
        {
            return Err( String::from("Attempt to divide by zero!") );
        }

        Ok(Node::Number(lresult / rresult))
    }

    fn visit_add(&mut self, left: Node, right: Node) -> Result<Node, String>
    {
        let lresult = self.visit(left)?;
        let rresult = self.visit(right)?;

        Ok(Node::Number(lresult + rresult))
    }

    fn visit_sub(&mut self, left: Node, right: Node) -> Result<Node, String>
    {
        let lresult = self.visit(left)?;
        let rresult = self.visit(right)?;

        Ok(Node::Number(lresult - rresult))
    }

    fn visit_plus(&mut self, node: Node) -> Result<Node, String>
    {
        let result = self.visit(node)?;
        Ok(Node::Number(result * 1.0))
    }

    fn visit_minus(&mut self, node: Node) -> Result<Node, String>
    {
        let result = self.visit(node)?;
        Ok(Node::Number(result * -1.0))
    }
}