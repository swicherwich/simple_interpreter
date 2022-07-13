use crate::lexer::TokenKind;
use std::collections::HashMap;

fn math_op(top_token: TokenKind, prev_token: TokenKind, op: TokenKind) -> TokenKind {
    match prev_token {
        // if 1st item is Float we can cast 2nd item and add
        TokenKind::Float(top) => {
            match op {
                TokenKind::OpAdd => {
                    let res = top + top_token.get_as_float().expect("Float type expected!");
                    return TokenKind::Float(res)
                }
                TokenKind::OpSub => {
                    let res = top - top_token.get_as_float().expect("Float type expected!");
                    return TokenKind::Float(res)
                }
                TokenKind::OpDiv => {
                    let res = top / top_token.get_as_float().expect("Float type expected!");
                    return TokenKind::Float(res)
                }
                TokenKind::OpMul => {
                    let res = top * top_token.get_as_float().expect("Float type expected!");
                    return TokenKind::Float(res)
                }
                _ => panic!("Unknown operation on data!")
            }
        }
        // if 1st item is integer we can add only if 2nd is integer too otherwise panic
        TokenKind::Integer(top) => {
            match top_token {
                // check if 2nd item is Float (is there need to panic)
                TokenKind::Float(_) => {
                    panic!("Cannot automatically cast Float to Integer. Element of type Integer expected!")
                }
                _ => { }
            }
            match op {
                TokenKind::OpAdd => {
                    let res = top + top_token.get_as_integer().expect("Float type expected!");
                    return TokenKind::Integer(res)
                }
                TokenKind::OpSub => {
                    let res = top - top_token.get_as_integer().expect("Float type expected!");
                    return TokenKind::Integer(res)
                }
                TokenKind::OpDiv => {
                    let res = top / top_token.get_as_integer().expect("Float type expected!");
                    return TokenKind::Integer(res)
                }
                TokenKind::OpMul => {
                    let res = top * top_token.get_as_integer().expect("Float type expected!");
                    return TokenKind::Integer(res)
                }
                _ => panic!("Unknown operation on data!")
            }
        }
        // if 1st item is not Float neither Integer
        _ => panic!("Type mismatch: integer or float expected! Got: {:#?}", prev_token)
    }
}

fn swap(stack: &mut Vec<TokenKind>) {
    let a = stack.pop().expect("Stack is empty!");
    let b = stack.pop().expect("Stack is empty!");
    stack.push(a);
    stack.push(b);
}

pub fn run_program(program: &Vec<TokenKind>, stack: &mut Vec<TokenKind>) -> Vec<TokenKind> {
    let mut methods: HashMap<&String, &Vec<TokenKind>> = HashMap::new();

    let mut ip: usize = 0;
    loop {
        if ip == program.len() {
            break;
        }

        let token = program.get(ip).unwrap();

        match token {
            TokenKind::Integer(i) => {
                stack.push(TokenKind::Integer(*i));
                ip += 1;
            }
            TokenKind::Float(f) => {
                stack.push(TokenKind::Float(*f));
                ip += 1;
            }
            TokenKind::Bool(b) => {
                stack.push(TokenKind::Bool(*b));
                ip += 1;
            }
            TokenKind::OpAdd => {
                let top_token = stack.pop().expect("Stack is empty!"); // 1st item on stack
                let prev_token = stack.pop().expect("Stack is empty!");  // 2nd item on stack
                let res_token = math_op(top_token, prev_token, TokenKind::OpAdd);
                stack.push(res_token);
                ip += 1;
            }
            TokenKind::OpSub => {
                let top_token = stack.pop().expect("Stack is empty!"); // 1st item on stack
                let prev_token = stack.pop().expect("Stack is empty!");  // 2nd item on stack
                let res_token = math_op(top_token, prev_token, TokenKind::OpSub);
                stack.push(res_token);
                ip += 1;
            }
            TokenKind::OpMul => {
                let top_token = stack.pop().expect("Stack is empty!"); // 1st item on stack
                let prev_token = stack.pop().expect("Stack is empty!");  // 2nd item on stack
                let res_token = math_op(top_token, prev_token, TokenKind::OpMul);
                stack.push(res_token);
                ip += 1;
            }
            TokenKind::OpDiv => {
                let top_token = stack.pop().expect("Stack is empty!"); // 1st item on stack
                let prev_token = stack.pop().expect("Stack is empty!");  // 2nd item on stack
                let res_token = math_op(top_token, prev_token, TokenKind::OpDiv);
                stack.push(res_token);
                ip += 1;
            }
            TokenKind::OpSwap => {
                swap(stack);
                ip += 1;
            }
            TokenKind::OpDup => {
                let a = stack.pop().expect("Stack is empty!");
                stack.push(a.clone());
                stack.push(a);
                ip += 1;
            }
            TokenKind::OpDrop => {
                stack.pop().expect("Stack is empty!");
                ip += 1;
            }
            TokenKind::OpPeek => {
                let a = stack.pop().expect("Stack is empty!");
                stack.push(a);
                ip += 1;
            }
            TokenKind::OpDump => {
                let a = stack.pop().expect("Stack is empty!");
                match a {
                    TokenKind::Integer(_) => println!("dump: {}", a.get_as_integer().expect("Integer expected")),
                    TokenKind::Float(_) => println!("dump: {}", a.get_as_float().expect("Integer expected")),
                    TokenKind::Bool(_) => println!("dump: {}", a.get_as_bool().expect("Integer expected")),
                    _ => panic!("Illegal operand for 'dupm'. Integer/Float/Bool expected.")
                }
                ip += 1;
            }
            TokenKind::Proc {  name, proc} => {
                methods.insert(name, proc);
                ip += 1;
            }
            TokenKind::Call(method_name) => {
                let program: &Vec<TokenKind> = methods.get(method_name).expect("No such method!");
                run_program(program, stack);
                ip += 1;
            }
            TokenKind::OpEquals => {
                let top = stack.pop().expect("Stack is empty!");
                let prev = stack.pop().expect("Stack is empty!");

                stack.push(prev.clone());
                stack.push(top.clone());

                if top == prev {
                    stack.push(TokenKind::Bool(true));
                } else {
                    stack.push(TokenKind::Bool(false));
                }
                ip += 1;
            }
            TokenKind::OpNotEquals => {
                let top = stack.pop().expect("Stack is empty!");
                let prev = stack.pop().expect("Stack is empty!");

                stack.push(prev.clone());
                stack.push(top.clone());

                if top != prev {
                    stack.push(TokenKind::Bool(true));
                } else {
                    stack.push(TokenKind::Bool(false));
                }
                ip += 1;
            }
            TokenKind::OpLess => {
                let top = stack.pop().expect("Stack is empty!");
                let prev = stack.pop().expect("Stack is empty!");

                stack.push(prev.clone());
                stack.push(top.clone());

                if top < prev {
                    stack.push(TokenKind::Bool(true));
                } else {
                    stack.push(TokenKind::Bool(false));
                }
                ip += 1;
            }
            TokenKind::OpLessOrEquals => {
                let top = stack.pop().expect("Stack is empty!");
                let prev = stack.pop().expect("Stack is empty!");

                stack.push(prev.clone());
                stack.push(top.clone());

                if top <= prev {
                    stack.push(TokenKind::Bool(true));
                } else {
                    stack.push(TokenKind::Bool(false));
                }
                ip += 1;
            }
            TokenKind::OpGreater => {
                let top = stack.pop().expect("Stack is empty!");
                let prev = stack.pop().expect("Stack is empty!");

                stack.push(prev.clone());
                stack.push(top.clone());

                if top > prev {
                    stack.push(TokenKind::Bool(true));
                } else {
                    stack.push(TokenKind::Bool(false));
                }
                ip += 1;
            }
            TokenKind::OpGreaterOrEquals => {
                let top = stack.pop().expect("Stack is empty!");
                let prev = stack.pop().expect("Stack is empty!");

                stack.push(prev.clone());
                stack.push(top.clone());

                if top >= prev {
                    stack.push(TokenKind::Bool(true));
                } else {
                    stack.push(TokenKind::Bool(false));
                }
                ip += 1;
            }
            TokenKind::While(program) => {
                let top = stack.pop().expect("Stack is empty!");

                if top.get_as_bool().unwrap() == true {
                    swap(stack);
                    run_program(program, stack);
                    swap(stack);
                    ip -= 1;
                } else {
                    // 2nd operand for comparison dropped
                    stack.pop();
                    ip += 1;
                }
            }

        }
    }

    stack.to_vec()
}
