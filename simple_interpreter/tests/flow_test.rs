#[cfg(test)]
mod tests {

    use simple_interpreter::interpreter;
    use simple_interpreter::lexer::TokenKind;

    #[test]
    fn test_while_loop() {
        let program: Vec<TokenKind> = vec![
            TokenKind::Integer(1),
            TokenKind::Integer(3),
            TokenKind::OpGreater,
            TokenKind::While(vec![
                TokenKind::OpDup,
                TokenKind::OpDump,
                TokenKind::Integer(1),
                TokenKind::OpAdd,
            ])
        ];

        let mut res_stack = interpreter::run_program(&program, &mut Vec::new());
        let last_stack_item = res_stack.pop();

        assert_eq!(3, last_stack_item.expect("Stack is empty").get_as_integer().unwrap());
    }

    #[test]
    fn test_while_loop_skip() {
        let program: Vec<TokenKind> = vec![
            TokenKind::Integer(5),
            TokenKind::Integer(3),
            TokenKind::OpGreater,
            TokenKind::While(vec![
                TokenKind::OpDup,
                TokenKind::OpDump,
                TokenKind::Integer(1),
                TokenKind::OpAdd,
            ])
        ];

        let mut res_stack = interpreter::run_program(&program, &mut Vec::new());
        let last_stack_item = res_stack.pop();

        assert_eq!(5, last_stack_item.expect("Stack is empty").get_as_integer().unwrap());
    }
}