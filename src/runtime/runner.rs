use std::fs;

use crate::frontend::parser::Parser;

pub struct Runner {
    
}

impl Runner {
    pub fn run(file: &str) {
        let content = fs::read_to_string(file);
        match content {
            Ok(source) => {
                let mut parser = Parser::new(&source);
                let ast = parser.produce_ast();
                println!("{:?}", ast)
            },
            Err(err) => println!("An error ocurred on read file {}", err)
        }
    }
}