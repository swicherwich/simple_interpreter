pub mod lexer {

    pub enum TokenKind {
        Integer(i32),
        OpPlus,
        OpDump,
        Method{name: String, proc: Vec<TokenKind>},
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
    for token in program {
        match token {
            TokenKind::Integer(int) => {
                println!("    push: {}", int);
                main_stack.push(*int);
            }
            TokenKind::OpPlus => {
                let a: i32 = main_stack.pop()
                    .expect("Stack is empty!");
                println!("    pop:  {}", &a);
                let b: i32 = main_stack.pop()
                    .expect("Stack is empty!");
                println!("    pop:  {}", &b);
                let res: i32 = a + b;
                println!("    plus");
                main_stack.push(res);
                println!("    push: {}", res);
            },
            TokenKind::OpDump => {
                let a: i32 = main_stack.pop()
                    .expect("Stack is empty!");
                println!("    pop:  {}", &a);
                println!("    dump: {}", a);
            },
            TokenKind::Call(method_name) => {
                let program: &Vec<TokenKind> = methods.get(method_name)
                    .expect("No such method!");
                execute_method(program, main_stack, methods);
            },
            TokenKind::Method { .. } => println!("Method declaration scope not allowed!"),
            _ => continue
        }
    }
}

fn run_program(program: &Vec<TokenKind>) {
    let mut stack: Vec<i32> = Vec::new();
    let mut methods: HashMap<&String, &Vec<TokenKind>> = HashMap::new();

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
            TokenKind::Method {  name, proc} => {
                methods.insert(name, proc);
            },
            TokenKind::Call(method_name) => {
                let program: &Vec<TokenKind> = methods.get(method_name).expect("No such method!");
                println!("{}:", &method_name);
                execute_method(program, &mut stack, &mut methods);
            },
            _ => continue
        }
    }
}

fn main() {
    let program: Vec<TokenKind> = vec![
        TokenKind::Method {
            name: String::from("sum"),
            proc: vec![
                TokenKind::Integer(1),
                TokenKind::Integer(2),
                TokenKind::OpPlus
        ]},
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