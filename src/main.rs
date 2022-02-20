pub mod lexer;

fn main(){
    let mut lexer = lexer::Lexer::new("<view><text/>123</view>".to_string());
    lexer.tokenize_all();
    println!("{:#?}",lexer.buf);
}