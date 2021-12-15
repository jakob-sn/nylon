extern crate lalrpop_util;

use std::env;
use parser::Parser;

#[allow(dead_code)]
mod ast;

#[allow(dead_code)]
mod parser;
mod visitor;

fn main() {
    if let Some(filepath) = env::args().nth(1) {
        let mut parser = Parser::new_from_file(filepath).expect("Could not open file.");

        let ast = parser.parse();

    } else {
        eprintln!("No file given!");
    }



    println!("Hello, world!");
}