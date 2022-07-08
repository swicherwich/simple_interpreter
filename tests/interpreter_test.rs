#[cfg(test)]
mod tests {

    use simple_interpreter::interpreter;
    use simple_interpreter::lexer::TokenKind;

    #[test]
    fn test_simple_autocast() {
        let program: Vec<TokenKind> = vec![
            TokenKind::Float(2.0),
            TokenKind::Integer(2),
            TokenKind::OpAdd
        ];

        let mut res_stack = interpreter::run_program(&program, &mut Vec::new());
        println!("{:?}", &res_stack);
        let last_stack_item = res_stack.pop();

        assert_eq!(4.0, last_stack_item.expect("").get_as_float().expect(""));
    }

    #[test]
    #[should_panic(expected = "Cannot automatically cast Float to Integer. Element of type Integer expected!")]
    fn test_autocast_panic() {
        let program: Vec<TokenKind> = vec![
            TokenKind::Integer(2),
            TokenKind::Float(2.0),
            TokenKind::OpAdd,
        ];

        interpreter::run_program(&program, &mut Vec::new());
    }

    #[test]
    #[should_panic(expected = "Type mismatch: integer or float expected!")]
    fn test_illegal_type_panic() {
        let program: Vec<TokenKind> = vec![
            TokenKind::Bool(true),
            TokenKind::Integer(2),
            TokenKind::OpAdd,
        ];

        interpreter::run_program(&program, &mut Vec::new());
    }

    #[test]
    fn test_simple_float_add() {
        let program: Vec<TokenKind> = vec![
            TokenKind::Float(2.0),
            TokenKind::Float(2.0),
            TokenKind::OpAdd,
        ];

        let mut res_stack = interpreter::run_program(&program, &mut Vec::new());
        println!("{:?}", &res_stack);
        let last_stack_item = res_stack.pop();

        assert_eq!(4.0, last_stack_item.expect("").get_as_float().expect(""));
    }

    #[test]
    fn test_simple_integer_add() {
        let program: Vec<TokenKind> = vec![
            TokenKind::Integer(2),
            TokenKind::Integer(2),
            TokenKind::OpAdd,
        ];

        let mut res_stack = interpreter::run_program(&program, &mut Vec::new());
        println!("{:?}", &res_stack);
        let last_stack_item = res_stack.pop();

        assert_eq!(4, last_stack_item.expect("").get_as_integer().expect(""));
    }

    #[test]
    fn test_swap_op() {
        let program: Vec<TokenKind> = vec![
            TokenKind::Bool(true),
            TokenKind::Integer(2),
            TokenKind::OpSwap,
        ];

        let mut res_stack = interpreter::run_program(&program, &mut Vec::new());
        let first_stack_item = res_stack.pop();

        assert_eq!(TokenKind::Bool(true).get_as_bool().expect("Bool expected!"),
                   first_stack_item.expect("Stack is empty!")
                       .get_as_bool().expect("Bool expected!"));
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
        assert_eq!(2, first_stack_item.expect("Stack is empty!").get_as_integer().expect("Integer expected!"));
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

        let mut res_stack = interpreter::run_program(&program, &mut Vec::new());
        let first_stack_item = res_stack.pop();

        assert_eq!(1, first_stack_item.expect("Item in stack expected")
            .get_as_integer().expect("Integer expected"));
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

        assert_eq!(9, first_stack_item.expect("Stack is empty!").get_as_integer().expect("Integer expected!"));
    }

    #[test]
    fn test_float_cat_proc() {
        let program: Vec<TokenKind> = vec![
            TokenKind::Proc {
                name: String::from("cast_float"),
                proc: vec![
                    TokenKind::Float(1.0),
                    TokenKind::OpSwap,
                    TokenKind::OpMul
                ]
            },
            TokenKind::Integer(2),
            TokenKind::Call(String::from("cast_float"))
        ];

        let mut res_stack = interpreter::run_program(&program, &mut Vec::new());
        let first_stack_item = res_stack.pop();

        assert_eq!(2.0, first_stack_item.expect("Stack is empty!").get_as_float().expect("Integer expected!"));
    }
}