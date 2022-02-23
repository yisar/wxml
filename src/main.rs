pub mod generator;
pub mod lexer;
pub mod parser;

fn main() {
  let mut parser = parser::Parser::new(
    "<text>
      <text wx:if=\"{{aaa}}\" wx:for=\"{{bbb}}\"></text>
      <text wx:else=\"{{aaa}}\"></text>
      <text wx:if=\"{{aaa}}\"></text>
      <text wx:elseif=\"{{aaa}}\"></text>
      <text wx:else=\"{{aaa}}\"></text>
    </text>",
  );
  let res = parser.parse_all();
  match res {
    Ok(ast) => {
      let mut gen = generator::Generator::new(ast);
      let code = gen.generate_fre();
      println!("{:#?}", code)
    }
    Err(_) => {}
  }
}
