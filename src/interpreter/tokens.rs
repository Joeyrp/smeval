
#[derive(Debug)]
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

#[derive(Debug)]
pub struct Token
{
    pub ttype: TokenType,
    pub value: f32
}