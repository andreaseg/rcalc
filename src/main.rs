extern crate regex;

mod eval;
mod parser;
mod scanner;

use std::io;
use std::io::prelude::*;
use std::panic;

fn main() {
    loop {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(line) => {
                    if line == "exit" {
                        return;
                    }
                    let tokens = match panic::catch_unwind(|| scanner::tokenize(&line)) {
                        Ok(ok) => ok,
                        Err(e) => {
                            print!("Error parsing tokens: {:?}", e);
                            continue;
                        }
                    };
                    let ast = match panic::catch_unwind(|| parser::parse(tokens)) {
                        Ok(ok) => ok,
                        Err(e) => {
                            print!("Error parsing AST: {:?}", e);
                            continue;
                        }
                    };
                    let result = match panic::catch_unwind(|| eval::eval(ast)) {
                        Ok(ok) => ok,
                        Err(e) => {
                            print!("Error parsing AST: {:?}", e);
                            continue;
                        }
                    };
                    println!("{:?}", result);
                }
                Err(e) => panic!("IO error {}", e),
            }
        }
    }
}
