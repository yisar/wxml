pub mod lexer;
pub mod parser;

fn main() {
    let mut parser = parser::Parser::new("<view class=\"abc\"  aaa=\"{{aaa}}\"><text/><view>222</view>123</view>");
    let res = parser.parse_all();
    println!("{:#?}", res);
}
