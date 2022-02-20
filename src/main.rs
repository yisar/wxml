pub mod lexer;

fn main(){
    let lexer = lexer::Lexer::new("<view>123</view>".to_string());
    println!("{}",lexer.code);
}