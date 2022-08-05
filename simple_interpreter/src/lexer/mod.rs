use std::cmp::Ordering;

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
    OpRot,        // (1 2 3 -- 3 1 2)
    OpEquals,
    OpNotEquals,
    OpLess,
    OpLessOrEquals,
    OpGreater,
    OpGreaterOrEquals,
    Proc {name: String, proc: Vec<TokenKind>},
    Call(String),
    While(Vec<TokenKind>, Vec<TokenKind>)
}

enum LogicOpType {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

fn perform_logic(self_: &TokenKind, other: &TokenKind, logic_op_type: LogicOpType) -> bool{
    match *self_ {
        TokenKind::Integer(self_i) => {
            match *other {
                TokenKind::Integer(other_i) => {
                    return match logic_op_type {
                        LogicOpType::Eq => self_i == other_i,
                        LogicOpType::Ne => self_i != other_i,
                        LogicOpType::Lt => self_i < other_i,
                        LogicOpType::Le => self_i <= other_i,
                        LogicOpType::Gt => self_i > other_i,
                        LogicOpType::Ge => self_i >= other_i
                    }
                    // self_i == other_i
                }
                _ => panic!("Illegal type check! Integer expected!")
            }
        }
        TokenKind::Float(self_f) => {
            match *other {
                TokenKind::Float(other_f) => {
                    return match logic_op_type {
                        LogicOpType::Eq => self_f == other_f,
                        LogicOpType::Ne => self_f != other_f,
                        LogicOpType::Lt => self_f < other_f,
                        LogicOpType::Le => self_f <= other_f,
                        LogicOpType::Gt => self_f > other_f,
                        LogicOpType::Ge => self_f >= other_f
                    }
                }
                TokenKind::Integer(other_i) => {
                    return match logic_op_type {
                        LogicOpType::Eq => self_f == other_i as f32,
                        LogicOpType::Ne => self_f != other_i as f32,
                        LogicOpType::Lt => self_f < other_i as f32,
                        LogicOpType::Le => self_f <= other_i as f32,
                        LogicOpType::Gt => self_f > other_i as f32,
                        LogicOpType::Ge => self_f >= other_i as f32
                    }
                }
                _ => panic!("Illegal type check! Integer or Float expected!")
            }
        }
        _ => panic!("Illegal type check!")
    }
}

impl PartialEq for TokenKind {

    fn eq(&self, other: &TokenKind) -> bool {
        perform_logic(self, other, LogicOpType::Eq)
    }

    fn ne(&self, other: &Self) -> bool {
        perform_logic(self, other, LogicOpType::Ne)
    }
}

impl PartialOrd for TokenKind {

    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        todo!()
    }

    fn lt(&self, other: &Self) -> bool {
        perform_logic(self, other, LogicOpType::Lt)
    }

    fn le(&self, other: &Self) -> bool {
        perform_logic(self, other, LogicOpType::Le)
    }

    fn gt(&self, other: &Self) -> bool {
        perform_logic(self, other, LogicOpType::Gt)
    }

    fn ge(&self, other: &Self) -> bool {
        perform_logic(self, other, LogicOpType::Ge)
    }
}

impl TokenKind {

    pub fn get_as_integer(self: &Self) -> Option<i32> {
        match self  {
            TokenKind::Integer(i) => Some(*i),
            _ => None
        }
    }

    /// Automatic Integer conversion to Float if needed
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