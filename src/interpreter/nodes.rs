
#[derive(Debug, PartialEq, Clone)]
pub enum Node
{
    Number(f32),
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Plus(Box<Node>),
    Minus(Box<Node>),
    Nil
}