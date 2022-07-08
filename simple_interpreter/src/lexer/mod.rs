#[derive(Clone, Debug)]
pub enum TokenKind {
    Integer(i32), // literal
    Float(f32),   // literal
    Bool(bool),   // literal
    OpAdd,        // ( 1 2 -- 3 )
    OpSub,        // ( 1 2 -- 1 ) top - bott
    OpMul,        // ( 2 3 -- 6 )
    OpDiv,        // ( 2 4 -- 2 ) top / bott
    OpPeek,       // ( 2 4 -- 2 4 ) puts 2
    OpSwap,       // ( 1 2 -- 2 1 )
    OpDup,        // ( 1 2 -- 1 2 2 )
    OpDrop,       // ( 1 2 -- 1 )
    OpDump,       // ( 1 2 -- 1 ) puts 2
    Proc {name: String, proc: Vec<TokenKind>},
    Call(String),
}

impl TokenKind {

    pub fn get_as_integer(self: &Self) -> Option<i32> {
        match self  {
            TokenKind::Integer(i) => Some(*i),
            _ => None
        }
    }

    /// Automatic Integer convertion to Float if needed
    pub fn get_as_float(self: &Self) -> Option<f32> {
        match self  {
            TokenKind::Float(f) => Some(*f),
            TokenKind::Integer(i) => Some(*i as f32),
            _ => None
        }
    }

    pub fn get_as_bool(self: &Self) -> Option<bool> {
        match self  {
            TokenKind::Bool(b) => Some(*b),
            _ => None
        }
    }
}