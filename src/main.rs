pub mod lexer;
pub mod parser;

fn main() {
    let mut parser = parser::Parser::new("<html>
    <head>
    <meta charset=\"utf-8\">
    <title>菜鸟教程(runoob.com)</title>
    <!-- 在此处写注释 -->
    </head>
    <body>
        <h1>我的第一个标题</h1>
        <p>我的第一个段落。</p>
    </body>
    </html>");
    let res = parser.parse_all();
    println!("{:#?}", res);
}
