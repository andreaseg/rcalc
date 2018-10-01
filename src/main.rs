extern crate regex;

mod scanner;
mod parser;
mod eval;

use std::io;
use std::io::prelude::*;


fn main() {

    loop {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(l) => {
                    if l=="exit" {return}
                    let t = scanner::tokenize(l);
                    let ast = parser::parse(t);
                    let result = eval::eval(ast);
                    println!("{:?}", result);
                    },
                Err(_) => panic!("IO error")
            }
        }
    }
    
}
