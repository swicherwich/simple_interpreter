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

pub fn run_program(program: &Vec<TokenKind>, stack: &mut Vec<TokenKind>) -> Vec<TokenKind> {
    let mut methods: HashMap<&String, &Vec<TokenKind>> = HashMap::new();

    for token in program {
        match token {
            TokenKind::Integer(i) => {
                println!("push: {}", i);
                stack.push(TokenKind::Integer(*i));
            }
            TokenKind::Float(f) => {
                println!("push: {}", f);
                stack.push(TokenKind::Float(*f));
            }
            TokenKind::Bool(b) => {
                println!("push: {}", b);
                stack.push(TokenKind::Bool(*b));
            }
            TokenKind::OpAdd => {
                let top_token = stack.pop().expect("Stack is empty!"); // 1st item on stack
                let prev_token = stack.pop().expect("Stack is empty!");  // 2nd item on stack
                let res_token = math_op(top_token, prev_token, TokenKind::OpAdd);
                stack.push(res_token);
            }
            TokenKind::OpSub => {
                let top_token = stack.pop().expect("Stack is empty!"); // 1st item on stack
                let prev_token = stack.pop().expect("Stack is empty!");  // 2nd item on stack
                let res_token = math_op(top_token, prev_token, TokenKind::OpSub);
                stack.push(res_token);
            }
            TokenKind::OpMul => {
                let top_token = stack.pop().expect("Stack is empty!"); // 1st item on stack
                let prev_token = stack.pop().expect("Stack is empty!");  // 2nd item on stack
                let res_token = math_op(top_token, prev_token, TokenKind::OpMul);
                stack.push(res_token);
            }
            TokenKind::OpDiv => {
                let top_token = stack.pop().expect("Stack is empty!"); // 1st item on stack
                let prev_token = stack.pop().expect("Stack is empty!");  // 2nd item on stack
                let res_token = math_op(top_token, prev_token, TokenKind::OpDiv);
                stack.push(res_token);
            }
            TokenKind::OpSwap => {
                let a = stack.pop().expect("Stack is empty!");
                let b = stack.pop().expect("Stack is empty!");
                stack.push(a);
                stack.push(b);
            }
            TokenKind::OpDup => {
                let a = stack.pop().expect("Stack is empty!");
                stack.push(a.clone());
                stack.push(a);
            }
            TokenKind::OpDrop => {
                stack.pop().expect("Stack is empty!");
            }
            TokenKind::OpPeek => {
                let a = stack.pop().expect("Stack is empty!");
                stack.push(a);
            }
            TokenKind::OpDump => {
                let a = stack.pop().expect("Stack is empty!");
                match a {
                    TokenKind::Integer(_) => println!("dump: {}", a.get_as_integer().expect("Integer expected")),
                    TokenKind::Float(_) => println!("dump: {}", a.get_as_float().expect("Integer expected")),
                    TokenKind::Bool(_) => println!("dump: {}", a.get_as_bool().expect("Integer expected")),
                    _ => panic!("Illegal operand for 'dupm'. Integer/Float/Bool expected.")
                }
            }
            TokenKind::Proc {  name, proc} => {
                methods.insert(name, proc);
            }
            TokenKind::Call(method_name) => {
                let program: &Vec<TokenKind> = methods.get(method_name).expect("No such method!");
                println!("call {}:", &method_name);
                run_program(program, stack);
            }
            TokenKind::OpEquals => {
                let top = stack.pop().expect("Stack is empty!");
                let prev = stack.pop().expect("Stack is empty!");

                if top == prev {
                    stack.push(TokenKind::Bool(true));
                } else {
                    stack.push(TokenKind::Bool(false));
                }
            }
            TokenKind::OpNotEquals => {
                let top = stack.pop().expect("Stack is empty!");
                let prev = stack.pop().expect("Stack is empty!");

                if top != prev {
                    stack.push(TokenKind::Bool(true));
                } else {
                    stack.push(TokenKind::Bool(false));
                }
            }
            TokenKind::OpLess => {
                let top = stack.pop().expect("Stack is empty!");
                let prev = stack.pop().expect("Stack is empty!");

                if top < prev {
                    stack.push(TokenKind::Bool(true));
                } else {
                    stack.push(TokenKind::Bool(false));
                }
            }
            TokenKind::OpLessOrEquals => {
                let top = stack.pop().expect("Stack is empty!");
                let prev = stack.pop().expect("Stack is empty!");

                if top <= prev {
                    stack.push(TokenKind::Bool(true));
                } else {
                    stack.push(TokenKind::Bool(false));
                }
            }
            TokenKind::OpGreater => {
                let top = stack.pop().expect("Stack is empty!");
                let prev = stack.pop().expect("Stack is empty!");

                if top > prev {
                    stack.push(TokenKind::Bool(true));
                } else {
                    stack.push(TokenKind::Bool(false));
                }
            }
            TokenKind::OpGreaterOrEquals => {
                let top = stack.pop().expect("Stack is empty!");
                let prev = stack.pop().expect("Stack is empty!");

                if top >= prev {
                    stack.push(TokenKind::Bool(true));
                } else {
                    stack.push(TokenKind::Bool(false));
                }
            }
        }
    }

    stack.to_vec()
}
