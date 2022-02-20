pub mod lexer;

fn main(){
    let mut lexer = lexer::Lexer::new("<view>123</view>\n".to_string());
    lexer.tokenize_all();
}