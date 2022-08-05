pub mod lexer;
pub mod interpreter;

pub use lexer::TokenKind;


fn main() {
    let program: Vec<TokenKind> = vec![
        TokenKind::Integer(0),
        TokenKind::OpDup,
        TokenKind::OpDump,
        TokenKind::Integer(1),
        TokenKind::While(
            vec![
                TokenKind::Integer(20000),
                TokenKind::OpGreaterOrEquals,
            ],
            vec![
            TokenKind::OpDup,
            TokenKind::OpDump,
            TokenKind::OpDup,
            TokenKind::OpRot,
            TokenKind::OpAdd,
            ]
        )
    ];

    interpreter::run_program(&program, &mut Vec::new());
}