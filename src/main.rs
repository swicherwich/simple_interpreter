pub mod lexer;
pub mod interpreter;

pub use lexer::TokenKind;


fn main() {
    let program: Vec<TokenKind> = vec![
        TokenKind::Proc {
            name: String::from("sum"),
            proc: vec![
                TokenKind::Integer(1),
                TokenKind::Integer(2),
                TokenKind::OpAdd
        ]},
        TokenKind::Integer(2),
        TokenKind::Call(String::from("sum")),
        TokenKind::OpAdd,
        TokenKind::Integer(2),
        TokenKind::OpAdd,
        TokenKind::OpDump
    ];

    // sum:
    //      push 1
    //      push 2
    //      plus
    //      return
    // push 2
    // call sum
    // plus
    // push 2
    // plus
    // print

    interpreter::run_program(&program, &mut Vec::new());
}