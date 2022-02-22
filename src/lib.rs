extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

pub mod lexer;
pub mod parser;
pub mod generator;

#[wasm_bindgen]
pub fn compile(str: &str) -> String {
    let mut parser = parser::Parser::new(str);
    let ast = parser.parse_all().unwrap();
    let mut gen = generator::Generator::new(ast);
    let code = gen.generate_fre();
    println!("{:#?}", code);
    return code;
}