use crate::lexer::TokenKind;
use std::collections::HashMap;

pub fn run_program(program: &Vec<TokenKind>, stack: &mut Vec<i32>) -> Vec<i32> {
    let mut methods: HashMap<&String, &Vec<TokenKind>> = HashMap::new();

    for token in program {
        match token {
            TokenKind::Integer(int) => {
                println!("push: {}", int);
                stack.push(*int);
            }
            TokenKind::OpAdd => {
                let a: i32 = stack.pop().expect("Stack is empty!");
                println!("pop:  {}", a);
                let b: i32 = stack.pop().expect("Stack is empty!");
                println!("pop:  {}", b);
                let res: i32 = a + b;
                println!("add {} {}", a, b);
                stack.push(res);
                println!("push: {}", res);
            }
            TokenKind::OpSub => {
                let a: i32 = stack.pop().expect("Stack is empty!");
                println!("pop:  {}", a);
                let b: i32 = stack.pop().expect("Stack is empty!");
                println!("pop:  {}", b);
                let res: i32 = a - b;
                println!("sub {} {}", a, b);
                stack.push(res);
                println!("push: {}", res);
            }
            TokenKind::OpMul => {
                let a: i32 = stack.pop().expect("Stack is empty!");
                println!("pop:  {}", a);
                let b: i32 = stack.pop().expect("Stack is empty!");
                println!("pop:  {}", b);
                let res: i32 = a * b;
                println!("mul {} {}", a, b);
                stack.push(res);
                println!("push: {}", res);
            }
            TokenKind::OpDiv => {
                let a: i32 = stack.pop().expect("Stack is empty!");
                println!("pop:  {}", a);
                let b: i32 = stack.pop().expect("Stack is empty!");
                println!("pop:  {}", b);
                let res: i32 = a / b;
                stack.push(res);
                println!("push: {}", res);
            }
            TokenKind::OpSwap => {
                let a: i32 = stack.pop().expect("Stack is empty!");
                println!("pop:  {}", a);
                let b: i32 = stack.pop().expect("Stack is empty!");
                println!("pop:  {}", b);
                println!("swap {} {}", a, b);
                stack.push(a);
                stack.push(b);
                println!("push: {}", a);
                println!("push: {}", b);
            }
            TokenKind::OpDup => {
                let a: i32 = stack.pop().expect("Stack is empty!");
                println!("pop:  {}", a);
                println!("push: {}", a);
                stack.push(a);
                println!("push: {}", a);
                stack.push(a);
            }
            TokenKind::OpDrop => {
                let a: i32 = stack.pop().expect("Stack is empty!");
                println!("pop:  {}", a);
            }
            TokenKind::OpPeek => {
                let a: i32 = stack.pop().expect("Stack is empty!");
                println!("pop:  {}", a);
                println!("{}", a);
                stack.push(a);
                println!("push: {}", a);
            }
            TokenKind::OpDump => {
                let a: i32 = stack.pop().expect("Stack is empty!");
                println!("pop:  {}", a);
                println!("dump: {}", a);
            }
            TokenKind::Proc {  name, proc} => {
                methods.insert(name, proc);
            }
            TokenKind::Call(method_name) => {
                let program: &Vec<TokenKind> = methods.get(method_name).expect("No such method!");
                println!("{}:", &method_name);
                run_program(program, stack);
            }
        }
    }

    stack.to_owned()
}
