use crate::{frontend::{lexer::Lexer, parser::Parser}, runtime::interpreter::Interpreter};
use std::{io::{stdin, stdout, Write}, process::exit};

pub struct Repl {}

impl Repl {
    pub fn run() {
        println!("Repl v1.0");

        let mut history: Vec<String> = vec![];
        let stdin = stdin();
        let mut stdout = stdout();

        loop {
            print!("> ");
            let mut source = String::new();

            let _ = stdout.flush();
            stdin.read_line(&mut source).expect("Invalid code");

            if source == "exit\n" {
                exit(0)
            }
    
            history.push(source.clone());

            let result = Interpreter::new(&source).evaluate();
            println!("{}", result.value)
        }
    }
}