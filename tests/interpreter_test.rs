#[cfg(test)]
mod tests {

    use simple_interpreter::interpreter;
    use simple_interpreter::lexer::TokenKind;

    #[test]
    fn test_simple_math() {
        let program: Vec<TokenKind> = vec![
            TokenKind::Integer(2),
            TokenKind::Integer(2),
            TokenKind::OpAdd,
            TokenKind::Integer(3),
            TokenKind::OpSub,
        ];

        let mut res_stack = interpreter::run_program(&program, &mut Vec::new());
        let last_stack_item = res_stack.pop();

        assert_eq!(-1, last_stack_item.expect("Stack is empty!"));
    }

    #[test]
    fn test_swap_op() {
        let program: Vec<TokenKind> = vec![
            TokenKind::Integer(1),
            TokenKind::Integer(2),
            TokenKind::OpSwap,
        ];

        let mut res_stack = interpreter::run_program(&program, &mut Vec::new());
        let first_stack_item = res_stack.pop();

        assert_eq!(1, first_stack_item.expect("Stack is empty!"));
    }

    #[test]
    fn test_dup_op() {
        let program: Vec<TokenKind> = vec![
            TokenKind::Integer(2),
            TokenKind::OpDup,
        ];

        let mut res_stack = interpreter::run_program(&program, &mut Vec::new());
        assert_eq!(2, res_stack.len(), "Unexpected stack length");

        let first_stack_item = res_stack.pop();
        assert_eq!(2, first_stack_item.expect("Stack is empty!"));
    }

    #[test]
    fn test_drop_op() {
        let program: Vec<TokenKind> = vec![
            TokenKind::Integer(1),
            TokenKind::OpDrop,
        ];

        let res_stack = interpreter::run_program(&program, &mut Vec::new());

        assert!(res_stack.is_empty());
    }

    #[test]
    fn test_peek_op() {
        let program: Vec<TokenKind> = vec![
            TokenKind::Integer(1),
            TokenKind::OpPeek,
        ];

        let res_stack = interpreter::run_program(&program, &mut Vec::new());

        assert!(res_stack.contains(&1));
    }

    #[test]
    fn test_proc() {
        let program: Vec<TokenKind> = vec![
            TokenKind::Proc {
                name: String::from("sqrt"),
                proc: vec![
                    TokenKind::OpDup,
                    TokenKind::OpMul
                ]
            },
            TokenKind::Integer(3),
            TokenKind::Call(String::from("sqrt")),
        ];

        let mut res_stack = interpreter::run_program(&program, &mut Vec::new());
        let first_stack_item = res_stack.pop();

        assert_eq!(9, first_stack_item.expect("Stack is empty!"));
    }

    #[test]
    fn test_complex_program() {
        let program: Vec<TokenKind> = vec![
            TokenKind::Proc {
                name: String::from("sum"),
                proc: vec![
                    TokenKind::Integer(1),
                    TokenKind::Integer(2),
                    TokenKind::OpAdd
                ]
            },
            TokenKind::Integer(2),
            TokenKind::Call(String::from("sum")),
            TokenKind::OpAdd,
            TokenKind::Integer(2),
            TokenKind::OpAdd,
            TokenKind::OpDump
        ];

        let res_stack = interpreter::run_program(&program, &mut Vec::new());
        assert!(res_stack.is_empty())
    }
}