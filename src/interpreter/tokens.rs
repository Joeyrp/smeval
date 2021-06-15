
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType
{
    NUM,
    PLUS,
    MINUS,
    MULT,
    DIV,
    LPAREN,
    RPAREN
}

#[derive(Debug, Copy, Clone)]
pub struct Token
{
    pub ttype: TokenType,
    pub csym: char,
    pub value: f32
}