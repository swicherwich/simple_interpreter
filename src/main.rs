pub mod lexer {

    pub enum TokenKind {
        Integer(i32),
        OpPlus,
        OpDump,
        Method(String, Vec<TokenKind>),
        Return,
        Call(String),
    }
}

use std::collections::HashMap;

use lexer::TokenKind;

fn execute_method(
    program: &Vec<TokenKind>, 
    main_stack: &mut Vec<i32>, 
    methods: &mut HashMap<&String, &Vec<TokenKind>>
) {
    let mut method_stack: Vec<i32> = Vec::new();

    for token in program {
        match token {
            TokenKind::Integer(int) => {
                println!("    push: {}", int);
                method_stack.push(*int);
            }
            TokenKind::OpPlus => {
                let a: i32 = method_stack.pop()
                    .expect("Stack is empty!");
                println!("    pop:  {}", &a);
                let b: i32 = method_stack.pop()
                    .expect("Stack is empty!");
                println!("    pop:  {}", &b);
                let res: i32 = a + b;
                println!("    plus");
                method_stack.push(res);
                println!("    push: {}", res);
            },
            TokenKind::OpDump => {
                let a: i32 = method_stack.pop()
                    .expect("Stack is empty!");
                println!("    pop:  {}", &a);
                println!("    dump: {}", a);
            },
            TokenKind::Call(method_name) => {
                let program: &Vec<TokenKind> = methods.get(method_name)
                    .expect("No such method!");
                execute_method(program, main_stack, methods);
            },
            TokenKind::Return => {
                let a = method_stack.pop().expect("Method stack is empty!");
                main_stack.push(a);
            }
            TokenKind::Method(_, _) => println!("Method declaration scope not allowed!"),
            _ => continue
        }
    }
}

fn run_program(program: &Vec<TokenKind>) {
    let mut stack: Vec<i32> = Vec::new();
    let mut mathods: HashMap<&String, &Vec<TokenKind>> = HashMap::new();

    for token in program {
        match token {
            TokenKind::Integer(int) => {
                println!("push: {}", int);
                stack.push(*int);
            }
            TokenKind::OpPlus => {
                let a: i32 = stack.pop().expect("Stack is empty!");
                println!("pop:  {}", &a);
                let b: i32 = stack.pop().expect("Stack is empty!");
                println!("pop:  {}", &b);
                let res: i32 = a + b;
                println!("plus");
                stack.push(res);
                println!("push: {}", res);
            },
            TokenKind::OpDump => {
                let a: i32 = stack.pop().expect("Stack is empty!");
                println!("pop:  {}", &a);
                println!("dump: {}", a);
            },
            TokenKind::Method(method_name, program) => {
                mathods.insert(method_name, program);
            },
            TokenKind::Call(method_name) => {
                let program: &Vec<TokenKind> = mathods.get(method_name).expect("No such method!");
                println!("{}:", &method_name);
                execute_method(program, &mut stack, &mut mathods);
            },
            _ => continue
        }
    }
}

fn main() {
    let program: Vec<TokenKind> = vec![
        TokenKind::Method(String::from("sum"), vec![
            TokenKind::Integer(1),
            TokenKind::Integer(2),
            TokenKind::OpPlus,
            TokenKind::Return
        ]),
        TokenKind::Integer(2),
        TokenKind::Call(String::from("sum")),
        TokenKind::OpPlus,
        TokenKind::Integer(2),
        TokenKind::OpPlus,
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

    run_program(&program);
}