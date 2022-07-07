pub enum TokenKind {
    Integer(i32), // literal
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