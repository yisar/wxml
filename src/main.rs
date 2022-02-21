pub mod lexer;
pub mod parser;

fn main() {
    let mut parser = parser::Parser::new("<html><head><meta charset=\"utf-8\"><title>abc</title></head><body><h1>456</h1><p>123</p></body></html>");
    let res = parser.parse_all();
    println!("{:#?}", res);
}
