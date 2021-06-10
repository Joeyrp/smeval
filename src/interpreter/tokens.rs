

pub enum TokenType
{
    NUM,
    PLUS,
    MINUS,
    MULT,
    DIV
}

pub struct Token
{
    pub ttype: TokenType,
    pub value: i32
}