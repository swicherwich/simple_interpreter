use crate::lexer::TokenKind;
use std::collections::HashMap;


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
                let prev_token = stack.pop().expect("Stack is empty!"); // 2nd item on stack
                let top_token = stack.pop().expect("Stack is empty!"); // 1st item on stack
                match top_token {
                    // if 1st item is Float we can cast 2nd item and add
                    TokenKind::Float(top) => {
                        let res = top + prev_token.get_as_float().expect("Float type expected!");
                        stack.push(TokenKind::Float(res));
                    }
                    // if 1st item is integer we can add only if 2nd is integer too otherwise panic
                    TokenKind::Integer(top) => {
                        match prev_token {
                            // check if 2nd item is Float (is there need to panic)
                            TokenKind::Float(_) => {
                                panic!("Cannot automatically cast Float to Integer. Element of type Integer expected!")
                            }
                            _ => { }
                        }
                        let res = top + prev_token.get_as_integer().expect("Float type expected!");
                        stack.push(TokenKind::Integer(res));
                    }
                    // if 1st item is not Float neither Integer
                    _ => panic!("Type mismatch: integer or float expected!")
                }
            }
            TokenKind::OpSub => {
                let prev_token = stack.pop().expect("Stack is empty!"); // 2nd item on stack
                let top_token = stack.pop().expect("Stack is empty!"); // 1st item on stack
                match top_token {
                    // if 1st item is Float we can cast 2nd item and add
                    TokenKind::Float(top) => {
                        let res = top - prev_token.get_as_float().expect("Float type expected!");
                        stack.push(TokenKind::Float(res));
                    }
                    // if 1st item is integer we can add only if 2nd is integer too otherwise panic
                    TokenKind::Integer(top) => {
                        match prev_token {
                            // check if 2nd item is Float (is there need to panic)
                            TokenKind::Float(_) => {
                                panic!("Cannot automatically cast Float to Integer. Element of type Integer expected!")
                            }
                            _ => { }
                        }
                        let res = top - prev_token.get_as_integer().expect("Float type expected!");
                        stack.push(TokenKind::Integer(res));
                    }
                    // if 1st item is not Float neither Integer
                    _ => panic!("Type mismatch: integer or float expected!")
                }
            }
            TokenKind::OpMul => {
                let prev_token = stack.pop().expect("Stack is empty!"); // 2nd item on stack
                let top_token = stack.pop().expect("Stack is empty!"); // 1st item on stack
                match top_token {
                    // if 1st item is Float we can cast 2nd item and add
                    TokenKind::Float(top) => {
                        let res = top * prev_token.get_as_float().expect("Float type expected!");
                        stack.push(TokenKind::Float(res));
                    }
                    // if 1st item is integer we can add only if 2nd is integer too otherwise panic
                    TokenKind::Integer(top) => {
                        match prev_token {
                            // check if 2nd item is Float (is there need to panic)
                            TokenKind::Float(_) => {
                                panic!("Cannot automatically cast Float to Integer. Element of type Integer expected!")
                            }
                            _ => { }
                        }
                        let res = top * prev_token.get_as_integer().expect("Float type expected!");
                        stack.push(TokenKind::Integer(res));
                    }
                    // if 1st item is not Float neither Integer
                    _ => panic!("Type mismatch: integer or float expected!")
                }
            }
            TokenKind::OpDiv => {
                let prev_token = stack.pop().expect("Stack is empty!"); // 2nd item on stack
                let top_token = stack.pop().expect("Stack is empty!"); // 1st item on stack
                match top_token {
                    // if 1st item is Float we can cast 2nd item and add
                    TokenKind::Float(top) => {
                        let res = top / prev_token.get_as_float().expect("Float type expected!");
                        stack.push(TokenKind::Float(res));
                    }
                    // if 1st item is integer we can add only if 2nd is integer too otherwise panic
                    TokenKind::Integer(top) => {
                        match prev_token {
                            // check if 2nd item is Float (is there need to panic)
                            TokenKind::Float(_) => {
                                panic!("Cannot automatically cast Float to Integer. Element of type Integer expected!")
                            }
                            _ => { }
                        }
                        let res = top / prev_token.get_as_integer().expect("Float type expected!");
                        stack.push(TokenKind::Integer(res));
                    }
                    // if 1st item is not Float neither Integer
                    _ => panic!("Type mismatch: integer or float expected!")
                }
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
        }
    }

    stack.to_vec()
}
