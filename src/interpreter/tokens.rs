
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

#[derive(Debug, Copy, PartialEq, Clone)]
pub enum Value
{
    INT(i32),
    FLOAT(f32),
    VOID
}

impl Value
{
    pub fn is_zero(&self) -> bool
    {
        match self
        {
            Value::INT(x) => return *x == 0,
            Value::FLOAT(x) => return *x == 0.0,
            _ => false
        }
    }
}

impl std::fmt::Display for Value
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Value::INT(x) => write!(f, "{}", x),
            Value::FLOAT(x) => 
            {
                // FLOAT value should always display with at least 1 zero after the .
                let mut strx = format!("{}", x);
                if !strx.contains('.')
                {
                    strx += ".0";
                }
                
                write!(f, "{}", strx)
            },
            Value::VOID => write!(f, "void")
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Token
{
    pub ttype: TokenType,
    pub csym: char,
    pub value: Value
}