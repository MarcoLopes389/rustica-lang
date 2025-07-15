use std::fs;
use crate::runtime::interpreter::interpret;

pub struct Runner {
    
}

impl Runner {
    pub fn run(file: &str) {
        let content = fs::read_to_string(file);
        match content {
            Ok(source) => {
                let result = interpret(source.as_str());
                println!("{:?}", result)
            },
            Err(err) => println!("An error ocurred on read file {}", err)
        }
    }
}