#[cfg(test)]
mod tests {

    use simple_interpreter::interpreter;
    use simple_interpreter::lexer::TokenKind;

    #[test]
    fn test_equals() {
        let program: Vec<TokenKind> = vec![
            TokenKind::Integer(3),
            TokenKind::Integer(3),
            TokenKind::OpEquals,
        ];

        let mut res_stack = interpreter::run_program(&program, &mut Vec::new());
        let first_stack_item = res_stack.pop();

        assert!(first_stack_item.expect("Stack is empty!")
            .get_as_bool().expect("Boolean expected!"));

        let program: Vec<TokenKind> = vec![
            TokenKind::Float(3.0),
            TokenKind::Float(3.0),
            TokenKind::OpEquals,
        ];

        let mut res_stack = interpreter::run_program(&program, &mut Vec::new());
        let first_stack_item = res_stack.pop();

        assert!(first_stack_item.expect("Stack is empty!")
            .get_as_bool().expect("Boolean expected!"));
    }

    #[test]
    fn test_not_equals() {
        let program: Vec<TokenKind> = vec![
            TokenKind::Integer(3),
            TokenKind::Integer(4),
            TokenKind::OpNotEquals,
        ];

        let mut res_stack = interpreter::run_program(&program, &mut Vec::new());
        let first_stack_item = res_stack.pop();

        assert!(first_stack_item.expect("Stack is empty!")
            .get_as_bool().expect("Boolean expected!"));
    }

    #[test]
    fn test_less_than() {
        let program: Vec<TokenKind> = vec![
            TokenKind::Integer(4),
            TokenKind::Integer(3),
            TokenKind::OpLess,
        ];

        let mut res_stack = interpreter::run_program(&program, &mut Vec::new());
        let first_stack_item = res_stack.pop();

        assert!(first_stack_item.expect("Stack is empty!")
            .get_as_bool().expect("Boolean expected!"));
    }

    #[test]
    fn test_less_or_equals_than() {
        let program: Vec<TokenKind> = vec![
            TokenKind::Integer(3),
            TokenKind::Integer(3),
            TokenKind::OpLessOrEquals,
        ];

        let mut res_stack = interpreter::run_program(&program, &mut Vec::new());
        let first_stack_item = res_stack.pop();

        assert!(first_stack_item.expect("Stack is empty!")
            .get_as_bool().expect("Boolean expected!"));
    }

    #[test]
    fn test_greater_than() {
        let program: Vec<TokenKind> = vec![
            TokenKind::Integer(3),
            TokenKind::Integer(4),
            TokenKind::OpGreater,
        ];

        let mut res_stack = interpreter::run_program(&program, &mut Vec::new());
        let first_stack_item = res_stack.pop();

        assert!(first_stack_item.expect("Stack is empty!")
            .get_as_bool().expect("Boolean expected!"));
    }

    #[test]
    fn test_greater_or_equals_than() {
        let program: Vec<TokenKind> = vec![
            TokenKind::Integer(3),
            TokenKind::Integer(3),
            TokenKind::OpGreaterOrEquals,
        ];

        let mut res_stack = interpreter::run_program(&program, &mut Vec::new());
        let first_stack_item = res_stack.pop();

        assert!(first_stack_item.expect("Stack is empty!")
            .get_as_bool().expect("Boolean expected!"));
    }

}